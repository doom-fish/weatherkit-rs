import CoreLocation
import Foundation
import WeatherKit

struct WKPercentilesPayload: Codable {
    let p10: Double
    let p50: Double
    let p90: Double
}

struct WKDayTemperatureStatisticsPayload: Codable {
    let day: Int
    let averageLowTemperature: Double
    let averageHighTemperature: Double
}

struct WKDayPrecipitationStatisticsPayload: Codable {
    let day: Int
    let averagePrecipitationProbability: Double
    let averagePrecipitationAmount: Double
    let averageSnowfallAmount: Double
}

struct WKHourTemperatureStatisticsPayload: Codable {
    let hour: Int
    let percentiles: WKPercentilesPayload
}

struct WKMonthTemperatureStatisticsPayload: Codable {
    let month: Int
    let averageLowTemperature: Double
    let averageHighTemperature: Double
}

struct WKMonthPrecipitationStatisticsPayload: Codable {
    let month: Int
    let averagePrecipitationProbability: Double
    let averagePrecipitationAmount: Double
    let averageSnowfallAmount: Double
}

struct WKDayTemperatureSummaryPayload: Codable {
    let date: String
    let lowTemperature: Double
    let highTemperature: Double
}

struct WKDayPrecipitationSummaryPayload: Codable {
    let date: String
    let precipitationAmount: Double
    let snowfallAmount: Double
}

struct WKDailyWeatherStatisticsPayload<Element: Codable>: Codable {
    let days: [Element]
    let baselineStartDate: String
    let metadata: WKWeatherMetadataPayload
}

struct WKDailyWeatherSummaryPayload<Element: Codable>: Codable {
    let days: [Element]
    let metadata: WKWeatherMetadataPayload
}

struct WKHourlyWeatherStatisticsPayload<Element: Codable>: Codable {
    let hours: [Element]
    let baselineStartDate: String
    let metadata: WKWeatherMetadataPayload
}

struct WKMonthlyWeatherStatisticsPayload<Element: Codable>: Codable {
    let months: [Element]
    let baselineStartDate: String
    let metadata: WKWeatherMetadataPayload
}

@available(macOS 15.0, *)
func wkPayload(from percentiles: Percentiles<UnitTemperature>) -> WKPercentilesPayload {
    WKPercentilesPayload(
        p10: wkTemperature(percentiles.p10),
        p50: wkTemperature(percentiles.p50),
        p90: wkTemperature(percentiles.p90)
    )
}

@available(macOS 15.0, *)
func wkPayload(from value: DayTemperatureStatistics) -> WKDayTemperatureStatisticsPayload {
    WKDayTemperatureStatisticsPayload(
        day: value.day,
        averageLowTemperature: wkTemperature(value.averageLowTemperature),
        averageHighTemperature: wkTemperature(value.averageHighTemperature)
    )
}

@available(macOS 15.0, *)
func wkPayload(from value: DayPrecipitationStatistics) -> WKDayPrecipitationStatisticsPayload {
    WKDayPrecipitationStatisticsPayload(
        day: value.day,
        averagePrecipitationProbability: value.averagePrecipitationProbability,
        averagePrecipitationAmount: wkLength(value.averagePrecipitationAmount),
        averageSnowfallAmount: wkLength(value.averageSnowfallAmount)
    )
}

@available(macOS 15.0, *)
func wkPayload(from value: HourTemperatureStatistics) -> WKHourTemperatureStatisticsPayload {
    WKHourTemperatureStatisticsPayload(hour: value.hour, percentiles: wkPayload(from: value.percentiles))
}

@available(macOS 15.0, *)
func wkPayload(from value: MonthTemperatureStatistics) -> WKMonthTemperatureStatisticsPayload {
    WKMonthTemperatureStatisticsPayload(
        month: value.month,
        averageLowTemperature: wkTemperature(value.averageLowTemperature),
        averageHighTemperature: wkTemperature(value.averageHighTemperature)
    )
}

