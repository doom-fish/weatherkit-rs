#![allow(missing_docs, non_camel_case_types)]

use core::ffi::c_char;

pub mod availability_kind;
pub mod current_weather;
pub mod daily_forecast;
pub mod hourly_forecast;
pub mod minute_forecast;
pub mod moon_events;
pub mod pressure;
pub mod service;
pub mod sun_events;
pub mod weather_alert;
pub mod weather_attribution;
pub mod weather_condition;

extern "C" {
    pub fn wk_string_free(s: *mut c_char);
}

pub mod status {
    pub const OK: i32 = 0;
    pub const INVALID_ARGUMENT: i32 = -1;
    pub const FAILURE: i32 = -2;
    pub const TIMED_OUT: i32 = -3;
}
