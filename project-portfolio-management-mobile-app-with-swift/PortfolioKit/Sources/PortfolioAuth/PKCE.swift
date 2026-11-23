// PKCE (RFC 7636) with the S256 method, and the `state` parameter (spec/mobile-apps.md MA-2). Tested with the RFC 7636
// appendix B vector.

import CryptoKit
import Foundation
import Security

public enum Base64URL {
    /// Base64url without padding (RFC 7636 appendix A).
    public static func encode(_ bytes: some Sequence<UInt8>) -> String {
        Data(bytes).base64EncodedString()
            .replacingOccurrences(of: "+", with: "-")
            .replacingOccurrences(of: "/", with: "_")
            .replacingOccurrences(of: "=", with: "")
    }
}

public enum PKCE {
    /// The verifier for 32 random octets: 43 characters of base64url, as appendix B makes it.
    public static func verifier(from octets: [UInt8]) -> String {
        Base64URL.encode(octets)
    }

    /// A fresh verifier from 32 octets of the system's secure random source.
    public static func randomVerifier() -> String {
        verifier(from: randomOctets(32))
    }

    /// `BASE64URL(SHA256(ASCII(code_verifier)))`.
    public static func challenge(for verifier: String) -> String {
        Base64URL.encode(SHA256.hash(data: Data(verifier.utf8)))
    }

    /// Whether `verifier` is one RFC 7636 §4.1 allows: 43 to 128 unreserved characters.
    public static func isValidVerifier(_ verifier: String) -> Bool {
        let allowed = Set("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~")
        return (43...128).contains(verifier.count) && verifier.allSatisfy(allowed.contains)
    }

    static func randomOctets(_ count: Int) -> [UInt8] {
        var octets = [UInt8](repeating: 0, count: count)
        let status = SecRandomCopyBytes(kSecRandomDefault, count, &octets)
        precondition(status == errSecSuccess, "the system's secure random source failed")
        return octets
    }
}

public enum OAuthState {
    /// A fresh `state`: 16 random octets, base64url.
    public static func random() -> String {
        Base64URL.encode(PKCE.randomOctets(16))
    }

    /// Whether the `state` that came back is the one sent, compared in constant time.
    public static func matches(expected: String, received: String?) -> Bool {
        guard let received, !expected.isEmpty else { return false }
        let a = Array(expected.utf8)
        let b = Array(received.utf8)
        guard a.count == b.count else { return false }
        var difference: UInt8 = 0
        for (x, y) in zip(a, b) { difference |= x ^ y }
        return difference == 0
    }
}
