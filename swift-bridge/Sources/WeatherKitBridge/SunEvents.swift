import Foundation
import WeatherKit

struct WKSunEventsPayload: Codable {
    let astronomicalDawn: String?
    let nauticalDawn: String?
    let civilDawn: String?
    let sunrise: String?
    let solarNoon: String?
    let sunset: String?
    let civilDusk: String?
    let nauticalDusk: String?
    let astronomicalDusk: String?
    let solarMidnight: String?
}

func wkPayload(from sunEvents: SunEvents) -> WKSunEventsPayload {
    WKSunEventsPayload(
        astronomicalDawn: sunEvents.astronomicalDawn.map(wkISO8601String),
        nauticalDawn: sunEvents.nauticalDawn.map(wkISO8601String),
        civilDawn: sunEvents.civilDawn.map(wkISO8601String),
        sunrise: sunEvents.sunrise.map(wkISO8601String),
        solarNoon: sunEvents.solarNoon.map(wkISO8601String),
        sunset: sunEvents.sunset.map(wkISO8601String),
        civilDusk: sunEvents.civilDusk.map(wkISO8601String),
        nauticalDusk: sunEvents.nauticalDusk.map(wkISO8601String),
        astronomicalDusk: sunEvents.astronomicalDusk.map(wkISO8601String),
        solarMidnight: sunEvents.solarMidnight.map(wkISO8601String)
    )
}
