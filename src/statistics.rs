//! WeatherKit statistics and summary types.

use core::ffi::c_void;

use serde::Deserialize;

use crate::changes::{Percentiles, TemperatureUnit};
use crate::error::WeatherKitError;
use crate::ffi;
use crate::private::parse_json_from_handle;
use crate::service::WeatherMetadata;

/// Selects a WeatherKit daily statistics family.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DailyWeatherStatisticsQuery {
    /// Selects WeatherKit daily temperature statistics.
    Temperature,
    /// Selects WeatherKit daily precipitation statistics.
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

/// Selects a WeatherKit daily summary family.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DailyWeatherSummaryQuery {
    /// Selects the WeatherKit daily temperature summary.
    Temperature,
    /// Selects the WeatherKit daily precipitation summary.
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

/// Selects a WeatherKit hourly statistics family.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HourlyWeatherStatisticsQuery {
    /// Selects WeatherKit hourly temperature statistics.
    Temperature,
}

impl HourlyWeatherStatisticsQuery {
    pub(crate) const fn query_kind(self) -> i32 {
        match self {
            Self::Temperature => 0,
        }
    }
}

/// Selects a WeatherKit monthly statistics family.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonthlyWeatherStatisticsQuery {
    /// Selects WeatherKit monthly temperature statistics.
    Temperature,
    /// Selects WeatherKit monthly precipitation statistics.
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

/// Represents WeatherKit per-day temperature statistics.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DayTemperatureStatistics {
    /// Matches the WeatherKit day value.
    pub day: i64,
    /// Matches the WeatherKit average low temperature value.
    pub average_low_temperature: f64,
    /// Matches the WeatherKit average high temperature value.
    pub average_high_temperature: f64,
}

/// Represents WeatherKit per-day precipitation statistics.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DayPrecipitationStatistics {
    /// Matches the WeatherKit day value.
    pub day: i64,
    /// Matches the WeatherKit average precipitation probability value.
    pub average_precipitation_probability: f64,
    /// Matches the WeatherKit average precipitation amount value.
    pub average_precipitation_amount: f64,
    /// Matches the WeatherKit average snowfall amount value.
    pub average_snowfall_amount: f64,
}

/// Represents WeatherKit per-hour temperature statistics.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HourTemperatureStatistics {
    /// Matches the WeatherKit hour value.
    pub hour: i64,
    /// Matches the WeatherKit percentiles value.
    pub percentiles: Percentiles<TemperatureUnit>,
}

/// Represents WeatherKit per-month temperature statistics.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonthTemperatureStatistics {
    /// Matches the WeatherKit month value.
    pub month: i64,
    /// Matches the WeatherKit average low temperature value.
    pub average_low_temperature: f64,
    /// Matches the WeatherKit average high temperature value.
    pub average_high_temperature: f64,
}

/// Represents WeatherKit per-month precipitation statistics.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonthPrecipitationStatistics {
    /// Matches the WeatherKit month value.
    pub month: i64,
    /// Matches the WeatherKit average precipitation probability value.
    pub average_precipitation_probability: f64,
    /// Matches the WeatherKit average precipitation amount value.
    pub average_precipitation_amount: f64,
    /// Matches the WeatherKit average snowfall amount value.
    pub average_snowfall_amount: f64,
}

/// Represents a WeatherKit daily temperature summary.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DayTemperatureSummary {
    /// Matches the WeatherKit date value.
    pub date: String,
    /// Matches the WeatherKit low temperature value.
    pub low_temperature: f64,
    /// Matches the WeatherKit high temperature value.
    pub high_temperature: f64,
}

/// Represents a WeatherKit daily precipitation summary.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DayPrecipitationSummary {
    /// Matches the WeatherKit date value.
    pub date: String,
    /// Matches the WeatherKit precipitation amount value.
    pub precipitation_amount: f64,
    /// Matches the WeatherKit snowfall amount value.
    pub snowfall_amount: f64,
}

