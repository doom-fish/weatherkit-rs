#[path = "support/mod.rs"]
mod support;

use std::error::Error;

use weatherkit::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "precipitation={:?}",
        Precipitation::descriptors()?
            .iter()
            .map(|descriptor| descriptor.raw_value.as_str())
            .collect::<Vec<_>>()
    );
    println!(
        "wind_compass={:?}",
        WindCompassDirection::descriptors()?
            .iter()
            .map(|descriptor| descriptor.raw_value.as_str())
            .collect::<Vec<_>>()
    );
    println!(
        "uv_exposure={:?}",
        UVExposureCategory::descriptors()?
            .iter()
            .map(|descriptor| descriptor.raw_value.as_str())
            .collect::<Vec<_>>()
    );
    println!(
        "weather_errors={:?}",
        WeatherError::descriptors()?
            .iter()
            .map(|descriptor| descriptor.raw_value.as_str())
            .collect::<Vec<_>>()
    );

    support::finish("weather enum catalog");
    Ok(())
}
