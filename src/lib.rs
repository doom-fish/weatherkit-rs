#![doc = include_str!("../README.md")]
//! Safe Rust bindings for WeatherKit.
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

pub mod availability_kind;
pub mod changes;
pub mod current_weather;
pub mod daily_forecast;
pub mod error;
/// Low-level WeatherKit bridge bindings.
pub mod ffi;
pub mod hourly_forecast;
pub mod minute_forecast;
pub mod moon_events;
pub mod pressure;
mod private;
pub mod service;
pub mod statistics;
pub mod sun_events;
pub mod weather;
pub mod weather_alert;

#[cfg(feature = "async")]
pub mod async_api;
pub mod weather_attribution;
pub mod weather_condition;

pub use availability_kind::{AvailabilityKind, AvailabilityKindDescriptor, WeatherAvailability};
pub use changes::{
    Deviation, HistoricalComparison, HistoricalComparisons, LengthUnit, Percentiles,
    TemperatureUnit, Trend, TrendBaseline, TrendBaselineKind, WeatherChange,
    WeatherChangeDirection, WeatherChanges,
};
pub use current_weather::{
    CloudCoverByAltitude, CurrentWeather, UVExposureCategory, UVExposureCategoryDescriptor,
    UVIndex, Wind, WindCompassDirection, WindCompassDirectionDescriptor,
};
pub use daily_forecast::{
    DailyForecast, DayForecast, DayPartForecast, PrecipitationAmountByType, SnowfallAmount,
};
pub use error::{
    WeatherError, WeatherErrorDescriptor, WeatherKitError, WEATHERKIT_BRIDGE_ERROR_DOMAIN,
};
pub use hourly_forecast::{HourForecast, HourlyForecast};
pub use minute_forecast::{MinuteForecast, MinuteForecastCollection};
pub use moon_events::{MoonEvents, MoonPhase, MoonPhaseDescriptor};
pub use pressure::{Pressure, PressureTrend, PressureTrendDescriptor};
pub use service::{
    CLLocation, DateInterval, Weather, WeatherMetadata, WeatherQuery, WeatherQueryResult,
    WeatherService,
};
pub use statistics::{
    DailyWeatherStatistics, DailyWeatherStatisticsQuery, DailyWeatherStatisticsResult,
    DailyWeatherSummary, DailyWeatherSummaryQuery, DailyWeatherSummaryResult,
    DayPrecipitationStatistics, DayPrecipitationSummary, DayTemperatureStatistics,
    DayTemperatureSummary, HourTemperatureStatistics, HourlyWeatherStatistics,
    HourlyWeatherStatisticsQuery, MonthPrecipitationStatistics, MonthTemperatureStatistics,
    MonthlyWeatherStatistics, MonthlyWeatherStatisticsQuery, MonthlyWeatherStatisticsResult,
};
pub use sun_events::SunEvents;
pub use weather_alert::{WeatherAlert, WeatherSeverity, WeatherSeverityDescriptor};
pub use weather_attribution::WeatherAttribution;
pub use weather_condition::{
    Precipitation, PrecipitationDescriptor, WeatherCondition, WeatherConditionDescriptor,
};

/// Common imports.
pub mod prelude {
    pub use crate::availability_kind::{
        AvailabilityKind, AvailabilityKindDescriptor, WeatherAvailability,
    };
    pub use crate::changes::{
        Deviation, HistoricalComparison, HistoricalComparisons, LengthUnit, Percentiles,
        TemperatureUnit, Trend, TrendBaseline, TrendBaselineKind, WeatherChange,
        WeatherChangeDirection, WeatherChanges,
    };
    pub use crate::current_weather::{
        CloudCoverByAltitude, CurrentWeather, UVExposureCategory, UVExposureCategoryDescriptor,
        UVIndex, Wind, WindCompassDirection, WindCompassDirectionDescriptor,
    };
    pub use crate::daily_forecast::{
        DailyForecast, DayForecast, DayPartForecast, PrecipitationAmountByType, SnowfallAmount,
    };
    pub use crate::error::{
        WeatherError, WeatherErrorDescriptor, WeatherKitError, WEATHERKIT_BRIDGE_ERROR_DOMAIN,
    };
    pub use crate::hourly_forecast::{HourForecast, HourlyForecast};
    pub use crate::minute_forecast::{MinuteForecast, MinuteForecastCollection};
    pub use crate::moon_events::{MoonEvents, MoonPhase, MoonPhaseDescriptor};
    pub use crate::pressure::{Pressure, PressureTrend, PressureTrendDescriptor};
    pub use crate::service::{
        CLLocation, DateInterval, Weather, WeatherMetadata, WeatherQuery, WeatherQueryResult,
        WeatherService,
    };
    pub use crate::statistics::{
        DailyWeatherStatistics, DailyWeatherStatisticsQuery, DailyWeatherStatisticsResult,
        DailyWeatherSummary, DailyWeatherSummaryQuery, DailyWeatherSummaryResult,
        DayPrecipitationStatistics, DayPrecipitationSummary, DayTemperatureStatistics,
        DayTemperatureSummary, HourTemperatureStatistics, HourlyWeatherStatistics,
        HourlyWeatherStatisticsQuery, MonthPrecipitationStatistics, MonthTemperatureStatistics,
        MonthlyWeatherStatistics, MonthlyWeatherStatisticsQuery, MonthlyWeatherStatisticsResult,
    };
    pub use crate::sun_events::SunEvents;
    pub use crate::weather_alert::{WeatherAlert, WeatherSeverity, WeatherSeverityDescriptor};
    pub use crate::weather_attribution::WeatherAttribution;
    pub use crate::weather_condition::{
        Precipitation, PrecipitationDescriptor, WeatherCondition, WeatherConditionDescriptor,
    };
}
