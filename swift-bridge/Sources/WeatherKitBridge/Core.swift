import CoreLocation
import Dispatch
import Foundation
import WeatherKit

let WK_STATUS_OK: Int32 = 0
let WK_STATUS_INVALID_ARGUMENT: Int32 = -1
let WK_STATUS_FAILURE: Int32 = -2
let WK_STATUS_TIMED_OUT: Int32 = -3

private let wkISO8601Formatter: ISO8601DateFormatter = {
    let formatter = ISO8601DateFormatter()
    formatter.formatOptions = [.withInternetDateTime, .withFractionalSeconds]
    return formatter
}()

private struct WKErrorPayload: Codable {
    let domain: String
    let code: Int
    let message: String
}

final class WKBox<T> {
    let value: T

    init(_ value: T) {
        self.value = value
    }
}

struct WKLocationPayload: Codable {
    let latitude: Double
    let longitude: Double
}

struct WKWeatherMetadataPayload: Codable {
    let date: String
    let expirationDate: String
    let location: WKLocationPayload
}

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

struct WKCloudCoverByAltitudePayload: Codable {
    let low: Double
    let medium: Double
    let high: Double
}

struct WKSnowfallAmountPayload: Codable {
    let amount: Double
    let maximum: Double
    let minimum: Double
    let amountLiquidEquivalent: Double
    let maximumLiquidEquivalent: Double
    let minimumLiquidEquivalent: Double
}

struct WKPrecipitationAmountByTypePayload: Codable {
    let hail: Double
    let mixed: Double
    let rainfall: Double
    let sleet: Double
    let precipitation: Double
    let snowfallAmount: WKSnowfallAmountPayload
}

struct WKDayPartForecastPayload: Codable {
    let cloudCover: Double
    let cloudCoverByAltitude: WKCloudCoverByAltitudePayload?
    let condition: String
    let highTemperature: Double
    let lowTemperature: Double
    let precipitation: String
    let precipitationAmountByType: WKPrecipitationAmountByTypePayload?
    let precipitationChance: Double
    let maximumHumidity: Double?
    let minimumHumidity: Double?
    let maximumVisibility: Double?
    let minimumVisibility: Double?
    let wind: WKWindPayload
    let highWindSpeed: Double?
}

@_cdecl("wk_string_free")
public func wk_string_free(_ string: UnsafeMutablePointer<CChar>?) {
    guard let string else { return }
    string.deallocate()
}

func wkCString(_ string: String) -> UnsafeMutablePointer<CChar> {
    let buffer = UnsafeMutablePointer<CChar>.allocate(capacity: string.utf8.count + 1)
    _ = buffer.initialize(from: Array(string.utf8CString), count: string.utf8CString.count)
    return buffer
}

func wkJSONString<T: Encodable>(_ value: T) throws -> String {
    let encoder = JSONEncoder()
    encoder.outputFormatting = [.withoutEscapingSlashes]
    return String(data: try encoder.encode(value), encoding: .utf8) ?? "{}"
}

func wkWriteJSON<T: Encodable>(
    _ value: T,
    into outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
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
        outJSON.pointee = wkCString(try wkJSONString(value))
        return WK_STATUS_OK
    } catch {
        wkWriteError(error, into: outError)
        return wkStatus(from: error)
    }
}

func wkWriteError(_ error: Error, into outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?) {
    guard let outError else { return }
    let nsError = error as NSError
    let payload = WKErrorPayload(
        domain: nsError.domain,
        code: nsError.code,
        message: nsError.localizedDescription
    )
    let json: String
    do {
        json = try wkJSONString(payload)
    } catch {
        json = "{\"domain\":\"WeatherKitBridge\",\"code\":-2,\"message\":\"Failed to encode WeatherKit error payload\"}"
    }
    outError.pointee = wkCString(json)
}

func wkBridgeError(code: Int, message: String) -> NSError {
    NSError(domain: "WeatherKitBridge", code: code, userInfo: [NSLocalizedDescriptionKey: message])
}

func wkStatus(from error: Error) -> Int32 {
    let nsError = error as NSError
    if nsError.domain == "WeatherKitBridge" {
        return Int32(nsError.code)
    }
    return WK_STATUS_FAILURE
}

func wkAwait<T>(label: String, timeoutSeconds: TimeInterval = 60, work: @escaping () async throws -> T) throws -> T {
    let semaphore = DispatchSemaphore(value: 0)
    let lock = NSLock()
    var result: Result<T, Error>?

    Task {
        do {
            let value = try await work()
            lock.lock()
            result = .success(value)
            lock.unlock()
        } catch {
            lock.lock()
            result = .failure(error)
            lock.unlock()
        }
        semaphore.signal()
    }

    if semaphore.wait(timeout: .now() + timeoutSeconds) == .timedOut {
        throw wkBridgeError(code: Int(WK_STATUS_TIMED_OUT), message: "Timed out waiting for \(label)")
    }

    lock.lock()
    defer { lock.unlock() }
    guard let result else {
        throw wkBridgeError(code: Int(WK_STATUS_FAILURE), message: "Missing result for \(label)")
    }
    return try result.get()
}

func wkISO8601String(_ date: Date) -> String {
    wkISO8601Formatter.string(from: date)
}

func wkDate(fromUnixSeconds seconds: Double) -> Date {
    Date(timeIntervalSince1970: seconds)
}

