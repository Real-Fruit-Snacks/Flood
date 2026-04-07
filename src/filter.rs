use regex::Regex;

/// Metadata extracted from an HTTP response for filtering.
#[derive(Debug, Clone)]
pub struct ResponseData {
    pub status: u16,
    pub size: u64,
    pub words: u64,
    pub lines: u64,
    pub duration_ms: u64,
    pub body: String,
    pub redirect_to: Option<String>,
    pub content_type: Option<String>,
}

/// Configuration for the two-phase match/filter engine.
#[derive(Debug, Default)]
pub struct FilterConfig {
    pub match_codes: Vec<u16>,
    pub match_sizes: Vec<u64>,
    pub match_words: Vec<u64>,
    pub match_lines: Vec<u64>,
    pub match_regex: Option<Regex>,
    pub match_time: Option<u64>,
    pub filter_codes: Vec<u16>,
    pub filter_sizes: Vec<u64>,
    pub filter_words: Vec<u64>,
    pub filter_lines: Vec<u64>,
    pub filter_regex: Option<Regex>,
    pub filter_time: Option<u64>,
}

pub struct FilterEngine {
    config: FilterConfig,
}

impl FilterEngine {
    pub fn new(config: FilterConfig) -> Self {
        Self { config }
    }

    pub fn should_display(&self, resp: &ResponseData) -> bool {
        if !self.matches(resp) {
            return false;
        }
        if self.filtered(resp) {
            return false;
        }
        true
    }

    fn matches(&self, resp: &ResponseData) -> bool {
        if !self.config.match_codes.is_empty() && !self.config.match_codes.contains(&resp.status) {
            return false;
        }
        if !self.config.match_sizes.is_empty() && !self.config.match_sizes.contains(&resp.size) {
            return false;
        }
        if !self.config.match_words.is_empty() && !self.config.match_words.contains(&resp.words) {
            return false;
        }
        if !self.config.match_lines.is_empty() && !self.config.match_lines.contains(&resp.lines) {
            return false;
        }
        if let Some(ref re) = self.config.match_regex {
            if !re.is_match(&resp.body) {
                return false;
            }
        }
        if let Some(threshold) = self.config.match_time {
            if resp.duration_ms <= threshold {
                return false;
            }
        }
        true
    }

    fn filtered(&self, resp: &ResponseData) -> bool {
        if self.config.filter_codes.contains(&resp.status) {
            return true;
        }
        if self.config.filter_sizes.contains(&resp.size) {
            return true;
        }
        if self.config.filter_words.contains(&resp.words) {
            return true;
        }
        if self.config.filter_lines.contains(&resp.lines) {
            return true;
        }
        if let Some(ref re) = self.config.filter_regex {
            if re.is_match(&resp.body) {
                return true;
            }
        }
        if let Some(threshold) = self.config.filter_time {
            if resp.duration_ms > threshold {
                return true;
            }
        }
        false
    }
}
