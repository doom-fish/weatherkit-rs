#[path = "support/mod.rs"]
mod support;

use std::error::Error;

use weatherkit::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let service = WeatherService::shared();
    let location = support::sample_location();

    if let Some(weather) = support::handle_result("weather service", service.weather(&location))? {
        println!(
            "current={:.1}°C hourly={} daily={} alerts={}",
            weather.current_weather.temperature,
            weather.hourly_forecast.len(),
            weather.daily_forecast.len(),
            weather.weather_alerts.len()
        );
    }

    support::finish("weather service");
    Ok(())
}
