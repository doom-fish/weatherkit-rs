//! WeatherKit weather alert types.

use core::ffi::c_void;

use serde::Deserialize;

use crate::error::WeatherKitError;
use crate::ffi;
use crate::private::{parse_json_from_handle, parse_json_from_static};
use crate::service::WeatherMetadata;

/// Represents the WeatherKit `WeatherSeverity` value.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum WeatherSeverity {
    /// Matches the WeatherKit `Minor` case.
    Minor,
    /// Matches the WeatherKit `Moderate` case.
    Moderate,
    /// Matches the WeatherKit `Severe` case.
    Severe,
    /// Matches the WeatherKit `Extreme` case.
    Extreme,
    /// Stores an unrecognized WeatherKit case name.
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

    /// Returns the WeatherKit raw value for this case.
    pub fn raw_value(&self) -> &str {
        match self {
            Self::Minor => "minor",
            Self::Moderate => "moderate",
            Self::Severe => "severe",
            Self::Extreme => "extreme",
            Self::Unknown(value) => value.as_str(),
        }
    }

    /// Returns the WeatherKit descriptor catalog for this enum.
    pub fn descriptors() -> Result<Vec<WeatherSeverityDescriptor>, WeatherKitError> {
        parse_json_from_static(
            ffi::weather_alert::wk_weather_severity_copy_descriptors_json,
            "weather severity descriptors",
        )
    }
}

/// Describes a WeatherKit weather alert severity case.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherSeverityDescriptor {
    /// Matches the WeatherKit raw value value.
    pub raw_value: String,
    /// Matches the WeatherKit description value.
    pub description: String,
    /// Matches the WeatherKit accessibility description value.
    pub accessibility_description: String,
}

/// Represents a WeatherKit weather alert.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherAlert {
    /// Matches the WeatherKit summary value.
    pub summary: String,
    /// Matches the WeatherKit details url value.
    #[serde(rename = "detailsURL")]
    pub details_url: String,
    /// Matches the WeatherKit source value.
    pub source: String,
    /// Matches the WeatherKit severity value.
    pub severity: String,
    /// Matches the WeatherKit region value.
    pub region: Option<String>,
    /// Matches the WeatherKit metadata value.
    pub metadata: WeatherMetadata,
}

impl WeatherAlert {
    /// Returns the WeatherKit severity enum for this alert.
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
