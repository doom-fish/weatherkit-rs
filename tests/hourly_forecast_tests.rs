mod common;

use weatherkit::prelude::*;

#[test]
fn hourly_forecast_smoke_or_entitlement() {
    let service = WeatherService::shared();
    if let Some(hourly) =
        common::entitlement_ok(service.hourly_forecast(&common::sample_location()))
    {
        assert!(!hourly.forecast.is_empty());
        assert!(hourly.forecast[0].temperature.is_finite());
    }
}
