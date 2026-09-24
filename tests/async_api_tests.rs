//! Integration tests for `async_api` module.
//!
//! Run with:
//! ```sh
//! cargo test --features async --test async_api_tests
//! ```
//!
//! Tests that call the `WeatherKit` service run only with
//! `WEATHERKIT_LIVE_TESTS=1`, and then pass gracefully without a live
//! entitlement through an `is_entitlement_issue()` check.

mod common;

#[cfg(feature = "async")]
mod tests {
    use weatherkit::async_api::AsyncWeatherService;
    use weatherkit::service::CLLocation;

    use crate::common;

    /// San Jose, CA — a location with full `WeatherKit` data availability.
    const SAN_JOSE: CLLocation = CLLocation::new(37.3382, -121.8863);

    // Helper: return true when we're running without a valid entitlement so
    // the test can exit cleanly instead of failing.
    fn is_auth(e: &weatherkit::error::WeatherKitError) -> bool {
        e.is_entitlement_issue()
    }

    // -----------------------------------------------------------------------
    // Happy-path tests (succeed with entitlement, skip gracefully without)
    // -----------------------------------------------------------------------

    #[test]
    fn async_weather_returns_result() {
        if !common::live_tests_enabled("async_weather_returns_result") {
            return;
        }
        pollster::block_on(async {
            let svc = AsyncWeatherService::shared();
            match svc.weather(&SAN_JOSE).await {
                Ok(w) => {
                    // Basic structural check — temperature is a finite number.
                    assert!(
                        w.current_weather.temperature.is_finite(),
                        "temperature should be finite"
                    );
                }
                Err(e) if is_auth(&e) => {
                    eprintln!("async_weather_returns_result: skipped (auth) — {e}");
                }
                Err(e) => panic!("unexpected error: {e}"),
            }
        });
    }

    #[test]
    fn async_current_weather_returns_result() {
        if !common::live_tests_enabled("async_current_weather_returns_result") {
            return;
        }
        pollster::block_on(async {
            let svc = AsyncWeatherService::shared();
            match svc.current_weather(&SAN_JOSE).await {
                Ok(cw) => assert!(cw.temperature.is_finite()),
                Err(e) if is_auth(&e) => {
                    eprintln!("async_current_weather: skipped (auth) — {e}");
                }
                Err(e) => panic!("{e}"),
            }
        });
    }

    #[test]
    fn async_hourly_forecast_returns_result() {
        if !common::live_tests_enabled("async_hourly_forecast_returns_result") {
            return;
        }
        pollster::block_on(async {
            let svc = AsyncWeatherService::shared();
            match svc.hourly_forecast(&SAN_JOSE).await {
                Ok(hf) => assert!(!hf.forecast.is_empty(), "hourly forecast should be non-empty"),
                Err(e) if is_auth(&e) => {
                    eprintln!("async_hourly_forecast: skipped (auth) — {e}");
                }
                Err(e) => panic!("{e}"),
            }
        });
    }

    #[test]
    fn async_daily_forecast_returns_result() {
        if !common::live_tests_enabled("async_daily_forecast_returns_result") {
            return;
        }
        pollster::block_on(async {
            let svc = AsyncWeatherService::shared();
            match svc.daily_forecast(&SAN_JOSE).await {
                Ok(df) => assert!(!df.forecast.is_empty(), "daily forecast should be non-empty"),
                Err(e) if is_auth(&e) => {
                    eprintln!("async_daily_forecast: skipped (auth) — {e}");
                }
                Err(e) => panic!("{e}"),
            }
        });
    }

    #[test]
    fn async_availability_returns_result() {
        if !common::live_tests_enabled("async_availability_returns_result") {
            return;
        }
        pollster::block_on(async {
            let svc = AsyncWeatherService::shared();
            match svc.availability(&SAN_JOSE).await {
                Ok(_av) => { /* structural check: just ensure it parses */ }
                Err(e) if is_auth(&e) => {
                    eprintln!("async_availability: skipped (auth) — {e}");
                }
                Err(e) => panic!("{e}"),
            }
        });
    }

    #[test]
    fn async_attribution_returns_result() {
        if !common::live_tests_enabled("async_attribution_returns_result") {
            return;
        }
        pollster::block_on(async {
            let svc = AsyncWeatherService::shared();
            match svc.attribution().await {
                Ok(attr) => assert!(!attr.service_name.is_empty()),
                Err(e) if is_auth(&e) => {
                    eprintln!("async_attribution: skipped (auth) — {e}");
                }
                Err(e) => panic!("{e}"),
            }
        });
    }

