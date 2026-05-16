import CoreLocation
import Foundation
import WeatherKit

@_cdecl("wk_weather_for")
public func wk_weather_for(
    _ latitude: Double,
    _ longitude: Double,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let outJSON else {
        wkWriteError(
            wkBridgeError(code: Int(WK_STATUS_INVALID_ARGUMENT), message: "out_json must not be nil"),
            into: outError
        )
        return WK_STATUS_INVALID_ARGUMENT
    }

    outJSON.pointee = nil
    outError?.pointee = nil

    do {
        let location = CLLocation(latitude: latitude, longitude: longitude)
        let weather = try wkAwait(label: "WeatherService.weather(for:)") {
            try await WeatherService.shared.weather(for: location)
        }
        let payload = wkPayload(from: weather)
        outJSON.pointee = wkCString(try wkJSONString(payload))
        return WK_STATUS_OK
    } catch {
        wkWriteError(error, into: outError)
        let nsError = error as NSError
        if nsError.domain == "WeatherKitBridge" {
            return Int32(nsError.code)
        }
        return WK_STATUS_FAILURE
    }
}
