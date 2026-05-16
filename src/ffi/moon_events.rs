use core::ffi::c_char;

extern "C" {
    pub fn wk_moon_phase_copy_descriptors_json(
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
}
