use core::ffi::{c_char, c_void};

extern "C" {
    pub fn wk_weather_service_weather_changes(
        handle: *mut c_void,
        latitude: f64,
        longitude: f64,
        out_handle: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn wk_weather_service_historical_comparisons(
        handle: *mut c_void,
        latitude: f64,
        longitude: f64,
        out_handle: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
}
