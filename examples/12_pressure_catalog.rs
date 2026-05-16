#[path = "support/mod.rs"]
mod support;

use std::error::Error;

use weatherkit::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let descriptors = PressureTrend::descriptors()?;
    println!(
        "pressure_trends={:?}",
        descriptors
            .iter()
            .map(|d| d.raw_value.as_str())
            .collect::<Vec<_>>()
    );

    let service = WeatherService::shared();
    let location = support::sample_location();
    if let Some(pressure) = support::handle_result("pressure", service.pressure(&location))? {
        println!(
            "current_pressure={:.1}hPa trend={:?}",
            pressure.value, pressure.trend
        );
    }

    support::finish("pressure");
    Ok(())
}
