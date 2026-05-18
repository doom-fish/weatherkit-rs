# Changelog

## 0.3.2 - 2026-05-18

### Documentation

- Added concise Rustdoc coverage across the public WeatherKit wrapper surface outside `src/ffi/`, bringing public-item coverage to 100%.
- Added module-level docs for the non-FFI WeatherKit modules and refreshed the crate release metadata for the doc pass.

## 0.3.1 - 2026-05-16

### Fixes

- **Async safety**: all 10 `extern "C"` Swift-completion callbacks
  (`weather_cb`, `current_weather_cb`, `hourly_forecast_cb`,
  `daily_forecast_cb`, `minute_forecast_cb`, `weather_alerts_cb`,
  `availability_cb`, `attribution_cb`, `weather_changes_cb`,
  `historical_comparisons_cb`) were missing panic guards. Any Rust panic
  inside these callbacks would unwind across the FFI boundary — undefined
  behaviour. Each callback body is now wrapped in
  `doom_fish_utils::panic_safe::catch_user_panic`.
- **Unsafe hygiene**: added `// SAFETY:` comments to every `unsafe { … }`
  block in `src/async_api.rs`, `src/private.rs`, and `src/service.rs`
  (`OwnedHandle::drop`, `ServiceHandle::drop`, `ServiceHandle::acquire`,
  and all four `fetch_*_handle` helpers).
- **Cargo hygiene**: `doom-fish-utils` version constraint widened from
  `"0.1"` to `">=0.1, <0.3"` to leave room for the next minor release
  without requiring a version bump here.

## 0.3.0 - 2026-05-16

### Async API (Tier 1)

Added `async_api` module (enabled by the `async` Cargo feature) with
executor-agnostic `Future` newtypes wrapping every `async throws` surface
on `WeatherService`.  Backed by `@_cdecl` Swift thunks in
`swift-bridge/Sources/WeatherKitBridge/Async.swift`.

New Future types:

| Future type | Swift API |
|-------------|-----------|
| `WeatherFuture` | `WeatherService.weather(for:)` |
| `CurrentWeatherFuture` | `WeatherService.weather(for: including: .current)` |
| `HourlyForecastFuture` | `WeatherService.weather(for: including: .hourly)` |
| `DailyForecastFuture` | `WeatherService.weather(for: including: .daily)` |
| `MinuteForecastFuture` | `WeatherService.weather(for: including: .minute)` |
| `WeatherAlertsFuture` | `WeatherService.weather(for: including: .alerts)` |
| `AvailabilityFuture` | `WeatherService.weather(for: including: .availability)` |
| `AttributionFuture` | `WeatherService.attribution` |
| `WeatherChangesFuture` | `WeatherService.weather(for: including: .changes)` (macOS 15+) |
| `HistoricalComparisonsFuture` | `WeatherService.weather(for: including: .historicalComparisons)` (macOS 15+) |

New public item: `AsyncWeatherService` — entry point with methods matching
all of the above.

Added `examples/17_async_weather.rs` (run with `--features async`).
Added `tests/async_api_tests.rs` with 12 tests (happy-path + error-path,
auth-graceful).

### Other

- Added `doom-fish-utils` dependency (completion utilities).
- Added `pollster = "0.3"` dev-dependency.



- Added WeatherService multi-query helpers (`weather_including2` … `weather_including6` plus `weather_including_many`) and public `WeatherQuery` / `WeatherQueryResult` types.
- Wrapped the macOS 15 statistics, summary, weather-changes, and historical-comparison families, including `Trend`, `TrendBaseline`, `Percentiles`, and the new WeatherService statistics helpers.
- Added descriptor catalogs for `Precipitation`, `WeatherError`, `WindCompassDirection`, and `UVExposureCategory`, plus typed helper accessors on `Wind` and `UVIndex`.
- Added integration tests and examples for the new statistics, change-tracking, multi-query, and enum-catalog APIs.
- Updated `COVERAGE_AUDIT.md` to 100% non-exempt coverage.

## 0.2.0 - 2026-05-16

- Reworked the Swift bridge into one file per WeatherKit logical area with retained opaque handles and `_release` functions on the Rust side.
- Added individual `WeatherService` queries for current weather, daily/hourly/minute forecasts, weather alerts, availability, attribution, sun events, moon events, and pressure helpers.
- Added WeatherAttribution, SunEvents, MoonEvents, WeatherCondition, WeatherSeverity, PressureTrend, and MoonPhase descriptor catalogs.
- Added one example and one integration test per requested logical area, plus `COVERAGE.md` to track implemented vs deferred SDK surface.

## 0.1.0 - 2026-05-16

- Initial release.
- Added WeatherKit service access for current, hourly, daily, and minute forecasts.
- Added a smoke example that handles entitlement caveats gracefully.
