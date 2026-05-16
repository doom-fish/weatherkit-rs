mod common;

use weatherkit::prelude::*;

#[test]
fn sun_events_smoke_or_entitlement() {
    let service = WeatherService::shared();
    if let Some(sun) = common::entitlement_ok(service.sun_events(&common::sample_location())) {
        assert!(sun.sunrise.is_some() || sun.sunset.is_some() || sun.solar_noon.is_some());
    }
}
