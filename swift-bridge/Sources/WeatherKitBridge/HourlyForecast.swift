import Foundation
import WeatherKit

struct WKHourForecastPayload: Codable {
    let date: String
    let temperature: Double
    let feelsLike: Double
    let condition: String
    let symbolName: String
    let precipitation: String
    let precipitationChance: Double
    let precipitationAmount: Double
    let cloudCover: Double
    let dewPoint: Double
    let humidity: Double
    let isDaylight: Bool
    let pressure: Double
    let pressureTrend: String
    let uvIndex: WKUVIndexPayload
    let visibility: Double
    let wind: WKWindPayload
    let snowfallAmount: Double?
    let cloudCoverByAltitude: WKCloudCoverByAltitudePayload?
}

struct WKHourlyForecastPayload: Codable {
    let forecast: [WKHourForecastPayload]
    let metadata: WKWeatherMetadataPayload
}

func wkPayload(from hourWeather: HourWeather) -> WKHourForecastPayload {
    let snowfallAmount: Double?
    let cloudCoverByAltitude: WKCloudCoverByAltitudePayload?
    if #available(macOS 15.0, *) {
        snowfallAmount = wkLength(hourWeather.snowfallAmount)
        cloudCoverByAltitude = wkPayload(from: hourWeather.cloudCoverByAltitude)
    } else {
        snowfallAmount = nil
        cloudCoverByAltitude = nil
    }

    return WKHourForecastPayload(
        date: wkISO8601String(hourWeather.date),
        temperature: wkTemperature(hourWeather.temperature),
        feelsLike: wkTemperature(hourWeather.apparentTemperature),
        condition: hourWeather.condition.rawValue,
        symbolName: hourWeather.symbolName,
        precipitation: hourWeather.precipitation.rawValue,
        precipitationChance: hourWeather.precipitationChance,
        precipitationAmount: wkLength(hourWeather.precipitationAmount),
        cloudCover: hourWeather.cloudCover,
        dewPoint: wkTemperature(hourWeather.dewPoint),
        humidity: hourWeather.humidity,
        isDaylight: hourWeather.isDaylight,
        pressure: wkPressureValue(hourWeather.pressure),
        pressureTrend: hourWeather.pressureTrend.rawValue,
        uvIndex: wkPayload(from: hourWeather.uvIndex),
        visibility: wkLength(hourWeather.visibility),
        wind: wkPayload(from: hourWeather.wind),
        snowfallAmount: snowfallAmount,
        cloudCoverByAltitude: cloudCoverByAltitude
    )
}

func wkPayload(from hourlyForecast: Forecast<HourWeather>) -> WKHourlyForecastPayload {
    WKHourlyForecastPayload(
        forecast: hourlyForecast.forecast.map { wkPayload(from: $0) },
        metadata: wkPayload(from: hourlyForecast.metadata)
    )
}

@_cdecl("wk_hourly_forecast_retain")
public func wk_hourly_forecast_retain(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    wkRetainOpaque(handle)
}

@_cdecl("wk_hourly_forecast_release")
public func wk_hourly_forecast_release(_ handle: UnsafeMutableRawPointer?) {
    wkRelease(handle)
}

@_cdecl("wk_hourly_forecast_copy_json")
public func wk_hourly_forecast_copy_json(
    _ handle: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    wkCopyBoxJSON(handle, outJSON, outError, context: "HourlyForecast", payload: { (value: Forecast<HourWeather>) in wkPayload(from: value) })
}
