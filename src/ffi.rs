#![allow(missing_docs, non_camel_case_types)]

use core::ffi::c_char;

extern "C" {
    pub fn wk_string_free(s: *mut c_char);
    pub fn wk_weather_for(
        latitude: f64,
        longitude: f64,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
}

pub mod status {
    pub const OK: i32 = 0;
    pub const INVALID_ARGUMENT: i32 = -1;
    pub const FAILURE: i32 = -2;
    pub const TIMED_OUT: i32 = -3;
}
