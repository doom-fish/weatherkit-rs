use weatherkit::prelude::*;

#[test]
fn enum_and_error_descriptors_cover_public_cases() {
    let precipitation = Precipitation::descriptors().expect("precipitation descriptors");
    assert!(precipitation.iter().any(|descriptor| descriptor.raw_value == "rain"));

    let compass = WindCompassDirection::descriptors().expect("wind compass descriptors");
    let north = compass
        .iter()
        .find(|descriptor| descriptor.raw_value == "north")
        .expect("north descriptor");
    assert_eq!(north.abbreviation, "N");

    let uv = UVExposureCategory::descriptors().expect("UV exposure descriptors");
    let extreme = uv
        .iter()
        .find(|descriptor| descriptor.raw_value == "extreme")
        .expect("extreme descriptor");
    assert!(extreme.range_end >= extreme.range_start);

    let errors = WeatherError::descriptors().expect("weather error descriptors");
    assert!(errors.iter().any(|descriptor| descriptor.raw_value == "permissionDenied"));
}
