mod common;

use weatherkit::prelude::*;

#[test]
fn moon_phase_descriptors_and_events_work() {
    let descriptors = MoonPhase::descriptors().expect("moon phase descriptors");
    assert!(descriptors
        .iter()
        .any(|descriptor| descriptor.raw_value == "full"));

    let service = WeatherService::shared();
    if let Some(moon) = common::entitlement_ok(service.moon_events(&common::sample_location())) {
        assert!(!moon.phase.raw_value().is_empty());
    }
}
