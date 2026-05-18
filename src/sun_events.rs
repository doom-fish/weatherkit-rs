//! WeatherKit sun event types.

use serde::Deserialize;

/// Represents WeatherKit sun event values.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SunEvents {
    /// Matches the WeatherKit astronomical dawn value.
    pub astronomical_dawn: Option<String>,
    /// Matches the WeatherKit nautical dawn value.
    pub nautical_dawn: Option<String>,
    /// Matches the WeatherKit civil dawn value.
    pub civil_dawn: Option<String>,
    /// Matches the WeatherKit sunrise value.
    pub sunrise: Option<String>,
    /// Matches the WeatherKit solar noon value.
    pub solar_noon: Option<String>,
    /// Matches the WeatherKit sunset value.
    pub sunset: Option<String>,
    /// Matches the WeatherKit civil dusk value.
    pub civil_dusk: Option<String>,
    /// Matches the WeatherKit nautical dusk value.
    pub nautical_dusk: Option<String>,
    /// Matches the WeatherKit astronomical dusk value.
    pub astronomical_dusk: Option<String>,
    /// Matches the WeatherKit solar midnight value.
    pub solar_midnight: Option<String>,
}
