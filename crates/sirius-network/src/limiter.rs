//! Per-IP connection rate-limiter.
//!
//! Tracks how many new connections each IP address has opened within a
//! sliding window. Connections that arrive faster than the configured limit
//! are rejected before a socket is handed to the connection task.
//!
//! The implementation is intentionally simple: a `DashMap` of IP → token
//! bucket, cleaned up periodically. It doesn't need to be precise. A determined
//! attacker will find ways around rate limiting regardless. The goal is to shed
//! accidental thundering herds and trivial abuse, not to replace a proper
//! DDoS mitigation layer.

use dashmap::DashMap;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// State for a single IP's rate limit bucket.
#[derive(Debug)]
struct Bucket {
    /// Number of connections opened in the current window.
    count: u32,
    /// When the current window started.
    window_start: Instant,
}

impl Bucket {
    fn new() -> Self {
        Self {
            count: 1,
            window_start: Instant::now(),
        }
    }
}

/// Per-IP rate limiter.
#[derive(Debug, Clone)]
pub struct RateLimiter {
    inner: Arc<RateLimiterInner>,
}

#[derive(Debug)]
struct RateLimiterInner {
    buckets: DashMap<IpAddr, Bucket>,
    max_per_sec: u32,
    window: Duration,
}

impl RateLimiter {
    /// Creates a new rate limiter.
    ///
    /// `max_per_sec` is maximum new connections allowed from a single IP
    /// per one-second window before further connections are rejected.
    pub fn new(max_per_sec: u32) -> Self {
        Self {
            inner: Arc::new(RateLimiterInner {
                buckets: DashMap::new(),
                max_per_sec,
                window: Duration::from_secs(1),
            }),
        }
    }

    /// Returns `true` if the connection should be allowed.
    ///
    /// Increments the counter for `ip`. If the counter exceeds the limit
    /// within the current window, returns `false`. Call this once per
    /// incoming connection attempt, before accepting the socket.
    pub fn check(&self, ip: IpAddr) -> bool {
        let now = Instant::now();

        match self.inner.buckets.get_mut(&ip) {
            Some(mut bucket) => {
                if now.duration_since(bucket.window_start) >= self.inner.window
                {
                    bucket.count = 1;
                    bucket.window_start = now;
                    true
                } else if bucket.count < self.inner.max_per_sec {
                    bucket.count += 1;
                    true
                } else {
                    false
                }
            }
            None => {
                self.inner.buckets.insert(ip, Bucket::new());
                true
            }
        }
    }

    /// Removes expired buckets.
    ///
    /// Call this periodically (e.g. every 60 seconds via `sirius-scheduler`)
    /// to prevent the map from growing unbounded with stale entries.
    pub fn cleanup(&self) {
        let now = Instant::now();
        self.inner.buckets.retain(|_, bucket| {
            now.duration_since(bucket.window_start) < self.inner.window * 60
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    fn ip(a: u8) -> IpAddr {
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, a))
    }

    #[test]
    fn allows_connections_within_limit() {
        let limiter = RateLimiter::new(5);
        for _ in 0..5 {
            assert!(limiter.check(ip(1)));
        }
    }

    #[test]
    fn rejects_connections_over_limit() {
        let limiter = RateLimiter::new(3);
        assert!(limiter.check(ip(2)));
        assert!(limiter.check(ip(2)));
        assert!(limiter.check(ip(2)));
        assert!(!limiter.check(ip(2)));
    }

    #[test]
    fn different_ips_are_independent() {
        let limiter = RateLimiter::new(1);
        assert!(limiter.check(ip(3)));
        assert!(!limiter.check(ip(3)));
        assert!(limiter.check(ip(4)));
    }
}
