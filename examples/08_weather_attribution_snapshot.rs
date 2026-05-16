#[path = "support/mod.rs"]
mod support;

use std::error::Error;

use weatherkit::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let service = WeatherService::shared();

    if let Some(attribution) = support::handle_result("weather attribution", service.attribution())?
    {
        println!(
            "service={} legal_page={}",
            attribution.service_name, attribution.legal_page_url
        );
    }

    support::finish("weather attribution");
    Ok(())
}
