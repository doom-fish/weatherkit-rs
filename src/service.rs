use core::ffi::{c_char, c_void};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Deserialize;

use crate::availability_kind::WeatherAvailability;
use crate::current_weather::CurrentWeather;
use crate::daily_forecast::{DailyForecast, DayForecast};
use crate::error::WeatherKitError;
use crate::ffi;
use crate::hourly_forecast::HourlyForecast;
use crate::minute_forecast::MinuteForecastCollection;
use crate::moon_events::MoonEvents;
use crate::pressure::Pressure;
use crate::private::{error_from_status, parse_json_from_handle};
use crate::sun_events::SunEvents;
use crate::weather_alert::{alerts_from_owned_ptr, WeatherAlert};
use crate::weather_attribution::WeatherAttribution;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CLLocation {
    pub latitude: f64,
    pub longitude: f64,
}

impl CLLocation {
    pub const fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
        }
    }

    fn validate(&self) -> Result<(), WeatherKitError> {
        if !self.latitude.is_finite() || !self.longitude.is_finite() {
            return Err(WeatherKitError::bridge(
                -1,
                "latitude and longitude must be finite numbers",
            ));
        }
        if !(-90.0..=90.0).contains(&self.latitude) {
            return Err(WeatherKitError::bridge(
                -1,
                format!("latitude {} is outside -90..=90", self.latitude),
            ));
        }
        if !(-180.0..=180.0).contains(&self.longitude) {
            return Err(WeatherKitError::bridge(
                -1,
                format!("longitude {} is outside -180..=180", self.longitude),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateInterval {
    pub start: SystemTime,
    pub end: SystemTime,
}

impl DateInterval {
    pub fn new(start: SystemTime, end: SystemTime) -> Result<Self, WeatherKitError> {
        if start > end {
            return Err(WeatherKitError::bridge(
                -1,
                "date interval start must not be after end",
            ));
        }
        Ok(Self { start, end })
    }

    fn start_seconds(&self) -> Result<f64, WeatherKitError> {
        unix_seconds(self.start)
    }

    fn end_seconds(&self) -> Result<f64, WeatherKitError> {
        unix_seconds(self.end)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherMetadata {
    pub date: String,
    pub expiration_date: String,
    pub location: CLLocation,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weather {
    pub current_weather: CurrentWeather,
    pub hourly_forecast: Vec<crate::hourly_forecast::HourForecast>,
    pub daily_forecast: Vec<DayForecast>,
    pub minute_forecast: Option<Vec<crate::minute_forecast::MinuteForecast>>,
    #[serde(default)]
    pub weather_alerts: Vec<WeatherAlert>,
    pub availability: WeatherAvailability,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct WeatherService {
    kind: ServiceKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum ServiceKind {
    #[default]
    Shared,
    Owned,
}

struct ServiceHandle {
    ptr: *mut c_void,
}

type LocationFetchFn =
    unsafe extern "C" fn(*mut c_void, f64, f64, *mut *mut c_void, *mut *mut c_char) -> i32;
type IntervalFetchFn = unsafe extern "C" fn(
    *mut c_void,
    f64,
    f64,
    i32,
    f64,
    f64,
    *mut *mut c_void,
    *mut *mut c_char,
) -> i32;
type ServiceFetchFn = unsafe extern "C" fn(*mut c_void, *mut *mut c_void, *mut *mut c_char) -> i32;

impl ServiceHandle {
    fn acquire(kind: ServiceKind) -> Result<Self, WeatherKitError> {
        let ptr = unsafe {
            match kind {
                ServiceKind::Shared => ffi::service::wk_weather_service_shared(),
                ServiceKind::Owned => ffi::service::wk_weather_service_new(),
            }
        };
        if ptr.is_null() {
            Err(WeatherKitError::bridge(
                -1,
                "failed to acquire WeatherService handle",
            ))
        } else {
            Ok(Self { ptr })
        }
    }

    fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for ServiceHandle {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                ffi::service::wk_weather_service_release(self.ptr);
            }
            self.ptr = core::ptr::null_mut();
        }
    }
}

impl WeatherService {
    pub const fn shared() -> Self {
        Self {
            kind: ServiceKind::Shared,
        }
    }

    pub const fn new() -> Self {
        Self {
            kind: ServiceKind::Owned,
        }
    }

    pub fn attribution(&self) -> Result<WeatherAttribution, WeatherKitError> {
        let ptr = self.fetch_service_handle(
            ffi::service::wk_weather_service_attribution,
            "WeatherService.attribution",
        )?;
        WeatherAttribution::from_owned_ptr(ptr)
    }

    pub fn weather(&self, location: &CLLocation) -> Result<Weather, WeatherKitError> {
        let ptr = self.fetch_location_handle(
            location,
            ffi::service::wk_weather_service_weather,
            "WeatherService.weather(for:)",
        )?;
        parse_json_from_handle(
            ptr,
            ffi::service::wk_weather_release,
            ffi::service::wk_weather_copy_json,
            "weather",
        )
    }

    pub fn current_weather(
        &self,
        location: &CLLocation,
    ) -> Result<CurrentWeather, WeatherKitError> {
        let ptr = self.fetch_location_handle(
            location,
            ffi::service::wk_weather_service_current_weather,
            "WeatherService.weather(for: including: .current)",
        )?;
        CurrentWeather::from_owned_ptr(ptr)
    }

    pub fn hourly_forecast(
        &self,
        location: &CLLocation,
    ) -> Result<HourlyForecast, WeatherKitError> {
        let ptr = self.fetch_interval_handle(
            location,
            None,
            ffi::service::wk_weather_service_hourly_forecast,
            "WeatherService.weather(for: including: .hourly)",
        )?;
        HourlyForecast::from_owned_ptr(ptr)
    }

    pub fn hourly_forecast_in(
        &self,
        location: &CLLocation,
        interval: DateInterval,
    ) -> Result<HourlyForecast, WeatherKitError> {
        let ptr = self.fetch_interval_handle(
            location,
            Some(&interval),
            ffi::service::wk_weather_service_hourly_forecast,
            "WeatherService.weather(for: including: .hourly(startDate:endDate))",
        )?;
        HourlyForecast::from_owned_ptr(ptr)
    }

    pub fn daily_forecast(&self, location: &CLLocation) -> Result<DailyForecast, WeatherKitError> {
        let ptr = self.fetch_interval_handle(
            location,
            None,
            ffi::service::wk_weather_service_daily_forecast,
            "WeatherService.weather(for: including: .daily)",
        )?;
        DailyForecast::from_owned_ptr(ptr)
    }

    pub fn daily_forecast_in(
        &self,
        location: &CLLocation,
        interval: DateInterval,
    ) -> Result<DailyForecast, WeatherKitError> {
        let ptr = self.fetch_interval_handle(
            location,
            Some(&interval),
            ffi::service::wk_weather_service_daily_forecast,
            "WeatherService.weather(for: including: .daily(startDate:endDate))",
        )?;
        DailyForecast::from_owned_ptr(ptr)
    }

    pub fn minute_forecast(
        &self,
        location: &CLLocation,
    ) -> Result<Option<MinuteForecastCollection>, WeatherKitError> {
        let ptr = self.fetch_location_handle(
            location,
            ffi::service::wk_weather_service_minute_forecast,
            "WeatherService.weather(for: including: .minute)",
        )?;
        MinuteForecastCollection::option_from_owned_ptr(ptr)
    }

    pub fn weather_alerts(
        &self,
        location: &CLLocation,
    ) -> Result<Vec<WeatherAlert>, WeatherKitError> {
        let ptr = self.fetch_location_handle(
            location,
            ffi::service::wk_weather_service_weather_alerts,
            "WeatherService.weather(for: including: .alerts)",
        )?;
        alerts_from_owned_ptr(ptr)
    }

    pub fn availability(
        &self,
        location: &CLLocation,
    ) -> Result<WeatherAvailability, WeatherKitError> {
        let ptr = self.fetch_location_handle(
            location,
            ffi::service::wk_weather_service_availability,
            "WeatherService.weather(for: including: .availability)",
        )?;
        WeatherAvailability::from_owned_ptr(ptr)
    }

    pub fn sun_events(&self, location: &CLLocation) -> Result<SunEvents, WeatherKitError> {
        let forecast = self.daily_forecast(location)?;
        forecast
            .forecast
            .first()
            .map(|day| day.sun.clone())
            .ok_or_else(|| WeatherKitError::bridge(-1, "daily forecast returned no days"))
    }

    pub fn moon_events(&self, location: &CLLocation) -> Result<MoonEvents, WeatherKitError> {
        let forecast = self.daily_forecast(location)?;
        forecast
            .forecast
            .first()
            .map(|day| day.moon.clone())
            .ok_or_else(|| WeatherKitError::bridge(-1, "daily forecast returned no days"))
    }

    pub fn pressure(&self, location: &CLLocation) -> Result<Pressure, WeatherKitError> {
        Ok(self.current_weather(location)?.pressure_reading())
    }

    fn fetch_service_handle(
        &self,
        call: ServiceFetchFn,
        context: &str,
    ) -> Result<*mut c_void, WeatherKitError> {
        let service = ServiceHandle::acquire(self.kind)?;
        let mut out_handle = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe { call(service.as_ptr(), &mut out_handle, &mut out_error) };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        if out_handle.is_null() {
            return Err(WeatherKitError::bridge(
                -1,
                format!("missing handle for {context}"),
            ));
        }
        Ok(out_handle)
    }

    fn fetch_location_handle(
        &self,
        location: &CLLocation,
        call: LocationFetchFn,
        context: &str,
    ) -> Result<*mut c_void, WeatherKitError> {
        location.validate()?;
        let service = ServiceHandle::acquire(self.kind)?;
        let mut out_handle = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let status = unsafe {
            call(
                service.as_ptr(),
                location.latitude,
                location.longitude,
                &mut out_handle,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        if out_handle.is_null() {
            return Err(WeatherKitError::bridge(
                -1,
                format!("missing handle for {context}"),
            ));
        }
        Ok(out_handle)
    }

    fn fetch_interval_handle(
        &self,
        location: &CLLocation,
        interval: Option<&DateInterval>,
        call: IntervalFetchFn,
        context: &str,
    ) -> Result<*mut c_void, WeatherKitError> {
        location.validate()?;
        let service = ServiceHandle::acquire(self.kind)?;
        let mut out_handle = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let (has_range, start_seconds, end_seconds) = if let Some(interval) = interval {
            (1, interval.start_seconds()?, interval.end_seconds()?)
        } else {
            (0, 0.0, 0.0)
        };
        let status = unsafe {
            call(
                service.as_ptr(),
                location.latitude,
                location.longitude,
                has_range,
                start_seconds,
                end_seconds,
                &mut out_handle,
                &mut out_error,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { error_from_status(status, out_error) });
        }
        if out_handle.is_null() {
            return Err(WeatherKitError::bridge(
                -1,
                format!("missing handle for {context}"),
            ));
        }
        Ok(out_handle)
    }
}

fn unix_seconds(time: SystemTime) -> Result<f64, WeatherKitError> {
    let duration = time.duration_since(UNIX_EPOCH).map_err(|error| {
        WeatherKitError::bridge(-1, format!("time {time:?} is before UNIX_EPOCH: {error}"))
    })?;
    Ok(duration.as_secs_f64())
}
