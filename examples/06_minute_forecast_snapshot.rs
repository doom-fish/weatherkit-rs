#[path = "support/mod.rs"]
mod support;

use std::error::Error;

use weatherkit::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let service = WeatherService::shared();
    let location = support::sample_location();

    if let Some(minute) =
        support::handle_result("minute forecast", service.minute_forecast(&location))?
    {
        if let Some(minute) = minute {
            println!(
                "summary={} entries={}",
                minute.summary,
                minute.forecast.len()
            );
        } else {
            println!("minute forecast unavailable for this location");
        }
    }

    support::finish("minute forecast");
    Ok(())
}
