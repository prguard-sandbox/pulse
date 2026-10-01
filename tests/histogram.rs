use pulse::Histogram;

#[test]
fn records_into_the_first_bucket_that_fits() {
    let mut h = Histogram::new(vec![10, 100, 1_000]);
    h.record(5);
    h.record(10);
    h.record(50);
    let counts: Vec<u64> = h.buckets().map(|(_, c)| c).collect();
    assert_eq!(counts, vec![2, 1, 0, 0]);
}

#[test]
fn mean_is_none_when_empty() {
    let h = Histogram::new(vec![10]);
    assert_eq!(h.mean(), None);
}

#[test]
fn mean_of_recorded_values() {
    let mut h = Histogram::new(vec![10, 100]);
    for v in [10, 20, 30] {
        h.record(v);
    }
    assert_eq!(h.mean(), Some(20.0));
}
