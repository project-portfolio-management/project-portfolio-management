package io.github.projectportfoliomanagement.mobile.auth

import io.github.projectportfoliomanagement.mobile.api.HttpResponse
import io.github.projectportfoliomanagement.mobile.api.HttpTransport
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNull
import org.junit.Test

class SessionTest {
    private val issuer = "https://id.example.org"
    private val discovery = HttpResponse(
        200,
        """{"issuer":"$issuer","authorization_endpoint":"$issuer/a","token_endpoint":"$issuer/t"}""",
    )

    private fun client(vararg answers: HttpResponse): OidcClient {
        val queue = ArrayDeque(answers.toList())
        return OidcClient(OidcSettings(issuer, "c", OidcSettings.DEFAULT_REDIRECT_URI), HttpTransport { queue.removeFirst() }) { 1000 }
    }

    @Test fun aFreshTokenIsUsedAsIs() {
        val session = Session(InMemoryTokenStore(Tokens("a1", "r1", expiresAt = 2000)), { error("no refresh expected") }, { 1000 })
        assertEquals("a1", session.accessToken())
    }

    @Test fun aTokenNearExpiryIsRefreshedAndStored() {
        val store = InMemoryTokenStore(Tokens("a1", "r1", expiresAt = 1030))
        val oidc = client(discovery, HttpResponse(200, """{"access_token":"a2","expires_in":300}"""))
        val session = Session(store, { oidc }, { 1000 })
        assertEquals("a2", session.accessToken())
        assertEquals(Tokens("a2", "r1", null, 1300), store.load())
    }

    @Test fun aFailedRefreshSignsOut() {
        var signedOut = false
        val store = InMemoryTokenStore(Tokens("a1", "r1", expiresAt = 1000))
        val oidc = client(discovery, HttpResponse(400, """{"error":"invalid_grant"}"""))
        val session = Session(store, { oidc }, { 1000 }, onSignedOut = { signedOut = true })
        assertNull(session.accessToken())
        assertNull(store.load())
        assertEquals(true, signedOut)
        assertFalse(session.isSignedIn)
    }

    @Test fun signOutForgetsTheTokens() {
        val store = InMemoryTokenStore(Tokens("a1"))
        val session = Session(store, { null })
        assertEquals(Tokens("a1"), session.signOut())
        assertNull(store.load())
    }
}
