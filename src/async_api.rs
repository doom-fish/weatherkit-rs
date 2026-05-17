//! Async API for `WeatherKit` — executor-agnostic `Future` wrappers.
//!
//! Enabled with the `async` Cargo feature.  Every Apple `async throws` surface
//! on [`WeatherService`](crate::WeatherService) gets a corresponding Future
//! newtype here.  The pattern follows the doom-fish gold standard from
//! `screencapturekit-rs`: a `@_cdecl` Swift thunk launches a Swift `Task`,
//! fires a C callback on completion, and the Rust side wraps the call in an
//! [`AsyncCompletionFuture`] via [`doom_fish_utils::completion::AsyncCompletion`].
//!
//! ## Available Future types
//!
//! | Future type | Swift API |
//! |-------------|-----------|
//! | [`WeatherFuture`] | `WeatherService.weather(for:)` |
//! | [`CurrentWeatherFuture`] | `WeatherService.weather(for: including: .current)` |
//! | [`HourlyForecastFuture`] | `WeatherService.weather(for: including: .hourly)` |
//! | [`DailyForecastFuture`] | `WeatherService.weather(for: including: .daily)` |
//! | [`MinuteForecastFuture`] | `WeatherService.weather(for: including: .minute)` |
//! | [`WeatherAlertsFuture`] | `WeatherService.weather(for: including: .alerts)` |
//! | [`AvailabilityFuture`] | `WeatherService.weather(for: including: .availability)` |
//! | [`AttributionFuture`] | `WeatherService.attribution` |
//! | [`WeatherChangesFuture`] | `WeatherService.weather(for: including: .changes)` (macOS 15+) |
//! | [`HistoricalComparisonsFuture`] | `WeatherService.weather(for: including: .historicalComparisons)` (macOS 15+) |
//!
//! ## Tier-2 note (streams / KVO)
//!
//! [`WeatherChange`](crate::WeatherChange) / `WeatherUpdate` multi-fire
//! observation and server-sent-event style APIs are deferred to **Tier 2**
//! (async `Stream` pattern).
//!
//! ## Example
//!
//! ```rust,no_run
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use weatherkit::async_api::AsyncWeatherService;
//! use weatherkit::service::CLLocation;
//!
//! pollster::block_on(async {
//!     let svc = AsyncWeatherService::shared();
//!     let loc = CLLocation::new(37.3382, -121.8863);
//!     let weather = svc.weather(&loc).await
//!         .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
//!     println!("{:?}", weather.current_weather.condition);
//!     Ok::<_, Box<dyn std::error::Error>>(())
//! })
//! # }
//! ```

use std::ffi::c_void;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use doom_fish_utils::completion::{error_from_cstr, AsyncCompletion, AsyncCompletionFuture};

use crate::availability_kind::WeatherAvailability;
use crate::changes::{HistoricalComparisons, WeatherChanges};
use crate::current_weather::CurrentWeather;
use crate::daily_forecast::DailyForecast;
use crate::error::WeatherKitError;
use crate::error::ErrorPayload;
use crate::ffi;
use crate::hourly_forecast::HourlyForecast;
use crate::minute_forecast::MinuteForecastCollection;
use crate::private::parse_json_from_handle;
use crate::service::{CLLocation, DateInterval, Weather, WeatherService};
use crate::weather_alert::{alerts_from_owned_ptr, WeatherAlert};
use crate::weather_attribution::WeatherAttribution;

// ============================================================================
// Helpers
// ============================================================================

fn unix_seconds(t: std::time::SystemTime) -> f64 {
    t.duration_since(std::time::UNIX_EPOCH)
        .map_or(0.0, |d| d.as_secs_f64())
}

