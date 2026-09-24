mod common;

use weatherkit::prelude::*;

#[test]
fn current_weather_smoke_or_entitlement() {
    if !common::live_tests_enabled("current_weather_smoke_or_entitlement") {
        return;
    }
    let service = WeatherService::shared();
    if let Some(current) =
        common::entitlement_ok(service.current_weather(&common::sample_location()))
    {
        assert!(current.temperature.is_finite());
        assert!(current.pressure_reading().value.is_finite());
    }
}
