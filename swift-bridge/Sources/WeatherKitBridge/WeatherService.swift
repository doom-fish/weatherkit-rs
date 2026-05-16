import CoreLocation
import Foundation
import WeatherKit

struct WKWeatherPayload: Codable {
    let currentWeather: WKCurrentWeatherPayload
    let hourlyForecast: [WKHourForecastPayload]
    let dailyForecast: [WKDayForecastPayload]
    let minuteForecast: [WKMinuteForecastPayload]?
    let weatherAlerts: [WKWeatherAlertPayload]
    let availability: WKWeatherAvailabilityPayload
}

func wkPayload(from weather: Weather) -> WKWeatherPayload {
    WKWeatherPayload(
        currentWeather: wkPayload(from: weather.currentWeather),
        hourlyForecast: weather.hourlyForecast.forecast.map { wkPayload(from: $0) },
        dailyForecast: weather.dailyForecast.forecast.map { wkPayload(from: $0) },
        minuteForecast: weather.minuteForecast?.forecast.map { wkPayload(from: $0) },
        weatherAlerts: (weather.weatherAlerts ?? []).map { wkPayload(from: $0) },
        availability: wkPayload(from: weather.availability)
    )
}

func wkFetch<T>(
    serviceHandle: UnsafeMutableRawPointer?,
    outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    label: String,
    work: @escaping (WeatherService) async throws -> T
) -> Int32 {
    guard let outHandle else {
        wkWriteError(
            wkBridgeError(code: Int(WK_STATUS_INVALID_ARGUMENT), message: "out_handle must not be nil"),
            into: outError
        )
        return WK_STATUS_INVALID_ARGUMENT
    }

    outHandle.pointee = nil
    outError?.pointee = nil

    guard let serviceHandle,
          let service: WeatherService = wkUnretainedObject(serviceHandle)
    else {
        wkWriteError(
            wkBridgeError(code: Int(WK_STATUS_INVALID_ARGUMENT), message: "service handle must not be nil"),
            into: outError
        )
        return WK_STATUS_INVALID_ARGUMENT
    }

    do {
        let value = try wkAwait(label: label) {
            try await work(service)
        }
        outHandle.pointee = wkRetainBox(value)
        return WK_STATUS_OK
    } catch {
        wkWriteError(error, into: outError)
        return wkStatus(from: error)
    }
}

func wkFetchLocation<T>(
    serviceHandle: UnsafeMutableRawPointer?,
    latitude: Double,
    longitude: Double,
    outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    label: String,
    work: @escaping (WeatherService, CLLocation) async throws -> T
) -> Int32 {
    let location = CLLocation(latitude: latitude, longitude: longitude)
    return wkFetch(
        serviceHandle: serviceHandle,
        outHandle: outHandle,
        outError: outError,
        label: label
    ) { service in
        try await work(service, location)
    }
}

@_cdecl("wk_weather_service_shared")
public func wk_weather_service_shared() -> UnsafeMutableRawPointer? {
    wkRetainObject(WeatherService.shared)
}

@_cdecl("wk_weather_service_new")
public func wk_weather_service_new() -> UnsafeMutableRawPointer? {
    wkRetainObject(WeatherService())
}

@_cdecl("wk_weather_service_retain")
public func wk_weather_service_retain(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    wkRetainOpaque(handle)
}

@_cdecl("wk_weather_service_release")
public func wk_weather_service_release(_ handle: UnsafeMutableRawPointer?) {
    wkRelease(handle)
}

@_cdecl("wk_weather_service_weather")
public func wk_weather_service_weather(
    _ serviceHandle: UnsafeMutableRawPointer?,
    _ latitude: Double,
    _ longitude: Double,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    wkFetchLocation(
        serviceHandle: serviceHandle,
        latitude: latitude,
        longitude: longitude,
        outHandle: outHandle,
        outError: outError,
        label: "WeatherService.weather(for:)"
    ) { service, location in
        try await service.weather(for: location)
    }
}

@_cdecl("wk_weather_service_current_weather")
public func wk_weather_service_current_weather(
    _ serviceHandle: UnsafeMutableRawPointer?,
    _ latitude: Double,
    _ longitude: Double,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    wkFetchLocation(
        serviceHandle: serviceHandle,
        latitude: latitude,
        longitude: longitude,
        outHandle: outHandle,
        outError: outError,
        label: "WeatherService.weather(for: including: .current)"
    ) { service, location in
        try await service.weather(for: location, including: WeatherQuery<CurrentWeather>.current)
    }
}

