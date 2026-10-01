use std::collections::HashMap;
use std::sync::Mutex;

use crate::histogram::Histogram;

/// Named histograms shared across threads. Every histogram uses the same exponential layout:
/// 16 buckets from 100 upwards, doubling each time.
#[derive(Default)]
pub struct Registry {
    histograms: Mutex<HashMap<String, Histogram>>,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn observe(&self, name: &str, value: u64) {
        let mut map = self.histograms.lock().expect("registry lock poisoned");
        map.entry(name.to_string())
            .or_insert_with(|| Histogram::exponential(100, 2.0, 16))
            .record(value);
    }

    /// A copy of every histogram, sorted by name.
    pub fn snapshot(&self) -> Vec<(String, Histogram)> {
        let map = self.histograms.lock().expect("registry lock poisoned");
        let mut out: Vec<_> = map.iter().map(|(name, h)| (name.clone(), h.clone())).collect();
        out.sort_by(|a, b| a.0.cmp(&b.0));
        out
    }
}
