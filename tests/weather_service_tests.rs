mod common;

use weatherkit::prelude::*;

#[test]
fn weather_service_supports_shared_and_owned_handles() {
    let shared = WeatherService::shared();
    let _owned = WeatherService::new();
    let weather = common::entitlement_ok(shared.weather(&common::sample_location()));
    if let Some(weather) = weather {
        assert!(!weather.hourly_forecast.is_empty());
        assert!(!weather.daily_forecast.is_empty());
    }
}
