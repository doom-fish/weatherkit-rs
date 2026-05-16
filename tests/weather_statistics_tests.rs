mod common;

use std::time::{Duration, SystemTime};

use weatherkit::prelude::*;

type HourlyStatsIntervalFn = fn(
    &WeatherService,
    &CLLocation,
    DateInterval,
    HourlyWeatherStatisticsQuery,
) -> Result<HourlyWeatherStatistics<HourTemperatureStatistics>, WeatherKitError>;
type HourlyStatsRangeFn = fn(
    &WeatherService,
    &CLLocation,
    i64,
    i64,
    HourlyWeatherStatisticsQuery,
) -> Result<HourlyWeatherStatistics<HourTemperatureStatistics>, WeatherKitError>;

fn sample_interval(days: u64) -> DateInterval {
    let start = SystemTime::now();
    let end = start + Duration::from_secs(days * 24 * 60 * 60);
    DateInterval::new(start, end).expect("valid date interval")
}

#[test]
fn weather_statistics_and_summaries_smoke_or_entitlement() {
    let _: fn(
        &WeatherService,
        &CLLocation,
        DateInterval,
        DailyWeatherStatisticsQuery,
    ) -> Result<DailyWeatherStatisticsResult, WeatherKitError> = WeatherService::daily_statistics_in;
    let _: fn(
        &WeatherService,
        &CLLocation,
        i64,
        i64,
        DailyWeatherStatisticsQuery,
    ) -> Result<DailyWeatherStatisticsResult, WeatherKitError> =
        WeatherService::daily_statistics_between_days;
    let _: fn(
        &WeatherService,
        &CLLocation,
        DateInterval,
        DailyWeatherSummaryQuery,
    ) -> Result<DailyWeatherSummaryResult, WeatherKitError> = WeatherService::daily_summary_in;
    let _: HourlyStatsIntervalFn = WeatherService::hourly_statistics_in;
    let _: HourlyStatsRangeFn = WeatherService::hourly_statistics_between_hours;
    let _: fn(
        &WeatherService,
        &CLLocation,
        DateInterval,
        MonthlyWeatherStatisticsQuery,
    ) -> Result<MonthlyWeatherStatisticsResult, WeatherKitError> =
        WeatherService::monthly_statistics_in;
    let _: fn(
        &WeatherService,
        &CLLocation,
        i64,
        i64,
        MonthlyWeatherStatisticsQuery,
    ) -> Result<MonthlyWeatherStatisticsResult, WeatherKitError> =
        WeatherService::monthly_statistics_between_months;
    let _ = sample_interval(1);

    let service = WeatherService::shared();
    let location = common::sample_location();

    let daily = common::entitlement_ok(service.daily_statistics(
        &location,
        DailyWeatherStatisticsQuery::Temperature,
    ));
    if let Some(DailyWeatherStatisticsResult::Temperature(stats)) = daily {
        let _ = stats.iter().next();
    }

    let summary = common::entitlement_ok(service.daily_summary(
        &location,
        DailyWeatherSummaryQuery::Temperature,
    ));
    if let Some(DailyWeatherSummaryResult::Temperature(days)) = summary {
        let _ = days.iter().next();
    }

    let hourly = common::entitlement_ok(service.hourly_statistics(
        &location,
        HourlyWeatherStatisticsQuery::Temperature,
    ));
    if let Some(stats) = hourly {
        let _ = stats.iter().next();
    }

    let monthly = common::entitlement_ok(service.monthly_statistics(
        &location,
        MonthlyWeatherStatisticsQuery::Temperature,
    ));
    if let Some(MonthlyWeatherStatisticsResult::Temperature(stats)) = monthly {
        let _ = stats.iter().next();
    }
}
