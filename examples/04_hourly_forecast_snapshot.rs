#[path = "support/mod.rs"]
mod support;

use std::error::Error;

use weatherkit::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let service = WeatherService::shared();
    let location = support::sample_location();

    if let Some(hourly) =
        support::handle_result("hourly forecast", service.hourly_forecast(&location))?
    {
        if let Some(first) = hourly.forecast.first() {
            println!(
                "first_hour={} temp={:.1}°C precip={:?} wind={:.1}m/s",
                first.date, first.temperature, first.precipitation, first.wind.speed
            );
        } else {
            println!("hourly forecast returned no hours");
        }
    }

    support::finish("hourly forecast");
    Ok(())
}
