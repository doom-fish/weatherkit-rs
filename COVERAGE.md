# WeatherKit SDK coverage audit

Audited against `WeatherKit.framework/Versions/A/Modules/WeatherKit.swiftmodule/arm64e-apple-macos.swiftinterface` from Xcode 26.3 / macOS 26.2 SDK.

| API | Status | Notes |
| --- | --- | --- |
| `WeatherService.shared`, `WeatherService.init()` | ✅ implemented | `WeatherService::shared()` and `WeatherService::new()` mirror the singleton and owned service entry points. |
| `WeatherService.attribution` | ✅ implemented | `WeatherService::attribution()` returns `WeatherAttribution`. |
| `WeatherService.weather(for:)` | ✅ implemented | `WeatherService::weather()` returns the composite `Weather` snapshot. |
| `WeatherService.weather(... current/hourly/daily/minute/alerts/availability)` | ✅ implemented | Concrete per-dataset helpers are exposed on `WeatherService`. |
| `WeatherQuery.daily(startDate:endDate)` | ✅ implemented | `WeatherService::daily_forecast_in()` uses the ranged query initializer. |
| `WeatherQuery.hourly(startDate:endDate)` | ✅ implemented | `WeatherService::hourly_forecast_in()` uses the ranged query initializer. |
| `WeatherQuery.changes` | 🟡 partial | macOS 15+ `WeatherChanges` is deferred in this wave. |
| `WeatherQuery.historicalComparisons` | 🟡 partial | macOS 15+ historical comparison queries are deferred in this wave. |
| `WeatherService.dailyStatistics(...)` | 🟡 partial | macOS 15+ statistics APIs are deferred. |
| `WeatherService.dailySummary(...)` | 🟡 partial | macOS 15+ daily summaries are deferred. |
| `WeatherService.hourlyStatistics(...)` | 🟡 partial | macOS 15+ hourly statistics are deferred. |
| `WeatherService.monthlyStatistics(...)` | 🟡 partial | macOS 15+ monthly statistics are deferred. |
| `WeatherAvailability` / `AvailabilityKind` | ✅ implemented | Includes descriptor catalog for all availability cases. |
| `MinuteWeather` | ✅ implemented | Exposed via `MinuteForecast` / `MinuteForecastCollection`. |
| `DayWeather` | ✅ implemented | Exposed via `DayForecast` / `DailyForecast`, including macOS 15+ optional fields where available. |
| `PrecipitationAmountByType` | ✅ implemented | Included on `DayForecast` / `DayPartForecast` when available. |
| `Trend`, `TrendBaseline`, `Deviation` | 🟡 partial | Only the `Pressure` convenience helper is wrapped; generic trend types are deferred with the statistics APIs. |
| `HistoricalComparisons` / `HistoricalComparison` | 🟡 partial | Deferred with the macOS 15+ historical comparison queries. |
| `DayTemperatureStatistics` / `MonthTemperatureStatistics` | 🟡 partial | Deferred with the statistics APIs. |
| `DayPrecipitationStatistics` / `MonthPrecipitationStatistics` | 🟡 partial | Deferred with the statistics APIs. |
| `DayTemperatureSummary` / `DayPrecipitationSummary` | 🟡 partial | Deferred with the daily summary APIs. |
| `HourlyWeatherStatistics<T>` / `DailyWeatherStatistics<T>` / `MonthlyWeatherStatistics<T>` | 🟡 partial | Deferred with the statistics APIs. |
| `HourlyWeatherStatisticsQuery<T>` / `DailyWeatherStatisticsQuery<T>` / `MonthlyWeatherStatisticsQuery<T>` / `DailyWeatherSummaryQuery<T>` | 🟡 partial | Deferred with the statistics APIs. |
| `UVIndex` | ✅ implemented | Exposed on current, hourly, and daily weather payloads. |
| `WeatherChanges` / `WeatherChange` | 🟡 partial | Deferred with `WeatherQuery.changes`. |
| `HourWeather` | ✅ implemented | Exposed via `HourForecast` / `HourlyForecast`, including macOS 15+ optional fields where available. |
| `WeatherError` | 🟡 partial | Errors are surfaced as `WeatherKitError` domain/code/message payloads instead of a dedicated Rust enum. |
| `SunEvents` | ✅ implemented | Exposed directly on `DayForecast` and via `WeatherService::sun_events()`. |
| `WeatherCondition` | ✅ implemented | Rust enum plus descriptor catalog. |
| `WeatherAttribution` | ✅ implemented | Includes legal page and mark URLs plus legal attribution text when available. |
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