@available(macOS 15.0, *)
func wkPayload(from value: MonthPrecipitationStatistics) -> WKMonthPrecipitationStatisticsPayload {
    WKMonthPrecipitationStatisticsPayload(
        month: value.month,
        averagePrecipitationProbability: value.averagePrecipitationProbability,
        averagePrecipitationAmount: wkLength(value.averagePrecipitationAmount),
        averageSnowfallAmount: wkLength(value.averageSnowfallAmount)
    )
}

@available(macOS 15.0, *)
func wkPayload(from value: DayTemperatureSummary) -> WKDayTemperatureSummaryPayload {
    WKDayTemperatureSummaryPayload(
        date: wkISO8601String(value.date),
        lowTemperature: wkTemperature(value.lowTemperature),
        highTemperature: wkTemperature(value.highTemperature)
    )
}

@available(macOS 15.0, *)
func wkPayload(from value: DayPrecipitationSummary) -> WKDayPrecipitationSummaryPayload {
    WKDayPrecipitationSummaryPayload(
        date: wkISO8601String(value.date),
        precipitationAmount: wkLength(value.precipitationAmount),
        snowfallAmount: wkLength(value.snowfallAmount)
    )
}

@available(macOS 15.0, *)
func wkPayload(from value: DailyWeatherStatistics<DayTemperatureStatistics>) -> WKDailyWeatherStatisticsPayload<WKDayTemperatureStatisticsPayload> {
    WKDailyWeatherStatisticsPayload(
        days: value.days.map { wkPayload(from: $0) },
        baselineStartDate: wkISO8601String(value.baselineStartDate),
        metadata: wkPayload(from: value.metadata)
    )
}

@available(macOS 15.0, *)
func wkPayload(from value: DailyWeatherStatistics<DayPrecipitationStatistics>) -> WKDailyWeatherStatisticsPayload<WKDayPrecipitationStatisticsPayload> {
    WKDailyWeatherStatisticsPayload(
        days: value.days.map { wkPayload(from: $0) },
        baselineStartDate: wkISO8601String(value.baselineStartDate),
        metadata: wkPayload(from: value.metadata)
    )
}

@available(macOS 15.0, *)
func wkPayload(from value: DailyWeatherSummary<DayTemperatureSummary>) -> WKDailyWeatherSummaryPayload<WKDayTemperatureSummaryPayload> {
    WKDailyWeatherSummaryPayload(days: value.days.map { wkPayload(from: $0) }, metadata: wkPayload(from: value.metadata))
}

@available(macOS 15.0, *)
func wkPayload(from value: DailyWeatherSummary<DayPrecipitationSummary>) -> WKDailyWeatherSummaryPayload<WKDayPrecipitationSummaryPayload> {
    WKDailyWeatherSummaryPayload(days: value.days.map { wkPayload(from: $0) }, metadata: wkPayload(from: value.metadata))
}

@available(macOS 15.0, *)
func wkPayload(from value: HourlyWeatherStatistics<HourTemperatureStatistics>) -> WKHourlyWeatherStatisticsPayload<WKHourTemperatureStatisticsPayload> {
    WKHourlyWeatherStatisticsPayload(
        hours: value.hours.map { wkPayload(from: $0) },
        baselineStartDate: wkISO8601String(value.baselineStartDate),
        metadata: wkPayload(from: value.metadata)
    )
}

@available(macOS 15.0, *)
func wkPayload(from value: MonthlyWeatherStatistics<MonthTemperatureStatistics>) -> WKMonthlyWeatherStatisticsPayload<WKMonthTemperatureStatisticsPayload> {
    WKMonthlyWeatherStatisticsPayload(
        months: value.months.map { wkPayload(from: $0) },
        baselineStartDate: wkISO8601String(value.baselineStartDate),
        metadata: wkPayload(from: value.metadata)
    )
}

