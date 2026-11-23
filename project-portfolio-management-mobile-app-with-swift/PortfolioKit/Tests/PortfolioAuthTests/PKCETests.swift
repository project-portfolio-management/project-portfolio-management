// MA-6: the PKCE pieces with the RFC 7636 appendix B vector, and the `state` check.

import Foundation
import PortfolioAuth
import Testing

@Suite struct PKCETests {
    /// RFC 7636 appendix B: the 32 octets, the verifier they make, and its S256 challenge.
    static let octets: [UInt8] = [
        116, 24, 223, 180, 151, 153, 224, 37, 79, 250, 96, 125, 216, 173, 187, 186, 22, 212, 37, 77, 105, 214, 191, 240,
        91, 88, 5, 88, 83, 132, 141, 121,
    ]
    static let verifier = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk"
    static let challenge = "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM"

    @Test func theVerifierFromTheAppendixBOctets() {
        #expect(PKCE.verifier(from: Self.octets) == Self.verifier)
    }

    @Test func theS256ChallengeFromTheAppendixBVerifier() {
        #expect(PKCE.challenge(for: Self.verifier) == Self.challenge)
    }

    @Test func aRandomVerifierIsValidAndFresh() {
        let a = PKCE.randomVerifier()
        let b = PKCE.randomVerifier()
        #expect(a.count == 43)
        #expect(PKCE.isValidVerifier(a))
        #expect(a != b)
    }

    @Test func verifierRules() {
        #expect(PKCE.isValidVerifier(Self.verifier))
        #expect(!PKCE.isValidVerifier(String(repeating: "a", count: 42)))
        #expect(PKCE.isValidVerifier(String(repeating: "a", count: 128)))
        #expect(!PKCE.isValidVerifier(String(repeating: "a", count: 129)))
        #expect(!PKCE.isValidVerifier(String(repeating: "a", count: 42) + "+"))
    }

    @Test func theStateMustMatchExactly() {
        let state = OAuthState.random()
        #expect(state.count == 22)
        #expect(OAuthState.matches(expected: state, received: state))
        #expect(!OAuthState.matches(expected: state, received: nil))
        #expect(!OAuthState.matches(expected: state, received: ""))
        #expect(!OAuthState.matches(expected: state, received: state + "x"))
        #expect(!OAuthState.matches(expected: state, received: String(state.dropLast()) + "?"))
        #expect(!OAuthState.matches(expected: "", received: ""))
    }
}
