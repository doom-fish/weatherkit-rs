mod common;

use weatherkit::prelude::*;

#[test]
fn weather_attribution_smoke_or_entitlement() {
    let service = WeatherService::shared();
    if let Some(attribution) = common::entitlement_ok(service.attribution()) {
        assert!(!attribution.service_name.is_empty());
        assert!(attribution.legal_page_url.starts_with("http"));
    }
}
