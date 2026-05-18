//! WeatherKit current weather types.

use core::ffi::c_void;

use serde::Deserialize;

use crate::error::WeatherKitError;
use crate::ffi;
use crate::pressure::{deserialize_pressure_trend, Pressure, PressureTrend};
use crate::private::{parse_json_from_handle, parse_json_from_static};
use crate::service::WeatherMetadata;
use crate::weather_condition::{deserialize_weather_condition, WeatherCondition};

/// Represents WeatherKit cloud cover by altitude.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudCoverByAltitude {
    /// Matches the WeatherKit low value.
    pub low: f64,
    /// Matches the WeatherKit medium value.
    pub medium: f64,
    /// Matches the WeatherKit high value.
    pub high: f64,
}

/// Represents the WeatherKit `WindCompassDirection` value.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum WindCompassDirection {
    /// Matches the WeatherKit `North` case.
    North,
    /// Matches the WeatherKit `NorthNortheast` case.
    NorthNortheast,
    /// Matches the WeatherKit `Northeast` case.
    Northeast,
    /// Matches the WeatherKit `EastNortheast` case.
    EastNortheast,
    /// Matches the WeatherKit `East` case.
    East,
    /// Matches the WeatherKit `EastSoutheast` case.
    EastSoutheast,
    /// Matches the WeatherKit `Southeast` case.
    Southeast,
    /// Matches the WeatherKit `SouthSoutheast` case.
    SouthSoutheast,
    /// Matches the WeatherKit `South` case.
    South,
    /// Matches the WeatherKit `SouthSouthwest` case.
    SouthSouthwest,
    /// Matches the WeatherKit `Southwest` case.
    Southwest,
    /// Matches the WeatherKit `WestSouthwest` case.
    WestSouthwest,
    /// Matches the WeatherKit `West` case.
    West,
    /// Matches the WeatherKit `WestNorthwest` case.
    WestNorthwest,
    /// Matches the WeatherKit `Northwest` case.
    Northwest,
    /// Matches the WeatherKit `NorthNorthwest` case.
    NorthNorthwest,
    /// Stores an unrecognized WeatherKit case name.
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

    /// Returns the WeatherKit raw value for this case.
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

    /// Returns the WeatherKit descriptor catalog for this enum.
    pub fn descriptors() -> Result<Vec<WindCompassDirectionDescriptor>, WeatherKitError> {
        parse_json_from_static(
            ffi::current_weather::wk_wind_compass_direction_copy_descriptors_json,
            "wind compass direction descriptors",
        )
    }
}

/// Represents the WeatherKit `WindCompassDirectionDescriptor` payload.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindCompassDirectionDescriptor {
    /// Matches the WeatherKit raw value value.
    pub raw_value: String,
    /// Matches the WeatherKit abbreviation value.
    pub abbreviation: String,
    /// Matches the WeatherKit description value.
    pub description: String,
    /// Matches the WeatherKit accessibility description value.
    pub accessibility_description: String,
}

/// Represents a WeatherKit wind reading.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wind {
    /// Matches the WeatherKit speed value.
    pub speed: f64,
    /// Matches the WeatherKit direction value.
    pub direction: f64,
    /// Matches the WeatherKit compass direction value.
    pub compass_direction: String,
    /// Matches the WeatherKit gust value.
    pub gust: Option<f64>,
}

impl Wind {
    /// Returns the WeatherKit compass direction enum for this wind.
    pub fn compass_direction_kind(&self) -> WindCompassDirection {
        WindCompassDirection::from_raw(self.compass_direction.clone())
    }
}

/// Represents the WeatherKit `UVExposureCategory` value.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum UVExposureCategory {
    /// Matches the WeatherKit `Low` case.
    Low,
    /// Matches the WeatherKit `Moderate` case.
    Moderate,
    /// Matches the WeatherKit `High` case.
    High,
    /// Matches the WeatherKit `VeryHigh` case.
    VeryHigh,
    /// Matches the WeatherKit `Extreme` case.
    Extreme,
    /// Stores an unrecognized WeatherKit case name.
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

    /// Returns the WeatherKit raw value for this case.
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

    /// Returns the WeatherKit descriptor catalog for this enum.
    pub fn descriptors() -> Result<Vec<UVExposureCategoryDescriptor>, WeatherKitError> {
        parse_json_from_static(
            ffi::current_weather::wk_uv_exposure_category_copy_descriptors_json,
            "UV exposure category descriptors",
        )
    }
}

/// Represents the WeatherKit `UVExposureCategoryDescriptor` payload.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UVExposureCategoryDescriptor {
    /// Matches the WeatherKit raw value value.
    pub raw_value: String,
    /// Matches the WeatherKit description value.
    pub description: String,
    /// Matches the WeatherKit accessibility description value.
    pub accessibility_description: String,
    /// Matches the WeatherKit range start value.
    pub range_start: i64,
    /// Matches the WeatherKit range end value.
    pub range_end: i64,
}

/// Represents a WeatherKit UV index reading.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UVIndex {
    /// Matches the WeatherKit value value.
    pub value: i64,
    /// Matches the WeatherKit category value.
    pub category: String,
}

impl UVIndex {
    /// Returns the WeatherKit exposure category for this index.
    pub fn exposure_category(&self) -> UVExposureCategory {
        UVExposureCategory::from_raw(self.category.clone())
    }
}

/// Wraps the WeatherKit current weather payload.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentWeather {
    /// Matches the WeatherKit date value.
    pub date: String,
    /// Matches the WeatherKit temperature value.
    pub temperature: f64,
    /// Matches the WeatherKit feels like value.
    pub feels_like: f64,
    /// Matches the WeatherKit humidity value.
    pub humidity: f64,
    /// Matches the WeatherKit dew point value.
    pub dew_point: f64,
    /// Matches the WeatherKit pressure value.
    pub pressure: f64,
    /// Matches the WeatherKit pressure trend value.
    #[serde(deserialize_with = "deserialize_pressure_trend")]
    pub pressure_trend: PressureTrend,
    /// Matches the WeatherKit condition value.
    #[serde(deserialize_with = "deserialize_weather_condition")]
    pub condition: WeatherCondition,
    /// Matches the WeatherKit symbol name value.
    pub symbol_name: String,
    /// Matches the WeatherKit wind value.
    pub wind: Wind,
    /// Matches the WeatherKit uv index value.
    pub uv_index: UVIndex,
    /// Matches the WeatherKit visibility value.
    pub visibility: f64,
    /// Matches the WeatherKit cloud cover value.
    pub cloud_cover: f64,
    /// Matches the WeatherKit cloud cover by altitude value.
    #[serde(default)]
    pub cloud_cover_by_altitude: Option<CloudCoverByAltitude>,
    /// Matches the WeatherKit is daylight value.
    pub is_daylight: bool,
    /// Matches the WeatherKit precipitation intensity value.
    pub precipitation_intensity: f64,
    /// Matches the WeatherKit metadata value.
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

    /// Wraps the WeatherKit pressure value and trend.
    pub fn pressure_reading(&self) -> Pressure {
        Pressure::new(self.pressure, self.pressure_trend.clone())
    }
}
