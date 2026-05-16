use core::ffi::c_char;

extern "C" {
    pub fn wk_pressure_trend_copy_descriptors_json(
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
}
