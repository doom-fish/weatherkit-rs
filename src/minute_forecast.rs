use core::ffi::c_void;

use serde::Deserialize;

use crate::error::WeatherKitError;
use crate::ffi;
use crate::private::parse_json_from_handle;
use crate::service::WeatherMetadata;
use crate::weather_condition::{deserialize_precipitation, Precipitation};

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinuteForecast {
    pub date: String,
    #[serde(deserialize_with = "deserialize_precipitation")]
    pub precipitation: Precipitation,
    pub precipitation_chance: f64,
    pub precipitation_intensity: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinuteForecastCollection {
    pub forecast: Vec<MinuteForecast>,
    pub metadata: WeatherMetadata,
    pub summary: String,
}

impl MinuteForecastCollection {
    pub(crate) fn option_from_owned_ptr(ptr: *mut c_void) -> Result<Option<Self>, WeatherKitError> {
        parse_json_from_handle(
            ptr,
            ffi::minute_forecast::wk_minute_forecast_release,
            ffi::minute_forecast::wk_minute_forecast_copy_json,
            "minute forecast",
        )
    }

    pub fn len(&self) -> usize {
        self.forecast.len()
    }

    pub fn is_empty(&self) -> bool {
        self.forecast.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, MinuteForecast> {
        self.forecast.iter()
    }
}

impl<'a> IntoIterator for &'a MinuteForecastCollection {
    type Item = &'a MinuteForecast;
    type IntoIter = std::slice::Iter<'a, MinuteForecast>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
