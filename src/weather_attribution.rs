use core::ffi::c_void;

use serde::Deserialize;

use crate::error::WeatherKitError;
use crate::ffi;
use crate::private::parse_json_from_handle;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherAttribution {
    pub service_name: String,
    #[serde(rename = "legalPageURL")]
    pub legal_page_url: String,
    #[serde(rename = "squareMarkURL")]
    pub square_mark_url: String,
    #[serde(rename = "combinedMarkDarkURL")]
    pub combined_mark_dark_url: String,
    #[serde(rename = "combinedMarkLightURL")]
    pub combined_mark_light_url: String,
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