/// Parse an error string that may be a JSON `ErrorPayload` (as emitted by the
/// async Swift thunks via `wkAsyncErrorCString`).  Falls back to a plain
/// bridge error if the string is not valid JSON.
fn parse_async_error(msg: String) -> WeatherKitError {
    serde_json::from_str::<ErrorPayload>(&msg)
        .map_or_else(|_| WeatherKitError::bridge(-1, msg), WeatherKitError::from_payload)
}

/// Acquire a raw service handle from a [`WeatherService`] value.
///
/// Returns `(ptr, is_owned)`.  When `is_owned = true` the caller must release
/// the handle with [`ffi::service::wk_weather_service_release`] after use.
fn acquire_service_ptr(svc: WeatherService) -> Result<*mut c_void, WeatherKitError> {
    let ptr = unsafe {
        if svc.is_owned() {
            ffi::service::wk_weather_service_new()
        } else {
            ffi::service::wk_weather_service_shared()
        }
    };
    if ptr.is_null() {
        Err(WeatherKitError::bridge(
            -1,
            "failed to acquire WeatherService handle",
        ))
    } else {
        Ok(ptr)
    }
}

// ============================================================================
// Macro: generate Future newtype + callback for a simple "location" query
// ============================================================================

/// Internal callback type matching the Swift side.
type RawCb = unsafe extern "C" fn(*const c_void, *const i8, *mut c_void);

// ============================================================================
// WeatherFuture — WeatherService.weather(for:)
// ============================================================================

extern "C" fn weather_cb(result: *const c_void, error: *const i8, ctx: *mut c_void) {
    if !error.is_null() {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<Weather>::complete_err(ctx, msg) };
    } else if !result.is_null() {
        let ptr = result.cast_mut();
        match parse_json_from_handle::<Weather>(
            ptr,
            ffi::service::wk_weather_release,
            ffi::service::wk_weather_copy_json,
            "weather",
        ) {
            Ok(w) => unsafe { AsyncCompletion::complete_ok(ctx, w) },
            Err(e) => unsafe { AsyncCompletion::<Weather>::complete_err(ctx, e.to_string()) },
        }
    } else {
        unsafe { AsyncCompletion::<Weather>::complete_err(ctx, "weather_cb: null result and null error".into()) };
    }
}

/// Future returned by [`AsyncWeatherService::weather`].
pub struct WeatherFuture {
    inner: AsyncCompletionFuture<Weather>,
    _service_ptr: ServicePtrGuard,
}

impl Future for WeatherFuture {
    type Output = Result<Weather, WeatherKitError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(parse_async_error))
    }
}

// ============================================================================
// CurrentWeatherFuture
// ============================================================================

extern "C" fn current_weather_cb(result: *const c_void, error: *const i8, ctx: *mut c_void) {
    if !error.is_null() {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<CurrentWeather>::complete_err(ctx, msg) };
    } else if !result.is_null() {
        let ptr = result.cast_mut();
        match CurrentWeather::from_owned_ptr(ptr) {
            Ok(v) => unsafe { AsyncCompletion::complete_ok(ctx, v) },
            Err(e) => unsafe { AsyncCompletion::<CurrentWeather>::complete_err(ctx, e.to_string()) },
        }
    } else {
        unsafe { AsyncCompletion::<CurrentWeather>::complete_err(ctx, "current_weather_cb: null result and null error".into()) };
    }
}

/// Future returned by [`AsyncWeatherService::current_weather`].
pub struct CurrentWeatherFuture {
    inner: AsyncCompletionFuture<CurrentWeather>,
    _service_ptr: ServicePtrGuard,
}

impl Future for CurrentWeatherFuture {
    type Output = Result<CurrentWeather, WeatherKitError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(parse_async_error))
    }
}

// ============================================================================
// HourlyForecastFuture
// ============================================================================

