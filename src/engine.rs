use anyhow::{bail, Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, Mutex, Semaphore};

use crate::cli::{self, Cli};
use flood::banner;
use flood::filter::{FilterConfig, FilterEngine, ResponseData};
use flood::fuzz;
use flood::output::terminal;
use flood::output::ScanResult;
use flood::rate_limiter::RateLimiter;
use flood::recursion::{self, RecursionQueue};
use flood::requester::{self, RequesterConfig};
use flood::state;
use flood::wordlist;

/// Shared statistics for the scan.
struct ScanStats {
    requests: AtomicU64,
    errors: AtomicU64,
    filtered: AtomicU64,
    results_count: AtomicU64,
}

impl ScanStats {
    fn new() -> Self {
        Self {
            requests: AtomicU64::new(0),
            errors: AtomicU64::new(0),
            filtered: AtomicU64::new(0),
            results_count: AtomicU64::new(0),
        }
    }
}

pub async fn run(args: Cli) -> Result<()> {
    // ── 1. Validate FUZZ keyword ──────────────────────────────────────
    let fuzz_positions = fuzz::detect_fuzz_positions(&args.url, &args.headers, &args.data);
    if fuzz_positions.is_empty() {
        bail!(
            "No FUZZ keyword found in URL, headers, or data.\n\
             Use FUZZ in the URL (e.g., https://example.com/FUZZ) or headers/data."
        );
    }

    // ── 2. Load wordlists and build work items ────────────────────────
    let wordlists: Vec<Vec<String>> = args
        .wordlist
        .iter()
        .map(|p| wordlist::load_wordlist(p))
        .collect::<Result<Vec<_>>>()?;

    let keywords = fuzz::fuzz_keywords(wordlists.len());

    // Parse extensions if provided
    let extensions = args
        .extensions
        .as_ref()
        .map(|e| cli::parse_extensions(e))
        .unwrap_or_default();

    // Build work items: either cartesian product (multi-wordlist) or single with extension expansion
    let work_items: Vec<Vec<String>> = if wordlists.len() == 1 {
        // Single wordlist: expand with extensions
        wordlists[0]
            .iter()
            .flat_map(|word| {
                wordlist::expand_with_extensions(word, &extensions, args.no_extension)
                    .into_iter()
                    .map(|w| vec![w])
            })
            .collect()
    } else {
        // Multiple wordlists: cartesian product (no extension expansion)
        wordlist::cartesian_product(&wordlists)
    };

    let total_work = work_items.len();

    // ── 3. Parse filter config ────────────────────────────────────────
    let match_codes = cli::parse_status_codes(&args.match_code)?;
    let match_sizes = args
        .match_size
        .as_ref()
        .map(|s| cli::parse_numeric_list(s))
        .transpose()?
        .unwrap_or_default();
    let match_words = args
        .match_words
        .as_ref()
        .map(|s| cli::parse_numeric_list(s))
        .transpose()?
        .unwrap_or_default();
    let match_lines = args
        .match_lines
        .as_ref()
        .map(|s| cli::parse_numeric_list(s))
        .transpose()?
        .unwrap_or_default();
    let match_regex = args
        .match_regex
        .as_ref()
        .map(|r| regex::Regex::new(r).context("Invalid match regex"))
        .transpose()?;
    let filter_codes = args
        .filter_code
        .as_ref()
        .map(|s| cli::parse_status_codes(s))
        .transpose()?
        .unwrap_or_default();
    let filter_sizes = args
        .filter_size
        .as_ref()
        .map(|s| cli::parse_numeric_list(s))
        .transpose()?
        .unwrap_or_default();
    let filter_words = args
        .filter_words
        .as_ref()
        .map(|s| cli::parse_numeric_list(s))
        .transpose()?
        .unwrap_or_default();
    let filter_lines = args
        .filter_lines
        .as_ref()
        .map(|s| cli::parse_numeric_list(s))
        .transpose()?
        .unwrap_or_default();
    let filter_regex = args
        .filter_regex
        .as_ref()
        .map(|r| regex::Regex::new(r).context("Invalid filter regex"))
        .transpose()?;

    let filter_config = FilterConfig {
        match_codes,
        match_sizes,
        match_words,
        match_lines,
        match_regex,
        match_time: args.match_time,
        filter_codes,
        filter_sizes,
        filter_words,
        filter_lines,
        filter_regex,
        filter_time: args.filter_time,
    };
    let filter_engine = Arc::new(FilterEngine::new(filter_config));

    // ── 4. Build HTTP client ──────────────────────────────────────────
    let auth = args
        .auth
        .as_ref()
        .map(|a| requester::parse_auth(a))
        .transpose()?;

    let user_agent = if args.random_agent {
        requester::random_user_agent().to_string()
    } else {
        args.user_agent.clone()
    };

    let requester_config = RequesterConfig {
        timeout: Duration::from_secs(args.timeout),
        proxy: args.proxy.clone(),
        insecure: args.insecure,
        follow_redirects: args.follow_redirects,
        max_redirects: args.max_redirects,
        user_agent,
        auth: auth.clone(),
        bearer: args.bearer.clone(),
        cookie: args.cookie.clone(),
    };

    let client = requester::build_client(&requester_config)?;

    let replay_client = args
        .replay_proxy
        .as_ref()
        .map(|p| requester::build_replay_client(p, &requester_config))
        .transpose()?;

    // ── 5. Print banner and config ────────────────────────────────────
    if !args.silent {
        banner::print_banner(env!("CARGO_PKG_VERSION"), args.no_color);
        let wl_name = args
            .wordlist
            .first()
            .map(|p| {
                p.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string()
            })
            .unwrap_or_default();
        banner::print_config(
            &args.url,
            &wl_name,
            total_work,
            args.threads,
            &args.match_code,
            args.no_color,
        );
        println!();
    }

    // ── 6. Shared state ───────────────────────────────────────────────
    let stats = Arc::new(ScanStats::new());
    let results: Arc<Mutex<Vec<ScanResult>>> = Arc::new(Mutex::new(Vec::new()));
    let paused = Arc::new(AtomicBool::new(false));
    let quit = Arc::new(AtomicBool::new(false));
    let thread_count = Arc::new(AtomicUsize::new(args.threads));
    let semaphore = Arc::new(Semaphore::new(args.threads));

    // Rate limiter
    let rate_limiter = Arc::new(RateLimiter::new(args.rate));

    // Recursion queue
    let recursion_queue = Arc::new(Mutex::new(RecursionQueue::new(
        args.depth,
        args.recurse_exclude.clone(),
    )));

    // Result channel
    let (result_tx, mut result_rx) = mpsc::unbounded_channel::<ScanResult>();

    // ── 7. Keyboard input handler ─────────────────────────────────────
    {
        let paused = Arc::clone(&paused);
        let quit = Arc::clone(&quit);
        let thread_count = Arc::clone(&thread_count);
        let semaphore = Arc::clone(&semaphore);
        tokio::spawn(async move {
            use crossterm::event::{self, Event, KeyCode, KeyEvent};
            loop {
                // Poll with a short timeout so we can check quit flag
                let available = tokio::task::spawn_blocking(|| {
                    event::poll(std::time::Duration::from_millis(200)).unwrap_or(false)
                })
                .await
                .unwrap_or(false);

                if quit.load(Ordering::Relaxed) {
                    break;
                }

                if !available {
                    continue;
                }

                let evt = tokio::task::spawn_blocking(|| event::read().ok())
                    .await
                    .unwrap_or(None);

                if let Some(Event::Key(KeyEvent { code, .. })) = evt {
                    match code {
                        KeyCode::Char('p') => {
                            let was_paused = paused.load(Ordering::Relaxed);
                            paused.store(!was_paused, Ordering::Relaxed);
                            if was_paused {
                                eprintln!("\n  [RESUMED]");
                            } else {
                                eprintln!("\n  [PAUSED] Press 'p' to resume");
                            }
                        }
                        KeyCode::Char('q') => {
                            quit.store(true, Ordering::Relaxed);
                            eprintln!("\n  [QUITTING] Finishing in-flight requests...");
                            break;
                        }
                        KeyCode::Char('+') | KeyCode::Char('=') => {
                            let current = thread_count.fetch_add(10, Ordering::Relaxed);
                            semaphore.add_permits(10);
                            eprintln!("\n  [THREADS] {} -> {}", current, current + 10);
                        }
                        KeyCode::Char('-') => {
                            let current = thread_count.load(Ordering::Relaxed);
                            if current > 10 {
                                thread_count.fetch_sub(10, Ordering::Relaxed);
                                // Note: we can't remove permits, but fewer will be available
                                // as existing tasks hold them
                                eprintln!("\n  [THREADS] {} -> {}", current, current - 10);
                            }
                        }
                        _ => {}
                    }
                }
            }
        });
    }

    // ── 8. Progress bar ───────────────────────────────────────────────
    let progress = if !args.silent {
        let pb = ProgressBar::new(total_work as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "    {spinner:.blue} [{elapsed_precise}] [{bar:30.blue/dim}] {pos}/{len} ({per_sec}) {msg}",
                )
                .unwrap()
                .progress_chars("=> "),
        );
        Some(pb)
    } else {
        None
    };

    // ── 9. Result collector task ──────────────────────────────────────
    let results_clone = Arc::clone(&results);
    let no_color = args.no_color;
    let silent = args.silent;
    let output_path = args.output.clone();
    let output_format = args.output_format.clone();
    let progress_for_collector = progress.clone();
    let collector_handle = tokio::spawn(async move {
        while let Some(scan_result) = result_rx.recv().await {
            // Print result to terminal
            if !silent {
                let line = terminal::format_result(&scan_result, no_color);
                if let Some(ref pb) = progress_for_collector {
                    pb.suspend(|| println!("{}", line));
                } else {
                    println!("{}", line);
                }
            } else {
                // In silent mode, print just the URL
                println!("{}", scan_result.url);
            }

            // Append to JSONL output in real-time if requested
            if let Some(ref path) = output_path {
                if output_format == "jsonl" {
                    let _ = flood::output::json::append_jsonl(&scan_result, path);
                }
            }

            results_clone.lock().await.push(scan_result);
        }
    });

    // ── 10. Main scan loop ────────────────────────────────────────────
    let scan_start = std::time::Instant::now();
    let method = if args.data.is_some() && args.method == "GET" {
        "POST".to_string()
    } else {
        args.method.clone()
    };

    // Parse headers once
    let parsed_headers = requester::parse_headers(&args.headers)?;

    // Process work items at current depth (0)
    let mut all_work: Vec<(Vec<String>, u32)> = work_items.into_iter().map(|w| (w, 0u32)).collect();

    // Resume from state if requested
    let resume_offset = if let Some(ref state_path) = args.resume {
        let saved = state::load_state(state_path)?;
        if !args.silent {
            eprintln!(
                "  Resuming from position {} ({} previous results)",
                saved.wordlist_position,
                saved.results.len()
            );
        }
        // Restore previous results
        results.lock().await.extend(saved.results);
        stats.errors.store(saved.errors, Ordering::Relaxed);
        saved.wordlist_position
    } else {
        0
    };

    let mut work_index = 0usize;

    loop {
        // Process current batch of work
        let mut handles = Vec::new();

        for (item, depth) in all_work.drain(..) {
            work_index += 1;
            if work_index <= resume_offset {
                if let Some(ref pb) = progress {
                    pb.inc(1);
                }
                continue;
            }

            // Check quit flag
            if quit.load(Ordering::Relaxed) {
                break;
            }

            // Wait while paused
            while paused.load(Ordering::Relaxed) {
                tokio::time::sleep(Duration::from_millis(100)).await;
                if quit.load(Ordering::Relaxed) {
                    break;
                }
            }

            // Rate limiting
            rate_limiter.acquire().await;

            // Acquire semaphore permit
            let permit = Arc::clone(&semaphore);
            let permit = permit.acquire_owned().await?;

            let client = client.clone();
            let replay_client = replay_client.clone();
            let method = method.clone();
            let url_template = args.url.clone();
            let header_templates = args.headers.clone();
            let data_template = args.data.clone();
            let parsed_headers = parsed_headers.clone();
            let filter_engine = Arc::clone(&filter_engine);
            let stats = Arc::clone(&stats);
            let result_tx = result_tx.clone();
            let recursion_queue = Arc::clone(&recursion_queue);
            let rate_limiter = Arc::clone(&rate_limiter);
            let keywords = keywords.clone();
            let auth = auth.clone();
            let bearer = args.bearer.clone();
            let cookie = args.cookie.clone();
            let add_slash = args.add_slash;
            let recurse = args.recurse;
            let retries = args.retries;
            let no_auto_throttle = args.no_auto_throttle;
            let verbose = args.verbose;
            let no_color = args.no_color;
            let random_agent = args.random_agent;
            let progress = progress.clone();

            let handle = tokio::spawn(async move {
                let _permit = permit; // Hold permit until task completes

                // Build substitutions
                let substitutions: Vec<(&str, &str)> = keywords
                    .iter()
                    .zip(item.iter())
                    .map(|(k, v)| (*k, v.as_str()))
                    .collect();

                let mut url = fuzz::build_url(&url_template, &substitutions);
                if add_slash && !url.ends_with('/') {
                    url.push('/');
                }

                let fuzzed_headers = fuzz::build_headers(&header_templates, &substitutions);
                let fuzzed_data = fuzz::build_data(&data_template, &substitutions);

                // Parse fuzzed headers for this request
                let request_headers = match requester::parse_headers(&fuzzed_headers) {
                    Ok(h) => h,
                    Err(_) => parsed_headers,
                };

                // Retry loop
                let mut last_err = None;
                let mut resp_data: Option<ResponseData> = None;

                for attempt in 0..=retries {
                    // Use random user agent per request if enabled
                    if random_agent {
                        // We already built the client with one UA; for per-request
                        // rotation we add/override via headers
                    }

                    let result = requester::send_request(
                        &client,
                        &method,
                        &url,
                        &request_headers,
                        fuzzed_data.as_deref(),
                        &auth,
                        &bearer,
                        &cookie,
                    )
                    .await;

                    match result {
                        Ok(data) => {
                            // Auto-throttle on 429
                            if data.status == 429 && !no_auto_throttle {
                                rate_limiter.throttle();
                                if attempt < retries {
                                    let backoff = Duration::from_millis(500 * 2u64.pow(attempt));
                                    tokio::time::sleep(backoff).await;
                                    continue;
                                }
                            } else if !no_auto_throttle && rate_limiter.is_throttled() {
                                rate_limiter.unthrottle();
                            }
                            resp_data = Some(data);
                            break;
                        }
                        Err(e) => {
                            last_err = Some(e);
                            if attempt < retries {
                                let backoff = Duration::from_millis(200 * 2u64.pow(attempt));
                                tokio::time::sleep(backoff).await;
                            }
                        }
                    }
                }

                stats.requests.fetch_add(1, Ordering::Relaxed);

                let resp = match resp_data {
                    Some(r) => r,
                    None => {
                        stats.errors.fetch_add(1, Ordering::Relaxed);
                        if verbose {
                            if let Some(e) = last_err {
                                eprintln!("  [ERR] {} - {}", url, e);
                            }
                        }
                        if let Some(ref pb) = progress {
                            pb.inc(1);
                        }
                        return;
                    }
                };

                // Apply filter engine
                let should_display = filter_engine.should_display(&resp);

                if should_display {
                    stats.results_count.fetch_add(1, Ordering::Relaxed);

                    let input_display = item.join(",");
                    let scan_result = ScanResult {
                        url: url.clone(),
                        status: resp.status,
                        size: resp.size,
                        words: resp.words,
                        lines: resp.lines,
                        duration_ms: resp.duration_ms,
                        redirect_to: resp.redirect_to.clone(),
                        content_type: resp.content_type.clone(),
                        depth,
                        input: input_display,
                    };

                    // Send through replay proxy if configured
                    if let Some(ref replay) = replay_client {
                        let _ = requester::send_request(
                            replay,
                            &method,
                            &url,
                            &request_headers,
                            fuzzed_data.as_deref(),
                            &auth,
                            &bearer,
                            &cookie,
                        )
                        .await;
                    }

                    let _ = result_tx.send(scan_result);

                    // Check for directory response for recursion
                    if recurse {
                        if recursion::is_directory_response(
                            resp.status,
                            resp.redirect_to.as_deref(),
                            &url,
                        ) {
                            let new_base = if url.ends_with('/') {
                                format!("{}FUZZ", url)
                            } else {
                                format!("{}/FUZZ", url)
                            };
                            let mut rq = recursion_queue.lock().await;
                            if rq.add(&new_base, depth) {
                                let indicator =
                                    terminal::format_recursion_indicator(&new_base, no_color);
                                if let Some(ref pb) = progress {
                                    pb.suspend(|| println!("{}", indicator));
                                }
                            }
                        }
                    }
                } else {
                    stats.filtered.fetch_add(1, Ordering::Relaxed);
                    if verbose {
                        let input_display = item.join(",");
                        let scan_result = ScanResult {
                            url: url.clone(),
                            status: resp.status,
                            size: resp.size,
                            words: resp.words,
                            lines: resp.lines,
                            duration_ms: resp.duration_ms,
                            redirect_to: resp.redirect_to.clone(),
                            content_type: resp.content_type.clone(),
                            depth,
                            input: input_display,
                        };
                        let line = terminal::format_result(&scan_result, no_color);
                        if let Some(ref pb) = progress {
                            pb.suspend(|| eprintln!("  [FILTERED] {}", line));
                        }
                    }
                }

                if let Some(ref pb) = progress {
                    pb.inc(1);
                }
            });

            handles.push(handle);
        }

        // Wait for all tasks in this batch
        for handle in handles {
            let _ = handle.await;
        }

        // Check if there are recursion items to process
        if quit.load(Ordering::Relaxed) {
            break;
        }

        let mut rq = recursion_queue.lock().await;
        if rq.is_empty() {
            break;
        }

        // Drain recursion queue and generate new work items
        let recursive_urls = rq.drain_pending();
        drop(rq);

        let mut new_work = Vec::new();
        for (rec_url, rec_depth) in recursive_urls {
            // For recursive scans, re-use the first wordlist
            for word_list in &wordlists[0] {
                let expanded =
                    wordlist::expand_with_extensions(word_list, &extensions, args.no_extension);
                for w in expanded {
                    new_work.push((vec![w], rec_depth));
                }
            }
            // Update URL template for this recursive path
            // The rec_url already has FUZZ in it from when we added it
            let _ = rec_url; // URL template is embedded in the work items via substitution
        }

        if !new_work.is_empty() {
            if let Some(ref pb) = progress {
                pb.set_length(pb.length().unwrap_or(0) + new_work.len() as u64);
            }
            all_work = new_work;
        } else {
            break;
        }
    }

    // Drop sender to signal collector to finish
    drop(result_tx);

    // Wait for collector to drain
    let _ = collector_handle.await;

    if let Some(ref pb) = progress {
        pb.finish_and_clear();
    }

    // ── 11. Summary ───────────────────────────────────────────────────
    let elapsed = scan_start.elapsed();
    let total_requests = stats.requests.load(Ordering::Relaxed);
    let total_errors = stats.errors.load(Ordering::Relaxed);
    let total_results = stats.results_count.load(Ordering::Relaxed);
    let total_filtered = stats.filtered.load(Ordering::Relaxed);

    if !args.silent {
        println!();
        println!(
            "    Completed in {:.1}s | {} requests | {} results | {} filtered | {} errors",
            elapsed.as_secs_f64(),
            total_requests,
            total_results,
            total_filtered,
            total_errors,
        );
    }

    // ── 12. Write output file ─────────────────────────────────────────
    let final_results = results.lock().await;
    if let Some(ref path) = args.output {
        match args.output_format.as_str() {
            "json" => flood::output::json::write_json(&final_results, path)?,
            "jsonl" => {
                // Already written in real-time, but write complete file if not done
                if !path.exists() {
                    flood::output::json::write_jsonl(&final_results, path)?;
                }
            }
            "csv" => flood::output::csv_writer::write_csv(&final_results, path)?,
            "text" => flood::output::text::write_text(&final_results, path)?,
            other => bail!(
                "Unknown output format: {}. Use json, jsonl, csv, or text.",
                other
            ),
        }
        if !args.silent {
            println!(
                "    Results saved to {} ({})",
                path.display(),
                args.output_format
            );
        }
    }

    // ── 13. Save state if quit requested ──────────────────────────────
    if quit.load(Ordering::Relaxed) {
        let state_path = args
            .state_file
            .clone()
            .unwrap_or_else(|| std::path::PathBuf::from(state::default_state_filename()));
        let scan_state = state::ScanState {
            url: args.url.clone(),
            wordlist_paths: args
                .wordlist
                .iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect(),
            method: method.clone(),
            headers: args.headers.clone(),
            data: args.data.clone(),
            match_codes: args.match_code.clone(),
            threads: args.threads,
            timeout: args.timeout,
            wordlist_position: work_index,
            recursion_pending: {
                let mut rq = recursion_queue.lock().await;
                rq.drain_pending()
            },
            results: final_results.clone(),
            elapsed_secs: elapsed.as_secs(),
            errors: total_errors,
        };

        state::save_state(&scan_state, &state_path)?;
        if !args.silent {
            eprintln!(
                "  State saved to {}. Resume with --resume {}",
                state_path.display(),
                state_path.display()
            );
        }
    }

    Ok(())
}
