use core::ffi::c_void;

use serde::{Deserialize, Deserializer};

use crate::error::WeatherKitError;
use crate::ffi;
use crate::private::{parse_json_from_handle, parse_json_from_static};

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AvailabilityKind {
    Available,
    TemporarilyUnavailable,
    Unsupported,
    Unknown(String),
}

impl AvailabilityKind {
    pub(crate) fn from_raw(value: String) -> Self {
        match value.as_str() {
            "available" => Self::Available,
            "temporarilyUnavailable" => Self::TemporarilyUnavailable,
            "unsupported" => Self::Unsupported,
            "unknown" => Self::Unknown(value),
            other => Self::Unknown(other.to_owned()),
        }
    }

    pub fn raw_value(&self) -> &str {
        match self {
            Self::Available => "available",
            Self::TemporarilyUnavailable => "temporarilyUnavailable",
            Self::Unsupported => "unsupported",
            Self::Unknown(value) => value.as_str(),
        }
    }

    pub fn descriptors() -> Result<Vec<AvailabilityKindDescriptor>, WeatherKitError> {
        parse_json_from_static(
            ffi::availability_kind::wk_availability_kind_copy_descriptors_json,
            "availability kind descriptors",
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailabilityKindDescriptor {
    pub raw_value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherAvailability {
    #[serde(deserialize_with = "deserialize_availability_kind")]
    pub minute_availability: AvailabilityKind,
    #[serde(deserialize_with = "deserialize_availability_kind")]
    pub alert_availability: AvailabilityKind,
}

impl WeatherAvailability {
    pub(crate) fn from_owned_ptr(ptr: *mut c_void) -> Result<Self, WeatherKitError> {
        parse_json_from_handle(
            ptr,
            ffi::availability_kind::wk_weather_availability_release,
            ffi::availability_kind::wk_weather_availability_copy_json,
            "weather availability",
        )
    }
}

pub(crate) fn deserialize_availability_kind<'de, D>(
    deserializer: D,
) -> Result<AvailabilityKind, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = String::deserialize(deserializer)?;
    Ok(AvailabilityKind::from_raw(raw))
}
