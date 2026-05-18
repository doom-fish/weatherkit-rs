//! WeatherKit minute forecast types.

use core::ffi::c_void;

use serde::Deserialize;

use crate::error::WeatherKitError;
use crate::ffi;
use crate::private::parse_json_from_handle;
use crate::service::WeatherMetadata;
use crate::weather_condition::{deserialize_precipitation, Precipitation};

/// Represents a WeatherKit minute forecast entry.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinuteForecast {
    /// Matches the WeatherKit date value.
    pub date: String,
    /// Matches the WeatherKit precipitation value.
    #[serde(deserialize_with = "deserialize_precipitation")]
    pub precipitation: Precipitation,
    /// Matches the WeatherKit precipitation chance value.
    pub precipitation_chance: f64,
    /// Matches the WeatherKit precipitation intensity value.
    pub precipitation_intensity: f64,
}

/// Wraps the WeatherKit minute forecast payload.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinuteForecastCollection {
    /// Matches the WeatherKit forecast value.
    pub forecast: Vec<MinuteForecast>,
    /// Matches the WeatherKit metadata value.
    pub metadata: WeatherMetadata,
    /// Matches the WeatherKit summary value.
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

    /// Returns the number of WeatherKit minute forecasts in this collection.
    pub fn len(&self) -> usize {
        self.forecast.len()
    }

    /// Returns whether this WeatherKit minute forecast collection is empty.
    pub fn is_empty(&self) -> bool {
        self.forecast.is_empty()
    }

    /// Iterates over the WeatherKit minute forecasts in this collection.
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
