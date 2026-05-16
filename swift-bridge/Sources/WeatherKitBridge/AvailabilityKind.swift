import Foundation
import WeatherKit

struct WKAvailabilityKindDescriptorPayload: Codable {
    let rawValue: String
}

struct WKWeatherAvailabilityPayload: Codable {
    let minuteAvailability: String
    let alertAvailability: String
}

func wkPayload(from availability: WeatherAvailability) -> WKWeatherAvailabilityPayload {
    WKWeatherAvailabilityPayload(
        minuteAvailability: availability.minuteAvailability.rawValue,
        alertAvailability: availability.alertAvailability.rawValue
    )
}

@_cdecl("wk_weather_availability_retain")
public func wk_weather_availability_retain(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    wkRetainOpaque(handle)
}

@_cdecl("wk_weather_availability_release")
public func wk_weather_availability_release(_ handle: UnsafeMutableRawPointer?) {
    wkRelease(handle)
}

@_cdecl("wk_weather_availability_copy_json")
public func wk_weather_availability_copy_json(
    _ handle: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    wkCopyBoxJSON(handle, outJSON, outError, context: "WeatherAvailability", payload: { (value: WeatherAvailability) in wkPayload(from: value) })
}

@_cdecl("wk_availability_kind_copy_descriptors_json")
public func wk_availability_kind_copy_descriptors_json(
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let kinds: [WeatherAvailability.AvailabilityKind] = [.available, .temporarilyUnavailable, .unsupported, .unknown]
    return wkWriteJSON(
        kinds.map { WKAvailabilityKindDescriptorPayload(rawValue: $0.rawValue) },
        into: outJSON,
        outError: outError
    )
}