@available(macOS 15.0, *)
func wkPayload(from value: MonthlyWeatherStatistics<MonthPrecipitationStatistics>) -> WKMonthlyWeatherStatisticsPayload<WKMonthPrecipitationStatisticsPayload> {
    WKMonthlyWeatherStatisticsPayload(
        months: value.months.map { wkPayload(from: $0) },
        baselineStartDate: wkISO8601String(value.baselineStartDate),
        metadata: wkPayload(from: value.metadata)
    )
}

func wkStatisticsUnavailable(_ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?) -> Int32 {
    wkWriteError(
        wkBridgeError(code: Int(WK_STATUS_FAILURE), message: "WeatherKit statistics and summaries require macOS 15.0 or newer"),
        into: outError
    )
    return WK_STATUS_FAILURE
}

@available(macOS 15.0, *)
func wkDateInterval(startSeconds: Double, endSeconds: Double) -> Foundation.DateInterval {
    Foundation.DateInterval(start: wkDate(fromUnixSeconds: startSeconds), end: wkDate(fromUnixSeconds: endSeconds))
}

@available(macOS 15.0, *)
func wkDailyStatisticsJSON(
    service: WeatherService,
    location: CLLocation,
    queryKind: Int32,
    scopeKind: Int32,
    startSeconds: Double,
    endSeconds: Double,
    startIndex: Int64,
    endIndex: Int64
) async throws -> String {
    switch queryKind {
    case 0:
        let query = DailyWeatherStatisticsQuery<DayTemperatureStatistics>.temperature
        switch scopeKind {
        case 0:
            return try wkJSONString(wkPayload(from: try await service.dailyStatistics(for: location, including: query)))
        case 1:
            return try wkJSONString(wkPayload(from: try await service.dailyStatistics(for: location, forDaysIn: wkDateInterval(startSeconds: startSeconds, endSeconds: endSeconds), including: query)))
        case 2:
            return try wkJSONString(wkPayload(from: try await service.dailyStatistics(for: location, startDay: Int(startIndex), endDay: Int(endIndex), including: query)))
        default:
            throw wkBridgeError(code: Int(WK_STATUS_INVALID_ARGUMENT), message: "invalid daily statistics scope")
        }
    case 1:
        let query = DailyWeatherStatisticsQuery<DayPrecipitationStatistics>.precipitation
        switch scopeKind {
        case 0:
            return try wkJSONString(wkPayload(from: try await service.dailyStatistics(for: location, including: query)))
        case 1:
            return try wkJSONString(wkPayload(from: try await service.dailyStatistics(for: location, forDaysIn: wkDateInterval(startSeconds: startSeconds, endSeconds: endSeconds), including: query)))
        case 2:
            return try wkJSONString(wkPayload(from: try await service.dailyStatistics(for: location, startDay: Int(startIndex), endDay: Int(endIndex), including: query)))
        default:
            throw wkBridgeError(code: Int(WK_STATUS_INVALID_ARGUMENT), message: "invalid daily statistics scope")
        }
    default:
        throw wkBridgeError(code: Int(WK_STATUS_INVALID_ARGUMENT), message: "invalid daily statistics query")
    }
}

@available(macOS 15.0, *)
func wkDailySummaryJSON(
    service: WeatherService,
    location: CLLocation,
    queryKind: Int32,
    scopeKind: Int32,
    startSeconds: Double,
    endSeconds: Double
) async throws -> String {
    switch queryKind {
    case 0:
        let query = DailyWeatherSummaryQuery<DayTemperatureSummary>.temperature
        switch scopeKind {
        case 0:
            return try wkJSONString(wkPayload(from: try await service.dailySummary(for: location, including: query)))
        case 1:
            return try wkJSONString(wkPayload(from: try await service.dailySummary(for: location, forDaysIn: wkDateInterval(startSeconds: startSeconds, endSeconds: endSeconds), including: query)))
        default:
            throw wkBridgeError(code: Int(WK_STATUS_INVALID_ARGUMENT), message: "invalid daily summary scope")
        }
    case 1:
        let query = DailyWeatherSummaryQuery<DayPrecipitationSummary>.precipitation
        switch scopeKind {
        case 0:
            return try wkJSONString(wkPayload(from: try await service.dailySummary(for: location, including: query)))
        case 1:
            return try wkJSONString(wkPayload(from: try await service.dailySummary(for: location, forDaysIn: wkDateInterval(startSeconds: startSeconds, endSeconds: endSeconds), including: query)))
        default:
            throw wkBridgeError(code: Int(WK_STATUS_INVALID_ARGUMENT), message: "invalid daily summary scope")
        }
    default:
        throw wkBridgeError(code: Int(WK_STATUS_INVALID_ARGUMENT), message: "invalid daily summary query")
    }
}

