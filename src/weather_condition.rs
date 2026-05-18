//! WeatherKit condition and precipitation enums.

use serde::{Deserialize, Deserializer};

use crate::error::WeatherKitError;
use crate::ffi;
use crate::private::parse_json_from_static;

/// Represents the WeatherKit `WeatherCondition` value.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum WeatherCondition {
    /// Matches the WeatherKit `Blizzard` case.
    Blizzard,
    /// Matches the WeatherKit `BlowingDust` case.
    BlowingDust,
    /// Matches the WeatherKit `BlowingSnow` case.
    BlowingSnow,
    /// Matches the WeatherKit `Breezy` case.
    Breezy,
    /// Matches the WeatherKit `Clear` case.
    Clear,
    /// Matches the WeatherKit `Cloudy` case.
    Cloudy,
    /// Matches the WeatherKit `Drizzle` case.
    Drizzle,
    /// Matches the WeatherKit `Flurries` case.
    Flurries,
    /// Matches the WeatherKit `Foggy` case.
    Foggy,
    /// Matches the WeatherKit `FreezingDrizzle` case.
    FreezingDrizzle,
    /// Matches the WeatherKit `FreezingRain` case.
    FreezingRain,
    /// Matches the WeatherKit `Frigid` case.
    Frigid,
    /// Matches the WeatherKit `Hail` case.
    Hail,
    /// Matches the WeatherKit `Haze` case.
    Haze,
    /// Matches the WeatherKit `HeavyRain` case.
    HeavyRain,
    /// Matches the WeatherKit `HeavySnow` case.
    HeavySnow,
    /// Matches the WeatherKit `Hot` case.
    Hot,
    /// Matches the WeatherKit `Hurricane` case.
    Hurricane,
    /// Matches the WeatherKit `IsolatedThunderstorms` case.
    IsolatedThunderstorms,
    /// Matches the WeatherKit `MostlyClear` case.
    MostlyClear,
    /// Matches the WeatherKit `MostlyCloudy` case.
    MostlyCloudy,
    /// Matches the WeatherKit `PartlyCloudy` case.
    PartlyCloudy,
    /// Matches the WeatherKit `Rain` case.
    Rain,
    /// Matches the WeatherKit `ScatteredThunderstorms` case.
    ScatteredThunderstorms,
    /// Matches the WeatherKit `Sleet` case.
    Sleet,
    /// Matches the WeatherKit `Smoky` case.
    Smoky,
    /// Matches the WeatherKit `Snow` case.
    Snow,
    /// Matches the WeatherKit `StrongStorms` case.
    StrongStorms,
    /// Matches the WeatherKit `SunFlurries` case.
    SunFlurries,
    /// Matches the WeatherKit `SunShowers` case.
    SunShowers,
    /// Matches the WeatherKit `Thunderstorms` case.
    Thunderstorms,
    /// Matches the WeatherKit `TropicalStorm` case.
    TropicalStorm,
    /// Matches the WeatherKit `Windy` case.
    Windy,
    /// Matches the WeatherKit `WintryMix` case.
    WintryMix,
    /// Stores an unrecognized WeatherKit case name.
    Unknown(String),
}

impl WeatherCondition {
    pub(crate) fn from_raw(value: String) -> Self {
        match value.as_str() {
            "blizzard" => Self::Blizzard,
            "blowingDust" => Self::BlowingDust,
            "blowingSnow" => Self::BlowingSnow,
            "breezy" => Self::Breezy,
            "clear" => Self::Clear,
            "cloudy" => Self::Cloudy,
            "drizzle" => Self::Drizzle,
            "flurries" => Self::Flurries,
            "foggy" => Self::Foggy,
            "freezingDrizzle" => Self::FreezingDrizzle,
            "freezingRain" => Self::FreezingRain,
            "frigid" => Self::Frigid,
            "hail" => Self::Hail,
            "haze" => Self::Haze,
            "heavyRain" => Self::HeavyRain,
            "heavySnow" => Self::HeavySnow,
            "hot" => Self::Hot,
            "hurricane" => Self::Hurricane,
            "isolatedThunderstorms" => Self::IsolatedThunderstorms,
            "mostlyClear" => Self::MostlyClear,
            "mostlyCloudy" => Self::MostlyCloudy,
            "partlyCloudy" => Self::PartlyCloudy,
            "rain" => Self::Rain,
            "scatteredThunderstorms" => Self::ScatteredThunderstorms,
            "sleet" => Self::Sleet,
            "smoky" => Self::Smoky,
            "snow" => Self::Snow,
            "strongStorms" => Self::StrongStorms,
            "sunFlurries" => Self::SunFlurries,
            "sunShowers" => Self::SunShowers,
            "thunderstorms" => Self::Thunderstorms,
            "tropicalStorm" => Self::TropicalStorm,
            "windy" => Self::Windy,
            "wintryMix" => Self::WintryMix,
            other => Self::Unknown(other.to_owned()),
        }
    }

