#[path = "support/mod.rs"]
mod support;

use std::error::Error;

use weatherkit::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let service = WeatherService::shared();
    let location = support::sample_location();

    if let Some((current, daily, changes)) = support::handle_result(
        "weather multi query",
        service.weather_including3(
            &location,
            WeatherQuery::Current,
            WeatherQuery::Daily,
            WeatherQuery::Changes,
        ),
    )? {
        if let WeatherQueryResult::CurrentWeather(current) = current {
            println!("multi_query_current_temperature={:.1}", current.temperature);
        }
        if let WeatherQueryResult::DailyForecast(daily) = daily {
            println!("multi_query_daily_days={}", daily.len());
        }
        if let WeatherQueryResult::WeatherChanges(changes) = changes {
            println!("multi_query_changes_present={}", changes.is_some());
        }
    }

    support::finish("weather multi query");
    Ok(())
}
