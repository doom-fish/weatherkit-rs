//! WeatherKit daily forecast types.

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

/// Represents WeatherKit snowfall amount details.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnowfallAmount {
    /// Matches the WeatherKit amount value.
    pub amount: f64,
    /// Matches the WeatherKit maximum value.
    pub maximum: f64,
    /// Matches the WeatherKit minimum value.
    pub minimum: f64,
    /// Matches the WeatherKit amount liquid equivalent value.
    pub amount_liquid_equivalent: f64,
    /// Matches the WeatherKit maximum liquid equivalent value.
    pub maximum_liquid_equivalent: f64,
    /// Matches the WeatherKit minimum liquid equivalent value.
    pub minimum_liquid_equivalent: f64,
}

/// Represents WeatherKit precipitation totals broken down by type.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrecipitationAmountByType {
    /// Matches the WeatherKit hail value.
    pub hail: f64,
    /// Matches the WeatherKit mixed value.
    pub mixed: f64,
    /// Matches the WeatherKit rainfall value.
    pub rainfall: f64,
    /// Matches the WeatherKit sleet value.
    pub sleet: f64,
    /// Matches the WeatherKit precipitation value.
    pub precipitation: f64,
    /// Matches the WeatherKit snowfall amount value.
    pub snowfall_amount: SnowfallAmount,
}

/// Represents a WeatherKit day-part forecast.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DayPartForecast {
    /// Matches the WeatherKit cloud cover value.
    pub cloud_cover: f64,
    /// Matches the WeatherKit cloud cover by altitude value.
    #[serde(default)]
    pub cloud_cover_by_altitude: Option<CloudCoverByAltitude>,
    /// Matches the WeatherKit condition value.
    #[serde(deserialize_with = "deserialize_weather_condition")]
    pub condition: WeatherCondition,
    /// Matches the WeatherKit high temperature value.
    pub high_temperature: f64,
    /// Matches the WeatherKit low temperature value.
    pub low_temperature: f64,
    /// Matches the WeatherKit precipitation value.
    #[serde(deserialize_with = "deserialize_precipitation")]
    pub precipitation: Precipitation,
    /// Matches the WeatherKit precipitation amount by type value.
    #[serde(default)]
    pub precipitation_amount_by_type: Option<PrecipitationAmountByType>,
    /// Matches the WeatherKit precipitation chance value.
    pub precipitation_chance: f64,
    /// Matches the WeatherKit maximum humidity value.
    #[serde(default)]
    pub maximum_humidity: Option<f64>,
    /// Matches the WeatherKit minimum humidity value.
    #[serde(default)]
    pub minimum_humidity: Option<f64>,
    /// Matches the WeatherKit maximum visibility value.
    #[serde(default)]
    pub maximum_visibility: Option<f64>,
    /// Matches the WeatherKit minimum visibility value.
    #[serde(default)]
    pub minimum_visibility: Option<f64>,
    /// Matches the WeatherKit wind value.
    pub wind: Wind,
    /// Matches the WeatherKit high wind speed value.
    #[serde(default)]
    pub high_wind_speed: Option<f64>,
}

/// Represents a WeatherKit day forecast.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DayForecast {
    /// Matches the WeatherKit date value.
    pub date: String,
    /// Matches the WeatherKit high temperature value.
    pub high_temperature: f64,
    /// Matches the WeatherKit low temperature value.
    pub low_temperature: f64,
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
    /// Matches the WeatherKit rainfall amount value.
    pub rainfall_amount: f64,
    /// Matches the WeatherKit snowfall amount value.
    pub snowfall_amount: f64,
    /// Matches the WeatherKit sun value.
    pub sun: SunEvents,
    /// Matches the WeatherKit moon value.
    pub moon: MoonEvents,
    /// Matches the WeatherKit uv index value.
    pub uv_index: UVIndex,
    /// Matches the WeatherKit wind value.
    pub wind: Wind,
    /// Matches the WeatherKit high temperature time value.
    #[serde(default)]
    pub high_temperature_time: Option<String>,
    /// Matches the WeatherKit low temperature time value.
    #[serde(default)]
    pub low_temperature_time: Option<String>,
    /// Matches the WeatherKit maximum humidity value.
    #[serde(default)]
    pub maximum_humidity: Option<f64>,
    /// Matches the WeatherKit minimum humidity value.
    #[serde(default)]
    pub minimum_humidity: Option<f64>,
    /// Matches the WeatherKit precipitation amount by type value.
    #[serde(default)]
    pub precipitation_amount_by_type: Option<PrecipitationAmountByType>,
    /// Matches the WeatherKit maximum visibility value.
    #[serde(default)]
    pub maximum_visibility: Option<f64>,
    /// Matches the WeatherKit minimum visibility value.
    #[serde(default)]
    pub minimum_visibility: Option<f64>,
    /// Matches the WeatherKit high wind speed value.
    #[serde(default)]
    pub high_wind_speed: Option<f64>,
    /// Matches the WeatherKit daytime forecast value.
    #[serde(default)]
    pub daytime_forecast: Option<DayPartForecast>,
    /// Matches the WeatherKit overnight forecast value.
    #[serde(default)]
    pub overnight_forecast: Option<DayPartForecast>,
    /// Matches the WeatherKit rest of day forecast value.
    #[serde(default)]
    pub rest_of_day_forecast: Option<DayPartForecast>,
}

/// Wraps the WeatherKit daily forecast payload.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyForecast {
    /// Matches the WeatherKit forecast value.
    pub forecast: Vec<DayForecast>,
    /// Matches the WeatherKit metadata value.
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

    /// Returns the number of WeatherKit day forecasts in this collection.
    pub fn len(&self) -> usize {
        self.forecast.len()
    }

    /// Returns whether this WeatherKit day forecast collection is empty.
    pub fn is_empty(&self) -> bool {
        self.forecast.is_empty()
    }

    /// Iterates over the WeatherKit day forecasts in this collection.
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
