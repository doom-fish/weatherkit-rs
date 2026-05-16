use core::ffi::{c_char, c_void};

extern "C" {
    pub fn wk_weather_service_shared() -> *mut c_void;
    pub fn wk_weather_service_new() -> *mut c_void;
    pub fn wk_weather_service_retain(handle: *mut c_void) -> *mut c_void;
    pub fn wk_weather_service_release(handle: *mut c_void);

    pub fn wk_weather_service_weather(
        handle: *mut c_void,
        latitude: f64,
        longitude: f64,
        out_handle: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn wk_weather_service_current_weather(
        handle: *mut c_void,
        latitude: f64,
        longitude: f64,
        out_handle: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn wk_weather_service_hourly_forecast(
        handle: *mut c_void,
        latitude: f64,
        longitude: f64,
        has_range: i32,
        start_seconds: f64,
        end_seconds: f64,
        out_handle: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn wk_weather_service_daily_forecast(
        handle: *mut c_void,
        latitude: f64,
        longitude: f64,
        has_range: i32,
        start_seconds: f64,
        end_seconds: f64,
        out_handle: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn wk_weather_service_minute_forecast(
        handle: *mut c_void,
        latitude: f64,
        longitude: f64,
        out_handle: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn wk_weather_service_weather_alerts(
        handle: *mut c_void,
        latitude: f64,
        longitude: f64,
        out_handle: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn wk_weather_service_availability(
        handle: *mut c_void,
        latitude: f64,
        longitude: f64,
        out_handle: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn wk_weather_service_attribution(
        handle: *mut c_void,
        out_handle: *mut *mut c_void,
        out_error: *mut *mut c_char,
    ) -> i32;

    pub fn wk_weather_retain(handle: *mut c_void) -> *mut c_void;
    pub fn wk_weather_release(handle: *mut c_void);
    pub fn wk_weather_copy_json(
        handle: *mut c_void,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
}
