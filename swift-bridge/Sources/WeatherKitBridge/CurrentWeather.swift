import Foundation
import WeatherKit

struct WKCurrentWeatherPayload: Codable {
    let date: String
    let temperature: Double
    let feelsLike: Double
    let humidity: Double
    let dewPoint: Double
    let pressure: Double
    let pressureTrend: String
    let condition: String
    let symbolName: String
    let wind: WKWindPayload
    let uvIndex: WKUVIndexPayload
    let visibility: Double
    let cloudCover: Double
    let cloudCoverByAltitude: WKCloudCoverByAltitudePayload?
    let isDaylight: Bool
    let precipitationIntensity: Double
    let metadata: WKWeatherMetadataPayload
}

func wkPayload(from currentWeather: CurrentWeather) -> WKCurrentWeatherPayload {
    let cloudCoverByAltitude: WKCloudCoverByAltitudePayload?
    if #available(macOS 15.0, *) {
        cloudCoverByAltitude = wkPayload(from: currentWeather.cloudCoverByAltitude)
    } else {
        cloudCoverByAltitude = nil
    }

    return WKCurrentWeatherPayload(
        date: wkISO8601String(currentWeather.date),
        temperature: wkTemperature(currentWeather.temperature),
        feelsLike: wkTemperature(currentWeather.apparentTemperature),
        humidity: currentWeather.humidity,
        dewPoint: wkTemperature(currentWeather.dewPoint),
        pressure: wkPressureValue(currentWeather.pressure),
        pressureTrend: currentWeather.pressureTrend.rawValue,
        condition: currentWeather.condition.rawValue,
        symbolName: currentWeather.symbolName,
        wind: wkPayload(from: currentWeather.wind),
        uvIndex: wkPayload(from: currentWeather.uvIndex),
        visibility: wkLength(currentWeather.visibility),
        cloudCover: currentWeather.cloudCover,
        cloudCoverByAltitude: cloudCoverByAltitude,
        isDaylight: currentWeather.isDaylight,
        precipitationIntensity: wkSpeed(currentWeather.precipitationIntensity),
        metadata: wkPayload(from: currentWeather.metadata)
    )
}

@_cdecl("wk_current_weather_retain")
public func wk_current_weather_retain(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    wkRetainOpaque(handle)
}

@_cdecl("wk_current_weather_release")
public func wk_current_weather_release(_ handle: UnsafeMutableRawPointer?) {
    wkRelease(handle)
}

@_cdecl("wk_current_weather_copy_json")
public func wk_current_weather_copy_json(
    _ handle: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    wkCopyBoxJSON(handle, outJSON, outError, context: "CurrentWeather", payload: { (value: CurrentWeather) in wkPayload(from: value) })
}
