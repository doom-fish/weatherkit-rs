use core::ffi::c_char;

extern "C" {
    pub fn wk_weather_condition_copy_descriptors_json(
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
    pub fn wk_precipitation_copy_descriptors_json(
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
}
