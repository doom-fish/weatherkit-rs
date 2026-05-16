use core::ffi::{c_char, c_void};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Deserialize;

use crate::availability_kind::WeatherAvailability;
use crate::changes::{HistoricalComparisons, WeatherChanges};
use crate::current_weather::CurrentWeather;
use crate::daily_forecast::{DailyForecast, DayForecast};
use crate::error::WeatherKitError;
use crate::ffi;
use crate::hourly_forecast::HourlyForecast;
use crate::minute_forecast::MinuteForecastCollection;
use crate::moon_events::MoonEvents;
use crate::pressure::Pressure;
use crate::private::{error_from_status, parse_json_from_handle};
use crate::statistics::{
    DailyWeatherStatistics, DailyWeatherStatisticsQuery, DailyWeatherStatisticsResult,
    DailyWeatherSummary, DailyWeatherSummaryQuery, DailyWeatherSummaryResult,
    DayPrecipitationStatistics, DayPrecipitationSummary, DayTemperatureStatistics,
    DayTemperatureSummary, HourTemperatureStatistics, HourlyWeatherStatistics,
    HourlyWeatherStatisticsQuery, MonthPrecipitationStatistics, MonthTemperatureStatistics,
    MonthlyWeatherStatistics, MonthlyWeatherStatisticsQuery, MonthlyWeatherStatisticsResult,
};
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatherQuery {
    Current,
    Minute,
    Hourly,
    HourlyIn(DateInterval),
    Daily,
    DailyIn(DateInterval),
    Alerts,
    Availability,
    Changes,
    HistoricalComparisons,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WeatherQueryResult {
    CurrentWeather(Box<CurrentWeather>),
    MinuteForecast(Option<Box<MinuteForecastCollection>>),
    HourlyForecast(Box<HourlyForecast>),
    DailyForecast(Box<DailyForecast>),
    WeatherAlerts(Vec<WeatherAlert>),
    Availability(Box<WeatherAvailability>),
    WeatherChanges(Option<Box<WeatherChanges>>),
    HistoricalComparisons(Option<Box<HistoricalComparisons>>),
}

impl WeatherQuery {
    fn fetch(
        &self,
        service: WeatherService,
        location: &CLLocation,
    ) -> Result<WeatherQueryResult, WeatherKitError> {
        match self {
            Self::Current => service
                .current_weather(location)
                .map(Box::new)
                .map(WeatherQueryResult::CurrentWeather),
            Self::Minute => service
                .minute_forecast(location)
                .map(|forecast| forecast.map(Box::new))
                .map(WeatherQueryResult::MinuteForecast),
            Self::Hourly => service
                .hourly_forecast(location)
                .map(Box::new)
                .map(WeatherQueryResult::HourlyForecast),
            Self::HourlyIn(interval) => service
                .hourly_forecast_in(location, interval.clone())
                .map(Box::new)
                .map(WeatherQueryResult::HourlyForecast),
            Self::Daily => service
                .daily_forecast(location)
                .map(Box::new)
                .map(WeatherQueryResult::DailyForecast),
            Self::DailyIn(interval) => service
                .daily_forecast_in(location, interval.clone())
                .map(Box::new)
                .map(WeatherQueryResult::DailyForecast),
            Self::Alerts => service
                .weather_alerts(location)
                .map(WeatherQueryResult::WeatherAlerts),
            Self::Availability => service
                .availability(location)
                .map(Box::new)
                .map(WeatherQueryResult::Availability),
            Self::Changes => service
                .weather_changes(location)
                .map(|changes| changes.map(Box::new))
                .map(WeatherQueryResult::WeatherChanges),
            Self::HistoricalComparisons => service
                .historical_comparisons(location)
                .map(|comparisons| comparisons.map(Box::new))
                .map(WeatherQueryResult::HistoricalComparisons),
        }
    }
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
type ScopedQueryFetchFn = unsafe extern "C" fn(
    *mut c_void,
    f64,
    f64,
    i32,
    i32,
    f64,
    f64,
    i64,
    i64,
    *mut *mut c_void,
    *mut *mut c_char,
) -> i32;
type ServiceFetchFn = unsafe extern "C" fn(*mut c_void, *mut *mut c_void, *mut *mut c_char) -> i32;

#[derive(Debug, Clone, Copy)]
enum QueryScope<'a> {
    None,
    Interval(&'a DateInterval),
    Index { start: i64, end: i64 },
}

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

    pub fn weather_including(
        &self,
        location: &CLLocation,
        query: WeatherQuery,
    ) -> Result<WeatherQueryResult, WeatherKitError> {
        query.fetch(*self, location)
    }

    pub fn weather_including2(
        &self,
        location: &CLLocation,
        query1: WeatherQuery,
        query2: WeatherQuery,
    ) -> Result<(WeatherQueryResult, WeatherQueryResult), WeatherKitError> {
        Ok((query1.fetch(*self, location)?, query2.fetch(*self, location)?))
    }

    pub fn weather_including3(
        &self,
        location: &CLLocation,
        query1: WeatherQuery,
        query2: WeatherQuery,
        query3: WeatherQuery,
    ) -> Result<(WeatherQueryResult, WeatherQueryResult, WeatherQueryResult), WeatherKitError> {
        Ok((
            query1.fetch(*self, location)?,
            query2.fetch(*self, location)?,
            query3.fetch(*self, location)?,
        ))
    }

    pub fn weather_including4(
        &self,
        location: &CLLocation,
        query1: WeatherQuery,
        query2: WeatherQuery,
        query3: WeatherQuery,
        query4: WeatherQuery,
    ) -> Result<
        (
            WeatherQueryResult,
            WeatherQueryResult,
            WeatherQueryResult,
            WeatherQueryResult,
        ),
        WeatherKitError,
    > {
        Ok((
            query1.fetch(*self, location)?,
            query2.fetch(*self, location)?,
            query3.fetch(*self, location)?,
            query4.fetch(*self, location)?,
        ))
    }

    pub fn weather_including5(
        &self,
        location: &CLLocation,
        query1: WeatherQuery,
        query2: WeatherQuery,
        query3: WeatherQuery,
        query4: WeatherQuery,
        query5: WeatherQuery,
    ) -> Result<
        (
            WeatherQueryResult,
            WeatherQueryResult,
            WeatherQueryResult,
            WeatherQueryResult,
            WeatherQueryResult,
        ),
        WeatherKitError,
    > {
        Ok((
            query1.fetch(*self, location)?,
            query2.fetch(*self, location)?,
            query3.fetch(*self, location)?,
            query4.fetch(*self, location)?,
            query5.fetch(*self, location)?,
        ))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn weather_including6(
        &self,
        location: &CLLocation,
        query1: WeatherQuery,
        query2: WeatherQuery,
        query3: WeatherQuery,
        query4: WeatherQuery,
        query5: WeatherQuery,
        query6: WeatherQuery,
    ) -> Result<
        (
            WeatherQueryResult,
            WeatherQueryResult,
            WeatherQueryResult,
            WeatherQueryResult,
            WeatherQueryResult,
            WeatherQueryResult,
        ),
        WeatherKitError,
    > {
        Ok((
            query1.fetch(*self, location)?,
            query2.fetch(*self, location)?,
            query3.fetch(*self, location)?,
            query4.fetch(*self, location)?,
            query5.fetch(*self, location)?,
            query6.fetch(*self, location)?,
        ))
    }

    pub fn weather_including_many<I>(
        &self,
        location: &CLLocation,
        queries: I,
    ) -> Result<Vec<WeatherQueryResult>, WeatherKitError>
    where
        I: IntoIterator<Item = WeatherQuery>,
    {
        queries
            .into_iter()
            .map(|query| query.fetch(*self, location))
            .collect()
    }

    pub fn weather_changes(
        &self,
        location: &CLLocation,
    ) -> Result<Option<WeatherChanges>, WeatherKitError> {
        let ptr = self.fetch_location_handle(
            location,
            ffi::changes::wk_weather_service_weather_changes,
            "WeatherService.weather(for: including: .changes)",
        )?;
        WeatherChanges::option_from_owned_ptr(ptr)
    }

    pub fn historical_comparisons(
        &self,
        location: &CLLocation,
    ) -> Result<Option<HistoricalComparisons>, WeatherKitError> {
        let ptr = self.fetch_location_handle(
            location,
            ffi::changes::wk_weather_service_historical_comparisons,
            "WeatherService.weather(for: including: .historicalComparisons)",
        )?;
        HistoricalComparisons::option_from_owned_ptr(ptr)
    }

    pub fn daily_statistics(
        &self,
        location: &CLLocation,
        query: DailyWeatherStatisticsQuery,
    ) -> Result<DailyWeatherStatisticsResult, WeatherKitError> {
        self.daily_statistics_with_scope(location, query, QueryScope::None)
    }

    pub fn daily_statistics_in(
        &self,
        location: &CLLocation,
        interval: DateInterval,
        query: DailyWeatherStatisticsQuery,
    ) -> Result<DailyWeatherStatisticsResult, WeatherKitError> {
        self.daily_statistics_with_scope(location, query, QueryScope::Interval(&interval))
    }

    pub fn daily_statistics_between_days(
        &self,
        location: &CLLocation,
        start_day: i64,
        end_day: i64,
        query: DailyWeatherStatisticsQuery,
    ) -> Result<DailyWeatherStatisticsResult, WeatherKitError> {
        self.daily_statistics_with_scope(
            location,
            query,
            QueryScope::Index {
                start: start_day,
                end: end_day,
            },
        )
    }

    pub fn daily_summary(
        &self,
        location: &CLLocation,
        query: DailyWeatherSummaryQuery,
    ) -> Result<DailyWeatherSummaryResult, WeatherKitError> {
        self.daily_summary_with_scope(location, query, QueryScope::None)
    }

    pub fn daily_summary_in(
        &self,
        location: &CLLocation,
        interval: DateInterval,
        query: DailyWeatherSummaryQuery,
    ) -> Result<DailyWeatherSummaryResult, WeatherKitError> {
        self.daily_summary_with_scope(location, query, QueryScope::Interval(&interval))
    }

    pub fn hourly_statistics(
        &self,
        location: &CLLocation,
        query: HourlyWeatherStatisticsQuery,
    ) -> Result<HourlyWeatherStatistics<HourTemperatureStatistics>, WeatherKitError> {
        self.hourly_statistics_with_scope(location, query, QueryScope::None)
    }

    pub fn hourly_statistics_in(
        &self,
        location: &CLLocation,
        interval: DateInterval,
        query: HourlyWeatherStatisticsQuery,
    ) -> Result<HourlyWeatherStatistics<HourTemperatureStatistics>, WeatherKitError> {
        self.hourly_statistics_with_scope(location, query, QueryScope::Interval(&interval))
    }

    pub fn hourly_statistics_between_hours(
        &self,
        location: &CLLocation,
        start_hour: i64,
        end_hour: i64,
        query: HourlyWeatherStatisticsQuery,
    ) -> Result<HourlyWeatherStatistics<HourTemperatureStatistics>, WeatherKitError> {
        self.hourly_statistics_with_scope(
            location,
            query,
            QueryScope::Index {
                start: start_hour,
                end: end_hour,
            },
        )
    }

    pub fn monthly_statistics(
        &self,
        location: &CLLocation,
        query: MonthlyWeatherStatisticsQuery,
    ) -> Result<MonthlyWeatherStatisticsResult, WeatherKitError> {
        self.monthly_statistics_with_scope(location, query, QueryScope::None)
    }

    pub fn monthly_statistics_in(
        &self,
        location: &CLLocation,
        interval: DateInterval,
        query: MonthlyWeatherStatisticsQuery,
    ) -> Result<MonthlyWeatherStatisticsResult, WeatherKitError> {
        self.monthly_statistics_with_scope(location, query, QueryScope::Interval(&interval))
    }

    pub fn monthly_statistics_between_months(
        &self,
        location: &CLLocation,
        start_month: i64,
        end_month: i64,
        query: MonthlyWeatherStatisticsQuery,
    ) -> Result<MonthlyWeatherStatisticsResult, WeatherKitError> {
        self.monthly_statistics_with_scope(
            location,
            query,
            QueryScope::Index {
                start: start_month,
                end: end_month,
            },
        )
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

    fn daily_statistics_with_scope(
        &self,
        location: &CLLocation,
        query: DailyWeatherStatisticsQuery,
        scope: QueryScope<'_>,
    ) -> Result<DailyWeatherStatisticsResult, WeatherKitError> {
        let ptr = self.fetch_scoped_query_handle(
            location,
            query.query_kind(),
            scope,
            ffi::statistics::wk_weather_service_daily_statistics,
            "WeatherService.dailyStatistics",
        )?;
        match query {
            DailyWeatherStatisticsQuery::Temperature => Ok(DailyWeatherStatisticsResult::Temperature(
                DailyWeatherStatistics::<DayTemperatureStatistics>::from_owned_ptr(ptr)?,
            )),
            DailyWeatherStatisticsQuery::Precipitation => Ok(
                DailyWeatherStatisticsResult::Precipitation(
                    DailyWeatherStatistics::<DayPrecipitationStatistics>::from_owned_ptr(ptr)?,
                ),
            ),
        }
    }

    fn daily_summary_with_scope(
        &self,
        location: &CLLocation,
        query: DailyWeatherSummaryQuery,
        scope: QueryScope<'_>,
    ) -> Result<DailyWeatherSummaryResult, WeatherKitError> {
        let ptr = self.fetch_scoped_query_handle(
            location,
            query.query_kind(),
            scope,
            ffi::statistics::wk_weather_service_daily_summary,
            "WeatherService.dailySummary",
        )?;
        match query {
            DailyWeatherSummaryQuery::Temperature => Ok(DailyWeatherSummaryResult::Temperature(
                DailyWeatherSummary::<DayTemperatureSummary>::from_owned_ptr(ptr)?,
            )),
            DailyWeatherSummaryQuery::Precipitation => Ok(
                DailyWeatherSummaryResult::Precipitation(
                    DailyWeatherSummary::<DayPrecipitationSummary>::from_owned_ptr(ptr)?,
                ),
            ),
        }
    }

    fn hourly_statistics_with_scope(
        &self,
        location: &CLLocation,
        query: HourlyWeatherStatisticsQuery,
        scope: QueryScope<'_>,
    ) -> Result<HourlyWeatherStatistics<HourTemperatureStatistics>, WeatherKitError> {
        let ptr = self.fetch_scoped_query_handle(
            location,
            query.query_kind(),
            scope,
            ffi::statistics::wk_weather_service_hourly_statistics,
            "WeatherService.hourlyStatistics",
        )?;
        HourlyWeatherStatistics::<HourTemperatureStatistics>::from_owned_ptr(ptr)
    }

    fn monthly_statistics_with_scope(
        &self,
        location: &CLLocation,
        query: MonthlyWeatherStatisticsQuery,
        scope: QueryScope<'_>,
    ) -> Result<MonthlyWeatherStatisticsResult, WeatherKitError> {
        let ptr = self.fetch_scoped_query_handle(
            location,
            query.query_kind(),
            scope,
            ffi::statistics::wk_weather_service_monthly_statistics,
            "WeatherService.monthlyStatistics",
        )?;
        match query {
            MonthlyWeatherStatisticsQuery::Temperature => Ok(
                MonthlyWeatherStatisticsResult::Temperature(
                    MonthlyWeatherStatistics::<MonthTemperatureStatistics>::from_owned_ptr(ptr)?,
                ),
            ),
            MonthlyWeatherStatisticsQuery::Precipitation => Ok(
                MonthlyWeatherStatisticsResult::Precipitation(
                    MonthlyWeatherStatistics::<MonthPrecipitationStatistics>::from_owned_ptr(ptr)?,
                ),
            ),
        }
    }

    fn fetch_scoped_query_handle(
        &self,
        location: &CLLocation,
        query_kind: i32,
        scope: QueryScope<'_>,
        call: ScopedQueryFetchFn,
        context: &str,
    ) -> Result<*mut c_void, WeatherKitError> {
        location.validate()?;
        let service = ServiceHandle::acquire(self.kind)?;
        let mut out_handle = core::ptr::null_mut();
        let mut out_error = core::ptr::null_mut();
        let (scope_kind, start_seconds, end_seconds, start_index, end_index) = match scope {
            QueryScope::None => (0, 0.0, 0.0, 0, 0),
            QueryScope::Interval(interval) => (1, interval.start_seconds()?, interval.end_seconds()?, 0, 0),
            QueryScope::Index { start, end } => {
                validate_index_range(start, end, context)?;
                (2, 0.0, 0.0, start, end)
            }
        };
        let status = unsafe {
            call(
                service.as_ptr(),
                location.latitude,
                location.longitude,
                query_kind,
                scope_kind,
                start_seconds,
                end_seconds,
                start_index,
                end_index,
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

fn validate_index_range(start: i64, end: i64, context: &str) -> Result<(), WeatherKitError> {
    if start > end {
        return Err(WeatherKitError::bridge(
            -1,
            format!("{context} start index must not be after end index"),
        ));
    }
    Ok(())
}

fn unix_seconds(time: SystemTime) -> Result<f64, WeatherKitError> {
    let duration = time.duration_since(UNIX_EPOCH).map_err(|error| {
        WeatherKitError::bridge(-1, format!("time {time:?} is before UNIX_EPOCH: {error}"))
    })?;
    Ok(duration.as_secs_f64())
}
