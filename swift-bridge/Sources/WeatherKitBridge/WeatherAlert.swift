import Foundation
import WeatherKit

struct WKWeatherAlertPayload: Codable {
    let summary: String
    let detailsURL: String
    let source: String
    let severity: String
    let region: String?
    let metadata: WKWeatherMetadataPayload
}

struct WKWeatherSeverityDescriptorPayload: Codable {
    let rawValue: String
    let description: String
    let accessibilityDescription: String
}

func wkPayload(from weatherAlert: WeatherAlert) -> WKWeatherAlertPayload {
    WKWeatherAlertPayload(
        summary: weatherAlert.summary,
        detailsURL: weatherAlert.detailsURL.absoluteString,
        source: weatherAlert.source,
        severity: weatherAlert.severity.rawValue,
        region: weatherAlert.region,
        metadata: wkPayload(from: weatherAlert.metadata)
    )
}

@_cdecl("wk_weather_alerts_retain")
public func wk_weather_alerts_retain(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    wkRetainOpaque(handle)
}

@_cdecl("wk_weather_alerts_release")
public func wk_weather_alerts_release(_ handle: UnsafeMutableRawPointer?) {
    wkRelease(handle)
}

@_cdecl("wk_weather_alerts_copy_json")
public func wk_weather_alerts_copy_json(
    _ handle: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    wkCopyBoxJSON(
        handle,
        outJSON,
        outError,
        context: "WeatherAlerts",
        payload: { (alerts: [WeatherAlert]) in alerts.map { wkPayload(from: $0) } }
    )
}

@_cdecl("wk_weather_severity_copy_descriptors_json")
public func wk_weather_severity_copy_descriptors_json(
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    wkWriteJSON(
        WeatherSeverity.allCases.map {
            WKWeatherSeverityDescriptorPayload(
                rawValue: $0.rawValue,
                description: $0.description,
                accessibilityDescription: $0.accessibilityDescription
            )
        },
        into: outJSON,
        outError: outError
    )
}
