use serde::{Deserialize, Deserializer};

use crate::error::WeatherKitError;
use crate::ffi;
use crate::private::parse_json_from_static;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PressureTrend {
    Rising,
    Falling,
    Steady,
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

    pub fn raw_value(&self) -> &str {
        match self {
            Self::Rising => "rising",
            Self::Falling => "falling",
            Self::Steady => "steady",
            Self::Unknown(value) => value.as_str(),
        }
    }

    pub fn descriptors() -> Result<Vec<PressureTrendDescriptor>, WeatherKitError> {
        parse_json_from_static(
            ffi::pressure::wk_pressure_trend_copy_descriptors_json,
            "pressure trend descriptors",
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pressure {
    pub value: f64,
    pub trend: PressureTrend,
}

impl Pressure {
    pub const fn new(value: f64, trend: PressureTrend) -> Self {
        Self { value, trend }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PressureTrendDescriptor {
    pub raw_value: String,
    pub description: String,
    pub accessibility_description: String,
}

pub(crate) fn deserialize_pressure_trend<'de, D>(deserializer: D) -> Result<PressureTrend, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = String::deserialize(deserializer)?;
    Ok(PressureTrend::from_raw(raw))
}
