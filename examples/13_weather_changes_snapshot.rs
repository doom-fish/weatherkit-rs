#[path = "support/mod.rs"]
mod support;

use std::error::Error;

use weatherkit::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let service = WeatherService::shared();
    let location = support::sample_location();

    if let Some(changes) = support::handle_result("weather changes", service.weather_changes(&location))? {
        match changes {
            Some(changes) => println!("changes={} metadata_date={}", changes.len(), changes.metadata.date),
            None => println!("changes=none"),
        }
    }

    if let Some(comparisons) = support::handle_result(
        "historical comparisons",
        service.historical_comparisons(&location),
    )? {
        match comparisons {
            Some(comparisons) => println!(
                "historical_comparisons={} metadata_date={}",
                comparisons.len(), comparisons.metadata.date
            ),
            None => println!("historical_comparisons=none"),
        }
    }

    support::finish("weather changes");
    Ok(())
}
