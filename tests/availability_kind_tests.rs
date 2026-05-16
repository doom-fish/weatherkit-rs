use weatherkit::prelude::*;

#[test]
fn availability_kind_descriptors_cover_public_cases() {
    let descriptors = AvailabilityKind::descriptors().expect("availability descriptors");
    let values = descriptors
        .into_iter()
        .map(|descriptor| descriptor.raw_value)
        .collect::<Vec<_>>();
    assert_eq!(
        values,
        vec![
            "available",
            "temporarilyUnavailable",
            "unsupported",
            "unknown"
        ]
    );
}
