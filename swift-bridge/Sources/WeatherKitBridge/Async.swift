// Async.swift – non-blocking thunks for every WeatherKit `async throws` API.
//
// Each function:
//  1. Extracts the WeatherService from the opaque handle (unretained borrow; the
//     Task closure below captures it strongly, so it stays alive).
//  2. Launches a Swift Task that calls the real `async throws` method.
//  3. On success – fires `cb(retained_result_ptr, nil, ctx)`.
//  4. On error  – fires `cb(nil, cstr_error, ctx)`.
//
// The Rust side allocates an `AsyncCompletion` context, passes it as `ctx`, and
// polls the resulting future.  The retained result pointer must be released by
// Rust via the matching `_release` function once copy-json has been called.

import CoreLocation
import Foundation
import WeatherKit

// MARK: - Error helper

/// Encodes a Swift `Error` as the same JSON payload used by the sync bridge
/// (`wkWriteError`), so the Rust side can parse domain/code/message correctly
/// and `is_entitlement_issue()` works for async errors too.
private func wkAsyncErrorCString(_ error: Error) -> String {
    let nsErr = error as NSError
    let msg = nsErr.localizedDescription
        .replacingOccurrences(of: "\\", with: "\\\\")
        .replacingOccurrences(of: "\"", with: "\\\"")
    return "{\"domain\":\"\(nsErr.domain)\",\"code\":\(nsErr.code),\"message\":\"\(msg)\"}"
}

/// Fire the callback with a JSON-encoded error string.
private func wkAsyncFail(
    _ error: Error,
    cb: @convention(c) (UnsafeRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer?) -> Void,
    ctx: UnsafeMutableRawPointer?
) {
    wkAsyncErrorCString(error).withCString { cb(nil, $0, ctx) }
}

/// Fire the callback with a plain bridge-error JSON string (no NSError available).
private func wkAsyncBridgeFail(
    code: Int32 = -1,
    message: String,
    cb: @convention(c) (UnsafeRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer?) -> Void,
    ctx: UnsafeMutableRawPointer?
) {
    let escaped = message
        .replacingOccurrences(of: "\\", with: "\\\\")
        .replacingOccurrences(of: "\"", with: "\\\"")
    let json = "{\"domain\":\"WeatherKitBridge\",\"code\":\(code),\"message\":\"\(escaped)\"}"
    json.withCString { cb(nil, $0, ctx) }
}

// MARK: - weather(for:)

@_cdecl("wk_weather_service_weather_async")
public func wk_weather_service_weather_async(
    _ serviceHandle: UnsafeMutableRawPointer?,
    _ latitude: Double,
    _ longitude: Double,
    _ cb: @convention(c) (UnsafeRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer?) -> Void,
    _ ctx: UnsafeMutableRawPointer?
) {
    guard let serviceHandle,
          let service: WeatherService = wkUnretainedObject(serviceHandle)
    else {
        wkAsyncBridgeFail(message: "invalid service handle", cb: cb, ctx: ctx)
        return
    }
    let location = CLLocation(latitude: latitude, longitude: longitude)
    let ctxValue = ctx
    Task {
        do {
            let value = try await service.weather(for: location)
            cb(wkRetainBox(value), nil, ctxValue)
        } catch {
            wkAsyncFail(error, cb: cb, ctx: ctxValue)
        }
    }
}

// MARK: - weather(for: including: .current)

@_cdecl("wk_weather_service_current_weather_async")
public func wk_weather_service_current_weather_async(
    _ serviceHandle: UnsafeMutableRawPointer?,
    _ latitude: Double,
    _ longitude: Double,
    _ cb: @convention(c) (UnsafeRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer?) -> Void,
    _ ctx: UnsafeMutableRawPointer?
) {
    guard let serviceHandle,
          let service: WeatherService = wkUnretainedObject(serviceHandle)
    else {
        wkAsyncBridgeFail(message: "invalid service handle", cb: cb, ctx: ctx)
        return
    }
    let location = CLLocation(latitude: latitude, longitude: longitude)
    let ctxValue = ctx
    Task {
        do {
            let value = try await service.weather(
                for: location,
                including: WeatherQuery<CurrentWeather>.current
            )
            cb(wkRetainBox(value), nil, ctxValue)
        } catch {
            wkAsyncFail(error, cb: cb, ctx: ctxValue)
        }
    }
}

// MARK: - weather(for: including: .hourly) — with optional date range

