# Changelog

## 0.2.0 - 2026-05-16

- Reworked the Swift bridge into one file per WeatherKit logical area with retained opaque handles and `_release` functions on the Rust side.
- Added individual `WeatherService` queries for current weather, daily/hourly/minute forecasts, weather alerts, availability, attribution, sun events, moon events, and pressure helpers.
- Added WeatherAttribution, SunEvents, MoonEvents, WeatherCondition, WeatherSeverity, PressureTrend, and MoonPhase descriptor catalogs.
- Added one example and one integration test per requested logical area, plus `COVERAGE.md` to track implemented vs deferred SDK surface.

## 0.1.0 - 2026-05-16

- Initial release.
- Added WeatherKit service access for current, hourly, daily, and minute forecasts.
- Added a smoke example that handles entitlement caveats gracefully.