    #[test]
    fn async_weather_alerts_returns_result() {
        if !common::live_tests_enabled("async_weather_alerts_returns_result") {
            return;
        }
        pollster::block_on(async {
            let svc = AsyncWeatherService::shared();
            match svc.weather_alerts(&SAN_JOSE).await {
                Ok(_alerts) => { /* may be empty — that's fine */ }
                Err(e) if is_auth(&e) => {
                    eprintln!("async_weather_alerts: skipped (auth) — {e}");
                }
                Err(e) => panic!("{e}"),
            }
        });
    }

    #[test]
    fn async_minute_forecast_returns_result() {
        if !common::live_tests_enabled("async_minute_forecast_returns_result") {
            return;
        }
        pollster::block_on(async {
            let svc = AsyncWeatherService::shared();
            match svc.minute_forecast(&SAN_JOSE).await {
                Ok(_mf) => { /* may be None in some regions */ }
                Err(e) if is_auth(&e) => {
                    eprintln!("async_minute_forecast: skipped (auth) — {e}");
                }
                Err(e) => panic!("{e}"),
            }
        });
    }

    #[test]
    fn async_weather_changes_returns_result_or_os_error() {
        if !common::live_tests_enabled("async_weather_changes_returns_result_or_os_error") {
            return;
        }
        pollster::block_on(async {
            let svc = AsyncWeatherService::shared();
            match svc.weather_changes(&SAN_JOSE).await {
                Ok(_wc) => { /* may be None on macOS < 15 */ }
                // OS version restriction is reported as a bridge error
                Err(e) if e.message.contains("macOS 15") => {
                    eprintln!("async_weather_changes: macOS 15 required — skipped");
                }
                Err(e) if is_auth(&e) => {
                    eprintln!("async_weather_changes: skipped (auth) — {e}");
                }
                Err(e) => panic!("{e}"),
            }
        });
    }

    #[test]
    fn async_historical_comparisons_returns_result_or_os_error() {
        if !common::live_tests_enabled("async_historical_comparisons_returns_result_or_os_error") {
            return;
        }
        pollster::block_on(async {
            let svc = AsyncWeatherService::shared();
            match svc.historical_comparisons(&SAN_JOSE).await {
                Ok(_hc) => {}
                Err(e) if e.message.contains("macOS 15") => {
                    eprintln!("async_historical_comparisons: macOS 15 required — skipped");
                }
                Err(e) if is_auth(&e) => {
                    eprintln!("async_historical_comparisons: skipped (auth) — {e}");
                }
                Err(e) => panic!("{e}"),
            }
        });
    }

    // -----------------------------------------------------------------------
    // Error-path tests — invalid coordinates trigger a bridge error
    // -----------------------------------------------------------------------

    #[test]
    fn async_weather_bad_location_returns_err() {
        pollster::block_on(async {
            let svc = AsyncWeatherService::shared();
            let bad = CLLocation::new(999.0, 0.0);
            let error = svc
                .weather(&bad)
                .await
                .expect_err("bad latitude should produce an error");
            assert!(
                error.message.contains("latitude 999 is outside -90..=90"),
                "unexpected error: {error}"
            );
        });
    }

    #[test]
    fn async_hourly_forecast_bad_location_returns_err() {
        pollster::block_on(async {
            let svc = AsyncWeatherService::shared();
            let bad = CLLocation::new(0.0, 999.0);
            let error = svc
                .hourly_forecast(&bad)
                .await
                .expect_err("bad longitude should produce an error");
            assert!(
                error
                    .message
                    .contains("longitude 999 is outside -180..=180"),
                "unexpected error: {error}"
            );
        });
    }

    #[test]
    fn async_queries_reject_non_finite_coordinates_before_the_service() {
        pollster::block_on(async {
            let svc = AsyncWeatherService::shared();
            let bad = CLLocation::new(f64::NAN, 0.0);
            let results = [
                svc.current_weather(&bad).await.err(),
                svc.daily_forecast(&bad).await.err(),
                svc.minute_forecast(&bad).await.err(),
                svc.weather_alerts(&bad).await.err(),
                svc.availability(&bad).await.err(),
                svc.weather_changes(&bad).await.err(),
                svc.historical_comparisons(&bad).await.err(),
            ];
            for error in results {
                let error = error.expect("non-finite coordinates should produce an error");
                assert!(
                    error.message.contains("must be finite numbers"),
                    "unexpected error: {error}"
                );
            }
        });
    }
}
