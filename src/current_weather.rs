use core::ffi::c_void;

use serde::Deserialize;

use crate::error::WeatherKitError;
use crate::ffi;
use crate::pressure::{deserialize_pressure_trend, Pressure, PressureTrend};
use crate::private::parse_json_from_handle;
use crate::service::WeatherMetadata;
use crate::weather_condition::{deserialize_weather_condition, WeatherCondition};

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudCoverByAltitude {
    pub low: f64,
    pub medium: f64,
    pub high: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wind {
    pub speed: f64,
    pub direction: f64,
    pub compass_direction: String,
    pub gust: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UVIndex {
    pub value: i64,
    pub category: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentWeather {
    pub date: String,
    pub temperature: f64,
    pub feels_like: f64,
    pub humidity: f64,
    pub dew_point: f64,
    pub pressure: f64,
    #[serde(deserialize_with = "deserialize_pressure_trend")]
    pub pressure_trend: PressureTrend,
    #[serde(deserialize_with = "deserialize_weather_condition")]
    pub condition: WeatherCondition,
    pub symbol_name: String,
    pub wind: Wind,
    pub uv_index: UVIndex,
    pub visibility: f64,
    pub cloud_cover: f64,
    #[serde(default)]
    pub cloud_cover_by_altitude: Option<CloudCoverByAltitude>,
    pub is_daylight: bool,
    pub precipitation_intensity: f64,
    pub metadata: WeatherMetadata,
}

impl CurrentWeather {
    pub(crate) fn from_owned_ptr(ptr: *mut c_void) -> Result<Self, WeatherKitError> {
        parse_json_from_handle(
            ptr,
            ffi::current_weather::wk_current_weather_release,
            ffi::current_weather::wk_current_weather_copy_json,
            "current weather",
        )
    }

    pub fn pressure_reading(&self) -> Pressure {
        Pressure::new(self.pressure, self.pressure_trend.clone())
    }
}
