import Dispatch
import Foundation

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

    let timeoutResult = semaphore.wait(timeout: .now() + timeoutSeconds)
    if timeoutResult == .timedOut {
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
