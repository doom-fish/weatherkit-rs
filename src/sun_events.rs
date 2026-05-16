use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SunEvents {
    pub astronomical_dawn: Option<String>,
    pub nautical_dawn: Option<String>,
    pub civil_dawn: Option<String>,
    pub sunrise: Option<String>,
    pub solar_noon: Option<String>,
    pub sunset: Option<String>,
    pub civil_dusk: Option<String>,
    pub nautical_dusk: Option<String>,
    pub astronomical_dusk: Option<String>,
    pub solar_midnight: Option<String>,
}