@available(macOS 15.0, *)
func wkHourlyStatisticsJSON(
    service: WeatherService,
    location: CLLocation,
    scopeKind: Int32,
    startSeconds: Double,
    endSeconds: Double,
    startIndex: Int64,
    endIndex: Int64
) async throws -> String {
    let query = HourlyWeatherStatisticsQuery<HourTemperatureStatistics>.temperature
    switch scopeKind {
    case 0:
        return try wkJSONString(wkPayload(from: try await service.hourlyStatistics(for: location, including: query)))
    case 1:
        return try wkJSONString(wkPayload(from: try await service.hourlyStatistics(for: location, forHoursIn: wkDateInterval(startSeconds: startSeconds, endSeconds: endSeconds), including: query)))
    case 2:
        return try wkJSONString(wkPayload(from: try await service.hourlyStatistics(for: location, startHour: Int(startIndex), endHour: Int(endIndex), including: query)))
    default:
        throw wkBridgeError(code: Int(WK_STATUS_INVALID_ARGUMENT), message: "invalid hourly statistics scope")
    }
}

@available(macOS 15.0, *)
func wkMonthlyStatisticsJSON(
    service: WeatherService,
    location: CLLocation,
    queryKind: Int32,
    scopeKind: Int32,
    startSeconds: Double,
    endSeconds: Double,
    startIndex: Int64,
    endIndex: Int64
) async throws -> String {
    switch queryKind {
    case 0:
        let query = MonthlyWeatherStatisticsQuery<MonthTemperatureStatistics>.temperature
        switch scopeKind {
        case 0:
            return try wkJSONString(wkPayload(from: try await service.monthlyStatistics(for: location, including: query)))
        case 1:
            return try wkJSONString(wkPayload(from: try await service.monthlyStatistics(for: location, forMonthsIn: wkDateInterval(startSeconds: startSeconds, endSeconds: endSeconds), including: query)))
        case 2:
            return try wkJSONString(wkPayload(from: try await service.monthlyStatistics(for: location, startMonth: Int(startIndex), endMonth: Int(endIndex), including: query)))
        default:
            throw wkBridgeError(code: Int(WK_STATUS_INVALID_ARGUMENT), message: "invalid monthly statistics scope")
        }
    case 1:
        let query = MonthlyWeatherStatisticsQuery<MonthPrecipitationStatistics>.precipitation
        switch scopeKind {
        case 0:
            return try wkJSONString(wkPayload(from: try await service.monthlyStatistics(for: location, including: query)))
        case 1:
            return try wkJSONString(wkPayload(from: try await service.monthlyStatistics(for: location, forMonthsIn: wkDateInterval(startSeconds: startSeconds, endSeconds: endSeconds), including: query)))
        case 2:
            return try wkJSONString(wkPayload(from: try await service.monthlyStatistics(for: location, startMonth: Int(startIndex), endMonth: Int(endIndex), including: query)))
        default:
            throw wkBridgeError(code: Int(WK_STATUS_INVALID_ARGUMENT), message: "invalid monthly statistics scope")
        }
    default:
        throw wkBridgeError(code: Int(WK_STATUS_INVALID_ARGUMENT), message: "invalid monthly statistics query")
    }
}