extern "C" fn hourly_forecast_cb(result: *const c_void, error: *const i8, ctx: *mut c_void) {
    if !error.is_null() {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<HourlyForecast>::complete_err(ctx, msg) };
    } else if !result.is_null() {
        let ptr = result.cast_mut();
        match HourlyForecast::from_owned_ptr(ptr) {
            Ok(v) => unsafe { AsyncCompletion::complete_ok(ctx, v) },
            Err(e) => unsafe { AsyncCompletion::<HourlyForecast>::complete_err(ctx, e.to_string()) },
        }
    } else {
        unsafe { AsyncCompletion::<HourlyForecast>::complete_err(ctx, "hourly_forecast_cb: null result and null error".into()) };
    }
}

/// Future returned by [`AsyncWeatherService::hourly_forecast`] and
/// [`AsyncWeatherService::hourly_forecast_in`].
pub struct HourlyForecastFuture {
    inner: AsyncCompletionFuture<HourlyForecast>,
    _service_ptr: ServicePtrGuard,
}

impl Future for HourlyForecastFuture {
    type Output = Result<HourlyForecast, WeatherKitError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(parse_async_error))
    }
}

// ============================================================================
// DailyForecastFuture
// ============================================================================

extern "C" fn daily_forecast_cb(result: *const c_void, error: *const i8, ctx: *mut c_void) {
    if !error.is_null() {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<DailyForecast>::complete_err(ctx, msg) };
    } else if !result.is_null() {
        let ptr = result.cast_mut();
        match DailyForecast::from_owned_ptr(ptr) {
            Ok(v) => unsafe { AsyncCompletion::complete_ok(ctx, v) },
            Err(e) => unsafe { AsyncCompletion::<DailyForecast>::complete_err(ctx, e.to_string()) },
        }
    } else {
        unsafe { AsyncCompletion::<DailyForecast>::complete_err(ctx, "daily_forecast_cb: null result and null error".into()) };
    }
}

/// Future returned by [`AsyncWeatherService::daily_forecast`] and
/// [`AsyncWeatherService::daily_forecast_in`].
pub struct DailyForecastFuture {
    inner: AsyncCompletionFuture<DailyForecast>,
    _service_ptr: ServicePtrGuard,
}

impl Future for DailyForecastFuture {
    type Output = Result<DailyForecast, WeatherKitError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(parse_async_error))
    }
}

// ============================================================================
// MinuteForecastFuture
// ============================================================================

extern "C" fn minute_forecast_cb(result: *const c_void, error: *const i8, ctx: *mut c_void) {
    if !error.is_null() {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<Option<MinuteForecastCollection>>::complete_err(ctx, msg) };
    } else if !result.is_null() {
        let ptr = result.cast_mut();
        match MinuteForecastCollection::option_from_owned_ptr(ptr) {
            Ok(v) => unsafe { AsyncCompletion::complete_ok(ctx, v) },
            Err(e) => unsafe { AsyncCompletion::<Option<MinuteForecastCollection>>::complete_err(ctx, e.to_string()) },
        }
    } else {
        unsafe { AsyncCompletion::<Option<MinuteForecastCollection>>::complete_err(ctx, "minute_forecast_cb: null result and null error".into()) };
    }
}

/// Future returned by [`AsyncWeatherService::minute_forecast`].
pub struct MinuteForecastFuture {
    inner: AsyncCompletionFuture<Option<MinuteForecastCollection>>,
    _service_ptr: ServicePtrGuard,
}

impl Future for MinuteForecastFuture {
    type Output = Result<Option<MinuteForecastCollection>, WeatherKitError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(parse_async_error))
    }
}

// ============================================================================
// WeatherAlertsFuture
// ============================================================================

extern "C" fn weather_alerts_cb(result: *const c_void, error: *const i8, ctx: *mut c_void) {
    if !error.is_null() {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<Vec<WeatherAlert>>::complete_err(ctx, msg) };
    } else if !result.is_null() {
        let ptr = result.cast_mut();
        match alerts_from_owned_ptr(ptr) {
            Ok(v) => unsafe { AsyncCompletion::complete_ok(ctx, v) },
            Err(e) => unsafe { AsyncCompletion::<Vec<WeatherAlert>>::complete_err(ctx, e.to_string()) },
        }
    } else {
        unsafe { AsyncCompletion::<Vec<WeatherAlert>>::complete_err(ctx, "weather_alerts_cb: null result and null error".into()) };
    }
}

