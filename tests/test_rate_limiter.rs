use flood::rate_limiter::RateLimiter;
use std::time::{Duration, Instant};

#[tokio::test]
async fn test_unlimited_rate() {
    let limiter = RateLimiter::new(0);
    let start = Instant::now();
    for _ in 0..100 {
        limiter.acquire().await;
    }
    assert!(start.elapsed() < Duration::from_millis(100));
}

#[tokio::test]
async fn test_rate_limited() {
    let limiter = RateLimiter::new(100);
    let start = Instant::now();
    for _ in 0..10 {
        limiter.acquire().await;
    }
    let elapsed = start.elapsed();
    assert!(elapsed >= Duration::from_millis(50));
    assert!(elapsed < Duration::from_millis(500));
}

#[tokio::test]
async fn test_throttle_reduces_rate() {
    let limiter = RateLimiter::new(1000);
    limiter.throttle();
    assert!(limiter.effective_rate() <= 500);
}

#[tokio::test]
async fn test_unthrottle_restores_rate() {
    let limiter = RateLimiter::new(1000);
    limiter.throttle();
    assert!(limiter.effective_rate() <= 500);
    limiter.unthrottle();
    assert!(limiter.effective_rate() > 500);
}

#[tokio::test]
async fn test_throttle_minimum_rate() {
    let limiter = RateLimiter::new(10);
    for _ in 0..20 {
        limiter.throttle();
    }
    assert!(limiter.effective_rate() >= 1);
}
