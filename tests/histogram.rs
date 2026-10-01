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
fn values_above_the_last_bound_land_in_the_overflow_bucket() {
    let mut h = Histogram::new(vec![10, 100]);
    h.record(101);
    h.record(u64::MAX);
    let last = h.buckets().last().unwrap();
    assert_eq!(last, (None, 2));
}

#[test]
fn bounds_are_sorted_and_deduplicated() {
    let h = Histogram::new(vec![100, 10, 100]);
    let bounds: Vec<Option<u64>> = h.buckets().map(|(b, _)| b).collect();
    assert_eq!(bounds, vec![Some(10), Some(100), None]);
}

#[test]
fn exponential_bounds_double_from_the_start() {
    let h = Histogram::exponential(100, 2.0, 4);
    let bounds: Vec<Option<u64>> = h.buckets().map(|(b, _)| b).collect();
    assert_eq!(bounds, vec![Some(100), Some(200), Some(400), Some(800), None]);
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
    assert_eq!(h.sum(), 60);
}
