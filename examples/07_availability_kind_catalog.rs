#[path = "support/mod.rs"]
mod support;

use std::error::Error;

use weatherkit::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let descriptors = AvailabilityKind::descriptors()?;
    println!(
        "availability_kinds={:?}",
        descriptors
            .iter()
            .map(|d| d.raw_value.as_str())
            .collect::<Vec<_>>()
    );
    support::finish("availability kind");
    Ok(())
}
