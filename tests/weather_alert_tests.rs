mod common;

use weatherkit::prelude::*;

#[test]
fn weather_alert_descriptors_and_fetch_work() {
    let descriptors = WeatherSeverity::descriptors().expect("weather severity descriptors");
    assert!(descriptors
        .iter()
        .any(|descriptor| descriptor.raw_value == "unknown"));

    let service = WeatherService::shared();
    if let Some(alerts) = common::entitlement_ok(service.weather_alerts(&common::sample_location()))
    {
        for alert in alerts {
            let _ = alert.severity_kind();
        }
    }
}