/// Future returned by [`AsyncWeatherService::weather_alerts`].
pub struct WeatherAlertsFuture {
    inner: AsyncCompletionFuture<Vec<WeatherAlert>>,
    _service_ptr: ServicePtrGuard,
}

impl Future for WeatherAlertsFuture {
    type Output = Result<Vec<WeatherAlert>, WeatherKitError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(parse_async_error))
    }
}

// ============================================================================
// AvailabilityFuture
// ============================================================================

extern "C" fn availability_cb(result: *const c_void, error: *const i8, ctx: *mut c_void) {
    if !error.is_null() {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<WeatherAvailability>::complete_err(ctx, msg) };
    } else if !result.is_null() {
        let ptr = result.cast_mut();
        match WeatherAvailability::from_owned_ptr(ptr) {
            Ok(v) => unsafe { AsyncCompletion::complete_ok(ctx, v) },
            Err(e) => unsafe { AsyncCompletion::<WeatherAvailability>::complete_err(ctx, e.to_string()) },
        }
    } else {
        unsafe { AsyncCompletion::<WeatherAvailability>::complete_err(ctx, "availability_cb: null result and null error".into()) };
    }
}

/// Future returned by [`AsyncWeatherService::availability`].
pub struct AvailabilityFuture {
    inner: AsyncCompletionFuture<WeatherAvailability>,
    _service_ptr: ServicePtrGuard,
}

impl Future for AvailabilityFuture {
    type Output = Result<WeatherAvailability, WeatherKitError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(parse_async_error))
    }
}

// ============================================================================
// AttributionFuture
// ============================================================================

extern "C" fn attribution_cb(result: *const c_void, error: *const i8, ctx: *mut c_void) {
    if !error.is_null() {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<WeatherAttribution>::complete_err(ctx, msg) };
    } else if !result.is_null() {
        let ptr = result.cast_mut();
        match WeatherAttribution::from_owned_ptr(ptr) {
            Ok(v) => unsafe { AsyncCompletion::complete_ok(ctx, v) },
            Err(e) => unsafe { AsyncCompletion::<WeatherAttribution>::complete_err(ctx, e.to_string()) },
        }
    } else {
        unsafe { AsyncCompletion::<WeatherAttribution>::complete_err(ctx, "attribution_cb: null result and null error".into()) };
    }
}

/// Future returned by [`AsyncWeatherService::attribution`].
pub struct AttributionFuture {
    inner: AsyncCompletionFuture<WeatherAttribution>,
    _service_ptr: ServicePtrGuard,
}

impl Future for AttributionFuture {
    type Output = Result<WeatherAttribution, WeatherKitError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(parse_async_error))
    }
}

// ============================================================================
// WeatherChangesFuture — macOS 15+, returns WKBox<String> (JSON)
// ============================================================================

extern "C" fn weather_changes_cb(result: *const c_void, error: *const i8, ctx: *mut c_void) {
    if !error.is_null() {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<Option<WeatherChanges>>::complete_err(ctx, msg) };
    } else if !result.is_null() {
        let ptr = result.cast_mut();
        match WeatherChanges::option_from_owned_ptr(ptr) {
            Ok(v) => unsafe { AsyncCompletion::complete_ok(ctx, v) },
            Err(e) => unsafe { AsyncCompletion::<Option<WeatherChanges>>::complete_err(ctx, e.to_string()) },
        }
    } else {
        unsafe { AsyncCompletion::<Option<WeatherChanges>>::complete_err(ctx, "weather_changes_cb: null result and null error".into()) };
    }
}

