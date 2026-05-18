//! WeatherKit moon event types.

use serde::{Deserialize, Deserializer};

use crate::error::WeatherKitError;
use crate::ffi;
use crate::private::parse_json_from_static;

/// Represents the WeatherKit `MoonPhase` value.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MoonPhase {
    /// Matches the WeatherKit `New` case.
    New,
    /// Matches the WeatherKit `WaxingCrescent` case.
    WaxingCrescent,
    /// Matches the WeatherKit `FirstQuarter` case.
    FirstQuarter,
    /// Matches the WeatherKit `WaxingGibbous` case.
    WaxingGibbous,
    /// Matches the WeatherKit `Full` case.
    Full,
    /// Matches the WeatherKit `WaningGibbous` case.
    WaningGibbous,
    /// Matches the WeatherKit `LastQuarter` case.
    LastQuarter,
    /// Matches the WeatherKit `WaningCrescent` case.
    WaningCrescent,
    /// Stores an unrecognized WeatherKit case name.
    Unknown(String),
}

impl MoonPhase {
    pub(crate) fn from_raw(value: String) -> Self {
        match value.as_str() {
            "new" => Self::New,
            "waxingCrescent" => Self::WaxingCrescent,
            "firstQuarter" => Self::FirstQuarter,
            "waxingGibbous" => Self::WaxingGibbous,
            "full" => Self::Full,
            "waningGibbous" => Self::WaningGibbous,
            "lastQuarter" => Self::LastQuarter,
            "waningCrescent" => Self::WaningCrescent,
            other => Self::Unknown(other.to_owned()),
        }
    }

    /// Returns the WeatherKit raw value for this case.
    pub fn raw_value(&self) -> &str {
        match self {
            Self::New => "new",
            Self::WaxingCrescent => "waxingCrescent",
            Self::FirstQuarter => "firstQuarter",
            Self::WaxingGibbous => "waxingGibbous",
            Self::Full => "full",
            Self::WaningGibbous => "waningGibbous",
            Self::LastQuarter => "lastQuarter",
            Self::WaningCrescent => "waningCrescent",
            Self::Unknown(value) => value.as_str(),
        }
    }

    /// Returns the WeatherKit descriptor catalog for this enum.
    pub fn descriptors() -> Result<Vec<MoonPhaseDescriptor>, WeatherKitError> {
        parse_json_from_static(
            ffi::moon_events::wk_moon_phase_copy_descriptors_json,
            "moon phase descriptors",
        )
    }
}

/// Represents the WeatherKit `MoonPhaseDescriptor` payload.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoonPhaseDescriptor {
    /// Matches the WeatherKit raw value value.
    pub raw_value: String,
    /// Matches the WeatherKit description value.
    pub description: String,
    /// Matches the WeatherKit accessibility description value.
    pub accessibility_description: String,
    /// Matches the WeatherKit symbol name value.
    pub symbol_name: String,
}

/// Represents WeatherKit moon event values.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoonEvents {
    /// Matches the WeatherKit phase value.
    #[serde(deserialize_with = "deserialize_moon_phase")]
    pub phase: MoonPhase,
    /// Matches the WeatherKit moonrise value.
    pub moonrise: Option<String>,
    /// Matches the WeatherKit moonset value.
    pub moonset: Option<String>,
}

pub(crate) fn deserialize_moon_phase<'de, D>(deserializer: D) -> Result<MoonPhase, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = String::deserialize(deserializer)?;
    Ok(MoonPhase::from_raw(raw))
}
