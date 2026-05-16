use serde::{Deserialize, Deserializer};

use crate::error::WeatherKitError;
use crate::ffi;
use crate::private::parse_json_from_static;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum WeatherCondition {
    Blizzard,
    BlowingDust,
    BlowingSnow,
    Breezy,
    Clear,
    Cloudy,
    Drizzle,
    Flurries,
    Foggy,
    FreezingDrizzle,
    FreezingRain,
    Frigid,
    Hail,
    Haze,
    HeavyRain,
    HeavySnow,
    Hot,
    Hurricane,
    IsolatedThunderstorms,
    MostlyClear,
    MostlyCloudy,
    PartlyCloudy,
    Rain,
    ScatteredThunderstorms,
    Sleet,
    Smoky,
    Snow,
    StrongStorms,
    SunFlurries,
    SunShowers,
    Thunderstorms,
    TropicalStorm,
    Windy,
    WintryMix,
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

    pub fn descriptors() -> Result<Vec<WeatherConditionDescriptor>, WeatherKitError> {
        parse_json_from_static(
            ffi::weather_condition::wk_weather_condition_copy_descriptors_json,
            "weather condition descriptors",
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Precipitation {
    None,
    Hail,
    Mixed,
    Rain,
    Sleet,
    Snow,
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
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherConditionDescriptor {
    pub raw_value: String,
    pub description: String,
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
