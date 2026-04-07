use crate::filter::ResponseData;
use anyhow::{Context, Result};
use reqwest::{Client, ClientBuilder, Method, Proxy};
use std::str::FromStr;
use std::time::Duration;

pub struct RequesterConfig {
    pub timeout: Duration,
    pub proxy: Option<String>,
    pub insecure: bool,
    pub follow_redirects: bool,
    pub max_redirects: usize,
    pub user_agent: String,
    pub auth: Option<(String, String)>,
    pub bearer: Option<String>,
    pub cookie: Option<String>,
}

pub fn build_client(config: &RequesterConfig) -> Result<Client> {
    let redirect_policy = if config.follow_redirects {
        reqwest::redirect::Policy::limited(config.max_redirects)
    } else {
        reqwest::redirect::Policy::none()
    };
    let mut builder = ClientBuilder::new()
        .timeout(config.timeout)
        .redirect(redirect_policy)
        .user_agent(&config.user_agent)
        .danger_accept_invalid_certs(config.insecure)
        .pool_max_idle_per_host(0)
        .tcp_nodelay(true);
    if let Some(ref proxy_url) = config.proxy {
        let proxy =
            Proxy::all(proxy_url).with_context(|| format!("Invalid proxy URL: {}", proxy_url))?;
        builder = builder.proxy(proxy);
    }
    builder.build().context("Failed to build HTTP client")
}

pub fn build_replay_client(proxy_url: &str, config: &RequesterConfig) -> Result<Client> {
    let proxy = Proxy::all(proxy_url)
        .with_context(|| format!("Invalid replay proxy URL: {}", proxy_url))?;
    ClientBuilder::new()
        .timeout(config.timeout)
        .redirect(reqwest::redirect::Policy::none())
        .user_agent(&config.user_agent)
        .danger_accept_invalid_certs(config.insecure)
        .proxy(proxy)
        .build()
        .context("Failed to build replay HTTP client")
}

pub async fn send_request(
    client: &Client,
    method: &str,
    url: &str,
    headers: &[(String, String)],
    data: Option<&str>,
    auth: &Option<(String, String)>,
    bearer: &Option<String>,
    cookie: &Option<String>,
) -> Result<ResponseData> {
    let method =
        Method::from_str(method).map_err(|_| anyhow::anyhow!("Invalid HTTP method: {}", method))?;
    let start = std::time::Instant::now();
    let mut req = client.request(method, url);
    for (name, value) in headers {
        req = req.header(name.as_str(), value.as_str());
    }
    if let Some((user, pass)) = auth {
        req = req.basic_auth(user, Some(pass));
    }
    if let Some(token) = bearer {
        req = req.bearer_auth(token);
    }
    if let Some(cookie_str) = cookie {
        req = req.header("Cookie", cookie_str.as_str());
    }
    if let Some(body) = data {
        req = req.body(body.to_string());
        req = req.header("Content-Type", "application/x-www-form-urlencoded");
    }
    let resp = req.send().await.context("Request failed")?;
    let status = resp.status().as_u16();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    let redirect_to = resp
        .headers()
        .get("location")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    let body = resp.text().await.unwrap_or_default();
    let duration_ms = start.elapsed().as_millis() as u64;
    let words = body.split_whitespace().count() as u64;
    let lines = body.lines().count() as u64;
    let size = body.len() as u64;
    Ok(ResponseData {
        status,
        size,
        words,
        lines,
        duration_ms,
        body,
        redirect_to,
        content_type,
    })
}

pub fn parse_headers(raw_headers: &[String]) -> Result<Vec<(String, String)>> {
    raw_headers
        .iter()
        .map(|h| {
            let parts: Vec<&str> = h.splitn(2, ':').collect();
            if parts.len() != 2 {
                anyhow::bail!(
                    "Invalid header format '{}' \u{2014} expected 'Name: Value'",
                    h
                );
            }
            Ok((parts[0].trim().to_string(), parts[1].trim().to_string()))
        })
        .collect()
}

pub fn parse_auth(auth: &str) -> Result<(String, String)> {
    let parts: Vec<&str> = auth.splitn(2, ':').collect();
    if parts.len() != 2 {
        anyhow::bail!(
            "Invalid auth format '{}' \u{2014} expected 'user:pass'",
            auth
        );
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}

pub const USER_AGENTS: &[&str] = &[
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:121.0) Gecko/20100101 Firefox/121.0",
    "Mozilla/5.0 (X11; Linux x86_64; rv:121.0) Gecko/20100101 Firefox/121.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Edge/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Safari/605.1.15",
];

pub fn random_user_agent() -> &'static str {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    USER_AGENTS[rng.gen_range(0..USER_AGENTS.len())]
}
