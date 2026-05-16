mod common;

use weatherkit::prelude::*;

#[test]
fn weather_service_supports_multi_query_helpers() {
    let service = WeatherService::shared();
    let location = common::sample_location();

    let pair = common::entitlement_ok(service.weather_including2(
        &location,
        WeatherQuery::Current,
        WeatherQuery::Availability,
    ));
    if let Some((current, availability)) = pair {
        assert!(matches!(current, WeatherQueryResult::CurrentWeather(_)));
        assert!(matches!(availability, WeatherQueryResult::Availability(_)));
    }

    let many = common::entitlement_ok(service.weather_including_many(
        &location,
        vec![WeatherQuery::Daily, WeatherQuery::Changes],
    ));
    if let Some(results) = many {
        assert_eq!(results.len(), 2);
        assert!(matches!(results.first(), Some(WeatherQueryResult::DailyForecast(_))));
        assert!(matches!(results.get(1), Some(WeatherQueryResult::WeatherChanges(_))));
    }
}
