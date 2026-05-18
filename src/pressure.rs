//! WeatherKit pressure types.

use serde::{Deserialize, Deserializer};

use crate::error::WeatherKitError;
use crate::ffi;
use crate::private::parse_json_from_static;

/// Represents the WeatherKit `PressureTrend` value.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PressureTrend {
    /// Matches the WeatherKit `Rising` case.
    Rising,
    /// Matches the WeatherKit `Falling` case.
    Falling,
    /// Matches the WeatherKit `Steady` case.
    Steady,
    /// Stores an unrecognized WeatherKit case name.
    Unknown(String),
}

impl PressureTrend {
    pub(crate) fn from_raw(value: String) -> Self {
        match value.as_str() {
            "rising" => Self::Rising,
            "falling" => Self::Falling,
            "steady" => Self::Steady,
            other => Self::Unknown(other.to_owned()),
        }
    }

    /// Returns the WeatherKit raw value for this case.
    pub fn raw_value(&self) -> &str {
        match self {
            Self::Rising => "rising",
            Self::Falling => "falling",
            Self::Steady => "steady",
            Self::Unknown(value) => value.as_str(),
        }
    }

    /// Returns the WeatherKit descriptor catalog for this enum.
    pub fn descriptors() -> Result<Vec<PressureTrendDescriptor>, WeatherKitError> {
        parse_json_from_static(
            ffi::pressure::wk_pressure_trend_copy_descriptors_json,
            "pressure trend descriptors",
        )
    }
}

/// Represents a WeatherKit pressure reading.
#[derive(Debug, Clone, PartialEq)]
pub struct Pressure {
    /// Matches the WeatherKit value value.
    pub value: f64,
    /// Matches the WeatherKit trend value.
    pub trend: PressureTrend,
}

impl Pressure {
    /// Creates a WeatherKit pressure reading from a value and trend.
    pub const fn new(value: f64, trend: PressureTrend) -> Self {
        Self { value, trend }
    }
}

/// Represents the WeatherKit `PressureTrendDescriptor` payload.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PressureTrendDescriptor {
    /// Matches the WeatherKit raw value value.
    pub raw_value: String,
    /// Matches the WeatherKit description value.
    pub description: String,
    /// Matches the WeatherKit accessibility description value.
    pub accessibility_description: String,
}

pub(crate) fn deserialize_pressure_trend<'de, D>(deserializer: D) -> Result<PressureTrend, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = String::deserialize(deserializer)?;
    Ok(PressureTrend::from_raw(raw))
}
