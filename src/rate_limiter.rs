use std::sync::atomic::{AtomicU32, Ordering};
use tokio::time::{Duration, Instant};

/// Token-bucket rate limiter with auto-throttle support.
pub struct RateLimiter {
    base_rate: u32,
    effective_rate: AtomicU32,
    last_acquire: tokio::sync::Mutex<Instant>,
}

impl RateLimiter {
    pub fn new(rate: u32) -> Self {
        Self {
            base_rate: rate,
            effective_rate: AtomicU32::new(rate),
            last_acquire: tokio::sync::Mutex::new(Instant::now()),
        }
    }

    pub async fn acquire(&self) {
        let rate = self.effective_rate.load(Ordering::Relaxed);
        if rate == 0 {
            return;
        }
        let interval = Duration::from_secs_f64(1.0 / rate as f64);
        let mut last = self.last_acquire.lock().await;
        let now = Instant::now();
        let next_allowed = *last + interval;
        if now < next_allowed {
            tokio::time::sleep(next_allowed - now).await;
        }
        *last = Instant::now();
    }

    pub fn throttle(&self) {
        let current = self.effective_rate.load(Ordering::Relaxed);
        let new_rate = (current / 2).max(1);
        self.effective_rate.store(new_rate, Ordering::Relaxed);
    }

    pub fn unthrottle(&self) {
        let current = self.effective_rate.load(Ordering::Relaxed);
        let new_rate = ((current as f64 * 1.25) as u32).min(self.base_rate);
        let new_rate = if new_rate == current && current < self.base_rate {
            current + 1
        } else {
            new_rate
        };
        self.effective_rate.store(new_rate, Ordering::Relaxed);
    }

    pub fn effective_rate(&self) -> u32 {
        self.effective_rate.load(Ordering::Relaxed)
    }

    pub fn base_rate(&self) -> u32 {
        self.base_rate
    }

    pub fn is_throttled(&self) -> bool {
        self.base_rate > 0 && self.effective_rate() < self.base_rate
    }
}
