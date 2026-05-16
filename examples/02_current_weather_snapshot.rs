#[path = "support/mod.rs"]
mod support;

use std::error::Error;

use weatherkit::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let service = WeatherService::shared();
    let location = support::sample_location();

    if let Some(current) =
        support::handle_result("current weather", service.current_weather(&location))?
    {
        println!(
            "temp={:.1}°C feels_like={:.1}°C pressure={:.1}hPa daylight={}",
            current.temperature,
            current.feels_like,
            current.pressure_reading().value,
            current.is_daylight
        );
    }

    support::finish("current weather");
    Ok(())
}
