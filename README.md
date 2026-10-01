# pulse

In-process metrics for services that do not want a full metrics stack.

- `Histogram`: fixed, sorted bucket bounds plus an overflow bucket; count, sum, mean and percentiles.
- `Registry`: named histograms shared across threads.
- `RollingRate`: events per second over a sliding window.
- `export::prometheus`: every histogram in the Prometheus text format, with a p99 series.

```rust
use pulse::{export, Registry};

let registry = Registry::new();
registry.observe("http_request_us", 1_250);
print!("{}", export::prometheus(&registry, &[("service", "checkout")]));
```

Run the tests with `cargo test`.
