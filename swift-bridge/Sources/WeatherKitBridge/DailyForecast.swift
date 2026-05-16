import Foundation
import WeatherKit

struct WKDayForecastPayload: Codable {
    let date: String
    let highTemperature: Double
    let lowTemperature: Double
    let condition: String
    let symbolName: String
    let precipitation: String
    let precipitationChance: Double
    let precipitationAmount: Double
    let rainfallAmount: Double
    let snowfallAmount: Double
    let sun: WKSunEventsPayload
    let moon: WKMoonEventsPayload
    let uvIndex: WKUVIndexPayload
    let wind: WKWindPayload
    let highTemperatureTime: String?
    let lowTemperatureTime: String?
    let maximumHumidity: Double?
    let minimumHumidity: Double?
    let precipitationAmountByType: WKPrecipitationAmountByTypePayload?
    let maximumVisibility: Double?
    let minimumVisibility: Double?
    let highWindSpeed: Double?
    let daytimeForecast: WKDayPartForecastPayload?
    let overnightForecast: WKDayPartForecastPayload?
    let restOfDayForecast: WKDayPartForecastPayload?
}

struct WKDailyForecastPayload: Codable {
    let forecast: [WKDayForecastPayload]
    let metadata: WKWeatherMetadataPayload
}

func wkPayload(from dayWeather: DayWeather) -> WKDayForecastPayload {
    let highTemperatureTime: String?
    let lowTemperatureTime: String?
    let maximumHumidity: Double?
    let minimumHumidity: Double?
    let precipitationAmountByType: WKPrecipitationAmountByTypePayload?
    let maximumVisibility: Double?
    let minimumVisibility: Double?
    let highWindSpeed: Double?
    let daytimeForecast: WKDayPartForecastPayload?
    let overnightForecast: WKDayPartForecastPayload?
    let restOfDayForecast: WKDayPartForecastPayload?

    if #available(macOS 15.0, *) {
        highTemperatureTime = dayWeather.highTemperatureTime.map(wkISO8601String)
        lowTemperatureTime = dayWeather.lowTemperatureTime.map(wkISO8601String)
        maximumHumidity = dayWeather.maximumHumidity
        minimumHumidity = dayWeather.minimumHumidity
        precipitationAmountByType = wkPayload(from: dayWeather.precipitationAmountByType)
        maximumVisibility = dayWeather.maximumVisibility
        minimumVisibility = dayWeather.minimumVisibility
        highWindSpeed = dayWeather.highWindSpeed.map(wkSpeed)
        daytimeForecast = wkPayload(from: dayWeather.daytimeForecast)
        overnightForecast = wkPayload(from: dayWeather.overnightForecast)
        restOfDayForecast = dayWeather.restOfDayForecast.map { wkPayload(from: $0) }
    } else {
        highTemperatureTime = nil
        lowTemperatureTime = nil
        maximumHumidity = nil
        minimumHumidity = nil
        precipitationAmountByType = nil
        maximumVisibility = nil
        minimumVisibility = nil
        highWindSpeed = nil
        daytimeForecast = nil
        overnightForecast = nil
        restOfDayForecast = nil
    }

    return WKDayForecastPayload(
        date: wkISO8601String(dayWeather.date),
        highTemperature: wkTemperature(dayWeather.highTemperature),
        lowTemperature: wkTemperature(dayWeather.lowTemperature),
        condition: dayWeather.condition.rawValue,
        symbolName: dayWeather.symbolName,
        precipitation: dayWeather.precipitation.rawValue,
        precipitationChance: dayWeather.precipitationChance,
        precipitationAmount: wkLength(dayWeather.precipitationAmount),
        rainfallAmount: wkLength(dayWeather.rainfallAmount),
        snowfallAmount: wkLength(dayWeather.snowfallAmount),
        sun: wkPayload(from: dayWeather.sun),
        moon: wkPayload(from: dayWeather.moon),
        uvIndex: wkPayload(from: dayWeather.uvIndex),
        wind: wkPayload(from: dayWeather.wind),
        highTemperatureTime: highTemperatureTime,
        lowTemperatureTime: lowTemperatureTime,
        maximumHumidity: maximumHumidity,
        minimumHumidity: minimumHumidity,
        precipitationAmountByType: precipitationAmountByType,
        maximumVisibility: maximumVisibility,
        minimumVisibility: minimumVisibility,
        highWindSpeed: highWindSpeed,
        daytimeForecast: daytimeForecast,
        overnightForecast: overnightForecast,
        restOfDayForecast: restOfDayForecast
    )
}

func wkPayload(from dailyForecast: Forecast<DayWeather>) -> WKDailyForecastPayload {
    WKDailyForecastPayload(
        forecast: dailyForecast.forecast.map { wkPayload(from: $0) },
        metadata: wkPayload(from: dailyForecast.metadata)
    )
}

@_cdecl("wk_daily_forecast_retain")
public func wk_daily_forecast_retain(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    wkRetainOpaque(handle)
}

@_cdecl("wk_daily_forecast_release")
public func wk_daily_forecast_release(_ handle: UnsafeMutableRawPointer?) {
    wkRelease(handle)
}

@_cdecl("wk_daily_forecast_copy_json")
public func wk_daily_forecast_copy_json(
    _ handle: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    wkCopyBoxJSON(handle, outJSON, outError, context: "DailyForecast", payload: { (value: Forecast<DayWeather>) in wkPayload(from: value) })
}
