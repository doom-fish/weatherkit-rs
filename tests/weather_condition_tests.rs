use weatherkit::prelude::*;

#[test]
fn weather_condition_descriptors_cover_clear() {
    let descriptors = WeatherCondition::descriptors().expect("weather condition descriptors");
    assert!(descriptors
        .iter()
        .any(|descriptor| descriptor.raw_value == "clear"));
    assert!(descriptors
        .iter()
        .any(|descriptor| descriptor.raw_value == "rain"));
}
