mod common;

use weatherkit::prelude::*;

#[test]
fn pressure_descriptors_and_helper_work() {
    let descriptors = PressureTrend::descriptors().expect("pressure trend descriptors");
    assert!(descriptors
        .iter()
        .any(|descriptor| descriptor.raw_value == "steady"));

    let pressure = Pressure::new(1013.25, PressureTrend::Steady);
    assert!((pressure.value - 1013.25).abs() < f64::EPSILON);

    if !common::live_tests_enabled("pressure_descriptors_and_helper_work") {
        return;
    }
    let service = WeatherService::shared();
    if let Some(pressure) = common::entitlement_ok(service.pressure(&common::sample_location())) {
        assert!(pressure.value.is_finite());
    }
}
