# weatherkit-rs

Safe Rust bindings for Apple's [WeatherKit](https://developer.apple.com/documentation/weatherkit) framework on macOS 13+.

The published package is `weatherkit-doomfish`; the Rust library crate is imported as `weatherkit`.

> **Status:** v0.3.0 covers the full non-exempt macOS WeatherKit surface, including WeatherService multi-query helpers, statistics/summaries, weather changes, historical comparisons, enum descriptor catalogs, and an executor-agnostic async API.

## Quick start

```rust,no_run
use weatherkit::prelude::*;

fn main() -> Result<(), WeatherKitError> {
    let service = WeatherService::shared();
    let location = CLLocation::new(37.3349, -122.0090);
    let current = service.current_weather(&location)?;

    println!(
        "current temperature: {:.1}°C ({:?})",
        current.temperature,
        current.condition
    );
    Ok(())
}
```

## Async API

Enable with the `async` Cargo feature to get executor-agnostic `Future` wrappers for every `async throws` surface on `WeatherService`:

```rust,no_run
use weatherkit::async_api::AsyncWeatherService;
use weatherkit::service::CLLocation;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pollster::block_on(async {
        let svc = AsyncWeatherService::shared();
        let loc = CLLocation::new(37.3382, -121.8863);
        let weather = svc.weather(&loc).await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        println!("{:?}", weather.current_weather.condition);
        Ok(())
    })
}
```

Available futures: `WeatherFuture`, `CurrentWeatherFuture`, `HourlyForecastFuture`, `DailyForecastFuture`, `MinuteForecastFuture`, `WeatherAlertsFuture`, `AvailabilityFuture`, `AttributionFuture`, `WeatherChangesFuture` (macOS 15+), `HistoricalComparisonsFuture` (macOS 15+).

See `examples/17_async_weather.rs` and `tests/async_api_tests.rs` for full usage.

## Highlights

- `WeatherService::weather`, `current_weather`, `daily_forecast`, `hourly_forecast`, `minute_forecast`, `weather_alerts`, `availability`, `attribution`, and multi-query `weather_including{2..6}` / `weather_including_many`
- `DailyForecast` / `HourlyForecast` containers with WeatherKit metadata plus `DayForecast` / `HourForecast` entries
- macOS 15+ statistics and summary wrappers (`DailyWeatherStatistics`, `DailyWeatherSummary`, `HourlyWeatherStatistics`, `MonthlyWeatherStatistics`) with query enums and service helpers
- `WeatherChanges`, `HistoricalComparisons`, `Trend`, `TrendBaseline`, and `Percentiles` for the new WeatherKit comparison/change-tracking APIs
- `CurrentWeather` with precipitation intensity, metadata, and cloud-cover-by-altitude when available on macOS 15+
- `DayForecast` with `SunEvents`, `MoonEvents`, `UVIndex`, wind, day-part forecasts, and precipitation-by-type details when available
- Descriptor catalogs for `WeatherAttribution`, `WeatherSeverity`, `AvailabilityKind`, `WeatherCondition`, `Precipitation`, `PressureTrend`, `MoonPhase`, `WindCompassDirection`, `UVExposureCategory`, and `WeatherError`
- Async Swift APIs bridged to synchronous Rust via `DispatchSemaphore + Task`, with opaque handle ownership released on the Rust side

## Entitlements, attribution and caveats

- WeatherKit only answers apps signed with the `com.apple.developer.weatherkit` entitlement. Register the App ID with the WeatherKit capability (and the WeatherKit App Service) in Certificates, Identifiers & Profiles, then sign the app bundle with a provisioning profile that includes the entitlement. This needs a paid Apple Developer Program membership.
- Unsigned or ad-hoc-signed binaries, including `cargo run --example …`, usually fail with a permission or bundle-configuration error. `WeatherKitError::is_entitlement_issue()` detects that case, and every example in `examples/` treats it as a caveat instead of a hard failure.
- Apple requires apps that show WeatherKit data to display the Apple Weather mark and a link to the legal attribution page wherever that data appears. `WeatherService::attribution()` returns both: `combined_mark_light_url` / `combined_mark_dark_url` (or `square_mark_url`) and `legal_page_url`.

## Units

The Swift bridge converts every WeatherKit `Measurement` to a fixed SI unit before it reaches Rust, so the `f64` fields are not in the units WeatherKit's own formatters show:

| Quantity | Unit in this crate |
| --- | --- |
| Temperatures, including trends and percentiles | degrees Celsius |
| Pressure | hectopascals (millibars) |
| Precipitation, rainfall, snowfall, sleet and hail amounts | metres (×1000 for millimetres) |
| Precipitation intensity | metres per second (×3 600 000 for millimetres per hour) |
| Visibility | metres |
| Wind speed, gusts and highest wind speed | metres per second |
| Wind direction | degrees |
| Humidity, cloud cover, precipitation chance and probability | fraction from 0 to 1 |

`DayForecast::maximum_visibility` and `minimum_visibility` are the exception: WeatherKit exposes them as plain `Double` values, and they pass through unchanged.

## Examples

Each logical area has a dedicated example:

```bash
cargo run --example 01_weather_service_smoke
cargo run --example 02_current_weather_snapshot
cargo run --example 03_daily_forecast_snapshot
cargo run --example 04_hourly_forecast_snapshot
cargo run --example 05_weather_alerts_snapshot
cargo run --example 06_minute_forecast_snapshot
cargo run --example 07_availability_kind_catalog
cargo run --example 08_weather_attribution_snapshot
cargo run --example 09_sun_events_snapshot
cargo run --example 10_moon_events_snapshot
cargo run --example 11_weather_condition_catalog
cargo run --example 12_pressure_catalog
cargo run --example 13_weather_changes_snapshot
cargo run --example 14_weather_statistics_snapshot
cargo run --example 15_weather_multi_query_snapshot
cargo run --example 16_weather_enum_catalog
```

## License

Licensed under either [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
