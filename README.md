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

## Entitlements / caveats

WeatherKit usually requires an entitled bundle ID backed by a paid Apple Developer membership. Unsigned CLI binaries often fail with permission or bundle-configuration errors; every example in `examples/` treats that as a caveat instead of a hard failure.

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
