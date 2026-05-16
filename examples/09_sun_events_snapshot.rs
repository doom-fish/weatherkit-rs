#[path = "support/mod.rs"]
mod support;

use std::error::Error;

use weatherkit::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let service = WeatherService::shared();
    let location = support::sample_location();

    if let Some(sun) = support::handle_result("sun events", service.sun_events(&location))? {
        println!("sunrise={:?} sunset={:?}", sun.sunrise, sun.sunset);
    }

    support::finish("sun events");
    Ok(())
}