/// Wraps WeatherKit daily statistics results.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyWeatherStatistics<T> {
    /// Matches the WeatherKit days value.
    pub days: Vec<T>,
    /// Matches the WeatherKit baseline start date value.
    pub baseline_start_date: String,
    /// Matches the WeatherKit metadata value.
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
    /// Returns the number of WeatherKit day entries in this collection.
    pub fn len(&self) -> usize {
        self.days.len()
    }

    /// Returns whether this WeatherKit day collection is empty.
    pub fn is_empty(&self) -> bool {
        self.days.is_empty()
    }

    /// Iterates over the WeatherKit day entries in this collection.
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

/// Wraps WeatherKit daily summary results.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyWeatherSummary<T> {
    /// Matches the WeatherKit days value.
    pub days: Vec<T>,
    /// Matches the WeatherKit metadata value.
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
    /// Returns the number of WeatherKit summary entries in this collection.
    pub fn len(&self) -> usize {
        self.days.len()
    }

    /// Returns whether this WeatherKit summary collection is empty.
    pub fn is_empty(&self) -> bool {
        self.days.is_empty()
    }

    /// Iterates over the WeatherKit summary entries in this collection.
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

/// Wraps WeatherKit hourly statistics results.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HourlyWeatherStatistics<T> {
    /// Matches the WeatherKit hours value.
    pub hours: Vec<T>,
    /// Matches the WeatherKit baseline start date value.
    pub baseline_start_date: String,
    /// Matches the WeatherKit metadata value.
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
    /// Returns the number of WeatherKit hour entries in this collection.
    pub fn len(&self) -> usize {
        self.hours.len()
    }

    /// Returns whether this WeatherKit hour collection is empty.
    pub fn is_empty(&self) -> bool {
        self.hours.is_empty()
    }

    /// Iterates over the WeatherKit hour entries in this collection.
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

/// Wraps WeatherKit monthly statistics results.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonthlyWeatherStatistics<T> {
    /// Matches the WeatherKit months value.
    pub months: Vec<T>,
    /// Matches the WeatherKit baseline start date value.
    pub baseline_start_date: String,
    /// Matches the WeatherKit metadata value.
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
    /// Returns the number of WeatherKit month entries in this collection.
    pub fn len(&self) -> usize {
        self.months.len()
    }

    /// Returns whether this WeatherKit month collection is empty.
    pub fn is_empty(&self) -> bool {
        self.months.is_empty()
    }

    /// Iterates over the WeatherKit month entries in this collection.
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

/// Wraps a typed WeatherKit daily statistics result.
#[derive(Debug, Clone, PartialEq)]
pub enum DailyWeatherStatisticsResult {
    /// Carries WeatherKit daily temperature statistics.
    Temperature(DailyWeatherStatistics<DayTemperatureStatistics>),
    /// Carries WeatherKit daily precipitation statistics.
    Precipitation(DailyWeatherStatistics<DayPrecipitationStatistics>),
}

/// Wraps a typed WeatherKit daily summary result.
#[derive(Debug, Clone, PartialEq)]
pub enum DailyWeatherSummaryResult {
    /// Carries the WeatherKit daily temperature summary.
    Temperature(DailyWeatherSummary<DayTemperatureSummary>),
    /// Carries the WeatherKit daily precipitation summary.
    Precipitation(DailyWeatherSummary<DayPrecipitationSummary>),
}

/// Wraps a typed WeatherKit monthly statistics result.
#[derive(Debug, Clone, PartialEq)]
pub enum MonthlyWeatherStatisticsResult {
    /// Carries WeatherKit monthly temperature statistics.
    Temperature(MonthlyWeatherStatistics<MonthTemperatureStatistics>),
    /// Carries WeatherKit monthly precipitation statistics.
    Precipitation(MonthlyWeatherStatistics<MonthPrecipitationStatistics>),
}
