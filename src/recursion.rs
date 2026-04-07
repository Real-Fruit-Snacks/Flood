use std::collections::{HashSet, VecDeque};

pub fn is_directory_response(status: u16, redirect_location: Option<&str>, path: &str) -> bool {
    if matches!(status, 301 | 302) {
        if let Some(loc) = redirect_location {
            return loc.ends_with('/');
        }
    }
    if status == 200 && path.ends_with('/') {
        return true;
    }
    false
}

pub struct RecursionQueue {
    max_depth: u32,
    exclude_patterns: Vec<String>,
    seen: HashSet<String>,
    queue: VecDeque<(String, u32)>,
}

impl RecursionQueue {
    pub fn new(max_depth: u32, exclude_patterns: Vec<String>) -> Self {
        Self {
            max_depth,
            exclude_patterns,
            seen: HashSet::new(),
            queue: VecDeque::new(),
        }
    }

    pub fn add(&mut self, path: &str, current_depth: u32) -> bool {
        if current_depth >= self.max_depth {
            return false;
        }
        for pattern in &self.exclude_patterns {
            if path.contains(pattern.as_str()) {
                return false;
            }
        }
        if !self.seen.insert(path.to_string()) {
            return false;
        }
        self.queue.push_back((path.to_string(), current_depth + 1));
        true
    }

    pub fn pop(&mut self) -> Option<(String, u32)> {
        self.queue.pop_front()
    }
    pub fn len(&self) -> usize {
        self.queue.len()
    }
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
    pub fn total_seen(&self) -> usize {
        self.seen.len()
    }
    pub fn drain_pending(&mut self) -> Vec<(String, u32)> {
        self.queue.drain(..).collect()
    }
}
