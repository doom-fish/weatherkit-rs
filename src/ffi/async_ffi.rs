use core::ffi::{c_char, c_void};

pub type AsyncCb =
    unsafe extern "C" fn(*const c_void, *const c_char, *mut c_void);

extern "C" {
    /// `WeatherService.weather(for:)` async → `WKBox<Weather>`
    pub fn wk_weather_service_weather_async(
        handle: *mut c_void,
        latitude: f64,
        longitude: f64,
        cb: AsyncCb,
        ctx: *mut c_void,
    );

    /// `WeatherService.weather(for: including: .current)` async → `WKBox<CurrentWeather>`
    pub fn wk_weather_service_current_weather_async(
        handle: *mut c_void,
        latitude: f64,
        longitude: f64,
        cb: AsyncCb,
        ctx: *mut c_void,
    );

    /// `WeatherService.weather(for: including: .hourly)` async → `WKBox<Forecast<HourWeather>>`
    ///
    /// Pass `has_range = 1` and valid `start_seconds`/`end_seconds` for a date-ranged query.
    pub fn wk_weather_service_hourly_forecast_async(
        handle: *mut c_void,
        latitude: f64,
        longitude: f64,
        has_range: i32,
        start_seconds: f64,
        end_seconds: f64,
        cb: AsyncCb,
        ctx: *mut c_void,
    );

    /// `WeatherService.weather(for: including: .daily)` async → `WKBox<Forecast<DayWeather>>`
    ///
    /// Pass `has_range = 1` and valid `start_seconds`/`end_seconds` for a date-ranged query.
    pub fn wk_weather_service_daily_forecast_async(
        handle: *mut c_void,
        latitude: f64,
        longitude: f64,
        has_range: i32,
        start_seconds: f64,
        end_seconds: f64,
        cb: AsyncCb,
        ctx: *mut c_void,
    );

    /// `WeatherService.weather(for: including: .minute)` async → `WKBox<Forecast<MinuteWeather>?>`
    pub fn wk_weather_service_minute_forecast_async(
        handle: *mut c_void,
        latitude: f64,
        longitude: f64,
        cb: AsyncCb,
        ctx: *mut c_void,
    );

    /// `WeatherService.weather(for: including: .alerts)` async → `WKBox<[WeatherAlert]>`
    pub fn wk_weather_service_weather_alerts_async(
        handle: *mut c_void,
        latitude: f64,
        longitude: f64,
        cb: AsyncCb,
        ctx: *mut c_void,
    );

    /// `WeatherService.weather(for: including: .availability)` async → `WKBox<WeatherAvailability>`
    pub fn wk_weather_service_availability_async(
        handle: *mut c_void,
        latitude: f64,
        longitude: f64,
        cb: AsyncCb,
        ctx: *mut c_void,
    );

    /// `WeatherService.attribution` async → `WKBox<WeatherAttribution>`
    pub fn wk_weather_service_attribution_async(
        handle: *mut c_void,
        cb: AsyncCb,
        ctx: *mut c_void,
    );

    /// `WeatherService.weather(for: including: .changes)` async → `WKBox<String>` (JSON)
    ///
    /// Uses `wk_json_handle_copy_json` / `wk_json_handle_release`. macOS 15.0+ only.
    pub fn wk_weather_service_weather_changes_async(
        handle: *mut c_void,
        latitude: f64,
        longitude: f64,
        cb: AsyncCb,
        ctx: *mut c_void,
    );

    /// `WeatherService.weather(for: including: .historicalComparisons)` async → `WKBox<String>` (JSON)
    ///
    /// Uses `wk_json_handle_copy_json` / `wk_json_handle_release`. macOS 15.0+ only.
    pub fn wk_weather_service_historical_comparisons_async(
        handle: *mut c_void,
        latitude: f64,
        longitude: f64,
        cb: AsyncCb,
        ctx: *mut c_void,
    );
}
