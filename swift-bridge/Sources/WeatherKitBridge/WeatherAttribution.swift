import Foundation
import WeatherKit

struct WKWeatherAttributionPayload: Codable {
    let serviceName: String
    let legalPageURL: String
    let squareMarkURL: String
    let combinedMarkDarkURL: String
    let combinedMarkLightURL: String
    let legalAttributionText: String?
}

func wkPayload(from weatherAttribution: WeatherAttribution) -> WKWeatherAttributionPayload {
    let legalAttributionText: String?
    if #available(macOS 13.3, *) {
        legalAttributionText = weatherAttribution.legalAttributionText
    } else {
        legalAttributionText = nil
    }

    return WKWeatherAttributionPayload(
        serviceName: weatherAttribution.serviceName,
        legalPageURL: weatherAttribution.legalPageURL.absoluteString,
        squareMarkURL: weatherAttribution.squareMarkURL.absoluteString,
        combinedMarkDarkURL: weatherAttribution.combinedMarkDarkURL.absoluteString,
        combinedMarkLightURL: weatherAttribution.combinedMarkLightURL.absoluteString,
        legalAttributionText: legalAttributionText
    )
}

@_cdecl("wk_weather_attribution_retain")
public func wk_weather_attribution_retain(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    wkRetainOpaque(handle)
}

@_cdecl("wk_weather_attribution_release")
public func wk_weather_attribution_release(_ handle: UnsafeMutableRawPointer?) {
    wkRelease(handle)
}

@_cdecl("wk_weather_attribution_copy_json")
public func wk_weather_attribution_copy_json(
    _ handle: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    wkCopyBoxJSON(handle, outJSON, outError, context: "WeatherAttribution", payload: { (value: WeatherAttribution) in wkPayload(from: value) })
}
