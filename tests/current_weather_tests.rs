mod common;

use weatherkit::prelude::*;

#[test]
fn current_weather_smoke_or_entitlement() {
    let service = WeatherService::shared();
    if let Some(current) =
        common::entitlement_ok(service.current_weather(&common::sample_location()))
    {
        assert!(current.temperature.is_finite());
        assert!(current.pressure_reading().value.is_finite());
    }
}
