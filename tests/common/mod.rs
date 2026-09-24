#![allow(dead_code)]

use weatherkit::{CLLocation, WeatherKitError};

pub const fn sample_location() -> CLLocation {
    CLLocation::new(37.3349, -122.0090)
}

pub fn entitlement_ok<T>(result: Result<T, WeatherKitError>) -> Option<T> {
    match result {
        Ok(value) => Some(value),
        Err(error) if error.is_entitlement_issue() => None,
        Err(error) => panic!("unexpected WeatherKit error: {error}"),
    }
}

pub fn live_tests_enabled(test: &str) -> bool {
    let enabled = std::env::var("WEATHERKIT_LIVE_TESTS").as_deref() == Ok("1");
    if !enabled {
        eprintln!("{test}: skipped; set WEATHERKIT_LIVE_TESTS=1 to call the WeatherKit service");
    }
    enabled
}
