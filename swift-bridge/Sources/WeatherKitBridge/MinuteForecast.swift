import Foundation
import WeatherKit

struct WKMinuteForecastPayload: Codable {
    let date: String
    let precipitation: String
    let precipitationChance: Double
    let precipitationIntensity: Double
}

struct WKMinuteForecastCollectionPayload: Codable {
    let forecast: [WKMinuteForecastPayload]
    let metadata: WKWeatherMetadataPayload
    let summary: String
}

func wkPayload(from minuteWeather: MinuteWeather) -> WKMinuteForecastPayload {
    WKMinuteForecastPayload(
        date: wkISO8601String(minuteWeather.date),
        precipitation: minuteWeather.precipitation.rawValue,
        precipitationChance: minuteWeather.precipitationChance,
        precipitationIntensity: wkSpeed(minuteWeather.precipitationIntensity)
    )
}

func wkPayload(from minuteForecast: Forecast<MinuteWeather>) -> WKMinuteForecastCollectionPayload {
    WKMinuteForecastCollectionPayload(
        forecast: minuteForecast.forecast.map { wkPayload(from: $0) },
        metadata: wkPayload(from: minuteForecast.metadata),
        summary: minuteForecast.summary
    )
}

@_cdecl("wk_minute_forecast_retain")
public func wk_minute_forecast_retain(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    wkRetainOpaque(handle)
}

@_cdecl("wk_minute_forecast_release")
public func wk_minute_forecast_release(_ handle: UnsafeMutableRawPointer?) {
    wkRelease(handle)
}

@_cdecl("wk_minute_forecast_copy_json")
public func wk_minute_forecast_copy_json(
    _ handle: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    wkCopyBoxJSON(
        handle,
        outJSON,
        outError,
        context: "MinuteForecast",
        payload: { (forecast: Forecast<MinuteWeather>?) in forecast.map { wkPayload(from: $0) } }
    )
}
