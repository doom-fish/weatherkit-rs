use core::ffi::c_void;

use serde::Deserialize;

use crate::changes::{Percentiles, TemperatureUnit};
use crate::error::WeatherKitError;
use crate::ffi;
use crate::private::parse_json_from_handle;
use crate::service::WeatherMetadata;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DailyWeatherStatisticsQuery {
    Temperature,
    Precipitation,
}

impl DailyWeatherStatisticsQuery {
    pub(crate) const fn query_kind(self) -> i32 {
        match self {
            Self::Temperature => 0,
            Self::Precipitation => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DailyWeatherSummaryQuery {
    Temperature,
    Precipitation,
}

impl DailyWeatherSummaryQuery {
    pub(crate) const fn query_kind(self) -> i32 {
        match self {
            Self::Temperature => 0,
            Self::Precipitation => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HourlyWeatherStatisticsQuery {
    Temperature,
}

impl HourlyWeatherStatisticsQuery {
    pub(crate) const fn query_kind(self) -> i32 {
        match self {
            Self::Temperature => 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonthlyWeatherStatisticsQuery {
    Temperature,
    Precipitation,
}

impl MonthlyWeatherStatisticsQuery {
    pub(crate) const fn query_kind(self) -> i32 {
        match self {
            Self::Temperature => 0,
            Self::Precipitation => 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DayTemperatureStatistics {
    pub day: i64,
    pub average_low_temperature: f64,
    pub average_high_temperature: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DayPrecipitationStatistics {
    pub day: i64,
    pub average_precipitation_probability: f64,
    pub average_precipitation_amount: f64,
    pub average_snowfall_amount: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HourTemperatureStatistics {
    pub hour: i64,
    pub percentiles: Percentiles<TemperatureUnit>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonthTemperatureStatistics {
    pub month: i64,
    pub average_low_temperature: f64,
    pub average_high_temperature: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonthPrecipitationStatistics {
    pub month: i64,
    pub average_precipitation_probability: f64,
    pub average_precipitation_amount: f64,
    pub average_snowfall_amount: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DayTemperatureSummary {
    pub date: String,
    pub low_temperature: f64,
    pub high_temperature: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DayPrecipitationSummary {
    pub date: String,
    pub precipitation_amount: f64,
    pub snowfall_amount: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyWeatherStatistics<T> {
    pub days: Vec<T>,
    pub baseline_start_date: String,
    pub metadata: WeatherMetadata,
}

impl<T> DailyWeatherStatistics<T>
where
    T: for<'de> Deserialize<'de>,
{
    pub(crate) fn from_owned_ptr(ptr: *mut c_void) -> Result<Self, WeatherKitError> {
        parse_json_from_handle(
            ptr,
            ffi::json_handle::wk_json_handle_release,
            ffi::json_handle::wk_json_handle_copy_json,
            "daily weather statistics",
        )
    }
}

impl<T> DailyWeatherStatistics<T> {
    pub fn len(&self) -> usize {
        self.days.len()
    }

    pub fn is_empty(&self) -> bool {
        self.days.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.days.iter()
    }
}

impl<'a, T> IntoIterator for &'a DailyWeatherStatistics<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyWeatherSummary<T> {
    pub days: Vec<T>,
    pub metadata: WeatherMetadata,
}

impl<T> DailyWeatherSummary<T>
where
    T: for<'de> Deserialize<'de>,
{
    pub(crate) fn from_owned_ptr(ptr: *mut c_void) -> Result<Self, WeatherKitError> {
        parse_json_from_handle(
            ptr,
            ffi::json_handle::wk_json_handle_release,
            ffi::json_handle::wk_json_handle_copy_json,
            "daily weather summary",
        )
    }
}

impl<T> DailyWeatherSummary<T> {
    pub fn len(&self) -> usize {
        self.days.len()
    }

    pub fn is_empty(&self) -> bool {
        self.days.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.days.iter()
    }
}

impl<'a, T> IntoIterator for &'a DailyWeatherSummary<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HourlyWeatherStatistics<T> {
    pub hours: Vec<T>,
    pub baseline_start_date: String,
    pub metadata: WeatherMetadata,
}

impl<T> HourlyWeatherStatistics<T>
where
    T: for<'de> Deserialize<'de>,
{
    pub(crate) fn from_owned_ptr(ptr: *mut c_void) -> Result<Self, WeatherKitError> {
        parse_json_from_handle(
            ptr,
            ffi::json_handle::wk_json_handle_release,
            ffi::json_handle::wk_json_handle_copy_json,
            "hourly weather statistics",
        )
    }
}

impl<T> HourlyWeatherStatistics<T> {
    pub fn len(&self) -> usize {
        self.hours.len()
    }

    pub fn is_empty(&self) -> bool {
        self.hours.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.hours.iter()
    }
}

impl<'a, T> IntoIterator for &'a HourlyWeatherStatistics<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonthlyWeatherStatistics<T> {
    pub months: Vec<T>,
    pub baseline_start_date: String,
    pub metadata: WeatherMetadata,
}

impl<T> MonthlyWeatherStatistics<T>
where
    T: for<'de> Deserialize<'de>,
{
    pub(crate) fn from_owned_ptr(ptr: *mut c_void) -> Result<Self, WeatherKitError> {
        parse_json_from_handle(
            ptr,
            ffi::json_handle::wk_json_handle_release,
            ffi::json_handle::wk_json_handle_copy_json,
            "monthly weather statistics",
        )
    }
}

impl<T> MonthlyWeatherStatistics<T> {
    pub fn len(&self) -> usize {
        self.months.len()
    }

    pub fn is_empty(&self) -> bool {
        self.months.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.months.iter()
    }
}

impl<'a, T> IntoIterator for &'a MonthlyWeatherStatistics<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DailyWeatherStatisticsResult {
    Temperature(DailyWeatherStatistics<DayTemperatureStatistics>),
    Precipitation(DailyWeatherStatistics<DayPrecipitationStatistics>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum DailyWeatherSummaryResult {
    Temperature(DailyWeatherSummary<DayTemperatureSummary>),
    Precipitation(DailyWeatherSummary<DayPrecipitationSummary>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum MonthlyWeatherStatisticsResult {
    Temperature(MonthlyWeatherStatistics<MonthTemperatureStatistics>),
    Precipitation(MonthlyWeatherStatistics<MonthPrecipitationStatistics>),
}
