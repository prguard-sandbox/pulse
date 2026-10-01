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

Run the tests with `cargo test`.
