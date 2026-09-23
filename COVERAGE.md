# WeatherKit SDK coverage audit

Audited against `WeatherKit.framework/Versions/A/Modules/WeatherKit.swiftmodule/arm64e-apple-macos.swiftinterface` from the macOS 26.5 SDK. The macOS 27.0 SDK declares no WeatherKit API beyond 26.5.

All `f64` values are converted to SI units by the Swift bridge (degrees Celsius, hectopascals, metres, metres per second, degrees, fractions from 0 to 1); see the README's Units section.

| API | Status | Notes |
| --- | --- | --- |
| `WeatherService.shared`, `WeatherService.init()` | ✅ implemented | `WeatherService::shared()` and `WeatherService::new()` mirror the singleton and owned service entry points. |
| `WeatherService.attribution` | ✅ implemented | `WeatherService::attribution()` returns `WeatherAttribution`. |
| `WeatherService.weather(for:)` | ✅ implemented | `WeatherService::weather()` returns the composite `Weather` snapshot. |
| `WeatherService.weather(... current/hourly/daily/minute/alerts/availability)` | ✅ implemented | Concrete per-dataset helpers are exposed on `WeatherService`. |
| `WeatherQuery.daily(startDate:endDate)` | ✅ implemented | `WeatherService::daily_forecast_in()` uses the ranged query initializer. |
| `WeatherQuery.hourly(startDate:endDate)` | ✅ implemented | `WeatherService::hourly_forecast_in()` uses the ranged query initializer. |
| `WeatherQuery.changes` | ✅ implemented | macOS 15+ `WeatherService::weather_changes()` returns `Option<WeatherChanges>`. |
| `WeatherQuery.historicalComparisons` | ✅ implemented | macOS 15+ `WeatherService::historical_comparisons()` returns `Option<HistoricalComparisons>`. |
| `WeatherService.dailyStatistics(...)` | ✅ implemented | macOS 15+ `daily_statistics`, `daily_statistics_in` and `daily_statistics_between_days`. |
| `WeatherService.dailySummary(...)` | ✅ implemented | macOS 15+ `daily_summary` and `daily_summary_in`. |
| `WeatherService.hourlyStatistics(...)` | ✅ implemented | macOS 15+ `hourly_statistics`, `hourly_statistics_in` and `hourly_statistics_between_hours`. |
| `WeatherService.monthlyStatistics(...)` | ✅ implemented | macOS 15+ `monthly_statistics`, `monthly_statistics_in` and `monthly_statistics_between_months`. |
| `WeatherAvailability` / `AvailabilityKind` | ✅ implemented | Includes descriptor catalog for all availability cases. |
| `MinuteWeather` | ✅ implemented | Exposed via `MinuteForecast` / `MinuteForecastCollection`. |
| `DayWeather` | ✅ implemented | Exposed via `DayForecast` / `DailyForecast`, including macOS 15+ optional fields where available. |
| `PrecipitationAmountByType` | ✅ implemented | Included on `DayForecast` / `DayPartForecast` when available. |
| `Trend`, `TrendBaseline`, `Deviation` | ✅ implemented | `Trend<Unit>`, `TrendBaseline<Unit>` and `Deviation`, with `TemperatureUnit` (°C) and `LengthUnit` (metres) markers. |
| `HistoricalComparisons` / `HistoricalComparison` | ✅ implemented | Returned by `WeatherService::historical_comparisons()`. |
| `DayTemperatureStatistics` / `MonthTemperatureStatistics` | ✅ implemented | Statistics payload types in `statistics`. |
| `DayPrecipitationStatistics` / `MonthPrecipitationStatistics` | ✅ implemented | Statistics payload types in `statistics`. |
| `DayTemperatureSummary` / `DayPrecipitationSummary` | ✅ implemented | Daily summary payload types in `statistics`. |
| `HourlyWeatherStatistics<T>` / `DailyWeatherStatistics<T>` / `MonthlyWeatherStatistics<T>` | ✅ implemented | Generic containers in `statistics`. |
| `HourlyWeatherStatisticsQuery<T>` / `DailyWeatherStatisticsQuery<T>` / `MonthlyWeatherStatisticsQuery<T>` / `DailyWeatherSummaryQuery<T>` | ✅ implemented | Expressed as the query enums taken by the statistics and summary methods. |
| `UVIndex` | ✅ implemented | Exposed on current, hourly, and daily weather payloads. |
| `WeatherChanges` / `WeatherChange` | ✅ implemented | Returned by `WeatherService::weather_changes()`. |
| `HourWeather` | ✅ implemented | Exposed via `HourForecast` / `HourlyForecast`, including macOS 15+ optional fields where available. |
| `WeatherError` | ✅ implemented | `WeatherError` enum plus descriptor catalog; failures are returned as `WeatherKitError` domain/code/message payloads. |
| `SunEvents` | ✅ implemented | Exposed directly on `DayForecast` and via `WeatherService::sun_events()`. |
| `WeatherCondition` | ✅ implemented | Rust enum plus descriptor catalog. |
| `WeatherAttribution` | ✅ implemented | Includes legal page and mark URLs plus legal attribution text when available. Apps must display the mark and legal link wherever WeatherKit data appears. |
| `CloudCoverByAltitude` | ✅ implemented | Exposed as an optional macOS 15+ field on current, hourly, and day-part weather. |
| `MoonEvents` / `MoonPhase` | ✅ implemented | `MoonEvents` is exposed directly, with a descriptor catalog for `MoonPhase`. |
| `SnowfallAmount` | ✅ implemented | Exposed through `PrecipitationAmountByType`. |
| `WeatherMetadata` | ✅ implemented | Forecasts, alerts, and current weather include metadata. |
| `Weather` | ✅ implemented | Composite snapshot returned by `WeatherService::weather()`. |
| `Forecast<Element>` | ✅ implemented | Concrete daily, hourly, and minute forecast containers are exposed. |
| `Precipitation` | ✅ implemented | Rust enum used by day/hour/minute forecasts. |
| `PressureTrend` | ✅ implemented | Rust enum plus descriptor catalog. |
| `Wind` | ✅ implemented | Exposed on current, hourly, daily, and day-part payloads. |
| `WeatherSeverity` | ✅ implemented | Exposed through `WeatherAlert::severity_kind()` plus descriptor catalog. |
| `CurrentWeather` | ✅ implemented | Exposed with metadata, precipitation intensity, and optional cloud cover by altitude. |
| `DayPartForecast` | ✅ implemented | Exposed as optional day/overnight/rest-of-day payloads on `DayForecast`. |
| `WeatherAlert` | ✅ implemented | Exposed with metadata and severity helpers. |
