use core::ffi::{c_char, c_void};
use std::ffi::CStr;

use serde::de::DeserializeOwned;

use crate::error::{ErrorPayload, WeatherKitError};
use crate::ffi;

pub(crate) type JsonHandleCopyFn =
    unsafe extern "C" fn(*mut c_void, *mut *mut c_char, *mut *mut c_char) -> i32;
pub(crate) type JsonStaticCopyFn = unsafe extern "C" fn(*mut *mut c_char, *mut *mut c_char) -> i32;
pub(crate) type ReleaseFn = unsafe extern "C" fn(*mut c_void);

pub(crate) struct OwnedHandle {
    ptr: *mut c_void,
    release: ReleaseFn,
}

impl OwnedHandle {
    pub(crate) unsafe fn new(
        ptr: *mut c_void,
        release: ReleaseFn,
        context: &str,
    ) -> Result<Self, WeatherKitError> {
        if ptr.is_null() {
            Err(WeatherKitError::bridge(
                -1,
                format!("missing handle for {context}"),
            ))
        } else {
            Ok(Self { ptr, release })
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for OwnedHandle {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                (self.release)(self.ptr);
            }
            self.ptr = core::ptr::null_mut();
        }
    }
}

pub(crate) unsafe fn take_string(ptr: *mut c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    let string = CStr::from_ptr(ptr).to_string_lossy().into_owned();
    ffi::wk_string_free(ptr);
    Some(string)
}

pub(crate) fn parse_json_str<T: DeserializeOwned>(
    json: &str,
    context: &str,
) -> Result<T, WeatherKitError> {
    serde_json::from_str(json).map_err(|error| {
        WeatherKitError::bridge(
            -1,
            format!("failed to parse {context} JSON: {error}; payload={json}"),
        )
    })
}

pub(crate) unsafe fn parse_json_ptr<T: DeserializeOwned>(
    ptr: *mut c_char,
    context: &str,
) -> Result<T, WeatherKitError> {
    let json = take_string(ptr).ok_or_else(|| {
        WeatherKitError::bridge(-1, format!("missing JSON payload for {context}"))
    })?;
    parse_json_str(&json, context)
}

pub(crate) unsafe fn parse_error_ptr(ptr: *mut c_char) -> WeatherKitError {
    if ptr.is_null() {
        return WeatherKitError::bridge(-2, "WeatherKit bridge returned an error without payload");
    }
    let json = CStr::from_ptr(ptr).to_string_lossy().into_owned();
    ffi::wk_string_free(ptr);
    match serde_json::from_str::<ErrorPayload>(&json) {
        Ok(payload) => WeatherKitError::from_payload(payload),
        Err(error) => WeatherKitError::bridge(
            -1,
            format!("failed to parse WeatherKit error payload: {error}; payload={json}"),
        ),
    }
}

pub(crate) unsafe fn error_from_status(status: i32, err_msg: *mut c_char) -> WeatherKitError {
    if !err_msg.is_null() {
        return parse_error_ptr(err_msg);
    }
    let message = match status {
        ffi::status::INVALID_ARGUMENT => "invalid argument",
        ffi::status::TIMED_OUT => "timed out waiting for WeatherKit",
        _ => "WeatherKit bridge failure",
    };
    WeatherKitError::bridge(i64::from(status), message)
}

pub(crate) fn parse_json_from_handle<T: DeserializeOwned>(
    ptr: *mut c_void,
    release: ReleaseFn,
    copy_json: JsonHandleCopyFn,
    context: &str,
) -> Result<T, WeatherKitError> {
    let handle = unsafe { OwnedHandle::new(ptr, release, context)? };
    let mut out_json = core::ptr::null_mut();
    let mut out_error = core::ptr::null_mut();
    let status = unsafe { copy_json(handle.as_ptr(), &mut out_json, &mut out_error) };
    if status != ffi::status::OK {
        return Err(unsafe { error_from_status(status, out_error) });
    }
    unsafe { parse_json_ptr(out_json, context) }
}

pub(crate) fn parse_json_from_static<T: DeserializeOwned>(
    copy_json: JsonStaticCopyFn,
    context: &str,
) -> Result<T, WeatherKitError> {
    let mut out_json = core::ptr::null_mut();
    let mut out_error = core::ptr::null_mut();
    let status = unsafe { copy_json(&mut out_json, &mut out_error) };
    if status != ffi::status::OK {
        return Err(unsafe { error_from_status(status, out_error) });
    }
    unsafe { parse_json_ptr(out_json, context) }
}
