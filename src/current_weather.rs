use core::ffi::c_void;

use serde::Deserialize;

use crate::error::WeatherKitError;
use crate::ffi;
use crate::pressure::{deserialize_pressure_trend, Pressure, PressureTrend};
use crate::private::{parse_json_from_handle, parse_json_from_static};
use crate::service::WeatherMetadata;
use crate::weather_condition::{deserialize_weather_condition, WeatherCondition};

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudCoverByAltitude {
    pub low: f64,
    pub medium: f64,
    pub high: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum WindCompassDirection {
    North,
    NorthNortheast,
    Northeast,
    EastNortheast,
    East,
    EastSoutheast,
    Southeast,
    SouthSoutheast,
    South,
    SouthSouthwest,
    Southwest,
    WestSouthwest,
    West,
    WestNorthwest,
    Northwest,
    NorthNorthwest,
    Unknown(String),
}

impl WindCompassDirection {
    pub(crate) fn from_raw(value: String) -> Self {
        match value.as_str() {
            "north" => Self::North,
            "northNortheast" => Self::NorthNortheast,
            "northeast" => Self::Northeast,
            "eastNortheast" => Self::EastNortheast,
            "east" => Self::East,
            "eastSoutheast" => Self::EastSoutheast,
            "southeast" => Self::Southeast,
            "southSoutheast" => Self::SouthSoutheast,
            "south" => Self::South,
            "southSouthwest" => Self::SouthSouthwest,
            "southwest" => Self::Southwest,
            "westSouthwest" => Self::WestSouthwest,
            "west" => Self::West,
            "westNorthwest" => Self::WestNorthwest,
            "northwest" => Self::Northwest,
            "northNorthwest" => Self::NorthNorthwest,
            other => Self::Unknown(other.to_owned()),
        }
    }

    pub fn raw_value(&self) -> &str {
        match self {
            Self::North => "north",
            Self::NorthNortheast => "northNortheast",
            Self::Northeast => "northeast",
            Self::EastNortheast => "eastNortheast",
            Self::East => "east",
            Self::EastSoutheast => "eastSoutheast",
            Self::Southeast => "southeast",
            Self::SouthSoutheast => "southSoutheast",
            Self::South => "south",
            Self::SouthSouthwest => "southSouthwest",
            Self::Southwest => "southwest",
            Self::WestSouthwest => "westSouthwest",
            Self::West => "west",
            Self::WestNorthwest => "westNorthwest",
            Self::Northwest => "northwest",
            Self::NorthNorthwest => "northNorthwest",
            Self::Unknown(value) => value.as_str(),
        }
    }

    pub fn descriptors() -> Result<Vec<WindCompassDirectionDescriptor>, WeatherKitError> {
        parse_json_from_static(
            ffi::current_weather::wk_wind_compass_direction_copy_descriptors_json,
            "wind compass direction descriptors",
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindCompassDirectionDescriptor {
    pub raw_value: String,
    pub abbreviation: String,
    pub description: String,
    pub accessibility_description: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wind {
    pub speed: f64,
    pub direction: f64,
    pub compass_direction: String,
    pub gust: Option<f64>,
}

impl Wind {
    pub fn compass_direction_kind(&self) -> WindCompassDirection {
        WindCompassDirection::from_raw(self.compass_direction.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum UVExposureCategory {
    Low,
    Moderate,
    High,
    VeryHigh,
    Extreme,
    Unknown(String),
}

impl UVExposureCategory {
    pub(crate) fn from_raw(value: String) -> Self {
        match value.as_str() {
            "low" => Self::Low,
            "moderate" => Self::Moderate,
            "high" => Self::High,
            "veryHigh" => Self::VeryHigh,
            "extreme" => Self::Extreme,
            other => Self::Unknown(other.to_owned()),
        }
    }

    pub fn raw_value(&self) -> &str {
        match self {
            Self::Low => "low",
            Self::Moderate => "moderate",
            Self::High => "high",
            Self::VeryHigh => "veryHigh",
            Self::Extreme => "extreme",
            Self::Unknown(value) => value.as_str(),
        }
    }

    pub fn descriptors() -> Result<Vec<UVExposureCategoryDescriptor>, WeatherKitError> {
        parse_json_from_static(
            ffi::current_weather::wk_uv_exposure_category_copy_descriptors_json,
            "UV exposure category descriptors",
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UVExposureCategoryDescriptor {
    pub raw_value: String,
    pub description: String,
    pub accessibility_description: String,
    pub range_start: i64,
    pub range_end: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UVIndex {
    pub value: i64,
    pub category: String,
}

impl UVIndex {
    pub fn exposure_category(&self) -> UVExposureCategory {
        UVExposureCategory::from_raw(self.category.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentWeather {
    pub date: String,
    pub temperature: f64,
    pub feels_like: f64,
    pub humidity: f64,
    pub dew_point: f64,
    pub pressure: f64,
    #[serde(deserialize_with = "deserialize_pressure_trend")]
    pub pressure_trend: PressureTrend,
    #[serde(deserialize_with = "deserialize_weather_condition")]
    pub condition: WeatherCondition,
    pub symbol_name: String,
    pub wind: Wind,
    pub uv_index: UVIndex,
    pub visibility: f64,
    pub cloud_cover: f64,
    #[serde(default)]
    pub cloud_cover_by_altitude: Option<CloudCoverByAltitude>,
    pub is_daylight: bool,
    pub precipitation_intensity: f64,
    pub metadata: WeatherMetadata,
}

impl CurrentWeather {
    pub(crate) fn from_owned_ptr(ptr: *mut c_void) -> Result<Self, WeatherKitError> {
        parse_json_from_handle(
            ptr,
            ffi::current_weather::wk_current_weather_release,
            ffi::current_weather::wk_current_weather_copy_json,
            "current weather",
        )
    }

    pub fn pressure_reading(&self) -> Pressure {
        Pressure::new(self.pressure, self.pressure_trend.clone())
    }
}
