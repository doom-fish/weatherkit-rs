# weatherkit-rs coverage audit v2 (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 481
VERIFIED: 301
GAPS: 0
EXEMPT: 180
COVERAGE_PCT: 100.00%

## Methodology

This v2 audit re-verifies the v1 audit with strict standards for EXEMPT classifications. All public symbols were enumerated from the macOS WeatherKit.swiftinterface (`arm64e-apple-macos.swiftinterface`). EXEMPT entries were re-verified to ensure they have SDK availability attributes or are synthetic protocol witness helpers (Codable, Equatable, Collection glue) that inherit availability from their parent types. The three macOS-deprecated DayWeather precipitation accessors (precipitationAmount, rainfallAmount, snowfallAmount) are retained as EXEMPT per their explicit @available deprecation attributes. All 180 EXEMPT entries are properly cited with either explicit SDK availability attributes or documented protocol-witness/typealias justifications.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `CloudCoverByAltitude` | struct | `WeatherKit.swiftinterface` | CloudCoverByAltitude |
| `CloudCoverByAltitude.high` | var | `WeatherKit.swiftinterface` | CloudCoverByAltitude::high |
| `CloudCoverByAltitude.low` | var | `WeatherKit.swiftinterface` | CloudCoverByAltitude::low |
| `CloudCoverByAltitude.medium` | var | `WeatherKit.swiftinterface` | CloudCoverByAltitude::medium |
| `CurrentWeather` | struct | `WeatherKit.swiftinterface` | CurrentWeather |
| `CurrentWeather.apparentTemperature` | var | `WeatherKit.swiftinterface` | CurrentWeather::feels_like |
| `CurrentWeather.cloudCover` | var | `WeatherKit.swiftinterface` | CurrentWeather::cloud_cover |
| `CurrentWeather.cloudCoverByAltitude` | var | `WeatherKit.swiftinterface` | CurrentWeather::cloud_cover_by_altitude |
| `CurrentWeather.condition` | var | `WeatherKit.swiftinterface` | CurrentWeather::condition |
| `CurrentWeather.date` | var | `WeatherKit.swiftinterface` | CurrentWeather::date |
| `CurrentWeather.dewPoint` | var | `WeatherKit.swiftinterface` | CurrentWeather::dew_point |
| `CurrentWeather.humidity` | var | `WeatherKit.swiftinterface` | CurrentWeather::humidity |
| `CurrentWeather.isDaylight` | var | `WeatherKit.swiftinterface` | CurrentWeather::is_daylight |
| `CurrentWeather.metadata` | var | `WeatherKit.swiftinterface` | CurrentWeather::metadata |
| `CurrentWeather.precipitationIntensity` | var | `WeatherKit.swiftinterface` | CurrentWeather::precipitation_intensity |
| `CurrentWeather.pressure` | var | `WeatherKit.swiftinterface` | CurrentWeather::pressure |
| `CurrentWeather.pressureTrend` | var | `WeatherKit.swiftinterface` | CurrentWeather::pressure_trend |
| `CurrentWeather.symbolName` | var | `WeatherKit.swiftinterface` | CurrentWeather::symbol_name |
| `CurrentWeather.temperature` | var | `WeatherKit.swiftinterface` | CurrentWeather::temperature |
| `CurrentWeather.uvIndex` | var | `WeatherKit.swiftinterface` | CurrentWeather::uv_index |
| `CurrentWeather.visibility` | var | `WeatherKit.swiftinterface` | CurrentWeather::visibility |
| `CurrentWeather.wind` | var | `WeatherKit.swiftinterface` | CurrentWeather::wind |
| `DayPartForecast` | struct | `WeatherKit.swiftinterface` | DayPartForecast |
| `DayPartForecast.cloudCover` | var | `WeatherKit.swiftinterface` | DayPartForecast::cloud_cover |
| `DayPartForecast.cloudCoverByAltitude` | var | `WeatherKit.swiftinterface` | DayPartForecast::cloud_cover_by_altitude |
| `DayPartForecast.condition` | var | `WeatherKit.swiftinterface` | DayPartForecast::condition |
| `DayPartForecast.highTemperature` | var | `WeatherKit.swiftinterface` | DayPartForecast::high_temperature |
| `DayPartForecast.highWindSpeed` | var | `WeatherKit.swiftinterface` | DayPartForecast::high_wind_speed |
| `DayPartForecast.lowTemperature` | var | `WeatherKit.swiftinterface` | DayPartForecast::low_temperature |
| `DayPartForecast.maximumHumidity` | var | `WeatherKit.swiftinterface` | DayPartForecast::maximum_humidity |
| `DayPartForecast.maximumVisibility` | var | `WeatherKit.swiftinterface` | DayPartForecast::maximum_visibility |
| `DayPartForecast.minimumHumidity` | var | `WeatherKit.swiftinterface` | DayPartForecast::minimum_humidity |
| `DayPartForecast.minimumVisibility` | var | `WeatherKit.swiftinterface` | DayPartForecast::minimum_visibility |
| `DayPartForecast.precipitation` | var | `WeatherKit.swiftinterface` | DayPartForecast::precipitation |
| `DayPartForecast.precipitationAmountByType` | var | `WeatherKit.swiftinterface` | DayPartForecast::precipitation_amount_by_type |
| `DayPartForecast.precipitationChance` | var | `WeatherKit.swiftinterface` | DayPartForecast::precipitation_chance |
| `DayPartForecast.wind` | var | `WeatherKit.swiftinterface` | DayPartForecast::wind |
| `DayWeather` | struct | `WeatherKit.swiftinterface` | DayForecast |
| `DayWeather.condition` | var | `WeatherKit.swiftinterface` | DayForecast::condition |
| `DayWeather.date` | var | `WeatherKit.swiftinterface` | DayForecast::date |
| `DayWeather.daytimeForecast` | var | `WeatherKit.swiftinterface` | DayForecast::daytime_forecast |
| `DayWeather.highTemperature` | var | `WeatherKit.swiftinterface` | DayForecast::high_temperature |
| `DayWeather.highTemperatureTime` | var | `WeatherKit.swiftinterface` | DayForecast::high_temperature_time |
| `DayWeather.highWindSpeed` | var | `WeatherKit.swiftinterface` | DayForecast::high_wind_speed |
| `DayWeather.lowTemperature` | var | `WeatherKit.swiftinterface` | DayForecast::low_temperature |
| `DayWeather.lowTemperatureTime` | var | `WeatherKit.swiftinterface` | DayForecast::low_temperature_time |
| `DayWeather.maximumHumidity` | var | `WeatherKit.swiftinterface` | DayForecast::maximum_humidity |
| `DayWeather.maximumVisibility` | var | `WeatherKit.swiftinterface` | DayForecast::maximum_visibility |
| `DayWeather.minimumHumidity` | var | `WeatherKit.swiftinterface` | DayForecast::minimum_humidity |
| `DayWeather.minimumVisibility` | var | `WeatherKit.swiftinterface` | DayForecast::minimum_visibility |
| `DayWeather.moon` | var | `WeatherKit.swiftinterface` | DayForecast::moon |
| `DayWeather.overnightForecast` | var | `WeatherKit.swiftinterface` | DayForecast::overnight_forecast |
| `DayWeather.precipitation` | var | `WeatherKit.swiftinterface` | DayForecast::precipitation |
| `DayWeather.precipitationAmountByType` | var | `WeatherKit.swiftinterface` | DayForecast::precipitation_amount_by_type |
| `DayWeather.precipitationChance` | var | `WeatherKit.swiftinterface` | DayForecast::precipitation_chance |
| `DayWeather.restOfDayForecast` | var | `WeatherKit.swiftinterface` | DayForecast::rest_of_day_forecast |
| `DayWeather.sun` | var | `WeatherKit.swiftinterface` | DayForecast::sun |
| `DayWeather.symbolName` | var | `WeatherKit.swiftinterface` | DayForecast::symbol_name |
| `DayWeather.uvIndex` | var | `WeatherKit.swiftinterface` | DayForecast::uv_index |
| `DayWeather.wind` | var | `WeatherKit.swiftinterface` | DayForecast::wind |
| `Forecast` | struct | `WeatherKit.swiftinterface` | DailyForecast / HourlyForecast / MinuteForecastCollection |
| `Forecast.forecast` | var | `WeatherKit.swiftinterface` | DailyForecast::forecast / HourlyForecast::forecast / MinuteForecastCollection::forecast |
| `Forecast.metadata` | var | `WeatherKit.swiftinterface` | DailyForecast::metadata / HourlyForecast::metadata / MinuteForecastCollection::metadata |
| `HourWeather` | struct | `WeatherKit.swiftinterface` | HourForecast |
| `HourWeather.apparentTemperature` | var | `WeatherKit.swiftinterface` | HourForecast::feels_like |
| `HourWeather.cloudCover` | var | `WeatherKit.swiftinterface` | HourForecast::cloud_cover |
| `HourWeather.cloudCoverByAltitude` | var | `WeatherKit.swiftinterface` | HourForecast::cloud_cover_by_altitude |
| `HourWeather.condition` | var | `WeatherKit.swiftinterface` | HourForecast::condition |
| `HourWeather.date` | var | `WeatherKit.swiftinterface` | HourForecast::date |
| `HourWeather.dewPoint` | var | `WeatherKit.swiftinterface` | HourForecast::dew_point |
| `HourWeather.humidity` | var | `WeatherKit.swiftinterface` | HourForecast::humidity |
| `HourWeather.isDaylight` | var | `WeatherKit.swiftinterface` | HourForecast::is_daylight |
| `HourWeather.precipitation` | var | `WeatherKit.swiftinterface` | HourForecast::precipitation |
| `HourWeather.precipitationAmount` | var | `WeatherKit.swiftinterface` | HourForecast::precipitation_amount |
| `HourWeather.precipitationChance` | var | `WeatherKit.swiftinterface` | HourForecast::precipitation_chance |
| `HourWeather.pressure` | var | `WeatherKit.swiftinterface` | HourForecast::pressure |
| `HourWeather.pressureTrend` | var | `WeatherKit.swiftinterface` | HourForecast::pressure_trend |
| `HourWeather.snowfallAmount` | var | `WeatherKit.swiftinterface` | HourForecast::snowfall_amount |
| `HourWeather.symbolName` | var | `WeatherKit.swiftinterface` | HourForecast::symbol_name |
| `HourWeather.temperature` | var | `WeatherKit.swiftinterface` | HourForecast::temperature |
| `HourWeather.uvIndex` | var | `WeatherKit.swiftinterface` | HourForecast::uv_index |
| `HourWeather.visibility` | var | `WeatherKit.swiftinterface` | HourForecast::visibility |
| `HourWeather.wind` | var | `WeatherKit.swiftinterface` | HourForecast::wind |
| `MinuteWeather` | struct | `WeatherKit.swiftinterface` | MinuteForecast |
| `MinuteWeather.date` | var | `WeatherKit.swiftinterface` | MinuteForecast::date |
| `MinuteWeather.precipitation` | var | `WeatherKit.swiftinterface` | MinuteForecast::precipitation |
| `MinuteWeather.precipitationChance` | var | `WeatherKit.swiftinterface` | MinuteForecast::precipitation_chance |
| `MinuteWeather.precipitationIntensity` | var | `WeatherKit.swiftinterface` | MinuteForecast::precipitation_intensity |
| `MoonEvents` | struct | `WeatherKit.swiftinterface` | MoonEvents |
| `MoonEvents.moonrise` | var | `WeatherKit.swiftinterface` | MoonEvents::moonrise |
| `MoonEvents.moonset` | var | `WeatherKit.swiftinterface` | MoonEvents::moonset |
| `MoonEvents.phase` | var | `WeatherKit.swiftinterface` | MoonEvents::phase |
| `MoonPhase` | enum | `WeatherKit.swiftinterface` | MoonPhase |
| `MoonPhase.accessibilityDescription` | var | `WeatherKit.swiftinterface` | MoonPhase::descriptors() |
| `MoonPhase.description` | var | `WeatherKit.swiftinterface` | MoonPhase::descriptors() |
| `MoonPhase.symbolName` | var | `WeatherKit.swiftinterface` | MoonPhase::descriptors() |
| `Precipitation` | enum | `WeatherKit.swiftinterface` | Precipitation |
| `PrecipitationAmountByType` | struct | `WeatherKit.swiftinterface` | PrecipitationAmountByType |
| `PrecipitationAmountByType.hail` | var | `WeatherKit.swiftinterface` | PrecipitationAmountByType::hail |
| `PrecipitationAmountByType.mixed` | var | `WeatherKit.swiftinterface` | PrecipitationAmountByType::mixed |
| `PrecipitationAmountByType.precipitation` | var | `WeatherKit.swiftinterface` | PrecipitationAmountByType::precipitation |
| `PrecipitationAmountByType.rainfall` | var | `WeatherKit.swiftinterface` | PrecipitationAmountByType::rainfall |
| `PrecipitationAmountByType.sleet` | var | `WeatherKit.swiftinterface` | PrecipitationAmountByType::sleet |
| `PrecipitationAmountByType.snowfallAmount` | var | `WeatherKit.swiftinterface` | PrecipitationAmountByType::snowfall_amount |
| `PressureTrend` | enum | `WeatherKit.swiftinterface` | PressureTrend |
| `PressureTrend.accessibilityDescription` | var | `WeatherKit.swiftinterface` | PressureTrend::descriptors() |
| `PressureTrend.description` | var | `WeatherKit.swiftinterface` | PressureTrend::descriptors() |
| `SnowfallAmount` | struct | `WeatherKit.swiftinterface` | SnowfallAmount |
| `SnowfallAmount.amount` | var | `WeatherKit.swiftinterface` | SnowfallAmount::amount |
| `SnowfallAmount.amountLiquidEquivalent` | var | `WeatherKit.swiftinterface` | SnowfallAmount::amount_liquid_equivalent |
| `SnowfallAmount.maximum` | var | `WeatherKit.swiftinterface` | SnowfallAmount::maximum |
| `SnowfallAmount.maximumLiquidEquivalent` | var | `WeatherKit.swiftinterface` | SnowfallAmount::maximum_liquid_equivalent |
| `SnowfallAmount.minimum` | var | `WeatherKit.swiftinterface` | SnowfallAmount::minimum |
| `SnowfallAmount.minimumLiquidEquivalent` | var | `WeatherKit.swiftinterface` | SnowfallAmount::minimum_liquid_equivalent |
| `SunEvents` | struct | `WeatherKit.swiftinterface` | SunEvents |
| `SunEvents.astronomicalDawn` | var | `WeatherKit.swiftinterface` | SunEvents::astronomical_dawn |
| `SunEvents.astronomicalDusk` | var | `WeatherKit.swiftinterface` | SunEvents::astronomical_dusk |
| `SunEvents.civilDawn` | var | `WeatherKit.swiftinterface` | SunEvents::civil_dawn |
| `SunEvents.civilDusk` | var | `WeatherKit.swiftinterface` | SunEvents::civil_dusk |
| `SunEvents.nauticalDawn` | var | `WeatherKit.swiftinterface` | SunEvents::nautical_dawn |
| `SunEvents.nauticalDusk` | var | `WeatherKit.swiftinterface` | SunEvents::nautical_dusk |
| `SunEvents.solarMidnight` | var | `WeatherKit.swiftinterface` | SunEvents::solar_midnight |
| `SunEvents.solarNoon` | var | `WeatherKit.swiftinterface` | SunEvents::solar_noon |
| `SunEvents.sunrise` | var | `WeatherKit.swiftinterface` | SunEvents::sunrise |
| `SunEvents.sunset` | var | `WeatherKit.swiftinterface` | SunEvents::sunset |
| `UVIndex` | struct | `WeatherKit.swiftinterface` | UVIndex |
| `UVIndex.category` | var | `WeatherKit.swiftinterface` | UVIndex::category / UVIndex::exposure_category() |
| `UVIndex.value` | var | `WeatherKit.swiftinterface` | UVIndex::value |
| `Weather` | struct | `WeatherKit.swiftinterface` | Weather |
| `Weather.availability` | var | `WeatherKit.swiftinterface` | Weather::availability |
| `Weather.currentWeather` | var | `WeatherKit.swiftinterface` | Weather::current_weather |
| `Weather.dailyForecast` | var | `WeatherKit.swiftinterface` | Weather::daily_forecast |
| `Weather.hourlyForecast` | var | `WeatherKit.swiftinterface` | Weather::hourly_forecast |
| `Weather.minuteForecast` | var | `WeatherKit.swiftinterface` | Weather::minute_forecast |
| `Weather.weatherAlerts` | var | `WeatherKit.swiftinterface` | Weather::weather_alerts |
| `WeatherAlert` | struct | `WeatherKit.swiftinterface` | WeatherAlert |
| `WeatherAlert.detailsURL` | var | `WeatherKit.swiftinterface` | WeatherAlert::details_url |
| `WeatherAlert.metadata` | var | `WeatherKit.swiftinterface` | WeatherAlert::metadata |
| `WeatherAlert.region` | var | `WeatherKit.swiftinterface` | WeatherAlert::region |
| `WeatherAlert.severity` | var | `WeatherKit.swiftinterface` | WeatherAlert::severity |
| `WeatherAlert.source` | var | `WeatherKit.swiftinterface` | WeatherAlert::source |
| `WeatherAlert.summary` | var | `WeatherKit.swiftinterface` | WeatherAlert::summary |
| `WeatherAttribution` | struct | `WeatherKit.swiftinterface` | WeatherAttribution |
| `WeatherAttribution.combinedMarkDarkURL` | var | `WeatherKit.swiftinterface` | WeatherAttribution::combined_mark_dark_url |
| `WeatherAttribution.combinedMarkLightURL` | var | `WeatherKit.swiftinterface` | WeatherAttribution::combined_mark_light_url |
| `WeatherAttribution.legalAttributionText` | var | `WeatherKit.swiftinterface` | WeatherAttribution::legal_attribution_text |
| `WeatherAttribution.legalPageURL` | var | `WeatherKit.swiftinterface` | WeatherAttribution::legal_page_url |
| `WeatherAttribution.serviceName` | var | `WeatherKit.swiftinterface` | WeatherAttribution::service_name |
| `WeatherAttribution.squareMarkURL` | var | `WeatherKit.swiftinterface` | WeatherAttribution::square_mark_url |
| `WeatherAvailability` | struct | `WeatherKit.swiftinterface` | WeatherAvailability |
| `WeatherAvailability.AvailabilityKind` | enum | `WeatherKit.swiftinterface` | WeatherAvailability::availability_kind |
| `WeatherAvailability.alertAvailability` | var | `WeatherKit.swiftinterface` | WeatherAvailability::alert_availability |
| `WeatherAvailability.minuteAvailability` | var | `WeatherKit.swiftinterface` | WeatherAvailability::minute_availability |
| `WeatherCondition` | enum | `WeatherKit.swiftinterface` | WeatherCondition |
| `WeatherCondition.accessibilityDescription` | var | `WeatherKit.swiftinterface` | WeatherCondition::descriptors() |
| `WeatherCondition.description` | var | `WeatherKit.swiftinterface` | WeatherCondition::descriptors() |
| `WeatherMetadata` | struct | `WeatherKit.swiftinterface` | WeatherMetadata |
| `WeatherMetadata.date` | var | `WeatherKit.swiftinterface` | WeatherMetadata::date |
| `WeatherMetadata.expirationDate` | var | `WeatherKit.swiftinterface` | WeatherMetadata::expiration_date |
| `WeatherMetadata.location` | var | `WeatherKit.swiftinterface` | WeatherMetadata::location |
| `WeatherQuery` | struct | `WeatherKit.swiftinterface` | WeatherQuery |
| `WeatherQuery.alerts` | var | `WeatherKit.swiftinterface` | WeatherQuery::Alerts |
| `WeatherQuery.availability` | var | `WeatherKit.swiftinterface` | WeatherQuery::Availability |
| `WeatherQuery.current` | var | `WeatherKit.swiftinterface` | WeatherQuery::Current |
| `WeatherQuery.daily` | var | `WeatherKit.swiftinterface` | WeatherQuery::Daily |
| `WeatherQuery.daily(startDate:endDate:)` | func | `WeatherKit.swiftinterface` | WeatherQuery::DailyIn(DateInterval) |
| `WeatherQuery.hourly` | var | `WeatherKit.swiftinterface` | WeatherQuery::Hourly |
| `WeatherQuery.hourly(startDate:endDate:)` | func | `WeatherKit.swiftinterface` | WeatherQuery::HourlyIn(DateInterval) |
| `WeatherQuery.minute` | var | `WeatherKit.swiftinterface` | WeatherQuery::Minute |
| `WeatherService` | class | `WeatherKit.swiftinterface` | WeatherService |
| `WeatherService.attribution` | var | `WeatherKit.swiftinterface` | WeatherService::attribution |
| `WeatherService.init()` | init | `WeatherKit.swiftinterface` | WeatherService::new |
| `WeatherService.shared` | let | `WeatherKit.swiftinterface` | WeatherService::shared |
| `WeatherService.weather(for:)` | func | `WeatherKit.swiftinterface` | WeatherService::weather |
| `WeatherService.weather(for:including:)` | func | `WeatherKit.swiftinterface` | WeatherService::weather_including / dedicated single-query helpers |
| `WeatherSeverity` | enum | `WeatherKit.swiftinterface` | WeatherSeverity |
| `WeatherSeverity.accessibilityDescription` | var | `WeatherKit.swiftinterface` | WeatherSeverity::descriptors() |
| `WeatherSeverity.description` | var | `WeatherKit.swiftinterface` | WeatherSeverity::descriptors() |
| `Wind` | struct | `WeatherKit.swiftinterface` | Wind |
| `Wind.compassDirection` | var | `WeatherKit.swiftinterface` | Wind::compass_direction / Wind::compass_direction_kind() |
| `Wind.direction` | var | `WeatherKit.swiftinterface` | Wind::direction |
| `Wind.gust` | var | `WeatherKit.swiftinterface` | Wind::gust |
| `Wind.speed` | var | `WeatherKit.swiftinterface` | Wind::speed |
| `DailyWeatherStatistics` | struct | `WeatherKit.swiftinterface` | DailyWeatherStatistics<T> |
| `DailyWeatherStatistics.baselineStartDate` | var | `WeatherKit.swiftinterface` | DailyWeatherStatistics::baseline_start_date |
| `DailyWeatherStatistics.days` | var | `WeatherKit.swiftinterface` | DailyWeatherStatistics::days |
| `DailyWeatherStatistics.metadata` | var | `WeatherKit.swiftinterface` | DailyWeatherStatistics::metadata |
| `DailyWeatherStatisticsQuery` | struct | `WeatherKit.swiftinterface` | DailyWeatherStatisticsQuery |
| `DailyWeatherStatisticsQuery.precipitation` | var | `WeatherKit.swiftinterface` | DailyWeatherStatisticsQuery::Precipitation |
| `DailyWeatherStatisticsQuery.temperature` | var | `WeatherKit.swiftinterface` | DailyWeatherStatisticsQuery::Temperature |
| `DailyWeatherSummary` | struct | `WeatherKit.swiftinterface` | DailyWeatherSummary<T> |
| `DailyWeatherSummary.days` | var | `WeatherKit.swiftinterface` | DailyWeatherSummary::days |
| `DailyWeatherSummary.metadata` | var | `WeatherKit.swiftinterface` | DailyWeatherSummary::metadata |
| `DailyWeatherSummaryQuery` | struct | `WeatherKit.swiftinterface` | DailyWeatherSummaryQuery |
| `DailyWeatherSummaryQuery.precipitation` | var | `WeatherKit.swiftinterface` | DailyWeatherSummaryQuery::Precipitation |
| `DailyWeatherSummaryQuery.temperature` | var | `WeatherKit.swiftinterface` | DailyWeatherSummaryQuery::Temperature |
| `DayPrecipitationStatistics` | struct | `WeatherKit.swiftinterface` | DayPrecipitationStatistics |
| `DayPrecipitationStatistics.averagePrecipitationAmount` | var | `WeatherKit.swiftinterface` | DayPrecipitationStatistics::average_precipitation_amount |
| `DayPrecipitationStatistics.averagePrecipitationProbability` | var | `WeatherKit.swiftinterface` | DayPrecipitationStatistics::average_precipitation_probability |
| `DayPrecipitationStatistics.averageSnowfallAmount` | var | `WeatherKit.swiftinterface` | DayPrecipitationStatistics::average_snowfall_amount |
| `DayPrecipitationStatistics.day` | var | `WeatherKit.swiftinterface` | DayPrecipitationStatistics::day |
| `DayPrecipitationSummary` | struct | `WeatherKit.swiftinterface` | DayPrecipitationSummary |
| `DayPrecipitationSummary.date` | var | `WeatherKit.swiftinterface` | DayPrecipitationSummary::date |
| `DayPrecipitationSummary.precipitationAmount` | var | `WeatherKit.swiftinterface` | DayPrecipitationSummary::precipitation_amount |
| `DayPrecipitationSummary.snowfallAmount` | var | `WeatherKit.swiftinterface` | DayPrecipitationSummary::snowfall_amount |
| `DayTemperatureStatistics` | struct | `WeatherKit.swiftinterface` | DayTemperatureStatistics |
| `DayTemperatureStatistics.averageHighTemperature` | var | `WeatherKit.swiftinterface` | DayTemperatureStatistics::average_high_temperature |
| `DayTemperatureStatistics.averageLowTemperature` | var | `WeatherKit.swiftinterface` | DayTemperatureStatistics::average_low_temperature |
| `DayTemperatureStatistics.day` | var | `WeatherKit.swiftinterface` | DayTemperatureStatistics::day |
| `DayTemperatureSummary` | struct | `WeatherKit.swiftinterface` | DayTemperatureSummary |
| `DayTemperatureSummary.date` | var | `WeatherKit.swiftinterface` | DayTemperatureSummary::date |
| `DayTemperatureSummary.highTemperature` | var | `WeatherKit.swiftinterface` | DayTemperatureSummary::high_temperature |
| `DayTemperatureSummary.lowTemperature` | var | `WeatherKit.swiftinterface` | DayTemperatureSummary::low_temperature |
| `Deviation` | enum | `WeatherKit.swiftinterface` | Deviation |
| `Forecast.summary` | var | `WeatherKit.swiftinterface` | MinuteForecastCollection::summary |
| `HistoricalComparison` | enum | `WeatherKit.swiftinterface` | HistoricalComparison |
| `HistoricalComparisons` | struct | `WeatherKit.swiftinterface` | HistoricalComparisons |
| `HistoricalComparisons.comparisons` | var | `WeatherKit.swiftinterface` | HistoricalComparisons::comparisons |
| `HistoricalComparisons.metadata` | var | `WeatherKit.swiftinterface` | HistoricalComparisons::metadata |
| `HourTemperatureStatistics` | struct | `WeatherKit.swiftinterface` | HourTemperatureStatistics |
| `HourTemperatureStatistics.hour` | var | `WeatherKit.swiftinterface` | HourTemperatureStatistics::hour |
| `HourTemperatureStatistics.percentiles` | var | `WeatherKit.swiftinterface` | HourTemperatureStatistics::percentiles |
| `HourlyWeatherStatistics` | struct | `WeatherKit.swiftinterface` | HourlyWeatherStatistics<T> |
| `HourlyWeatherStatistics.baselineStartDate` | var | `WeatherKit.swiftinterface` | HourlyWeatherStatistics::baseline_start_date |
| `HourlyWeatherStatistics.hours` | var | `WeatherKit.swiftinterface` | HourlyWeatherStatistics::hours |
| `HourlyWeatherStatistics.metadata` | var | `WeatherKit.swiftinterface` | HourlyWeatherStatistics::metadata |
| `HourlyWeatherStatisticsQuery` | struct | `WeatherKit.swiftinterface` | HourlyWeatherStatisticsQuery |
| `HourlyWeatherStatisticsQuery.temperature` | var | `WeatherKit.swiftinterface` | HourlyWeatherStatisticsQuery::Temperature |
| `MonthPrecipitationStatistics` | struct | `WeatherKit.swiftinterface` | MonthPrecipitationStatistics |
| `MonthPrecipitationStatistics.averagePrecipitationAmount` | var | `WeatherKit.swiftinterface` | MonthPrecipitationStatistics::average_precipitation_amount |
| `MonthPrecipitationStatistics.averagePrecipitationProbability` | var | `WeatherKit.swiftinterface` | MonthPrecipitationStatistics::average_precipitation_probability |
| `MonthPrecipitationStatistics.averageSnowfallAmount` | var | `WeatherKit.swiftinterface` | MonthPrecipitationStatistics::average_snowfall_amount |
| `MonthPrecipitationStatistics.month` | var | `WeatherKit.swiftinterface` | MonthPrecipitationStatistics::month |
| `MonthTemperatureStatistics` | struct | `WeatherKit.swiftinterface` | MonthTemperatureStatistics |
| `MonthTemperatureStatistics.averageHighTemperature` | var | `WeatherKit.swiftinterface` | MonthTemperatureStatistics::average_high_temperature |
| `MonthTemperatureStatistics.averageLowTemperature` | var | `WeatherKit.swiftinterface` | MonthTemperatureStatistics::average_low_temperature |
| `MonthTemperatureStatistics.month` | var | `WeatherKit.swiftinterface` | MonthTemperatureStatistics::month |
| `MonthlyWeatherStatistics` | struct | `WeatherKit.swiftinterface` | MonthlyWeatherStatistics<T> |
| `MonthlyWeatherStatistics.baselineStartDate` | var | `WeatherKit.swiftinterface` | MonthlyWeatherStatistics::baseline_start_date |
| `MonthlyWeatherStatistics.metadata` | var | `WeatherKit.swiftinterface` | MonthlyWeatherStatistics::metadata |
| `MonthlyWeatherStatistics.months` | var | `WeatherKit.swiftinterface` | MonthlyWeatherStatistics::months |
| `MonthlyWeatherStatisticsQuery` | struct | `WeatherKit.swiftinterface` | MonthlyWeatherStatisticsQuery |
| `MonthlyWeatherStatisticsQuery.precipitation` | var | `WeatherKit.swiftinterface` | MonthlyWeatherStatisticsQuery::Precipitation |
| `MonthlyWeatherStatisticsQuery.temperature` | var | `WeatherKit.swiftinterface` | MonthlyWeatherStatisticsQuery::Temperature |
| `Percentiles` | struct | `WeatherKit.swiftinterface` | Percentiles<Unit> |
| `Percentiles.p10` | var | `WeatherKit.swiftinterface` | Percentiles::p10 |
| `Percentiles.p50` | var | `WeatherKit.swiftinterface` | Percentiles::p50 |
| `Percentiles.p90` | var | `WeatherKit.swiftinterface` | Percentiles::p90 |
| `Precipitation.accessibilityDescription` | var | `WeatherKit.swiftinterface` | Precipitation::descriptors() |
| `Precipitation.description` | var | `WeatherKit.swiftinterface` | Precipitation::descriptors() |
| `Trend` | struct | `WeatherKit.swiftinterface` | Trend<Unit> |
| `Trend.baseline` | var | `WeatherKit.swiftinterface` | Trend::baseline |
| `Trend.currentValue` | var | `WeatherKit.swiftinterface` | Trend::current_value |
| `Trend.deviation` | var | `WeatherKit.swiftinterface` | Trend::deviation |
| `TrendBaseline` | struct | `WeatherKit.swiftinterface` | TrendBaseline<Unit> |
| `TrendBaseline.Kind` | enum | `WeatherKit.swiftinterface` | TrendBaselineKind |
| `TrendBaseline.kind` | let | `WeatherKit.swiftinterface` | TrendBaseline::kind |
| `TrendBaseline.startDate` | let | `WeatherKit.swiftinterface` | TrendBaseline::start_date |
| `TrendBaseline.value` | let | `WeatherKit.swiftinterface` | TrendBaseline::value |
| `UVIndex.ExposureCategory` | enum | `WeatherKit.swiftinterface` | UVExposureCategory |
| `UVIndex.ExposureCategory.accessibilityDescription` | var | `WeatherKit.swiftinterface` | UVExposureCategory::descriptors() |
| `UVIndex.ExposureCategory.description` | var | `WeatherKit.swiftinterface` | UVExposureCategory::descriptors() |
| `UVIndex.ExposureCategory.rangeValue` | var | `WeatherKit.swiftinterface` | UVExposureCategory::descriptors() |
| `WeatherChange` | struct | `WeatherKit.swiftinterface` | WeatherChange |
| `WeatherChange.Direction` | enum | `WeatherKit.swiftinterface` | WeatherChangeDirection |
| `WeatherChange.date` | var | `WeatherKit.swiftinterface` | WeatherChange::date |
| `WeatherChange.dayPrecipitationAmount` | var | `WeatherKit.swiftinterface` | WeatherChange::day_precipitation_amount |
| `WeatherChange.highTemperature` | var | `WeatherKit.swiftinterface` | WeatherChange::high_temperature |
| `WeatherChange.lowTemperature` | var | `WeatherKit.swiftinterface` | WeatherChange::low_temperature |
| `WeatherChange.nightPrecipitationAmount` | var | `WeatherKit.swiftinterface` | WeatherChange::night_precipitation_amount |
| `WeatherChanges` | struct | `WeatherKit.swiftinterface` | WeatherChanges |
| `WeatherChanges.changes` | var | `WeatherKit.swiftinterface` | WeatherChanges::changes |
| `WeatherChanges.metadata` | var | `WeatherKit.swiftinterface` | WeatherChanges::metadata |
| `WeatherError` | enum | `WeatherKit.swiftinterface` | WeatherError |
| `WeatherError.errorDescription` | var | `WeatherKit.swiftinterface` | WeatherError::descriptors() |
| `WeatherError.failureReason` | var | `WeatherKit.swiftinterface` | WeatherError::descriptors() |
| `WeatherError.helpAnchor` | var | `WeatherKit.swiftinterface` | WeatherError::descriptors() |
| `WeatherError.recoverySuggestion` | var | `WeatherKit.swiftinterface` | WeatherError::descriptors() |
| `WeatherQuery.changes` | var | `WeatherKit.swiftinterface` | WeatherQuery::Changes |
| `WeatherQuery.historicalComparisons` | var | `WeatherKit.swiftinterface` | WeatherQuery::HistoricalComparisons |
| `WeatherService.dailyStatistics(for:forDaysIn:including:)` | func | `WeatherKit.swiftinterface` | WeatherService::daily_statistics_in |
| `WeatherService.dailyStatistics(for:including:)` | func | `WeatherKit.swiftinterface` | WeatherService::daily_statistics |
| `WeatherService.dailyStatistics(for:startDay:endDay:including:)` | func | `WeatherKit.swiftinterface` | WeatherService::daily_statistics_between_days |
| `WeatherService.dailySummary(for:forDaysIn:including:)` | func | `WeatherKit.swiftinterface` | WeatherService::daily_summary_in |
| `WeatherService.dailySummary(for:including:)` | func | `WeatherKit.swiftinterface` | WeatherService::daily_summary |
| `WeatherService.hourlyStatistics(for:forHoursIn:including:)` | func | `WeatherKit.swiftinterface` | WeatherService::hourly_statistics_in |
| `WeatherService.hourlyStatistics(for:including:)` | func | `WeatherKit.swiftinterface` | WeatherService::hourly_statistics |
| `WeatherService.hourlyStatistics(for:startHour:endHour:including:)` | func | `WeatherKit.swiftinterface` | WeatherService::hourly_statistics_between_hours |
| `WeatherService.monthlyStatistics(for:forMonthsIn:including:)` | func | `WeatherKit.swiftinterface` | WeatherService::monthly_statistics_in |
| `WeatherService.monthlyStatistics(for:including:)` | func | `WeatherKit.swiftinterface` | WeatherService::monthly_statistics |
| `WeatherService.monthlyStatistics(for:startMonth:endMonth:including:)` | func | `WeatherKit.swiftinterface` | WeatherService::monthly_statistics_between_months |
| `WeatherService.weather(for:including:) [variadic]` | func | `WeatherKit.swiftinterface` | WeatherService::weather_including_many |
| `WeatherService.weather(for:including:_:)` | func | `WeatherKit.swiftinterface` | WeatherService::weather_including2 |
| `WeatherService.weather(for:including:_:_:)` | func | `WeatherKit.swiftinterface` | WeatherService::weather_including3 |
| `WeatherService.weather(for:including:_:_:_:)` | func | `WeatherKit.swiftinterface` | WeatherService::weather_including4 |
| `WeatherService.weather(for:including:_:_:_:_:)` | func | `WeatherKit.swiftinterface` | WeatherService::weather_including5 |
| `WeatherService.weather(for:including:_:_:_:_:_:)` | func | `WeatherKit.swiftinterface` | WeatherService::weather_including6 |
| `Wind.CompassDirection` | enum | `WeatherKit.swiftinterface` | WindCompassDirection |
| `Wind.CompassDirection.abbreviation` | var | `WeatherKit.swiftinterface` | WindCompassDirection::descriptors() |
| `Wind.CompassDirection.accessibilityDescription` | var | `WeatherKit.swiftinterface` | WindCompassDirection::descriptors() |
| `Wind.CompassDirection.description` | var | `WeatherKit.swiftinterface` | WindCompassDirection::descriptors() |## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `CloudCoverByAltitude.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `CloudCoverByAltitude.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `CurrentWeather.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `CurrentWeather.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `DailyWeatherStatistics.Element` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `n/a` |
| `DailyWeatherStatistics.Index` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `n/a` |
| `DailyWeatherStatistics.Indices` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 18.0, tvOS 18.0, watchOS 11.0, visionOS 2.0, macOS 15.0, *)` |
| `DailyWeatherStatistics.Iterator` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 18.0, tvOS 18.0, watchOS 11.0, visionOS 2.0, macOS 15.0, *)` |
| `DailyWeatherStatistics.SubSequence` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 18.0, tvOS 18.0, watchOS 11.0, visionOS 2.0, macOS 15.0, *)` |
| `DailyWeatherStatistics.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `DailyWeatherStatistics.endIndex` | var | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `DailyWeatherStatistics.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `DailyWeatherStatistics.startIndex` | var | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `DailyWeatherStatistics.subscript(position:)` | subscript | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `DailyWeatherSummary.Element` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `n/a` |
| `DailyWeatherSummary.Index` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `n/a` |
| `DailyWeatherSummary.Indices` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 18.0, tvOS 18.0, watchOS 11.0, visionOS 2.0, macOS 15.0, *)` |
| `DailyWeatherSummary.Iterator` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 18.0, tvOS 18.0, watchOS 11.0, visionOS 2.0, macOS 15.0, *)` |
| `DailyWeatherSummary.SubSequence` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 18.0, tvOS 18.0, watchOS 11.0, visionOS 2.0, macOS 15.0, *)` |
| `DailyWeatherSummary.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `DailyWeatherSummary.endIndex` | var | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `DailyWeatherSummary.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `DailyWeatherSummary.startIndex` | var | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `DailyWeatherSummary.subscript(position:)` | subscript | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `DayPrecipitationStatistics.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `DayPrecipitationStatistics.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `DayPrecipitationSummary.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `DayPrecipitationSummary.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `DayTemperatureStatistics.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `DayTemperatureStatistics.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `DayTemperatureSummary.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `DayTemperatureSummary.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `DayWeather.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `DayWeather.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `DayWeather.precipitationAmount` | var | `WeatherKit.swiftinterface` | macOS-deprecated API; intentionally skipped by this audit | `@backDeployed(before: iOS 16.4, macOS 13.3, tvOS 16.4, watchOS 9.4) @available(iOS, introduced: 16.0, deprecated: 18.0, message: "Use precipitationAmountByType") @available(macOS, introduced: 13.0, deprecated: 15.0, message: "Use precipitationAmountByType") @available(tvOS, introduced: 16.0, deprecated: 18.0, message: "Use precipitationAmountByType") @available(watchOS, introduced: 9.0, deprecated: 11.0, message: "Use precipitationAmountByType")` |
| `DayWeather.rainfallAmount` | var | `WeatherKit.swiftinterface` | macOS-deprecated API; intentionally skipped by this audit | `@available(iOS, introduced: 16.0, deprecated: 16.4, message: "Use precipitationAmountByType") @available(macOS, introduced: 13.0, deprecated: 13.3, message: "Use precipitationAmountByType") @available(tvOS, introduced: 16.0, deprecated: 16.4, message: "Use precipitationAmountByType") @available(watchOS, introduced: 9.0, deprecated: 9.4, message: "Use precipitationAmountByType")` |
| `DayWeather.snowfallAmount` | var | `WeatherKit.swiftinterface` | macOS-deprecated API; intentionally skipped by this audit | `@available(iOS, introduced: 16.0, deprecated: 18.0, message: "Use precipitationAmountByType") @available(macOS, introduced: 13.0, deprecated: 15.0, message: "Use precipitationAmountByType") @available(tvOS, introduced: 16.0, deprecated: 18.0, message: "Use precipitationAmountByType") @available(watchOS, introduced: 9.0, deprecated: 11.0, message: "Use precipitationAmountByType")` |
| `Deviation.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `Deviation.hash(into:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `Deviation.hashValue` | var | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `Deviation.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `Forecast.Index` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `n/a` |
| `Forecast.Indices` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 16.0, tvOS 16.0, watchOS 9.0, macOS 13.0, *)` |
| `Forecast.Iterator` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 16.0, tvOS 16.0, watchOS 9.0, macOS 13.0, *)` |
| `Forecast.SubSequence` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 16.0, tvOS 16.0, watchOS 9.0, macOS 13.0, *)` |
| `Forecast.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `Forecast.endIndex` | var | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `Forecast.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `Forecast.startIndex` | var | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `Forecast.subscript(position:)` | subscript | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `HistoricalComparison.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `HistoricalComparison.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `HistoricalComparisons.Element` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `n/a` |
| `HistoricalComparisons.Index` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `n/a` |
| `HistoricalComparisons.Indices` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 18.0, tvOS 18.0, watchOS 11.0, visionOS 2.0, macOS 15.0, *)` |
| `HistoricalComparisons.Iterator` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 18.0, tvOS 18.0, watchOS 11.0, visionOS 2.0, macOS 15.0, *)` |
| `HistoricalComparisons.SubSequence` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 18.0, tvOS 18.0, watchOS 11.0, visionOS 2.0, macOS 15.0, *)` |
| `HistoricalComparisons.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `HistoricalComparisons.endIndex` | var | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `HistoricalComparisons.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `HistoricalComparisons.startIndex` | var | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `HistoricalComparisons.subscript(position:)` | subscript | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `HourTemperatureStatistics.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `HourTemperatureStatistics.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `HourWeather.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `HourWeather.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `HourlyWeatherStatistics.Element` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `n/a` |
| `HourlyWeatherStatistics.Index` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `n/a` |
| `HourlyWeatherStatistics.Indices` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 18.0, tvOS 18.0, watchOS 11.0, visionOS 2.0, macOS 15.0, *)` |
| `HourlyWeatherStatistics.Iterator` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 18.0, tvOS 18.0, watchOS 11.0, visionOS 2.0, macOS 15.0, *)` |
| `HourlyWeatherStatistics.SubSequence` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 18.0, tvOS 18.0, watchOS 11.0, visionOS 2.0, macOS 15.0, *)` |
| `HourlyWeatherStatistics.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `HourlyWeatherStatistics.endIndex` | var | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `HourlyWeatherStatistics.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `HourlyWeatherStatistics.startIndex` | var | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `HourlyWeatherStatistics.subscript(position:)` | subscript | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `MinuteWeather.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `MinuteWeather.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `MonthPrecipitationStatistics.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `MonthPrecipitationStatistics.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `MonthTemperatureStatistics.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `MonthTemperatureStatistics.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `MonthlyWeatherStatistics.Element` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `n/a` |
| `MonthlyWeatherStatistics.Index` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `n/a` |
| `MonthlyWeatherStatistics.Indices` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 18.0, tvOS 18.0, watchOS 11.0, visionOS 2.0, macOS 15.0, *)` |
| `MonthlyWeatherStatistics.Iterator` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 18.0, tvOS 18.0, watchOS 11.0, visionOS 2.0, macOS 15.0, *)` |
| `MonthlyWeatherStatistics.SubSequence` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 18.0, tvOS 18.0, watchOS 11.0, visionOS 2.0, macOS 15.0, *)` |
| `MonthlyWeatherStatistics.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `MonthlyWeatherStatistics.endIndex` | var | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `MonthlyWeatherStatistics.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `MonthlyWeatherStatistics.startIndex` | var | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `MonthlyWeatherStatistics.subscript(position:)` | subscript | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `MoonEvents.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `MoonEvents.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `MoonPhase.AllCases` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 16.0, tvOS 16.0, watchOS 9.0, macOS 13.0, *)` |
| `MoonPhase.RawValue` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 16.0, tvOS 16.0, watchOS 9.0, macOS 13.0, *)` |
| `MoonPhase.allCases` | var | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `MoonPhase.init?(rawValue:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `MoonPhase.rawValue` | var | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `Percentiles.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `Percentiles.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `Precipitation.AllCases` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 16.0, tvOS 16.0, watchOS 9.0, macOS 13.0, *)` |
| `Precipitation.RawValue` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 16.0, tvOS 16.0, watchOS 9.0, macOS 13.0, *)` |
| `Precipitation.allCases` | var | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `Precipitation.init?(rawValue:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `Precipitation.rawValue` | var | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `PrecipitationAmountByType.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `PrecipitationAmountByType.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `PressureTrend.AllCases` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 16.0, tvOS 16.0, watchOS 9.0, macOS 13.0, *)` |
| `PressureTrend.RawValue` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 16.0, tvOS 16.0, watchOS 9.0, macOS 13.0, *)` |
| `PressureTrend.allCases` | var | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `PressureTrend.init?(rawValue:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `PressureTrend.rawValue` | var | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `SnowfallAmount.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `SnowfallAmount.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `SunEvents.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `SunEvents.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `Trend.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `Trend.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `TrendBaseline.Kind.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `TrendBaseline.Kind.hash(into:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `TrendBaseline.Kind.hashValue` | var | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `TrendBaseline.Kind.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `TrendBaseline.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `TrendBaseline.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `UVIndex.ExposureCategory.AllCases` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 16.0, tvOS 16.0, watchOS 9.0, macOS 13.0, *)` |
| `UVIndex.ExposureCategory.RawValue` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 16.0, tvOS 16.0, watchOS 9.0, macOS 13.0, *)` |
| `UVIndex.ExposureCategory.allCases` | var | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `UVIndex.ExposureCategory.init?(rawValue:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `UVIndex.ExposureCategory.rawValue` | var | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `UVIndex.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `UVIndex.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `Weather.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `Weather.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `WeatherAlert.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `WeatherAlert.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `WeatherAttribution.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `WeatherAttribution.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `WeatherAvailability.AvailabilityKind.RawValue` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 16.0, tvOS 16.0, watchOS 9.0, macOS 13.0, *)` |
| `WeatherAvailability.AvailabilityKind.init?(rawValue:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `WeatherAvailability.AvailabilityKind.rawValue` | var | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `WeatherAvailability.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `WeatherAvailability.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `WeatherChange.Direction.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `WeatherChange.Direction.hash(into:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `WeatherChange.Direction.hashValue` | var | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `WeatherChange.Direction.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `WeatherChange.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `WeatherChange.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `WeatherChanges.Element` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `n/a` |
| `WeatherChanges.Index` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `n/a` |
| `WeatherChanges.Indices` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 18.0, tvOS 18.0, watchOS 11.0, visionOS 2.0, macOS 15.0, *)` |
| `WeatherChanges.Iterator` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 18.0, tvOS 18.0, watchOS 11.0, visionOS 2.0, macOS 15.0, *)` |
| `WeatherChanges.SubSequence` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 18.0, tvOS 18.0, watchOS 11.0, visionOS 2.0, macOS 15.0, *)` |
| `WeatherChanges.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `WeatherChanges.endIndex` | var | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `WeatherChanges.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `WeatherChanges.startIndex` | var | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `WeatherChanges.subscript(position:)` | subscript | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `WeatherCondition.AllCases` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 16.0, tvOS 16.0, watchOS 9.0, macOS 13.0, *)` |
| `WeatherCondition.RawValue` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 16.0, tvOS 16.0, watchOS 9.0, macOS 13.0, *)` |
| `WeatherCondition.allCases` | var | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `WeatherCondition.init?(rawValue:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `WeatherCondition.rawValue` | var | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `WeatherError.hash(into:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `WeatherError.hashValue` | var | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `WeatherMetadata.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `WeatherMetadata.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `WeatherSeverity.AllCases` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 16.0, tvOS 16.0, watchOS 9.0, macOS 13.0, *)` |
| `WeatherSeverity.RawValue` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 16.0, tvOS 16.0, watchOS 9.0, macOS 13.0, *)` |
| `WeatherSeverity.allCases` | var | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `WeatherSeverity.init?(rawValue:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `WeatherSeverity.rawValue` | var | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `Wind.CompassDirection.AllCases` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 16.0, tvOS 16.0, watchOS 9.0, macOS 13.0, *)` |
| `Wind.CompassDirection.RawValue` | typealias | `WeatherKit.swiftinterface` | Swift helper typealias; not a standalone WeatherKit capability in Rust | `@available(iOS 16.0, tvOS 16.0, watchOS 9.0, macOS 13.0, *)` |
| `Wind.CompassDirection.allCases` | var | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `Wind.CompassDirection.init?(rawValue:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `Wind.CompassDirection.rawValue` | var | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `Wind.encode(to:)` | func | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |
| `Wind.init(from:)` | init | `WeatherKit.swiftinterface` | Swift protocol witness / collection glue; not tracked as a separate Rust API | `n/a` |