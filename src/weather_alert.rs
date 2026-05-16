use core::ffi::c_void;

use serde::Deserialize;

use crate::error::WeatherKitError;
use crate::ffi;
use crate::private::{parse_json_from_handle, parse_json_from_static};
use crate::service::WeatherMetadata;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum WeatherSeverity {
    Minor,
    Moderate,
    Severe,
    Extreme,
    Unknown(String),
}

impl WeatherSeverity {
    pub(crate) fn from_raw(value: String) -> Self {
        match value.as_str() {
            "minor" => Self::Minor,
            "moderate" => Self::Moderate,
            "severe" => Self::Severe,
            "extreme" => Self::Extreme,
            "unknown" => Self::Unknown(value),
            other => Self::Unknown(other.to_owned()),
        }
    }

    pub fn raw_value(&self) -> &str {
        match self {
            Self::Minor => "minor",
            Self::Moderate => "moderate",
            Self::Severe => "severe",
            Self::Extreme => "extreme",
            Self::Unknown(value) => value.as_str(),
        }
    }

    pub fn descriptors() -> Result<Vec<WeatherSeverityDescriptor>, WeatherKitError> {
        parse_json_from_static(
            ffi::weather_alert::wk_weather_severity_copy_descriptors_json,
            "weather severity descriptors",
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherSeverityDescriptor {
    pub raw_value: String,
    pub description: String,
    pub accessibility_description: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherAlert {
    pub summary: String,
    #[serde(rename = "detailsURL")]
    pub details_url: String,
    pub source: String,
    pub severity: String,
    pub region: Option<String>,
    pub metadata: WeatherMetadata,
}

impl WeatherAlert {
    pub fn severity_kind(&self) -> WeatherSeverity {
        WeatherSeverity::from_raw(self.severity.clone())
    }
}

pub(crate) fn alerts_from_owned_ptr(
    ptr: *mut c_void,
) -> Result<Vec<WeatherAlert>, WeatherKitError> {
    parse_json_from_handle(
        ptr,
        ffi::weather_alert::wk_weather_alerts_release,
        ffi::weather_alert::wk_weather_alerts_copy_json,
        "weather alerts",
    )
}