/// Future returned by [`AsyncWeatherService::weather_changes`].
///
/// Requires macOS 15.0+.  Returns an error on older OS versions.
pub struct WeatherChangesFuture {
    inner: AsyncCompletionFuture<Option<WeatherChanges>>,
    _service_ptr: ServicePtrGuard,
}

impl Future for WeatherChangesFuture {
    type Output = Result<Option<WeatherChanges>, WeatherKitError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(parse_async_error))
    }
}

// ============================================================================
// HistoricalComparisonsFuture — macOS 15+
// ============================================================================

extern "C" fn historical_comparisons_cb(result: *const c_void, error: *const i8, ctx: *mut c_void) {
    if !error.is_null() {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<Option<HistoricalComparisons>>::complete_err(ctx, msg) };
    } else if !result.is_null() {
        let ptr = result.cast_mut();
        match HistoricalComparisons::option_from_owned_ptr(ptr) {
            Ok(v) => unsafe { AsyncCompletion::complete_ok(ctx, v) },
            Err(e) => unsafe { AsyncCompletion::<Option<HistoricalComparisons>>::complete_err(ctx, e.to_string()) },
        }
    } else {
        unsafe { AsyncCompletion::<Option<HistoricalComparisons>>::complete_err(ctx, "historical_comparisons_cb: null result and null error".into()) };
    }
}

/// Future returned by [`AsyncWeatherService::historical_comparisons`].
///
/// Requires macOS 15.0+.  Returns an error on older OS versions.
pub struct HistoricalComparisonsFuture {
    inner: AsyncCompletionFuture<Option<HistoricalComparisons>>,
    _service_ptr: ServicePtrGuard,
}

impl Future for HistoricalComparisonsFuture {
    type Output = Result<Option<HistoricalComparisons>, WeatherKitError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(parse_async_error))
    }
}

// ============================================================================
// ServicePtrGuard — RAII wrapper that releases the service handle on drop
// ============================================================================

struct ServicePtrGuard(*mut c_void);

// SAFETY: the handle is an opaque pointer to a Swift ARC object; sending it
// across threads is safe because Swift's ARC is thread-safe and the pointer
// is valid until explicitly released.
unsafe impl Send for ServicePtrGuard {}
unsafe impl Sync for ServicePtrGuard {}

impl Drop for ServicePtrGuard {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { ffi::service::wk_weather_service_release(self.0) };
            self.0 = std::ptr::null_mut();
        }
    }
}

// ============================================================================
// AsyncWeatherService — the public entry point
// ============================================================================

/// Async wrapper around [`WeatherService`].
///
/// Create one with [`AsyncWeatherService::shared()`] or
/// [`AsyncWeatherService::new()`], then call any of its methods to get a
/// `Future`.  The futures are **executor-agnostic** — use `pollster`, Tokio,
/// async-std, or any other runtime.
///
/// # Example
///
/// ```rust,no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use weatherkit::async_api::AsyncWeatherService;
/// use weatherkit::service::CLLocation;
///
/// pollster::block_on(async {
///     let svc = AsyncWeatherService::shared();
///     let sf = CLLocation::new(37.7749, -122.4194);
///     let weather = svc.weather(&sf).await
///         .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
///     println!("Condition: {:?}", weather.current_weather.condition);
///     Ok::<_, Box<dyn std::error::Error>>(())
/// })
/// # }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct AsyncWeatherService {
    inner: WeatherService,
}

impl AsyncWeatherService {
    /// Use the shared `WeatherService` singleton.
    pub const fn shared() -> Self {
        Self { inner: WeatherService::shared() }
    }

    /// Create a new `WeatherService` instance.
    pub const fn new() -> Self {
        Self { inner: WeatherService::new() }
    }

    // -----------------------------------------------------------------------
    // Private helper: acquire a retained service handle to pass to Swift.
    // The handle is wrapped in ServicePtrGuard and released when the Future
    // (and its associated guard) is dropped.
    // -----------------------------------------------------------------------