@_cdecl("wk_weather_service_hourly_forecast")
public func wk_weather_service_hourly_forecast(
    _ serviceHandle: UnsafeMutableRawPointer?,
    _ latitude: Double,
    _ longitude: Double,
    _ hasRange: Int32,
    _ startSeconds: Double,
    _ endSeconds: Double,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    wkFetchLocation(
        serviceHandle: serviceHandle,
        latitude: latitude,
        longitude: longitude,
        outHandle: outHandle,
        outError: outError,
        label: "WeatherService.weather(for: including: .hourly)"
    ) { service, location in
        if hasRange != 0 {
            let query = WeatherQuery<Forecast<HourWeather>>.hourly(
                startDate: wkDate(fromUnixSeconds: startSeconds),
                endDate: wkDate(fromUnixSeconds: endSeconds)
            )
            return try await service.weather(for: location, including: query)
        }
        return try await service.weather(for: location, including: WeatherQuery<Forecast<HourWeather>>.hourly)
    }
}

@_cdecl("wk_weather_service_daily_forecast")
public func wk_weather_service_daily_forecast(
    _ serviceHandle: UnsafeMutableRawPointer?,
    _ latitude: Double,
    _ longitude: Double,
    _ hasRange: Int32,
    _ startSeconds: Double,
    _ endSeconds: Double,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    wkFetchLocation(
        serviceHandle: serviceHandle,
        latitude: latitude,
        longitude: longitude,
        outHandle: outHandle,
        outError: outError,
        label: "WeatherService.weather(for: including: .daily)"
    ) { service, location in
        if hasRange != 0 {
            let query = WeatherQuery<Forecast<DayWeather>>.daily(
                startDate: wkDate(fromUnixSeconds: startSeconds),
                endDate: wkDate(fromUnixSeconds: endSeconds)
            )
            return try await service.weather(for: location, including: query)
        }
        return try await service.weather(for: location, including: WeatherQuery<Forecast<DayWeather>>.daily)
    }
}

@_cdecl("wk_weather_service_minute_forecast")
public func wk_weather_service_minute_forecast(
    _ serviceHandle: UnsafeMutableRawPointer?,
    _ latitude: Double,
    _ longitude: Double,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    wkFetchLocation(
        serviceHandle: serviceHandle,
        latitude: latitude,
        longitude: longitude,
        outHandle: outHandle,
        outError: outError,
        label: "WeatherService.weather(for: including: .minute)"
    ) { service, location in
        try await service.weather(for: location, including: WeatherQuery<Forecast<MinuteWeather>?>.minute)
    }
}

@_cdecl("wk_weather_service_weather_alerts")
public func wk_weather_service_weather_alerts(
    _ serviceHandle: UnsafeMutableRawPointer?,
    _ latitude: Double,
    _ longitude: Double,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    wkFetchLocation(
        serviceHandle: serviceHandle,
        latitude: latitude,
        longitude: longitude,
        outHandle: outHandle,
        outError: outError,
        label: "WeatherService.weather(for: including: .alerts)"
    ) { service, location in
        (try await service.weather(for: location, including: WeatherQuery<[WeatherAlert]?>.alerts)) ?? []
    }
}

@_cdecl("wk_weather_service_availability")
public func wk_weather_service_availability(
    _ serviceHandle: UnsafeMutableRawPointer?,
    _ latitude: Double,
    _ longitude: Double,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    wkFetchLocation(
        serviceHandle: serviceHandle,
        latitude: latitude,
        longitude: longitude,
        outHandle: outHandle,
        outError: outError,
        label: "WeatherService.weather(for: including: .availability)"
    ) { service, location in
        try await service.weather(for: location, including: WeatherQuery<WeatherAvailability>.availability)
    }
}

@_cdecl("wk_weather_service_attribution")
public func wk_weather_service_attribution(
    _ serviceHandle: UnsafeMutableRawPointer?,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    wkFetch(
        serviceHandle: serviceHandle,
        outHandle: outHandle,
        outError: outError,
        label: "WeatherService.attribution"
    ) { service in
        try await service.attribution
    }
}

@_cdecl("wk_weather_retain")
public func wk_weather_retain(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    wkRetainOpaque(handle)
}

@_cdecl("wk_weather_release")
public func wk_weather_release(_ handle: UnsafeMutableRawPointer?) {
    wkRelease(handle)
}

@_cdecl("wk_weather_copy_json")
public func wk_weather_copy_json(
    _ handle: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    wkCopyBoxJSON(handle, outJSON, outError, context: "Weather", payload: { (value: Weather) in wkPayload(from: value) })
}
