//! WeatherKit attribution types.

use core::ffi::c_void;

use serde::Deserialize;

use crate::error::WeatherKitError;
use crate::ffi;
use crate::private::parse_json_from_handle;

/// Represents WeatherKit attribution metadata.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherAttribution {
    /// Matches the WeatherKit service name value.
    pub service_name: String,
    /// Matches the WeatherKit legal page url value.
    #[serde(rename = "legalPageURL")]
    pub legal_page_url: String,
    /// Matches the WeatherKit square mark url value.
    #[serde(rename = "squareMarkURL")]
    pub square_mark_url: String,
    /// Matches the WeatherKit combined mark dark url value.
    #[serde(rename = "combinedMarkDarkURL")]
    pub combined_mark_dark_url: String,
    /// Matches the WeatherKit combined mark light url value.
    #[serde(rename = "combinedMarkLightURL")]
    pub combined_mark_light_url: String,
    /// Matches the WeatherKit legal attribution text value.
    pub legal_attribution_text: Option<String>,
}

impl WeatherAttribution {
    pub(crate) fn from_owned_ptr(ptr: *mut c_void) -> Result<Self, WeatherKitError> {
        parse_json_from_handle(
            ptr,
            ffi::weather_attribution::wk_weather_attribution_release,
            ffi::weather_attribution::wk_weather_attribution_copy_json,
            "weather attribution",
        )
    }
}
