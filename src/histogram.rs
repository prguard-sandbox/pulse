/// A histogram with fixed, sorted upper bounds. Values above the last bound land in an
/// overflow bucket, so there is always one more count than there are bounds.
#[derive(Debug, Clone)]
pub struct Histogram {
    bounds: Vec<u64>,
    counts: Vec<u64>,
    sum: u128,
    total: u64,
}

impl Histogram {
    pub fn new(mut bounds: Vec<u64>) -> Self {
        bounds.sort_unstable();
        bounds.dedup();
        let buckets = bounds.len() + 1;
        Self { bounds, counts: vec![0; buckets], sum: 0, total: 0 }
    }

    /// `count` bounds growing by `factor`: start, start * factor, ...
    pub fn exponential(start: u64, factor: f64, count: usize) -> Self {
        let mut bounds = Vec::with_capacity(count);
        let mut next = start as f64;
        for _ in 0..count {
            bounds.push(next.round() as u64);
            next *= factor;
        }
        Self::new(bounds)
    }

    pub fn record(&mut self, value: u64) {
        // The first bucket whose bound is >= value; past the last bound is the overflow bucket.
        let index = self.bounds.partition_point(|&bound| bound < value);
        self.counts[index] += 1;
        self.sum += value as u128;
        self.total += 1;
    }

    pub fn count(&self) -> u64 {
        self.total
    }

    pub fn sum(&self) -> u128 {
        self.sum
    }

    pub fn mean(&self) -> Option<f64> {
        if self.total == 0 {
            return None;
        }
        Some(self.sum as f64 / self.total as f64)
    }

    /// Each bucket's upper bound (`None` for the overflow bucket) and its count.
    pub fn buckets(&self) -> impl Iterator<Item = (Option<u64>, u64)> + '_ {
        self.counts.iter().enumerate().map(|(i, &count)| (self.bounds.get(i).copied(), count))
    }
}
