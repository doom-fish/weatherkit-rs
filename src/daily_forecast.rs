use core::ffi::c_void;

use serde::Deserialize;

use crate::current_weather::{CloudCoverByAltitude, UVIndex, Wind};
use crate::error::WeatherKitError;
use crate::ffi;
use crate::moon_events::MoonEvents;
use crate::private::parse_json_from_handle;
use crate::service::WeatherMetadata;
use crate::sun_events::SunEvents;
use crate::weather_condition::{
    deserialize_precipitation, deserialize_weather_condition, Precipitation, WeatherCondition,
};

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnowfallAmount {
    pub amount: f64,
    pub maximum: f64,
    pub minimum: f64,
    pub amount_liquid_equivalent: f64,
    pub maximum_liquid_equivalent: f64,
    pub minimum_liquid_equivalent: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrecipitationAmountByType {
    pub hail: f64,
    pub mixed: f64,
    pub rainfall: f64,
    pub sleet: f64,
    pub precipitation: f64,
    pub snowfall_amount: SnowfallAmount,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DayPartForecast {
    pub cloud_cover: f64,
    #[serde(default)]
    pub cloud_cover_by_altitude: Option<CloudCoverByAltitude>,
    #[serde(deserialize_with = "deserialize_weather_condition")]
    pub condition: WeatherCondition,
    pub high_temperature: f64,
    pub low_temperature: f64,
    #[serde(deserialize_with = "deserialize_precipitation")]
    pub precipitation: Precipitation,
    #[serde(default)]
    pub precipitation_amount_by_type: Option<PrecipitationAmountByType>,
    pub precipitation_chance: f64,
    #[serde(default)]
    pub maximum_humidity: Option<f64>,
    #[serde(default)]
    pub minimum_humidity: Option<f64>,
    #[serde(default)]
    pub maximum_visibility: Option<f64>,
    #[serde(default)]
    pub minimum_visibility: Option<f64>,
    pub wind: Wind,
    #[serde(default)]
    pub high_wind_speed: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DayForecast {
    pub date: String,
    pub high_temperature: f64,
    pub low_temperature: f64,
    #[serde(deserialize_with = "deserialize_weather_condition")]
    pub condition: WeatherCondition,
    pub symbol_name: String,
    #[serde(deserialize_with = "deserialize_precipitation")]
    pub precipitation: Precipitation,
    pub precipitation_chance: f64,
    pub precipitation_amount: f64,
    pub rainfall_amount: f64,
    pub snowfall_amount: f64,
    pub sun: SunEvents,
    pub moon: MoonEvents,
    pub uv_index: UVIndex,
    pub wind: Wind,
    #[serde(default)]
    pub high_temperature_time: Option<String>,
    #[serde(default)]
    pub low_temperature_time: Option<String>,
    #[serde(default)]
    pub maximum_humidity: Option<f64>,
    #[serde(default)]
    pub minimum_humidity: Option<f64>,
    #[serde(default)]
    pub precipitation_amount_by_type: Option<PrecipitationAmountByType>,
    #[serde(default)]
    pub maximum_visibility: Option<f64>,
    #[serde(default)]
    pub minimum_visibility: Option<f64>,
    #[serde(default)]
    pub high_wind_speed: Option<f64>,
    #[serde(default)]
    pub daytime_forecast: Option<DayPartForecast>,
    #[serde(default)]
    pub overnight_forecast: Option<DayPartForecast>,
    #[serde(default)]
    pub rest_of_day_forecast: Option<DayPartForecast>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyForecast {
    pub forecast: Vec<DayForecast>,
    pub metadata: WeatherMetadata,
}

impl DailyForecast {
    pub(crate) fn from_owned_ptr(ptr: *mut c_void) -> Result<Self, WeatherKitError> {
        parse_json_from_handle(
            ptr,
            ffi::daily_forecast::wk_daily_forecast_release,
            ffi::daily_forecast::wk_daily_forecast_copy_json,
            "daily forecast",
        )
    }

    pub fn len(&self) -> usize {
        self.forecast.len()
    }

    pub fn is_empty(&self) -> bool {
        self.forecast.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, DayForecast> {
        self.forecast.iter()
    }
}

impl<'a> IntoIterator for &'a DailyForecast {
    type Item = &'a DayForecast;
    type IntoIter = std::slice::Iter<'a, DayForecast>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
