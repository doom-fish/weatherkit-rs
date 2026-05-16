import Foundation
import WeatherKit

struct WKMoonEventsPayload: Codable {
    let phase: String
    let moonrise: String?
    let moonset: String?
}

struct WKMoonPhaseDescriptorPayload: Codable {
    let rawValue: String
    let description: String
    let accessibilityDescription: String
    let symbolName: String
}

func wkPayload(from moonEvents: MoonEvents) -> WKMoonEventsPayload {
    WKMoonEventsPayload(
        phase: moonEvents.phase.rawValue,
        moonrise: moonEvents.moonrise.map(wkISO8601String),
        moonset: moonEvents.moonset.map(wkISO8601String)
    )
}

@_cdecl("wk_moon_phase_copy_descriptors_json")
public func wk_moon_phase_copy_descriptors_json(
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    wkWriteJSON(
        MoonPhase.allCases.map {
            WKMoonPhaseDescriptorPayload(
                rawValue: $0.rawValue,
                description: $0.description,
                accessibilityDescription: $0.accessibilityDescription,
                symbolName: $0.symbolName
            )
        },
        into: outJSON,
        outError: outError
    )
}
