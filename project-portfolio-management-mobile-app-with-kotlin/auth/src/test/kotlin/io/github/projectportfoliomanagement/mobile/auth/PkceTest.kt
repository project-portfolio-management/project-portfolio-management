// MA-6: the PKCE pieces against RFC 7636 appendix B, and the state check.
package io.github.projectportfoliomanagement.mobile.auth

import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNotEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class PkceTest {
    private val appendixBOctets = intArrayOf(
        116, 24, 223, 180, 151, 153, 224, 37, 79, 250, 96, 125, 216, 173, 187, 186,
        22, 212, 37, 77, 105, 214, 191, 240, 91, 88, 5, 88, 83, 132, 141, 121,
    ).map { it.toByte() }.toByteArray()

    @Test fun theVerifierOfAppendixB() {
        assertEquals("dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk", Pkce.verifierFrom(appendixBOctets))
    }

    @Test fun theS256ChallengeOfAppendixB() {
        assertEquals("E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM", Pkce.challenge("dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk"))
        assertEquals("S256", Pkce.METHOD)
    }

    @Test fun aNewVerifierIs43UnreservedCharacters() {
        val v = Pkce.newVerifier()
        assertEquals(43, v.length)
        assertTrue(v.all { it.isLetterOrDigit() || it == '-' || it == '_' })
        assertNotEquals(v, Pkce.newVerifier())
    }

    @Test fun stateMustMatchExactly() {
        val s = Pkce.newState()
        assertTrue(Pkce.stateMatches(s, s))
        assertFalse(Pkce.stateMatches(s, s + "x"))
        assertFalse(Pkce.stateMatches(s, null))
        assertFalse(Pkce.stateMatches(s, ""))
    }
}
