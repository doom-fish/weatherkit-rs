use core::ffi::{c_char, c_void};

extern "C" {
    pub fn wk_json_handle_release(handle: *mut c_void);
    pub fn wk_json_handle_copy_json(
        handle: *mut c_void,
        out_json: *mut *mut c_char,
        out_error: *mut *mut c_char,
    ) -> i32;
}
