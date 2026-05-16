use core::ffi::{c_char, c_void};

extern "C" {
    pub fn wk_daily_forecast_retain(handle: *mut c_void) -> *mut c_void;
    pub fn wk_daily_forecast_release(handle: *mut c_void);
    pub fn wk_daily_forecast_copy_json(
        handle: *mut c_void,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
}
