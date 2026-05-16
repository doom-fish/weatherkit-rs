# weatherkit-rs

Safe Rust bindings for Apple's [WeatherKit](https://developer.apple.com/documentation/weatherkit) framework on macOS 13+.

The published package is `weatherkit-doomfish`; the Rust library crate is imported as `weatherkit`.

> **Status:** v0.1.0 covers `WeatherService.shared.weather(for:)`, current conditions, hourly and daily forecasts, minute forecasts, weather alerts, and availability metadata.

## Quick start

```rust,no_run
use weatherkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = WeatherService::shared();
    let location = CLLocation::new(37.3349, -122.0090);
    let weather = service.weather(&location)?;

    println!(
        "current temperature: {:.1}°C ({:?})",
        weather.current_weather.temperature,
        weather.current_weather.condition
    );
    Ok(())
}
```

## Highlights

- `WeatherService::shared` singleton wrapper
- `WeatherService::weather` for `CLLocation` coordinates
- `Weather` with `current_weather`, `hourly_forecast`, `daily_forecast`, `minute_forecast`, `weather_alerts`, and `availability`
- `CurrentWeather` with temperature, feels-like, humidity, dew point, pressure, pressure trend, condition, symbol name, wind, UV index, visibility, cloud cover, and daylight state
- `HourForecast` and `DayForecast` with temperatures and precipitation
- `WeatherCondition`, `Precipitation`, `PressureTrend`, and availability enums
- Async Swift APIs bridged to synchronous Rust via `DispatchSemaphore + Task`

## Entitlements / caveats

WeatherKit usually requires an entitled bundle ID backed by a paid Apple Developer membership. Unsigned CLI binaries often fail with permission or bundle-configuration errors; the smoke example reports that case cleanly.

## Smoke example

Run the WeatherKit smoke test with:

```bash
cargo run --all-features --example 01_weather_smoke
```

It requests weather for Apple Park, prints the current temperature when available, otherwise prints `weatherkit requires entitled bundle ID; check developer.apple.com/account`, and always prints `✅ weatherkit smoke (with caveats)` when the entitlement caveat is handled.

## License

Licensed under either [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
