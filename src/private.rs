use core::ffi::c_char;
use std::ffi::CStr;

use serde::de::DeserializeOwned;

use crate::error::{ErrorPayload, WeatherKitError};
use crate::ffi;

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
    let json = take_string(ptr)
        .ok_or_else(|| WeatherKitError::bridge(-1, format!("missing JSON payload for {context}")))?;
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
