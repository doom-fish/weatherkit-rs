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

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HourForecast {
    pub date: String,
    pub temperature: f64,
    pub feels_like: f64,
    #[serde(deserialize_with = "deserialize_weather_condition")]
    pub condition: WeatherCondition,
    pub symbol_name: String,
    #[serde(deserialize_with = "deserialize_precipitation")]
    pub precipitation: Precipitation,
    pub precipitation_chance: f64,
    pub precipitation_amount: f64,
    pub cloud_cover: f64,
    pub dew_point: f64,
    pub humidity: f64,
    pub is_daylight: bool,
    pub pressure: f64,
    #[serde(deserialize_with = "deserialize_pressure_trend")]
    pub pressure_trend: PressureTrend,
    pub uv_index: UVIndex,
    pub visibility: f64,
    pub wind: Wind,
    #[serde(default)]
    pub snowfall_amount: Option<f64>,
    #[serde(default)]
    pub cloud_cover_by_altitude: Option<CloudCoverByAltitude>,
}

impl HourForecast {
    pub fn pressure_reading(&self) -> Pressure {
        Pressure::new(self.pressure, self.pressure_trend.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HourlyForecast {
    pub forecast: Vec<HourForecast>,
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

    pub fn len(&self) -> usize {
        self.forecast.len()
    }

    pub fn is_empty(&self) -> bool {
        self.forecast.is_empty()
    }

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
