use crate::error::WeatherKitError;
use crate::ffi;
use crate::private::{error_from_status, parse_json_ptr};
use crate::weather::{Weather, WeatherPayload};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CLLocation {
    pub latitude: f64,
    pub longitude: f64,
}

impl CLLocation {
    pub const fn new(latitude: f64, longitude: f64) -> Self {
        Self { latitude, longitude }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct WeatherService;

impl WeatherService {
    pub const fn shared() -> Self {
        Self
    }

    pub fn weather(&self, location: &CLLocation) -> Result<Weather, WeatherKitError> {
        let mut out_json = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            ffi::wk_weather_for(
                location.latitude,
                location.longitude,
                &mut out_json,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        let payload: WeatherPayload = unsafe { parse_json_ptr(out_json, "weather payload")? };
        Ok(Weather::from_payload(payload))
    }
}
