use core::{ffi::c_void, marker::PhantomData};

use serde::{Deserialize, Deserializer};

use crate::error::WeatherKitError;
use crate::ffi;
use crate::private::parse_json_from_handle;
use crate::service::WeatherMetadata;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemperatureUnit {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LengthUnit {}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Deviation {
    MuchHigher,
    Higher,
    Normal,
    Lower,
    MuchLower,
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

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TrendBaselineKind {
    Mean,
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", bound(deserialize = ""))]
pub struct TrendBaseline<Unit> {
    #[serde(deserialize_with = "deserialize_trend_baseline_kind")]
    pub kind: TrendBaselineKind,
    pub value: f64,
    pub start_date: String,
    #[serde(skip)]
    marker: PhantomData<Unit>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", bound(deserialize = ""))]
pub struct Trend<Unit> {
    pub baseline: TrendBaseline<Unit>,
    pub current_value: f64,
    #[serde(deserialize_with = "deserialize_deviation")]
    pub deviation: Deviation,
    #[serde(skip)]
    marker: PhantomData<Unit>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", bound(deserialize = ""))]
pub struct Percentiles<Unit> {
    pub p10: f64,
    pub p50: f64,
    pub p90: f64,
    #[serde(skip)]
    marker: PhantomData<Unit>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "kind", content = "trend", rename_all = "camelCase")]
pub enum HistoricalComparison {
    HighTemperature(Trend<TemperatureUnit>),
    LowTemperature(Trend<TemperatureUnit>),
    PrecipitationAmount(Trend<LengthUnit>),
    SnowfallAmount(Trend<LengthUnit>),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalComparisons {
    pub comparisons: Vec<HistoricalComparison>,
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

    pub fn len(&self) -> usize {
        self.comparisons.len()
    }

    pub fn is_empty(&self) -> bool {
        self.comparisons.is_empty()
    }

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

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum WeatherChangeDirection {
    Increase,
    Decrease,
    Steady,
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherChange {
    pub date: String,
    #[serde(deserialize_with = "deserialize_weather_change_direction")]
    pub day_precipitation_amount: WeatherChangeDirection,
    #[serde(deserialize_with = "deserialize_weather_change_direction")]
    pub high_temperature: WeatherChangeDirection,
    #[serde(deserialize_with = "deserialize_weather_change_direction")]
    pub low_temperature: WeatherChangeDirection,
    #[serde(deserialize_with = "deserialize_weather_change_direction")]
    pub night_precipitation_amount: WeatherChangeDirection,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherChanges {
    pub changes: Vec<WeatherChange>,
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

    pub fn len(&self) -> usize {
        self.changes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }

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
