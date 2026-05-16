mod common;

use weatherkit::prelude::*;

#[test]
fn weather_changes_and_historical_comparisons_smoke_or_entitlement() {
    let service = WeatherService::shared();
    let location = common::sample_location();

    let changes = common::entitlement_ok(service.weather_changes(&location));
    if let Some(Some(changes)) = changes {
        let _ = changes.iter().next();
    }

    let comparisons = common::entitlement_ok(service.historical_comparisons(&location));
    if let Some(Some(comparisons)) = comparisons {
        if let Some(comparison) = comparisons.iter().next() {
            match comparison {
                HistoricalComparison::HighTemperature(trend)
                | HistoricalComparison::LowTemperature(trend) => {
                    assert!(!trend.baseline.start_date.is_empty());
                }
                HistoricalComparison::PrecipitationAmount(trend)
                | HistoricalComparison::SnowfallAmount(trend) => {
                    assert!(!trend.baseline.start_date.is_empty());
                }
            }
        }
    }
}
