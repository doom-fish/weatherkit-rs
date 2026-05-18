//! WeatherKit change-tracking and historical comparison types.

use core::{ffi::c_void, marker::PhantomData};

use serde::{Deserialize, Deserializer};

use crate::error::WeatherKitError;
use crate::ffi;
use crate::private::parse_json_from_handle;
use crate::service::WeatherMetadata;

/// Marks WeatherKit temperature-based trend values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemperatureUnit {}

/// Marks WeatherKit precipitation- and snowfall-based length values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LengthUnit {}

/// Represents the WeatherKit `Deviation` value.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Deviation {
    /// Matches the WeatherKit `MuchHigher` case.
    MuchHigher,
    /// Matches the WeatherKit `Higher` case.
    Higher,
    /// Matches the WeatherKit `Normal` case.
    Normal,
    /// Matches the WeatherKit `Lower` case.
    Lower,
    /// Matches the WeatherKit `MuchLower` case.
    MuchLower,
    /// Stores an unrecognized WeatherKit case name.
    Unknown(String),
}

impl Deviation {
    pub(crate) fn from_raw(value: String) -> Self {
        match value.as_str() {
            "muchHigher" => Self::MuchHigher,
            "higher" => Self::Higher,
            "normal" => Self::Normal,
            "lower" => Self::Lower,
            "muchLower" => Self::MuchLower,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

/// Represents the WeatherKit `TrendBaselineKind` value.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TrendBaselineKind {
    /// Matches the WeatherKit `Mean` case.
    Mean,
    /// Stores an unrecognized WeatherKit case name.
    Unknown(String),
}

impl TrendBaselineKind {
    pub(crate) fn from_raw(value: String) -> Self {
        match value.as_str() {
            "mean" => Self::Mean,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

/// Represents a WeatherKit trend baseline.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", bound(deserialize = ""))]
pub struct TrendBaseline<Unit> {
    /// Matches the WeatherKit kind value.
    #[serde(deserialize_with = "deserialize_trend_baseline_kind")]
    pub kind: TrendBaselineKind,
    /// Matches the WeatherKit value value.
    pub value: f64,
    /// Matches the WeatherKit start date value.
    pub start_date: String,
    #[serde(skip)]
    marker: PhantomData<Unit>,
}

/// Represents a WeatherKit trend payload.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", bound(deserialize = ""))]
pub struct Trend<Unit> {
    /// Matches the WeatherKit baseline value.
    pub baseline: TrendBaseline<Unit>,
    /// Matches the WeatherKit current value value.
    pub current_value: f64,
    /// Matches the WeatherKit deviation value.
    #[serde(deserialize_with = "deserialize_deviation")]
    pub deviation: Deviation,
    #[serde(skip)]
    marker: PhantomData<Unit>,
}

/// Represents WeatherKit percentile values.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", bound(deserialize = ""))]
pub struct Percentiles<Unit> {
    /// Matches the WeatherKit p10 value.
    pub p10: f64,
    /// Matches the WeatherKit p50 value.
    pub p50: f64,
    /// Matches the WeatherKit p90 value.
    pub p90: f64,
    #[serde(skip)]
    marker: PhantomData<Unit>,
}

/// Represents a WeatherKit historical comparison result.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "kind", content = "trend", rename_all = "camelCase")]
pub enum HistoricalComparison {
    /// Carries the WeatherKit high-temperature trend.
    HighTemperature(Trend<TemperatureUnit>),
    /// Carries the WeatherKit low-temperature trend.
    LowTemperature(Trend<TemperatureUnit>),
    /// Carries the WeatherKit precipitation-amount trend.
    PrecipitationAmount(Trend<LengthUnit>),
    /// Carries the WeatherKit snowfall-amount trend.
    SnowfallAmount(Trend<LengthUnit>),
}

/// Wraps the WeatherKit historical comparisons payload.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalComparisons {
    /// Matches the WeatherKit comparisons value.
    pub comparisons: Vec<HistoricalComparison>,
    /// Matches the WeatherKit metadata value.
    pub metadata: WeatherMetadata,
}

impl HistoricalComparisons {
    pub(crate) fn option_from_owned_ptr(ptr: *mut c_void) -> Result<Option<Self>, WeatherKitError> {
        parse_json_from_handle(
            ptr,
            ffi::json_handle::wk_json_handle_release,
            ffi::json_handle::wk_json_handle_copy_json,
            "historical comparisons",
        )
    }

    /// Returns the number of WeatherKit comparisons in this collection.
    pub fn len(&self) -> usize {
        self.comparisons.len()
    }

    /// Returns whether this WeatherKit comparison collection is empty.
    pub fn is_empty(&self) -> bool {
        self.comparisons.is_empty()
    }

    /// Iterates over the WeatherKit comparisons in this collection.
    pub fn iter(&self) -> std::slice::Iter<'_, HistoricalComparison> {
        self.comparisons.iter()
    }
}

impl<'a> IntoIterator for &'a HistoricalComparisons {
    type Item = &'a HistoricalComparison;
    type IntoIter = std::slice::Iter<'a, HistoricalComparison>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Represents the WeatherKit `WeatherChangeDirection` value.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum WeatherChangeDirection {
    /// Matches the WeatherKit `Increase` case.
    Increase,
    /// Matches the WeatherKit `Decrease` case.
    Decrease,
    /// Matches the WeatherKit `Steady` case.
    Steady,
    /// Stores an unrecognized WeatherKit case name.
    Unknown(String),
}

impl WeatherChangeDirection {
    pub(crate) fn from_raw(value: String) -> Self {
        match value.as_str() {
            "increase" => Self::Increase,
            "decrease" => Self::Decrease,
            "steady" => Self::Steady,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

/// Represents a WeatherKit weather change entry.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherChange {
    /// Matches the WeatherKit date value.
    pub date: String,
    /// Matches the WeatherKit day precipitation amount value.
    #[serde(deserialize_with = "deserialize_weather_change_direction")]
    pub day_precipitation_amount: WeatherChangeDirection,
    /// Matches the WeatherKit high temperature value.
    #[serde(deserialize_with = "deserialize_weather_change_direction")]
    pub high_temperature: WeatherChangeDirection,
    /// Matches the WeatherKit low temperature value.
    #[serde(deserialize_with = "deserialize_weather_change_direction")]
    pub low_temperature: WeatherChangeDirection,
    /// Matches the WeatherKit night precipitation amount value.
    #[serde(deserialize_with = "deserialize_weather_change_direction")]
    pub night_precipitation_amount: WeatherChangeDirection,
}

/// Wraps the WeatherKit weather changes payload.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherChanges {
    /// Matches the WeatherKit changes value.
    pub changes: Vec<WeatherChange>,
    /// Matches the WeatherKit metadata value.
    pub metadata: WeatherMetadata,
}

impl WeatherChanges {
    pub(crate) fn option_from_owned_ptr(ptr: *mut c_void) -> Result<Option<Self>, WeatherKitError> {
        parse_json_from_handle(
            ptr,
            ffi::json_handle::wk_json_handle_release,
            ffi::json_handle::wk_json_handle_copy_json,
            "weather changes",
        )
    }

    /// Returns the number of WeatherKit changes in this collection.
    pub fn len(&self) -> usize {
        self.changes.len()
    }

    /// Returns whether this WeatherKit change collection is empty.
    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }

    /// Iterates over the WeatherKit changes in this collection.
    pub fn iter(&self) -> std::slice::Iter<'_, WeatherChange> {
        self.changes.iter()
    }
}

impl<'a> IntoIterator for &'a WeatherChanges {
    type Item = &'a WeatherChange;
    type IntoIter = std::slice::Iter<'a, WeatherChange>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub(crate) fn deserialize_deviation<'de, D>(deserializer: D) -> Result<Deviation, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = String::deserialize(deserializer)?;
    Ok(Deviation::from_raw(raw))
}

pub(crate) fn deserialize_trend_baseline_kind<'de, D>(
    deserializer: D,
) -> Result<TrendBaselineKind, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = String::deserialize(deserializer)?;
    Ok(TrendBaselineKind::from_raw(raw))
}

pub(crate) fn deserialize_weather_change_direction<'de, D>(
    deserializer: D,
) -> Result<WeatherChangeDirection, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = String::deserialize(deserializer)?;
    Ok(WeatherChangeDirection::from_raw(raw))
}