    fn service_ptr(self) -> Result<*mut c_void, WeatherKitError> {
        acquire_service_ptr(self.inner)
    }

    // -----------------------------------------------------------------------
    // Public async API
    // -----------------------------------------------------------------------

    /// Fetch the full [`Weather`](crate::service::Weather) bundle.
    ///
    /// Wraps `WeatherService.weather(for:) async throws`.
    ///
    /// # Errors
    /// Returns a [`WeatherKitError`] if the app lacks a valid WeatherKit
    /// entitlement or if the network request fails.
    pub fn weather(&self, location: &CLLocation) -> WeatherFuture {
        let ptr = match self.service_ptr() {
            Ok(p) => p,
            Err(e) => {
                let (future, ctx) = AsyncCompletion::create();
                unsafe { AsyncCompletion::<Weather>::complete_err(ctx, e.to_string()) };
                return WeatherFuture { inner: future, _service_ptr: ServicePtrGuard(std::ptr::null_mut()) };
            }
        };
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            ffi::async_ffi::wk_weather_service_weather_async(
                ptr,
                location.latitude,
                location.longitude,
                weather_cb as RawCb,
                ctx,
            );
        }
        WeatherFuture { inner: future, _service_ptr: ServicePtrGuard(ptr) }
    }

    /// Fetch only the current conditions.
    ///
    /// Wraps `WeatherService.weather(for: including: .current) async throws`.
    pub fn current_weather(&self, location: &CLLocation) -> CurrentWeatherFuture {
        let ptr = match self.service_ptr() {
            Ok(p) => p,
            Err(e) => {
                let (future, ctx) = AsyncCompletion::create();
                unsafe { AsyncCompletion::<CurrentWeather>::complete_err(ctx, e.to_string()) };
                return CurrentWeatherFuture { inner: future, _service_ptr: ServicePtrGuard(std::ptr::null_mut()) };
            }
        };
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            ffi::async_ffi::wk_weather_service_current_weather_async(
                ptr,
                location.latitude,
                location.longitude,
                current_weather_cb as RawCb,
                ctx,
            );
        }
        CurrentWeatherFuture { inner: future, _service_ptr: ServicePtrGuard(ptr) }
    }

    /// Fetch the hourly forecast.
    ///
    /// Wraps `WeatherService.weather(for: including: .hourly) async throws`.
    pub fn hourly_forecast(&self, location: &CLLocation) -> HourlyForecastFuture {
        self.hourly_forecast_impl(location, None)
    }

    /// Fetch the hourly forecast for a specific date interval.
    ///
    /// Wraps `WeatherService.weather(for: including: .hourly(startDate:endDate:)) async throws`.
    pub fn hourly_forecast_in(
        &self,
        location: &CLLocation,
        interval: DateInterval,
    ) -> HourlyForecastFuture {
        self.hourly_forecast_impl(location, Some(interval))
    }

    fn hourly_forecast_impl(
        &self,
        location: &CLLocation,
        interval: Option<DateInterval>,
    ) -> HourlyForecastFuture {
        let ptr = match self.service_ptr() {
            Ok(p) => p,
            Err(e) => {
                let (future, ctx) = AsyncCompletion::create();
                unsafe { AsyncCompletion::<HourlyForecast>::complete_err(ctx, e.to_string()) };
                return HourlyForecastFuture { inner: future, _service_ptr: ServicePtrGuard(std::ptr::null_mut()) };
            }
        };
        let (has_range, start_s, end_s) = interval
            .as_ref()
            .map_or((0, 0.0, 0.0), |iv| (1_i32, unix_seconds(iv.start), unix_seconds(iv.end)));
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            ffi::async_ffi::wk_weather_service_hourly_forecast_async(
                ptr,
                location.latitude,
                location.longitude,
                has_range,
                start_s,
                end_s,
                hourly_forecast_cb as RawCb,
                ctx,
            );
        }
        HourlyForecastFuture { inner: future, _service_ptr: ServicePtrGuard(ptr) }
    }

    /// Fetch the daily forecast.
    ///
    /// Wraps `WeatherService.weather(for: including: .daily) async throws`.
    pub fn daily_forecast(&self, location: &CLLocation) -> DailyForecastFuture {
        self.daily_forecast_impl(location, None)
    }

    /// Fetch the daily forecast for a specific date interval.
    ///
    /// Wraps `WeatherService.weather(for: including: .daily(startDate:endDate:)) async throws`.
    pub fn daily_forecast_in(
        &self,
        location: &CLLocation,
        interval: DateInterval,
    ) -> DailyForecastFuture {
        self.daily_forecast_impl(location, Some(interval))
    }

    fn daily_forecast_impl(
        &self,
        location: &CLLocation,
        interval: Option<DateInterval>,
    ) -> DailyForecastFuture {
        let ptr = match self.service_ptr() {
            Ok(p) => p,
            Err(e) => {
                let (future, ctx) = AsyncCompletion::create();
                unsafe { AsyncCompletion::<DailyForecast>::complete_err(ctx, e.to_string()) };
                return DailyForecastFuture { inner: future, _service_ptr: ServicePtrGuard(std::ptr::null_mut()) };
            }
        };
        let (has_range, start_s, end_s) = interval
            .as_ref()
            .map_or((0, 0.0, 0.0), |iv| (1_i32, unix_seconds(iv.start), unix_seconds(iv.end)));
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            ffi::async_ffi::wk_weather_service_daily_forecast_async(
                ptr,
                location.latitude,
                location.longitude,
                has_range,
                start_s,
                end_s,
                daily_forecast_cb as RawCb,
                ctx,
            );
        }
        DailyForecastFuture { inner: future, _service_ptr: ServicePtrGuard(ptr) }
    }

    /// Fetch the minute-by-minute precipitation forecast.
    ///
    /// Wraps `WeatherService.weather(for: including: .minute) async throws`.
    ///
    /// Returns `None` if minute forecast is not available for the location.
    pub fn minute_forecast(&self, location: &CLLocation) -> MinuteForecastFuture {
        let ptr = match self.service_ptr() {
            Ok(p) => p,
            Err(e) => {
                let (future, ctx) = AsyncCompletion::create();
                unsafe { AsyncCompletion::<Option<MinuteForecastCollection>>::complete_err(ctx, e.to_string()) };
                return MinuteForecastFuture { inner: future, _service_ptr: ServicePtrGuard(std::ptr::null_mut()) };
            }
        };
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            ffi::async_ffi::wk_weather_service_minute_forecast_async(
                ptr,
                location.latitude,
                location.longitude,
                minute_forecast_cb as RawCb,
                ctx,
            );
        }
        MinuteForecastFuture { inner: future, _service_ptr: ServicePtrGuard(ptr) }
    }

    /// Fetch weather alerts for the location.
    ///
    /// Wraps `WeatherService.weather(for: including: .alerts) async throws`.
    pub fn weather_alerts(&self, location: &CLLocation) -> WeatherAlertsFuture {
        let ptr = match self.service_ptr() {
            Ok(p) => p,
            Err(e) => {
                let (future, ctx) = AsyncCompletion::create();
                unsafe { AsyncCompletion::<Vec<WeatherAlert>>::complete_err(ctx, e.to_string()) };
                return WeatherAlertsFuture { inner: future, _service_ptr: ServicePtrGuard(std::ptr::null_mut()) };
            }
        };
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            ffi::async_ffi::wk_weather_service_weather_alerts_async(
                ptr,
                location.latitude,
                location.longitude,
                weather_alerts_cb as RawCb,
                ctx,
            );
        }
        WeatherAlertsFuture { inner: future, _service_ptr: ServicePtrGuard(ptr) }
    }

    /// Fetch data-availability information for the location.
    ///
    /// Wraps `WeatherService.weather(for: including: .availability) async throws`.
    pub fn availability(&self, location: &CLLocation) -> AvailabilityFuture {
        let ptr = match self.service_ptr() {
            Ok(p) => p,
            Err(e) => {
                let (future, ctx) = AsyncCompletion::create();
                unsafe { AsyncCompletion::<WeatherAvailability>::complete_err(ctx, e.to_string()) };
                return AvailabilityFuture { inner: future, _service_ptr: ServicePtrGuard(std::ptr::null_mut()) };
            }
        };
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            ffi::async_ffi::wk_weather_service_availability_async(
                ptr,
                location.latitude,
                location.longitude,
                availability_cb as RawCb,
                ctx,
            );
        }
        AvailabilityFuture { inner: future, _service_ptr: ServicePtrGuard(ptr) }
    }

    /// Fetch the WeatherKit attribution metadata required to be displayed
    /// in your UI.
    ///
    /// Wraps `WeatherService.attribution async throws`.
    pub fn attribution(&self) -> AttributionFuture {
        let ptr = match self.service_ptr() {
            Ok(p) => p,
            Err(e) => {
                let (future, ctx) = AsyncCompletion::create();
                unsafe { AsyncCompletion::<WeatherAttribution>::complete_err(ctx, e.to_string()) };
                return AttributionFuture { inner: future, _service_ptr: ServicePtrGuard(std::ptr::null_mut()) };
            }
        };
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            ffi::async_ffi::wk_weather_service_attribution_async(
                ptr,
                attribution_cb as RawCb,
                ctx,
            );
        }
        AttributionFuture { inner: future, _service_ptr: ServicePtrGuard(ptr) }
    }

    /// Fetch weather-change forecasts (macOS 15.0+).
    ///
    /// Wraps `WeatherService.weather(for: including: .changes) async throws`.
    ///
    /// Returns an error on macOS < 15.0.
    pub fn weather_changes(&self, location: &CLLocation) -> WeatherChangesFuture {
        let ptr = match self.service_ptr() {
            Ok(p) => p,
            Err(e) => {
                let (future, ctx) = AsyncCompletion::create();
                unsafe { AsyncCompletion::<Option<WeatherChanges>>::complete_err(ctx, e.to_string()) };
                return WeatherChangesFuture { inner: future, _service_ptr: ServicePtrGuard(std::ptr::null_mut()) };
            }
        };
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            ffi::async_ffi::wk_weather_service_weather_changes_async(
                ptr,
                location.latitude,
                location.longitude,
                weather_changes_cb as RawCb,
                ctx,
            );
        }
        WeatherChangesFuture { inner: future, _service_ptr: ServicePtrGuard(ptr) }
    }

    /// Fetch historical weather comparisons (macOS 15.0+).
    ///
    /// Wraps `WeatherService.weather(for: including: .historicalComparisons) async throws`.
    ///
    /// Returns an error on macOS < 15.0.
    pub fn historical_comparisons(&self, location: &CLLocation) -> HistoricalComparisonsFuture {
        let ptr = match self.service_ptr() {
            Ok(p) => p,
            Err(e) => {
                let (future, ctx) = AsyncCompletion::create();
                unsafe { AsyncCompletion::<Option<HistoricalComparisons>>::complete_err(ctx, e.to_string()) };
                return HistoricalComparisonsFuture { inner: future, _service_ptr: ServicePtrGuard(std::ptr::null_mut()) };
            }
        };
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            ffi::async_ffi::wk_weather_service_historical_comparisons_async(
                ptr,
                location.latitude,
                location.longitude,
                historical_comparisons_cb as RawCb,
                ctx,
            );
        }
        HistoricalComparisonsFuture { inner: future, _service_ptr: ServicePtrGuard(ptr) }
    }
}

impl Default for AsyncWeatherService {
    fn default() -> Self {
        Self::shared()
    }
}
