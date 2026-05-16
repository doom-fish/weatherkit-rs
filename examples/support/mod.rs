#![allow(dead_code)]

use std::error::Error;

use weatherkit::{CLLocation, WeatherKitError};

pub const fn sample_location() -> CLLocation {
    CLLocation::new(37.3349, -122.0090)
}

pub fn handle_result<T>(
    area: &str,
    result: Result<T, WeatherKitError>,
) -> Result<Option<T>, Box<dyn Error>> {
    match result {
        Ok(value) => Ok(Some(value)),
        Err(error) if error.is_entitlement_issue() => {
            println!("weatherkit {area}: entitled bundle ID required; treating as caveat");
            Ok(None)
        }
        Err(error) => Err(Box::new(error)),
    }
}

pub fn finish(area: &str) {
    println!("✅ {area}");
}