@_cdecl("wk_weather_service_hourly_forecast_async")
public func wk_weather_service_hourly_forecast_async(
    _ serviceHandle: UnsafeMutableRawPointer?,
    _ latitude: Double,
    _ longitude: Double,
    _ hasRange: Int32,
    _ startSeconds: Double,
    _ endSeconds: Double,
    _ cb: @convention(c) (UnsafeRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer?) -> Void,
    _ ctx: UnsafeMutableRawPointer?
) {
    guard let serviceHandle,
          let service: WeatherService = wkUnretainedObject(serviceHandle)
    else {
        wkAsyncBridgeFail(message: "invalid service handle", cb: cb, ctx: ctx)
        return
    }
    let location = CLLocation(latitude: latitude, longitude: longitude)
    let useRange = hasRange != 0
    let start = wkDate(fromUnixSeconds: startSeconds)
    let end = wkDate(fromUnixSeconds: endSeconds)
    let ctxValue = ctx
    Task {
        do {
            let value: Forecast<HourWeather>
            if useRange {
                let query = WeatherQuery<Forecast<HourWeather>>.hourly(
                    startDate: start,
                    endDate: end
                )
                value = try await service.weather(for: location, including: query)
            } else {
                value = try await service.weather(
                    for: location,
                    including: WeatherQuery<Forecast<HourWeather>>.hourly
                )
            }
            cb(wkRetainBox(value), nil, ctxValue)
        } catch {
            wkAsyncFail(error, cb: cb, ctx: ctxValue)
        }
    }
}

// MARK: - weather(for: including: .daily) — with optional date range

@_cdecl("wk_weather_service_daily_forecast_async")
public func wk_weather_service_daily_forecast_async(
    _ serviceHandle: UnsafeMutableRawPointer?,
    _ latitude: Double,
    _ longitude: Double,
    _ hasRange: Int32,
    _ startSeconds: Double,
    _ endSeconds: Double,
    _ cb: @convention(c) (UnsafeRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer?) -> Void,
    _ ctx: UnsafeMutableRawPointer?
) {
    guard let serviceHandle,
          let service: WeatherService = wkUnretainedObject(serviceHandle)
    else {
        wkAsyncBridgeFail(message: "invalid service handle", cb: cb, ctx: ctx)
        return
    }
    let location = CLLocation(latitude: latitude, longitude: longitude)
    let useRange = hasRange != 0
    let start = wkDate(fromUnixSeconds: startSeconds)
    let end = wkDate(fromUnixSeconds: endSeconds)
    let ctxValue = ctx
    Task {
        do {
            let value: Forecast<DayWeather>
            if useRange {
                let query = WeatherQuery<Forecast<DayWeather>>.daily(
                    startDate: start,
                    endDate: end
                )
                value = try await service.weather(for: location, including: query)
            } else {
                value = try await service.weather(
                    for: location,
                    including: WeatherQuery<Forecast<DayWeather>>.daily
                )
            }
            cb(wkRetainBox(value), nil, ctxValue)
        } catch {
            wkAsyncFail(error, cb: cb, ctx: ctxValue)
        }
    }
}

// MARK: - weather(for: including: .minute)

@_cdecl("wk_weather_service_minute_forecast_async")
public func wk_weather_service_minute_forecast_async(
    _ serviceHandle: UnsafeMutableRawPointer?,
    _ latitude: Double,
    _ longitude: Double,
    _ cb: @convention(c) (UnsafeRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer?) -> Void,
    _ ctx: UnsafeMutableRawPointer?
) {
    guard let serviceHandle,
          let service: WeatherService = wkUnretainedObject(serviceHandle)
    else {
        wkAsyncBridgeFail(message: "invalid service handle", cb: cb, ctx: ctx)
        return
    }
    let location = CLLocation(latitude: latitude, longitude: longitude)
    let ctxValue = ctx
    Task {
        do {
            let value = try await service.weather(
                for: location,
                including: WeatherQuery<Forecast<MinuteWeather>?>.minute
            )
            cb(wkRetainBox(value), nil, ctxValue)
        } catch {
            wkAsyncFail(error, cb: cb, ctx: ctxValue)
        }
    }
}

// MARK: - weather(for: including: .alerts)

@_cdecl("wk_weather_service_weather_alerts_async")
public func wk_weather_service_weather_alerts_async(
    _ serviceHandle: UnsafeMutableRawPointer?,
    _ latitude: Double,
    _ longitude: Double,
    _ cb: @convention(c) (UnsafeRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer?) -> Void,
    _ ctx: UnsafeMutableRawPointer?
) {
    guard let serviceHandle,
          let service: WeatherService = wkUnretainedObject(serviceHandle)
    else {
        wkAsyncBridgeFail(message: "invalid service handle", cb: cb, ctx: ctx)
        return
    }
    let location = CLLocation(latitude: latitude, longitude: longitude)
    let ctxValue = ctx
    Task {
        do {
            let value = (try await service.weather(
                for: location,
                including: WeatherQuery<[WeatherAlert]?>.alerts
            )) ?? []
            cb(wkRetainBox(value), nil, ctxValue)
        } catch {
            wkAsyncFail(error, cb: cb, ctx: ctxValue)
        }
    }
}

