import CoreLocation
import Foundation
import WeatherKit

struct WKWindPayload: Codable {
    let speed: Double
    let direction: Double
    let compassDirection: String
    let gust: Double?
}

struct WKUVIndexPayload: Codable {
    let value: Int
    let category: String
}

struct WKCurrentWeatherPayload: Codable {
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
    let isDaylight: Bool
}

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
}

struct WKDayForecastPayload: Codable {
    let date: String
    let highTemperature: Double
    let lowTemperature: Double
    let condition: String
    let symbolName: String
    let precipitation: String
    let precipitationChance: Double
    let precipitationAmount: Double
}

struct WKMinuteForecastPayload: Codable {
    let date: String
    let precipitation: String
    let precipitationChance: Double
    let precipitationIntensity: Double
}

struct WKWeatherAlertPayload: Codable {
    let summary: String
    let detailsURL: String
    let source: String
    let severity: String
    let region: String?
}

struct WKWeatherAvailabilityPayload: Codable {
    let minuteAvailability: String
    let alertAvailability: String
}

struct WKWeatherPayload: Codable {
    let currentWeather: WKCurrentWeatherPayload
    let hourlyForecast: [WKHourForecastPayload]
    let dailyForecast: [WKDayForecastPayload]
    let minuteForecast: [WKMinuteForecastPayload]?
    let weatherAlerts: [WKWeatherAlertPayload]
    let availability: WKWeatherAvailabilityPayload
}

func wkTemperature(_ value: Measurement<UnitTemperature>) -> Double {
    value.converted(to: .celsius).value
}

func wkPressure(_ value: Measurement<UnitPressure>) -> Double {
    value.converted(to: .hectopascals).value
}

func wkLength(_ value: Measurement<UnitLength>) -> Double {
    value.converted(to: .meters).value
}

func wkSpeed(_ value: Measurement<UnitSpeed>) -> Double {
    value.converted(to: .metersPerSecond).value
}

func wkAngle(_ value: Measurement<UnitAngle>) -> Double {
    value.converted(to: .degrees).value
}

func wkPayload(from wind: Wind) -> WKWindPayload {
    WKWindPayload(
        speed: wkSpeed(wind.speed),
        direction: wkAngle(wind.direction),
        compassDirection: wind.compassDirection.rawValue,
        gust: wind.gust.map(wkSpeed)
    )
}

func wkPayload(from uvIndex: UVIndex) -> WKUVIndexPayload {
    WKUVIndexPayload(value: uvIndex.value, category: uvIndex.category.rawValue)
}

func wkPayload(from currentWeather: CurrentWeather) -> WKCurrentWeatherPayload {
    WKCurrentWeatherPayload(
        temperature: wkTemperature(currentWeather.temperature),
        feelsLike: wkTemperature(currentWeather.apparentTemperature),
        humidity: currentWeather.humidity,
        dewPoint: wkTemperature(currentWeather.dewPoint),
        pressure: wkPressure(currentWeather.pressure),
        pressureTrend: currentWeather.pressureTrend.rawValue,
        condition: currentWeather.condition.rawValue,
        symbolName: currentWeather.symbolName,
        wind: wkPayload(from: currentWeather.wind),
        uvIndex: wkPayload(from: currentWeather.uvIndex),
        visibility: wkLength(currentWeather.visibility),
        cloudCover: currentWeather.cloudCover,
        isDaylight: currentWeather.isDaylight
    )
}

func wkPayload(from hourWeather: HourWeather) -> WKHourForecastPayload {
    WKHourForecastPayload(
        date: wkISO8601String(hourWeather.date),
        temperature: wkTemperature(hourWeather.temperature),
        feelsLike: wkTemperature(hourWeather.apparentTemperature),
        condition: hourWeather.condition.rawValue,
        symbolName: hourWeather.symbolName,
        precipitation: hourWeather.precipitation.rawValue,
        precipitationChance: hourWeather.precipitationChance,
        precipitationAmount: wkLength(hourWeather.precipitationAmount),
        cloudCover: hourWeather.cloudCover
    )
}

func wkPayload(from dayWeather: DayWeather) -> WKDayForecastPayload {
    WKDayForecastPayload(
        date: wkISO8601String(dayWeather.date),
        highTemperature: wkTemperature(dayWeather.highTemperature),
        lowTemperature: wkTemperature(dayWeather.lowTemperature),
        condition: dayWeather.condition.rawValue,
        symbolName: dayWeather.symbolName,
        precipitation: dayWeather.precipitation.rawValue,
        precipitationChance: dayWeather.precipitationChance,
        precipitationAmount: wkLength(dayWeather.precipitationAmount)
    )
}

func wkPayload(from minuteWeather: MinuteWeather) -> WKMinuteForecastPayload {
    WKMinuteForecastPayload(
        date: wkISO8601String(minuteWeather.date),
        precipitation: minuteWeather.precipitation.rawValue,
        precipitationChance: minuteWeather.precipitationChance,
        precipitationIntensity: wkSpeed(minuteWeather.precipitationIntensity)
    )
}

func wkPayload(from alert: WeatherAlert) -> WKWeatherAlertPayload {
    WKWeatherAlertPayload(
        summary: alert.summary,
        detailsURL: alert.detailsURL.absoluteString,
        source: alert.source,
        severity: alert.severity.rawValue,
        region: alert.region
    )
}

func wkPayload(from availability: WeatherAvailability) -> WKWeatherAvailabilityPayload {
    WKWeatherAvailabilityPayload(
        minuteAvailability: availability.minuteAvailability.rawValue,
        alertAvailability: availability.alertAvailability.rawValue
    )
}

func wkPayload(from weather: Weather) -> WKWeatherPayload {
    WKWeatherPayload(
        currentWeather: wkPayload(from: weather.currentWeather),
        hourlyForecast: weather.hourlyForecast.forecast.map(wkPayload),
        dailyForecast: weather.dailyForecast.forecast.map(wkPayload),
        minuteForecast: weather.minuteForecast?.forecast.map(wkPayload),
        weatherAlerts: (weather.weatherAlerts ?? []).map(wkPayload),
        availability: wkPayload(from: weather.availability)
    )
}
