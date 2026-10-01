use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Events per second over a sliding window.
#[derive(Debug)]
pub struct RollingRate {
    window: Duration,
    events: VecDeque<Instant>,
}

impl RollingRate {
    pub fn new(window: Duration) -> Self {
        assert!(!window.is_zero(), "window must be longer than zero");
        Self { window, events: VecDeque::new() }
    }

    pub fn mark(&mut self) {
        self.mark_at(Instant::now());
    }

    pub fn mark_at(&mut self, at: Instant) {
        self.events.push_back(at);
        self.prune(at);
    }

    /// Events per second over the full window.
    pub fn rate(&mut self) -> f64 {
        self.prune(Instant::now());
        self.events.len() as f64 / self.window.as_secs_f64()
    }

    fn prune(&mut self, now: Instant) {
        while let Some(&oldest) = self.events.front() {
            if now.duration_since(oldest) > self.window {
                self.events.pop_front();
            } else {
                break;
            }
        }
    }
}