// MARK: - weather(for: including: .availability)

@_cdecl("wk_weather_service_availability_async")
public func wk_weather_service_availability_async(
    _ serviceHandle: UnsafeMutableRawPointer?,
    _ latitude: Double,
    _ longitude: Double,
    _ cb: @convention(c) (UnsafeRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer?) -> Void,
    _ ctx: UnsafeMutableRawPointer?
) {
    guard let serviceHandle,
          let service: WeatherService = wkUnretainedObject(serviceHandle)
    else {
        wkAsyncBridgeFail(message: "invalid service handle", cb: cb, ctx: ctx)
        return
    }
    let location = CLLocation(latitude: latitude, longitude: longitude)
    let ctxValue = ctx
    Task {
        do {
            let value = try await service.weather(
                for: location,
                including: WeatherQuery<WeatherAvailability>.availability
            )
            cb(wkRetainBox(value), nil, ctxValue)
        } catch {
            wkAsyncFail(error, cb: cb, ctx: ctxValue)
        }
    }
}

// MARK: - attribution (no location)

@_cdecl("wk_weather_service_attribution_async")
public func wk_weather_service_attribution_async(
    _ serviceHandle: UnsafeMutableRawPointer?,
    _ cb: @convention(c) (UnsafeRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer?) -> Void,
    _ ctx: UnsafeMutableRawPointer?
) {
    guard let serviceHandle,
          let service: WeatherService = wkUnretainedObject(serviceHandle)
    else {
        wkAsyncBridgeFail(message: "invalid service handle", cb: cb, ctx: ctx)
        return
    }
    let ctxValue = ctx
    Task {
        do {
            let value = try await service.attribution
            cb(wkRetainBox(value), nil, ctxValue)
        } catch {
            wkAsyncFail(error, cb: cb, ctx: ctxValue)
        }
    }
}

// MARK: - weather(for: including: .changes)
//
// Returns a WKBox<String> JSON handle (same as the sync thunk) so the Rust side
// can call wk_json_handle_copy_json + wk_json_handle_release.

@_cdecl("wk_weather_service_weather_changes_async")
public func wk_weather_service_weather_changes_async(
    _ serviceHandle: UnsafeMutableRawPointer?,
    _ latitude: Double,
    _ longitude: Double,
    _ cb: @convention(c) (UnsafeRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer?) -> Void,
    _ ctx: UnsafeMutableRawPointer?
) {
    guard let serviceHandle,
          let service: WeatherService = wkUnretainedObject(serviceHandle)
    else {
        wkAsyncBridgeFail(message: "invalid service handle", cb: cb, ctx: ctx)
        return
    }
    let location = CLLocation(latitude: latitude, longitude: longitude)
    let ctxValue = ctx
    Task {
        do {
            guard #available(macOS 15.0, *) else {
                wkAsyncBridgeFail(code: -3, message: "weather changes require macOS 15.0 or later", cb: cb, ctx: ctxValue)
                return
            }
            let json = try await wkWeatherChangesJSON(service: service, location: location)
            cb(wkRetainBox(json), nil, ctxValue)
        } catch {
            wkAsyncFail(error, cb: cb, ctx: ctxValue)
        }
    }
}

// MARK: - weather(for: including: .historicalComparisons)

@_cdecl("wk_weather_service_historical_comparisons_async")
public func wk_weather_service_historical_comparisons_async(
    _ serviceHandle: UnsafeMutableRawPointer?,
    _ latitude: Double,
    _ longitude: Double,
    _ cb: @convention(c) (UnsafeRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer?) -> Void,
    _ ctx: UnsafeMutableRawPointer?
) {
    guard let serviceHandle,
          let service: WeatherService = wkUnretainedObject(serviceHandle)
    else {
        wkAsyncBridgeFail(message: "invalid service handle", cb: cb, ctx: ctx)
        return
    }
    let location = CLLocation(latitude: latitude, longitude: longitude)
    let ctxValue = ctx
    Task {
        do {
            guard #available(macOS 15.0, *) else {
                wkAsyncBridgeFail(code: -3, message: "historical comparisons require macOS 15.0 or later", cb: cb, ctx: ctxValue)
                return
            }
            let json = try await wkHistoricalComparisonsJSON(service: service, location: location)
            cb(wkRetainBox(json), nil, ctxValue)
        } catch {
            wkAsyncFail(error, cb: cb, ctx: ctxValue)
        }
    }
}
