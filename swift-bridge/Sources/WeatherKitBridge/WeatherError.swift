import Foundation
import WeatherKit

struct WKWeatherErrorDescriptorPayload: Codable {
    let rawValue: String
    let errorDescription: String?
    let failureReason: String?
    let helpAnchor: String?
    let recoverySuggestion: String?
}

private func wkRawValue(from error: WeatherError) -> String {
    switch error {
    case .permissionDenied:
        return "permissionDenied"
    case .unknown:
        return "unknown"
    @unknown default:
        return "unknown"
    }
}

@_cdecl("wk_weather_error_copy_descriptors_json")
public func wk_weather_error_copy_descriptors_json(
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let descriptors: [WKWeatherErrorDescriptorPayload] = [WeatherError.permissionDenied, .unknown].map {
        WKWeatherErrorDescriptorPayload(
            rawValue: wkRawValue(from: $0),
            errorDescription: $0.errorDescription,
            failureReason: $0.failureReason,
            helpAnchor: $0.helpAnchor,
            recoverySuggestion: $0.recoverySuggestion
        )
    }
    return wkWriteJSON(descriptors, into: outJSON, outError: outError)
}
