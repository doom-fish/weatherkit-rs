use core::ffi::{c_char, c_void};

extern "C" {
    pub fn wk_current_weather_retain(handle: *mut c_void) -> *mut c_void;
    pub fn wk_current_weather_release(handle: *mut c_void);
    pub fn wk_current_weather_copy_json(
        handle: *mut c_void,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn wk_wind_compass_direction_copy_descriptors_json(
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn wk_uv_exposure_category_copy_descriptors_json(
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
}
