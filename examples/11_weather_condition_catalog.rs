#[path = "support/mod.rs"]
mod support;

use std::error::Error;

use weatherkit::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let descriptors = WeatherCondition::descriptors()?;
    println!(
        "condition_count={} first={}",
        descriptors.len(),
        descriptors
            .first()
            .map_or("<none>", |d| d.raw_value.as_str())
    );
    support::finish("weather condition");
    Ok(())
}
