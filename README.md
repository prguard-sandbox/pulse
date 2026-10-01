# pulse

In-process metrics for services that do not want a full metrics stack.

- `Histogram`: fixed, sorted bucket bounds plus an overflow bucket; count, sum and mean.
- `Registry`: named histograms shared across threads.
- `RollingRate`: events per second over a sliding window.

```rust
use pulse::Registry;

let registry = Registry::new();
registry.observe("http_request_us", 1_250);
for (name, histogram) in registry.snapshot() {
    println!("{name}: {} samples, mean {:?}", histogram.count(), histogram.mean());
}
```

## Bucket layout

A histogram keeps one count per bound plus an overflow bucket for values above the last bound.
A value goes into the first bucket whose bound is greater than or equal to it, so with bounds
`[10, 100]` the values 10, 11 and 101 land in the first, second and overflow buckets.

Bounds are sorted and deduplicated when the histogram is built. `Histogram::exponential(start,
factor, count)` builds `count` bounds that grow by `factor`; the registry uses 16 bounds from 100
doubling each time, which covers 100 µs to about 3.3 s when values are microseconds.

Run the tests with `cargo test`.
