#[path = "support/mod.rs"]
mod support;

use std::error::Error;

use weatherkit::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let service = WeatherService::shared();
    let location = support::sample_location();

    if let Some(alerts) =
        support::handle_result("weather alerts", service.weather_alerts(&location))?
    {
        let first_severity = alerts.first().map(WeatherAlert::severity_kind);
        println!("alerts={} first_severity={first_severity:?}", alerts.len());
    }

    support::finish("weather alerts");
    Ok(())
}
