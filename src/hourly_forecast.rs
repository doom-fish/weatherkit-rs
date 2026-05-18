//! WeatherKit hourly forecast types.

use core::ffi::c_void;

use serde::Deserialize;

use crate::current_weather::{CloudCoverByAltitude, UVIndex, Wind};
use crate::error::WeatherKitError;
use crate::ffi;
use crate::pressure::{deserialize_pressure_trend, Pressure, PressureTrend};
use crate::private::parse_json_from_handle;
use crate::service::WeatherMetadata;
use crate::weather_condition::{
    deserialize_precipitation, deserialize_weather_condition, Precipitation, WeatherCondition,
};

/// Represents a WeatherKit hour forecast.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HourForecast {
    /// Matches the WeatherKit date value.
    pub date: String,
    /// Matches the WeatherKit temperature value.
    pub temperature: f64,
    /// Matches the WeatherKit feels like value.
    pub feels_like: f64,
    /// Matches the WeatherKit condition value.
    #[serde(deserialize_with = "deserialize_weather_condition")]
    pub condition: WeatherCondition,
    /// Matches the WeatherKit symbol name value.
    pub symbol_name: String,
    /// Matches the WeatherKit precipitation value.
    #[serde(deserialize_with = "deserialize_precipitation")]
    pub precipitation: Precipitation,
    /// Matches the WeatherKit precipitation chance value.
    pub precipitation_chance: f64,
    /// Matches the WeatherKit precipitation amount value.
    pub precipitation_amount: f64,
    /// Matches the WeatherKit cloud cover value.
    pub cloud_cover: f64,
    /// Matches the WeatherKit dew point value.
    pub dew_point: f64,
    /// Matches the WeatherKit humidity value.
    pub humidity: f64,
    /// Matches the WeatherKit is daylight value.
    pub is_daylight: bool,
    /// Matches the WeatherKit pressure value.
    pub pressure: f64,
    /// Matches the WeatherKit pressure trend value.
    #[serde(deserialize_with = "deserialize_pressure_trend")]
    pub pressure_trend: PressureTrend,
    /// Matches the WeatherKit uv index value.
    pub uv_index: UVIndex,
    /// Matches the WeatherKit visibility value.
    pub visibility: f64,
    /// Matches the WeatherKit wind value.
    pub wind: Wind,
    /// Matches the WeatherKit snowfall amount value.
    #[serde(default)]
    pub snowfall_amount: Option<f64>,
    /// Matches the WeatherKit cloud cover by altitude value.
    #[serde(default)]
    pub cloud_cover_by_altitude: Option<CloudCoverByAltitude>,
}

impl HourForecast {
    /// Wraps the WeatherKit pressure value and trend.
    pub fn pressure_reading(&self) -> Pressure {
        Pressure::new(self.pressure, self.pressure_trend.clone())
    }
}

/// Wraps the WeatherKit hourly forecast payload.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HourlyForecast {
    /// Matches the WeatherKit forecast value.
    pub forecast: Vec<HourForecast>,
    /// Matches the WeatherKit metadata value.
    pub metadata: WeatherMetadata,
}

impl HourlyForecast {
    pub(crate) fn from_owned_ptr(ptr: *mut c_void) -> Result<Self, WeatherKitError> {
        parse_json_from_handle(
            ptr,
            ffi::hourly_forecast::wk_hourly_forecast_release,
            ffi::hourly_forecast::wk_hourly_forecast_copy_json,
            "hourly forecast",
        )
    }

    /// Returns the number of WeatherKit hour forecasts in this collection.
    pub fn len(&self) -> usize {
        self.forecast.len()
    }

    /// Returns whether this WeatherKit hour forecast collection is empty.
    pub fn is_empty(&self) -> bool {
        self.forecast.is_empty()
    }

    /// Iterates over the WeatherKit hour forecasts in this collection.
    pub fn iter(&self) -> std::slice::Iter<'_, HourForecast> {
        self.forecast.iter()
    }
}

impl<'a> IntoIterator for &'a HourlyForecast {
    type Item = &'a HourForecast;
    type IntoIter = std::slice::Iter<'a, HourForecast>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
