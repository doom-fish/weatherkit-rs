use serde::{Deserialize, Deserializer};

use crate::error::WeatherKitError;
use crate::ffi;
use crate::private::parse_json_from_static;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MoonPhase {
    New,
    WaxingCrescent,
    FirstQuarter,
    WaxingGibbous,
    Full,
    WaningGibbous,
    LastQuarter,
    WaningCrescent,
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

    pub fn descriptors() -> Result<Vec<MoonPhaseDescriptor>, WeatherKitError> {
        parse_json_from_static(
            ffi::moon_events::wk_moon_phase_copy_descriptors_json,
            "moon phase descriptors",
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoonPhaseDescriptor {
    pub raw_value: String,
    pub description: String,
    pub accessibility_description: String,
    pub symbol_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoonEvents {
    #[serde(deserialize_with = "deserialize_moon_phase")]
    pub phase: MoonPhase,
    pub moonrise: Option<String>,
    pub moonset: Option<String>,
}

pub(crate) fn deserialize_moon_phase<'de, D>(deserializer: D) -> Result<MoonPhase, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = String::deserialize(deserializer)?;
    Ok(MoonPhase::from_raw(raw))
}
