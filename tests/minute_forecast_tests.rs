mod common;

use weatherkit::prelude::*;

#[test]
fn minute_forecast_smoke_or_entitlement() {
    if !common::live_tests_enabled("minute_forecast_smoke_or_entitlement") {
        return;
    }
    let service = WeatherService::shared();
    if let Some(Some(minute)) =
        common::entitlement_ok(service.minute_forecast(&common::sample_location()))
    {
        assert!(!minute.summary.is_empty() || minute.forecast.is_empty());
    }
}
