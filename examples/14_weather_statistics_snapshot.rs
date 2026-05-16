#[path = "support/mod.rs"]
mod support;

use std::error::Error;
use std::time::{Duration, SystemTime};

use weatherkit::prelude::*;

fn sample_interval(days: u64) -> DateInterval {
    let start = SystemTime::now();
    let end = start + Duration::from_secs(days * 24 * 60 * 60);
    DateInterval::new(start, end).expect("valid date interval")
}

fn main() -> Result<(), Box<dyn Error>> {
    let service = WeatherService::shared();
    let location = support::sample_location();

    if let Some(result) = support::handle_result(
        "daily statistics",
        service.daily_statistics(&location, DailyWeatherStatisticsQuery::Temperature),
    )? {
        match result {
            DailyWeatherStatisticsResult::Temperature(stats) => {
                println!("daily_temperature_stats={}", stats.len());
            }
            DailyWeatherStatisticsResult::Precipitation(stats) => {
                println!("daily_precipitation_stats={}", stats.len());
            }
        }
    }

    if let Some(result) = support::handle_result(
        "daily summary",
        service.daily_summary_in(
            &location,
            sample_interval(7),
            DailyWeatherSummaryQuery::Precipitation,
        ),
    )? {
        match result {
            DailyWeatherSummaryResult::Temperature(days) => {
                println!("daily_temperature_summary={}", days.len());
            }
            DailyWeatherSummaryResult::Precipitation(days) => {
                println!("daily_precipitation_summary={}", days.len());
            }
        }
    }

    if let Some(hourly) = support::handle_result(
        "hourly statistics",
        service.hourly_statistics(&location, HourlyWeatherStatisticsQuery::Temperature),
    )? {
        println!("hourly_temperature_stats={}", hourly.len());
    }

    if let Some(result) = support::handle_result(
        "monthly statistics",
        service.monthly_statistics_in(
            &location,
            sample_interval(120),
            MonthlyWeatherStatisticsQuery::Precipitation,
        ),
    )? {
        match result {
            MonthlyWeatherStatisticsResult::Temperature(stats) => {
                println!("monthly_temperature_stats={}", stats.len());
            }
            MonthlyWeatherStatisticsResult::Precipitation(stats) => {
                println!("monthly_precipitation_stats={}", stats.len());
            }
        }
    }

    support::finish("weather statistics");
    Ok(())
}
