#[path = "support/mod.rs"]
mod support;

use std::error::Error;

use weatherkit::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let service = WeatherService::shared();
    let location = support::sample_location();

    if let Some(moon) = support::handle_result("moon events", service.moon_events(&location))? {
        println!(
            "phase={:?} moonrise={:?} moonset={:?}",
            moon.phase, moon.moonrise, moon.moonset
        );
    }

    support::finish("moon events");
    Ok(())
}
