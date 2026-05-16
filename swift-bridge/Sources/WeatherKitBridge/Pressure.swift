import Foundation
import WeatherKit

struct WKPressureTrendDescriptorPayload: Codable {
    let rawValue: String
    let description: String
    let accessibilityDescription: String
}

@_cdecl("wk_pressure_trend_copy_descriptors_json")
public func wk_pressure_trend_copy_descriptors_json(
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    wkWriteJSON(
        PressureTrend.allCases.map {
            WKPressureTrendDescriptorPayload(
                rawValue: $0.rawValue,
                description: $0.description,
                accessibilityDescription: $0.accessibilityDescription
            )
        },
        into: outJSON,
        outError: outError
    )
}
