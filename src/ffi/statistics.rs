use core::ffi::{c_char, c_void};

extern "C" {
    pub fn wk_weather_service_daily_statistics(
        handle: *mut c_void,
        latitude: f64,
        longitude: f64,
        query_kind: i32,
        scope_kind: i32,
        start_seconds: f64,
        end_seconds: f64,
        start_index: i64,
        end_index: i64,
        out_handle: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn wk_weather_service_daily_summary(
        handle: *mut c_void,
        latitude: f64,
        longitude: f64,
        query_kind: i32,
        scope_kind: i32,
        start_seconds: f64,
        end_seconds: f64,
        start_index: i64,
        end_index: i64,
        out_handle: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn wk_weather_service_hourly_statistics(
        handle: *mut c_void,
        latitude: f64,
        longitude: f64,
        query_kind: i32,
        scope_kind: i32,
        start_seconds: f64,
        end_seconds: f64,
        start_index: i64,
        end_index: i64,
        out_handle: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn wk_weather_service_monthly_statistics(
        handle: *mut c_void,
        latitude: f64,
        longitude: f64,
        query_kind: i32,
        scope_kind: i32,
        start_seconds: f64,
        end_seconds: f64,
        start_index: i64,
        end_index: i64,
        out_handle: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
}