    /// Returns the WeatherKit raw value for this case.
    pub fn raw_value(&self) -> &str {
        match self {
            Self::Blizzard => "blizzard",
            Self::BlowingDust => "blowingDust",
            Self::BlowingSnow => "blowingSnow",
            Self::Breezy => "breezy",
            Self::Clear => "clear",
            Self::Cloudy => "cloudy",
            Self::Drizzle => "drizzle",
            Self::Flurries => "flurries",
            Self::Foggy => "foggy",
            Self::FreezingDrizzle => "freezingDrizzle",
            Self::FreezingRain => "freezingRain",
            Self::Frigid => "frigid",
            Self::Hail => "hail",
            Self::Haze => "haze",
            Self::HeavyRain => "heavyRain",
            Self::HeavySnow => "heavySnow",
            Self::Hot => "hot",
            Self::Hurricane => "hurricane",
            Self::IsolatedThunderstorms => "isolatedThunderstorms",
            Self::MostlyClear => "mostlyClear",
            Self::MostlyCloudy => "mostlyCloudy",
            Self::PartlyCloudy => "partlyCloudy",
            Self::Rain => "rain",
            Self::ScatteredThunderstorms => "scatteredThunderstorms",
            Self::Sleet => "sleet",
            Self::Smoky => "smoky",
            Self::Snow => "snow",
            Self::StrongStorms => "strongStorms",
            Self::SunFlurries => "sunFlurries",
            Self::SunShowers => "sunShowers",
            Self::Thunderstorms => "thunderstorms",
            Self::TropicalStorm => "tropicalStorm",
            Self::Windy => "windy",
            Self::WintryMix => "wintryMix",
            Self::Unknown(value) => value.as_str(),
        }
    }

    /// Returns the WeatherKit descriptor catalog for this enum.
    pub fn descriptors() -> Result<Vec<WeatherConditionDescriptor>, WeatherKitError> {
        parse_json_from_static(
            ffi::weather_condition::wk_weather_condition_copy_descriptors_json,
            "weather condition descriptors",
        )
    }
}

/// Represents the WeatherKit `Precipitation` value.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Precipitation {
    /// Matches the WeatherKit `None` case.
    None,
    /// Matches the WeatherKit `Hail` case.
    Hail,
    /// Matches the WeatherKit `Mixed` case.
    Mixed,
    /// Matches the WeatherKit `Rain` case.
    Rain,
    /// Matches the WeatherKit `Sleet` case.
    Sleet,
    /// Matches the WeatherKit `Snow` case.
    Snow,
    /// Stores an unrecognized WeatherKit case name.
    Unknown(String),
}

impl Precipitation {
    pub(crate) fn from_raw(value: String) -> Self {
        match value.as_str() {
            "none" => Self::None,
            "hail" => Self::Hail,
            "mixed" => Self::Mixed,
            "rain" => Self::Rain,
            "sleet" => Self::Sleet,
            "snow" => Self::Snow,
            other => Self::Unknown(other.to_owned()),
        }
    }

    /// Returns the WeatherKit raw value for this case.
    pub fn raw_value(&self) -> &str {
        match self {
            Self::None => "none",
            Self::Hail => "hail",
            Self::Mixed => "mixed",
            Self::Rain => "rain",
            Self::Sleet => "sleet",
            Self::Snow => "snow",
            Self::Unknown(value) => value.as_str(),
        }
    }

    /// Returns the WeatherKit descriptor catalog for this enum.
    pub fn descriptors() -> Result<Vec<PrecipitationDescriptor>, WeatherKitError> {
        parse_json_from_static(
            ffi::weather_condition::wk_precipitation_copy_descriptors_json,
            "precipitation descriptors",
        )
    }
}

/// Describes a WeatherKit precipitation case.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrecipitationDescriptor {
    /// Matches the WeatherKit raw value value.
    pub raw_value: String,
    /// Matches the WeatherKit description value.
    pub description: String,
    /// Matches the WeatherKit accessibility description value.
    pub accessibility_description: String,
}

/// Describes a WeatherKit weather condition case.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherConditionDescriptor {
    /// Matches the WeatherKit raw value value.
    pub raw_value: String,
    /// Matches the WeatherKit description value.
    pub description: String,
    /// Matches the WeatherKit accessibility description value.
    pub accessibility_description: String,
}

pub(crate) fn deserialize_weather_condition<'de, D>(
    deserializer: D,
) -> Result<WeatherCondition, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = String::deserialize(deserializer)?;
    Ok(WeatherCondition::from_raw(raw))
}

pub(crate) fn deserialize_precipitation<'de, D>(deserializer: D) -> Result<Precipitation, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = String::deserialize(deserializer)?;
    Ok(Precipitation::from_raw(raw))
}
