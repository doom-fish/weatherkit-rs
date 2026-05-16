#[path = "support/mod.rs"]
mod support;

use std::error::Error;

use weatherkit::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let service = WeatherService::shared();
    let location = support::sample_location();

    if let Some(daily) =
        support::handle_result("daily forecast", service.daily_forecast(&location))?
    {
        if let Some(first) = daily.forecast.first() {
            println!(
                "first_day={} high={:.1}°C low={:.1}°C sunrise={:?}",
                first.date, first.high_temperature, first.low_temperature, first.sun.sunrise
            );
        } else {
            println!("daily forecast returned no days");
        }
    }

    support::finish("daily forecast");
    Ok(())
}