func wkRetainObject(_ object: AnyObject) -> UnsafeMutableRawPointer {
    UnsafeMutableRawPointer(Unmanaged.passRetained(object).toOpaque())
}

func wkRetainBox<T>(_ value: T) -> UnsafeMutableRawPointer {
    wkRetainObject(WKBox(value))
}

func wkRetainOpaque(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else { return nil }
    let rebound = handle.assumingMemoryBound(to: UInt8.self)
    let object = Unmanaged<AnyObject>.fromOpaque(UnsafeRawPointer(rebound)).takeUnretainedValue()
    return wkRetainObject(object)
}

func wkUnretainedObject<T: AnyObject>(_ handle: UnsafeMutableRawPointer?, as _: T.Type = T.self) -> T? {
    guard let handle else { return nil }
    let rebound = handle.assumingMemoryBound(to: UInt8.self)
    return Unmanaged<T>.fromOpaque(UnsafeRawPointer(rebound)).takeUnretainedValue()
}

func wkUnretainedBoxValue<T>(_ handle: UnsafeMutableRawPointer?, as _: T.Type = T.self) -> T? {
    wkUnretainedObject(handle, as: WKBox<T>.self)?.value
}

func wkRelease(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else { return }
    let rebound = handle.assumingMemoryBound(to: UInt8.self)
    Unmanaged<AnyObject>.fromOpaque(UnsafeRawPointer(rebound)).release()
}

func wkCopyBoxJSON<T, Payload: Encodable>(
    _ handle: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    context: String,
    payload: (T) -> Payload
) -> Int32 {
    guard let value: T = wkUnretainedBoxValue(handle) else {
        wkWriteError(
            wkBridgeError(code: Int(WK_STATUS_INVALID_ARGUMENT), message: "\(context) handle must not be nil"),
            into: outError
        )
        return WK_STATUS_INVALID_ARGUMENT
    }
    return wkWriteJSON(payload(value), into: outJSON, outError: outError)
}

func wkTemperature(_ value: Measurement<UnitTemperature>) -> Double {
    value.converted(to: .celsius).value
}

func wkPressureValue(_ value: Measurement<UnitPressure>) -> Double {
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

func wkPayload(from location: CLLocation) -> WKLocationPayload {
    WKLocationPayload(latitude: location.coordinate.latitude, longitude: location.coordinate.longitude)
}

func wkPayload(from metadata: WeatherMetadata) -> WKWeatherMetadataPayload {
    WKWeatherMetadataPayload(
        date: wkISO8601String(metadata.date),
        expirationDate: wkISO8601String(metadata.expirationDate),
        location: wkPayload(from: metadata.location)
    )
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

@available(macOS 15.0, *)
func wkPayload(from cloudCoverByAltitude: CloudCoverByAltitude) -> WKCloudCoverByAltitudePayload {
    WKCloudCoverByAltitudePayload(
        low: cloudCoverByAltitude.low,
        medium: cloudCoverByAltitude.medium,
        high: cloudCoverByAltitude.high
    )
}

@available(macOS 15.0, *)
func wkPayload(from snowfallAmount: SnowfallAmount) -> WKSnowfallAmountPayload {
    WKSnowfallAmountPayload(
        amount: wkLength(snowfallAmount.amount),
        maximum: wkLength(snowfallAmount.maximum),
        minimum: wkLength(snowfallAmount.minimum),
        amountLiquidEquivalent: wkLength(snowfallAmount.amountLiquidEquivalent),
        maximumLiquidEquivalent: wkLength(snowfallAmount.maximumLiquidEquivalent),
        minimumLiquidEquivalent: wkLength(snowfallAmount.minimumLiquidEquivalent)
    )
}

@available(macOS 15.0, *)
func wkPayload(from precipitationAmountByType: PrecipitationAmountByType) -> WKPrecipitationAmountByTypePayload {
    WKPrecipitationAmountByTypePayload(
        hail: wkLength(precipitationAmountByType.hail),
        mixed: wkLength(precipitationAmountByType.mixed),
        rainfall: wkLength(precipitationAmountByType.rainfall),
        sleet: wkLength(precipitationAmountByType.sleet),
        precipitation: wkLength(precipitationAmountByType.precipitation),
        snowfallAmount: wkPayload(from: precipitationAmountByType.snowfallAmount)
    )
}

@available(macOS 15.0, *)
func wkPayload(from dayPartForecast: DayPartForecast) -> WKDayPartForecastPayload {
    WKDayPartForecastPayload(
        cloudCover: dayPartForecast.cloudCover,
        cloudCoverByAltitude: wkPayload(from: dayPartForecast.cloudCoverByAltitude),
        condition: dayPartForecast.condition.rawValue,
        highTemperature: wkTemperature(dayPartForecast.highTemperature),
        lowTemperature: wkTemperature(dayPartForecast.lowTemperature),
        precipitation: dayPartForecast.precipitation.rawValue,
        precipitationAmountByType: wkPayload(from: dayPartForecast.precipitationAmountByType),
        precipitationChance: dayPartForecast.precipitationChance,
        maximumHumidity: dayPartForecast.maximumHumidity,
        minimumHumidity: dayPartForecast.minimumHumidity,
        maximumVisibility: wkLength(dayPartForecast.maximumVisibility),
        minimumVisibility: wkLength(dayPartForecast.minimumVisibility),
        wind: wkPayload(from: dayPartForecast.wind),
        highWindSpeed: wkSpeed(dayPartForecast.highWindSpeed)
    )
}
