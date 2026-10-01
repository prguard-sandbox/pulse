use std::fmt::Write;

use crate::registry::Registry;

/// Renders every histogram in the Prometheus text exposition format, with the given labels
/// on every series (e.g. `[("service", "checkout"), ("region", region)]`).
pub fn prometheus(registry: &Registry, labels: &[(&str, &str)]) -> String {
    let mut out = String::new();
    let base = labels
        .iter()
        .map(|(key, value)| format!("{key}=\"{value}\""))
        .collect::<Vec<_>>()
        .join(",");

    for (name, histogram) in registry.snapshot() {
        writeln!(out, "# TYPE {name} histogram").unwrap();
        let mut cumulative = 0;
        for (bound, count) in histogram.buckets() {
            cumulative += count;
            let le = bound.map(|b| b.to_string()).unwrap_or_else(|| "+Inf".to_string());
            writeln!(out, "{name}_bucket{{{base},le=\"{le}\"}} {cumulative}").unwrap();
        }
        writeln!(out, "{name}_count{{{base}}} {}", histogram.count()).unwrap();
        writeln!(out, "{name}_p99{{{base}}} {}", histogram.percentile(0.99)).unwrap();
    }
    out
}
