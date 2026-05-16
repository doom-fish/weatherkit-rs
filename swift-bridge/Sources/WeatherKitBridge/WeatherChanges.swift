import CoreLocation
import Foundation
import WeatherKit

struct WKTrendBaselinePayload: Codable {
    let kind: String
    let value: Double
    let startDate: String
}

struct WKTrendPayload: Codable {
    let baseline: WKTrendBaselinePayload
    let currentValue: Double
    let deviation: String
}

struct WKHistoricalComparisonPayload: Codable {
    let kind: String
    let trend: WKTrendPayload
}

struct WKHistoricalComparisonsPayload: Codable {
    let comparisons: [WKHistoricalComparisonPayload]
    let metadata: WKWeatherMetadataPayload
}

struct WKWeatherChangePayload: Codable {
    let date: String
    let dayPrecipitationAmount: String
    let highTemperature: String
    let lowTemperature: String
    let nightPrecipitationAmount: String
}

struct WKWeatherChangesPayload: Codable {
    let changes: [WKWeatherChangePayload]
    let metadata: WKWeatherMetadataPayload
}

@available(macOS 15.0, *)
func wkRawValue<Dimension>(from kind: TrendBaseline<Dimension>.Kind) -> String where Dimension: Foundation.Dimension {
    switch kind {
    case .mean:
        return "mean"
    }
}

@available(macOS 15.0, *)
func wkRawValue(from deviation: Deviation) -> String {
    switch deviation {
    case .muchHigher:
        return "muchHigher"
    case .higher:
        return "higher"
    case .normal:
        return "normal"
    case .lower:
        return "lower"
    case .muchLower:
        return "muchLower"
    }
}

@available(macOS 15.0, *)
func wkRawValue(from direction: WeatherChange.Direction) -> String {
    switch direction {
    case .increase:
        return "increase"
    case .decrease:
        return "decrease"
    case .steady:
        return "steady"
    }
}

@available(macOS 15.0, *)
func wkPayload(from baseline: TrendBaseline<UnitTemperature>) -> WKTrendBaselinePayload {
    WKTrendBaselinePayload(
        kind: wkRawValue(from: baseline.kind),
        value: wkTemperature(baseline.value),
        startDate: wkISO8601String(baseline.startDate)
    )
}

@available(macOS 15.0, *)
func wkPayload(from baseline: TrendBaseline<UnitLength>) -> WKTrendBaselinePayload {
    WKTrendBaselinePayload(
        kind: wkRawValue(from: baseline.kind),
        value: wkLength(baseline.value),
        startDate: wkISO8601String(baseline.startDate)
    )
}

@available(macOS 15.0, *)
func wkPayload(from trend: Trend<UnitTemperature>) -> WKTrendPayload {
    WKTrendPayload(
        baseline: wkPayload(from: trend.baseline),
        currentValue: wkTemperature(trend.currentValue),
        deviation: wkRawValue(from: trend.deviation)
    )
}

@available(macOS 15.0, *)
func wkPayload(from trend: Trend<UnitLength>) -> WKTrendPayload {
    WKTrendPayload(
        baseline: wkPayload(from: trend.baseline),
        currentValue: wkLength(trend.currentValue),
        deviation: wkRawValue(from: trend.deviation)
    )
}

@available(macOS 15.0, *)
func wkPayload(from comparison: HistoricalComparison) -> WKHistoricalComparisonPayload {
    switch comparison {
    case .highTemperature(let trend):
        return WKHistoricalComparisonPayload(kind: "highTemperature", trend: wkPayload(from: trend))
    case .lowTemperature(let trend):
        return WKHistoricalComparisonPayload(kind: "lowTemperature", trend: wkPayload(from: trend))
    case .precipitationAmount(let trend):
        return WKHistoricalComparisonPayload(kind: "precipitationAmount", trend: wkPayload(from: trend))
    case .snowfallAmount(let trend):
        return WKHistoricalComparisonPayload(kind: "snowfallAmount", trend: wkPayload(from: trend))
    }
}

@available(macOS 15.0, *)
func wkPayload(from comparisons: HistoricalComparisons) -> WKHistoricalComparisonsPayload {
    WKHistoricalComparisonsPayload(
        comparisons: comparisons.comparisons.map { wkPayload(from: $0) },
        metadata: wkPayload(from: comparisons.metadata)
    )
}

@available(macOS 15.0, *)
func wkPayload(from change: WeatherChange) -> WKWeatherChangePayload {
    WKWeatherChangePayload(
        date: wkISO8601String(change.date),
        dayPrecipitationAmount: wkRawValue(from: change.dayPrecipitationAmount),
        highTemperature: wkRawValue(from: change.highTemperature),
        lowTemperature: wkRawValue(from: change.lowTemperature),
        nightPrecipitationAmount: wkRawValue(from: change.nightPrecipitationAmount)
    )
}

@available(macOS 15.0, *)
func wkPayload(from changes: WeatherChanges) -> WKWeatherChangesPayload {
    WKWeatherChangesPayload(
        changes: changes.changes.map { wkPayload(from: $0) },
        metadata: wkPayload(from: changes.metadata)
    )
}

func wkChangesUnavailable(_ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?) -> Int32 {
    wkWriteError(
        wkBridgeError(code: Int(WK_STATUS_FAILURE), message: "WeatherKit changes and historical comparisons require macOS 15.0 or newer"),
        into: outError
    )
    return WK_STATUS_FAILURE
}

@available(macOS 15.0, *)
func wkWeatherChangesJSON(service: WeatherService, location: CLLocation) async throws -> String {
    try wkJSONString(
        (try await service.weather(for: location, including: WeatherQuery<WeatherChanges?>.changes))
            .map { wkPayload(from: $0) }
    )
}

@available(macOS 15.0, *)
func wkHistoricalComparisonsJSON(service: WeatherService, location: CLLocation) async throws -> String {
    try wkJSONString(
        (try await service.weather(for: location, including: WeatherQuery<HistoricalComparisons?>.historicalComparisons))
            .map { wkPayload(from: $0) }
    )
}

@_cdecl("wk_weather_service_weather_changes")
public func wk_weather_service_weather_changes(
    _ serviceHandle: UnsafeMutableRawPointer?,
    _ latitude: Double,
    _ longitude: Double,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 15.0, *) else {
        return wkChangesUnavailable(outError)
    }
    return wkFetchLocation(
        serviceHandle: serviceHandle,
        latitude: latitude,
        longitude: longitude,
        outHandle: outHandle,
        outError: outError,
        label: "WeatherService.weather(for: including: .changes)"
    ) { service, location in
        try await wkWeatherChangesJSON(service: service, location: location)
    }
}

@_cdecl("wk_weather_service_historical_comparisons")
public func wk_weather_service_historical_comparisons(
    _ serviceHandle: UnsafeMutableRawPointer?,
    _ latitude: Double,
    _ longitude: Double,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 15.0, *) else {
        return wkChangesUnavailable(outError)
    }
    return wkFetchLocation(
        serviceHandle: serviceHandle,
        latitude: latitude,
        longitude: longitude,
        outHandle: outHandle,
        outError: outError,
        label: "WeatherService.weather(for: including: .historicalComparisons)"
    ) { service, location in
        try await wkHistoricalComparisonsJSON(service: service, location: location)
    }
}
