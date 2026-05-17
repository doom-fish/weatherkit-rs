//! Example: async weather fetch via `AsyncWeatherService`
//!
//! Run with:
//! ```sh
//! cargo run --example 17_async_weather --features async
//! ```
//!
//! The `WeatherKit` entitlement is required for a real result; without it the
//! call returns an error that this example handles gracefully.

#[cfg(feature = "async")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use weatherkit::async_api::AsyncWeatherService;
    use weatherkit::service::CLLocation;

    // San Jose, CA
    let loc = CLLocation::new(37.3382, -121.8863);

    pollster::block_on(async {
        let svc = AsyncWeatherService::shared();

        // --- full weather bundle ---
        match svc.weather(&loc).await {
            Ok(weather) => {
                println!("[async] WeatherService.weather(for:)");
                println!("  condition : {:?}", weather.current_weather.condition);
                println!(
                    "  temperature: {:.1}°C",
                    weather.current_weather.temperature
                );
            }
            Err(e) if e.is_entitlement_issue() => {
                println!("[async] weather: entitlement/auth issue (expected in CI) — {e}");
            }
            Err(e) => {
                eprintln!("[async] weather error: {e}");
            }
        }

        // --- attribution (no location needed) ---
        match svc.attribution().await {
            Ok(attr) => {
                println!("[async] attribution service: {}", attr.service_name);
            }
            Err(e) if e.is_entitlement_issue() => {
                println!("[async] attribution: entitlement/auth issue (expected in CI) — {e}");
            }
            Err(e) => {
                eprintln!("[async] attribution error: {e}");
            }
        }

        // --- current weather ---
        match svc.current_weather(&loc).await {
            Ok(cw) => println!("[async] current temp: {:.1}°C", cw.temperature),
            Err(e) if e.is_entitlement_issue() => {
                println!("[async] current_weather: auth issue — {e}");
            }
            Err(e) => eprintln!("[async] current_weather error: {e}"),
        }

        // --- hourly forecast ---
        match svc.hourly_forecast(&loc).await {
            Ok(hf) => println!("[async] hourly forecast: {} hours", hf.forecast.len()),
            Err(e) if e.is_entitlement_issue() => {
                println!("[async] hourly_forecast: auth issue — {e}");
            }
            Err(e) => eprintln!("[async] hourly_forecast error: {e}"),
        }

        // --- daily forecast ---
        match svc.daily_forecast(&loc).await {
            Ok(df) => println!("[async] daily forecast: {} days", df.forecast.len()),
            Err(e) if e.is_entitlement_issue() => {
                println!("[async] daily_forecast: auth issue — {e}");
            }
            Err(e) => eprintln!("[async] daily_forecast error: {e}"),
        }

        // --- availability ---
        match svc.availability(&loc).await {
            Ok(av) => println!("[async] availability: {av:?}"),
            Err(e) if e.is_entitlement_issue() => {
                println!("[async] availability: auth issue — {e}");
            }
            Err(e) => eprintln!("[async] availability error: {e}"),
        }

        Ok(())
    })
}

#[cfg(not(feature = "async"))]
fn main() {
    eprintln!("Re-run with --features async");
    std::process::exit(1);
}
