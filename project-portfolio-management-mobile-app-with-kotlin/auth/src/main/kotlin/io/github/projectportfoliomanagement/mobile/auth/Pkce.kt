// PKCE (RFC 7636) with S256, and the `state` value (RFC 6749 §10.12), for a public client (MA-2).
package io.github.projectportfoliomanagement.mobile.auth

import java.security.MessageDigest
import java.security.SecureRandom
import java.util.Base64

public object Pkce {
    private val encoder = Base64.getUrlEncoder().withoutPadding()

    /** A new `code_verifier`: 32 random octets, base64url without padding (43 characters, RFC 7636 §4.1). */
    public fun newVerifier(random: SecureRandom = SecureRandom()): String =
        verifierFrom(ByteArray(32).also(random::nextBytes))

    /** The verifier for [octets] (RFC 7636 appendix B gives one such pair). */
    public fun verifierFrom(octets: ByteArray): String = encoder.encodeToString(octets)

    /** `code_challenge` = BASE64URL(SHA256(ASCII(verifier))), method `S256`. */
    public fun challenge(verifier: String): String =
        encoder.encodeToString(MessageDigest.getInstance("SHA-256").digest(verifier.toByteArray(Charsets.US_ASCII)))

    public const val METHOD: String = "S256"

    /** A new `state`: 16 random octets, base64url. */
    public fun newState(random: SecureRandom = SecureRandom()): String =
        encoder.encodeToString(ByteArray(16).also(random::nextBytes))

    /** True only when [received] equals [expected]; compared in constant time. A missing value never matches. */
    public fun stateMatches(expected: String, received: String?): Boolean =
        received != null && MessageDigest.isEqual(expected.toByteArray(), received.toByteArray())
}
