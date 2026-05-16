use weatherkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = WeatherService::shared();
    let location = CLLocation::new(37.3349, -122.0090);

    match service.weather(&location) {
        Ok(weather) => {
            println!(
                "current temperature: {:.1}°C",
                weather.current_weather.temperature
            );
        }
        Err(error) if error.is_entitlement_issue() => {
            eprintln!(
                "weatherkit requires entitled bundle ID; check developer.apple.com/account"
            );
        }
        Err(error) => return Err(error.into()),
    }

    println!("✅ weatherkit smoke (with caveats)");
    Ok(())
}
