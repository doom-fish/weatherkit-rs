#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::cargo_common_metadata,
    clippy::cast_precision_loss,
    clippy::doc_markdown,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::needless_pass_by_value,
    clippy::redundant_pub_crate,
    clippy::return_self_not_must_use,
    clippy::struct_field_names,
    clippy::type_complexity,
    clippy::use_self
)]

pub mod error;
pub mod ffi;
mod private;
pub mod service;
pub mod weather;

pub use error::{WeatherKitError, WEATHERKIT_BRIDGE_ERROR_DOMAIN};
pub use service::{CLLocation, WeatherService};
pub use weather::{
    AvailabilityKind, CurrentWeather, DayForecast, HourForecast, MinuteForecast, Precipitation,
    PressureTrend, UVIndex, Weather, WeatherAlert, WeatherAvailability, WeatherCondition, Wind,
};

/// Common imports.
pub mod prelude {
    pub use crate::error::{WeatherKitError, WEATHERKIT_BRIDGE_ERROR_DOMAIN};
    pub use crate::service::{CLLocation, WeatherService};
    pub use crate::weather::{
        AvailabilityKind, CurrentWeather, DayForecast, HourForecast, MinuteForecast,
        Precipitation, PressureTrend, UVIndex, Weather, WeatherAlert, WeatherAvailability,
        WeatherCondition, Wind,
    };
}
