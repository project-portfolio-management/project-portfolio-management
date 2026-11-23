// Which addresses the app accepts (spec/mobile-apps.md MA-2): HTTPS for the service and the issuer, except a development
// address. On iOS a development address is a loopback one (`localhost`, `127.0.0.0/8`, `::1`); the Android emulator's
// host `10.0.2.2` is not special on an iPhone or the iOS Simulator, which reach the Mac's services at 127.0.0.1.

import Foundation

public enum ServerAddress {
    public enum Problem: Error, Sendable, Hashable {
        /// Not an absolute `http` or `https` URL with a host.
        case notAURL
        /// `http` to an address that is not a development one.
        case needsHTTPS
    }

    /// Parses an address a person typed, trimmed, and checks it.
    public static func parse(_ text: String) -> Result<URL, Problem> {
        let trimmed = text.trimmingCharacters(in: .whitespacesAndNewlines)
        guard let url = URL(string: trimmed), let scheme = url.scheme?.lowercased(),
            scheme == "https" || scheme == "http", let host = url.host(), !host.isEmpty
        else { return .failure(.notAURL) }
        if scheme == "http" && !isDevelopment(url) { return .failure(.needsHTTPS) }
        return .success(url)
    }

    /// Whether `url` is a development address: one where plain HTTP and **Continue without signing in** are allowed.
    public static func isDevelopment(_ url: URL) -> Bool {
        guard let host = url.host()?.lowercased() else { return false }
        let bare = host.trimmingCharacters(in: CharacterSet(charactersIn: "[]"))
        if bare == "localhost" || bare.hasSuffix(".localhost") || bare == "::1" { return true }
        let octets = bare.split(separator: ".", omittingEmptySubsequences: false)
        return octets.count == 4 && octets[0] == "127" && octets.allSatisfy { UInt8($0) != nil }
    }

    /// Whether `url` may be used for the service or the issuer: HTTPS, or a development address.
    public static func isAllowed(_ url: URL) -> Bool {
        switch url.scheme?.lowercased() {
        case "https": return url.host() != nil
        case "http": return isDevelopment(url)
        default: return false
        }
    }
}
