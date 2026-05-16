use serde::Deserialize;

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
    fn from_raw(value: String) -> Self {
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
    fn from_raw(value: String) -> Self {
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PressureTrend {
    Rising,
    Falling,
    Steady,
    Unknown(String),
}

impl PressureTrend {
    fn from_raw(value: String) -> Self {
        match value.as_str() {
            "rising" => Self::Rising,
            "falling" => Self::Falling,
            "steady" => Self::Steady,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AvailabilityKind {
    Available,
    TemporarilyUnavailable,
    Unsupported,
    Unknown(String),
}

impl AvailabilityKind {
    fn from_raw(value: String) -> Self {
        match value.as_str() {
            "available" => Self::Available,
            "temporarilyUnavailable" => Self::TemporarilyUnavailable,
            "unsupported" => Self::Unsupported,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Wind {
    pub speed: f64,
    pub direction: f64,
    pub compass_direction: String,
    pub gust: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UVIndex {
    pub value: i64,
    pub category: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CurrentWeather {
    pub temperature: f64,
    pub feels_like: f64,
    pub humidity: f64,
    pub dew_point: f64,
    pub pressure: f64,
    pub pressure_trend: PressureTrend,
    pub condition: WeatherCondition,
    pub symbol_name: String,
    pub wind: Wind,
    pub uv_index: UVIndex,
    pub visibility: f64,
    pub cloud_cover: f64,
    pub is_daylight: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HourForecast {
    pub date: String,
    pub temperature: f64,
    pub feels_like: f64,
    pub condition: WeatherCondition,
    pub symbol_name: String,
    pub precipitation: Precipitation,
    pub precipitation_chance: f64,
    pub precipitation_amount: f64,
    pub cloud_cover: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DayForecast {
    pub date: String,
    pub high_temperature: f64,
    pub low_temperature: f64,
    pub condition: WeatherCondition,
    pub symbol_name: String,
    pub precipitation: Precipitation,
    pub precipitation_chance: f64,
    pub precipitation_amount: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MinuteForecast {
    pub date: String,
    pub precipitation: Precipitation,
    pub precipitation_chance: f64,
    pub precipitation_intensity: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeatherAlert {
    pub summary: String,
    pub details_url: String,
    pub source: String,
    pub severity: String,
    pub region: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeatherAvailability {
    pub minute_availability: AvailabilityKind,
    pub alert_availability: AvailabilityKind,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Weather {
    pub current_weather: CurrentWeather,
    pub hourly_forecast: Vec<HourForecast>,
    pub daily_forecast: Vec<DayForecast>,
    pub minute_forecast: Option<Vec<MinuteForecast>>,
    pub weather_alerts: Vec<WeatherAlert>,
    pub availability: WeatherAvailability,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WindPayload {
    pub speed: f64,
    pub direction: f64,
    pub compass_direction: String,
    pub gust: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UVIndexPayload {
    pub value: i64,
    pub category: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CurrentWeatherPayload {
    pub temperature: f64,
    pub feels_like: f64,
    pub humidity: f64,
    pub dew_point: f64,
    pub pressure: f64,
    pub pressure_trend: String,
    pub condition: String,
    pub symbol_name: String,
    pub wind: WindPayload,
    pub uv_index: UVIndexPayload,
    pub visibility: f64,
    pub cloud_cover: f64,
    pub is_daylight: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HourForecastPayload {
    pub date: String,
    pub temperature: f64,
    pub feels_like: f64,
    pub condition: String,
    pub symbol_name: String,
    pub precipitation: String,
    pub precipitation_chance: f64,
    pub precipitation_amount: f64,
    pub cloud_cover: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DayForecastPayload {
    pub date: String,
    pub high_temperature: f64,
    pub low_temperature: f64,
    pub condition: String,
    pub symbol_name: String,
    pub precipitation: String,
    pub precipitation_chance: f64,
    pub precipitation_amount: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MinuteForecastPayload {
    pub date: String,
    pub precipitation: String,
    pub precipitation_chance: f64,
    pub precipitation_intensity: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WeatherAlertPayload {
    pub summary: String,
    pub details_url: String,
    pub source: String,
    pub severity: String,
    pub region: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WeatherAvailabilityPayload {
    pub minute_availability: String,
    pub alert_availability: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WeatherPayload {
    pub current_weather: CurrentWeatherPayload,
    pub hourly_forecast: Vec<HourForecastPayload>,
    pub daily_forecast: Vec<DayForecastPayload>,
    pub minute_forecast: Option<Vec<MinuteForecastPayload>>,
    pub weather_alerts: Vec<WeatherAlertPayload>,
    pub availability: WeatherAvailabilityPayload,
}

impl Weather {
    pub(crate) fn from_payload(payload: WeatherPayload) -> Self {
        Self {
            current_weather: CurrentWeather::from_payload(payload.current_weather),
            hourly_forecast: payload
                .hourly_forecast
                .into_iter()
                .map(HourForecast::from_payload)
                .collect(),
            daily_forecast: payload
                .daily_forecast
                .into_iter()
                .map(DayForecast::from_payload)
                .collect(),
            minute_forecast: payload.minute_forecast.map(|forecast| {
                forecast
                    .into_iter()
                    .map(MinuteForecast::from_payload)
                    .collect()
            }),
            weather_alerts: payload
                .weather_alerts
                .into_iter()
                .map(WeatherAlert::from_payload)
                .collect(),
            availability: WeatherAvailability::from_payload(payload.availability),
        }
    }
}

impl CurrentWeather {
    fn from_payload(payload: CurrentWeatherPayload) -> Self {
        Self {
            temperature: payload.temperature,
            feels_like: payload.feels_like,
            humidity: payload.humidity,
            dew_point: payload.dew_point,
            pressure: payload.pressure,
            pressure_trend: PressureTrend::from_raw(payload.pressure_trend),
            condition: WeatherCondition::from_raw(payload.condition),
            symbol_name: payload.symbol_name,
            wind: Wind::from_payload(payload.wind),
            uv_index: UVIndex::from_payload(payload.uv_index),
            visibility: payload.visibility,
            cloud_cover: payload.cloud_cover,
            is_daylight: payload.is_daylight,
        }
    }
}

impl HourForecast {
    fn from_payload(payload: HourForecastPayload) -> Self {
        Self {
            date: payload.date,
            temperature: payload.temperature,
            feels_like: payload.feels_like,
            condition: WeatherCondition::from_raw(payload.condition),
            symbol_name: payload.symbol_name,
            precipitation: Precipitation::from_raw(payload.precipitation),
            precipitation_chance: payload.precipitation_chance,
            precipitation_amount: payload.precipitation_amount,
            cloud_cover: payload.cloud_cover,
        }
    }
}

impl DayForecast {
    fn from_payload(payload: DayForecastPayload) -> Self {
        Self {
            date: payload.date,
            high_temperature: payload.high_temperature,
            low_temperature: payload.low_temperature,
            condition: WeatherCondition::from_raw(payload.condition),
            symbol_name: payload.symbol_name,
            precipitation: Precipitation::from_raw(payload.precipitation),
            precipitation_chance: payload.precipitation_chance,
            precipitation_amount: payload.precipitation_amount,
        }
    }
}

impl MinuteForecast {
    fn from_payload(payload: MinuteForecastPayload) -> Self {
        Self {
            date: payload.date,
            precipitation: Precipitation::from_raw(payload.precipitation),
            precipitation_chance: payload.precipitation_chance,
            precipitation_intensity: payload.precipitation_intensity,
        }
    }
}

impl WeatherAlert {
    fn from_payload(payload: WeatherAlertPayload) -> Self {
        Self {
            summary: payload.summary,
            details_url: payload.details_url,
            source: payload.source,
            severity: payload.severity,
            region: payload.region,
        }
    }
}

impl WeatherAvailability {
    fn from_payload(payload: WeatherAvailabilityPayload) -> Self {
        Self {
            minute_availability: AvailabilityKind::from_raw(payload.minute_availability),
            alert_availability: AvailabilityKind::from_raw(payload.alert_availability),
        }
    }
}

impl Wind {
    fn from_payload(payload: WindPayload) -> Self {
        Self {
            speed: payload.speed,
            direction: payload.direction,
            compass_direction: payload.compass_direction,
            gust: payload.gust,
        }
    }
}

impl UVIndex {
    fn from_payload(payload: UVIndexPayload) -> Self {
        Self {
            value: payload.value,
            category: payload.category,
        }
    }
}
