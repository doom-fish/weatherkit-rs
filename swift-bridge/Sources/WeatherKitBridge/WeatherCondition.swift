import Foundation
import WeatherKit

struct WKWeatherConditionDescriptorPayload: Codable {
    let rawValue: String
    let description: String
    let accessibilityDescription: String
}

struct WKPrecipitationDescriptorPayload: Codable {
    let rawValue: String
    let description: String
    let accessibilityDescription: String
}

@_cdecl("wk_weather_condition_copy_descriptors_json")
public func wk_weather_condition_copy_descriptors_json(
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    wkWriteJSON(
        WeatherCondition.allCases.map {
            WKWeatherConditionDescriptorPayload(
                rawValue: $0.rawValue,
                description: $0.description,
                accessibilityDescription: $0.accessibilityDescription
            )
        },
        into: outJSON,
        outError: outError
    )
}

@_cdecl("wk_precipitation_copy_descriptors_json")
public func wk_precipitation_copy_descriptors_json(
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    wkWriteJSON(
        Precipitation.allCases.map {
            WKPrecipitationDescriptorPayload(
                rawValue: $0.rawValue,
                description: $0.description,
                accessibilityDescription: $0.accessibilityDescription
            )
        },
        into: outJSON,
        outError: outError
    )
}