@_cdecl("wk_weather_service_daily_statistics")
public func wk_weather_service_daily_statistics(
    _ serviceHandle: UnsafeMutableRawPointer?,
    _ latitude: Double,
    _ longitude: Double,
    _ queryKind: Int32,
    _ scopeKind: Int32,
    _ startSeconds: Double,
    _ endSeconds: Double,
    _ startIndex: Int64,
    _ endIndex: Int64,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 15.0, *) else {
        return wkStatisticsUnavailable(outError)
    }
    return wkFetchLocation(
        serviceHandle: serviceHandle,
        latitude: latitude,
        longitude: longitude,
        outHandle: outHandle,
        outError: outError,
        label: "WeatherService.dailyStatistics"
    ) { service, location in
        try await wkDailyStatisticsJSON(
            service: service,
            location: location,
            queryKind: queryKind,
            scopeKind: scopeKind,
            startSeconds: startSeconds,
            endSeconds: endSeconds,
            startIndex: startIndex,
            endIndex: endIndex
        )
    }
}

@_cdecl("wk_weather_service_daily_summary")
public func wk_weather_service_daily_summary(
    _ serviceHandle: UnsafeMutableRawPointer?,
    _ latitude: Double,
    _ longitude: Double,
    _ queryKind: Int32,
    _ scopeKind: Int32,
    _ startSeconds: Double,
    _ endSeconds: Double,
    _ startIndex: Int64,
    _ endIndex: Int64,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let _ = startIndex
    let _ = endIndex
    guard #available(macOS 15.0, *) else {
        return wkStatisticsUnavailable(outError)
    }
    return wkFetchLocation(
        serviceHandle: serviceHandle,
        latitude: latitude,
        longitude: longitude,
        outHandle: outHandle,
        outError: outError,
        label: "WeatherService.dailySummary"
    ) { service, location in
        try await wkDailySummaryJSON(
            service: service,
            location: location,
            queryKind: queryKind,
            scopeKind: scopeKind,
            startSeconds: startSeconds,
            endSeconds: endSeconds
        )
    }
}

@_cdecl("wk_weather_service_hourly_statistics")
public func wk_weather_service_hourly_statistics(
    _ serviceHandle: UnsafeMutableRawPointer?,
    _ latitude: Double,
    _ longitude: Double,
    _ queryKind: Int32,
    _ scopeKind: Int32,
    _ startSeconds: Double,
    _ endSeconds: Double,
    _ startIndex: Int64,
    _ endIndex: Int64,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let _ = queryKind
    guard #available(macOS 15.0, *) else {
        return wkStatisticsUnavailable(outError)
    }
    return wkFetchLocation(
        serviceHandle: serviceHandle,
        latitude: latitude,
        longitude: longitude,
        outHandle: outHandle,
        outError: outError,
        label: "WeatherService.hourlyStatistics"
    ) { service, location in
        try await wkHourlyStatisticsJSON(
            service: service,
            location: location,
            scopeKind: scopeKind,
            startSeconds: startSeconds,
            endSeconds: endSeconds,
            startIndex: startIndex,
            endIndex: endIndex
        )
    }
}

@_cdecl("wk_weather_service_monthly_statistics")
public func wk_weather_service_monthly_statistics(
    _ serviceHandle: UnsafeMutableRawPointer?,
    _ latitude: Double,
    _ longitude: Double,
    _ queryKind: Int32,
    _ scopeKind: Int32,
    _ startSeconds: Double,
    _ endSeconds: Double,
    _ startIndex: Int64,
    _ endIndex: Int64,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 15.0, *) else {
        return wkStatisticsUnavailable(outError)
    }
    return wkFetchLocation(
        serviceHandle: serviceHandle,
        latitude: latitude,
        longitude: longitude,
        outHandle: outHandle,
        outError: outError,
        label: "WeatherService.monthlyStatistics"
    ) { service, location in
        try await wkMonthlyStatisticsJSON(
            service: service,
            location: location,
            queryKind: queryKind,
            scopeKind: scopeKind,
            startSeconds: startSeconds,
            endSeconds: endSeconds,
            startIndex: startIndex,
            endIndex: endIndex
        )
    }
}
