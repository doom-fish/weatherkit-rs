mod common;

use weatherkit::prelude::*;

#[test]
fn daily_forecast_smoke_or_entitlement() {
    let service = WeatherService::shared();
    if let Some(daily) = common::entitlement_ok(service.daily_forecast(&common::sample_location()))
    {
        assert!(!daily.forecast.is_empty());
        let first = &daily.forecast[0];
        assert!(
            first.high_temperature >= first.low_temperature
                || (first.high_temperature.is_nan() && first.low_temperature.is_nan())
        );
    }
}
