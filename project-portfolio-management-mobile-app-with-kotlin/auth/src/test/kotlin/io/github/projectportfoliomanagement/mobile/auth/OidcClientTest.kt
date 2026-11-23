package io.github.projectportfoliomanagement.mobile.auth

import io.github.projectportfoliomanagement.mobile.api.HttpRequest
import io.github.projectportfoliomanagement.mobile.api.HttpResponse
import io.github.projectportfoliomanagement.mobile.api.HttpTransport
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test
import java.io.IOException

class OidcClientTest {
    private val issuer = "https://id.example.org/realms/ppm"
    private val settings = OidcSettings(issuer, "ppm-mobile", OidcSettings.DEFAULT_REDIRECT_URI)
    private fun discovery(iss: String = issuer, endSession: Boolean = true) = HttpResponse(
        200,
        """{"issuer":"$iss","authorization_endpoint":"$issuer/auth","token_endpoint":"$issuer/token",
           ${if (endSession) "\"end_session_endpoint\":\"$issuer/logout\"," else ""}"jwks_uri":"$issuer/certs"}""",
    )

    private class Stub(vararg answers: Any) : HttpTransport {
        val requests = mutableListOf<HttpRequest>()
        private val queue = ArrayDeque(answers.toList())
        override fun execute(request: HttpRequest): HttpResponse {
            requests += request
            return when (val n = queue.removeFirst()) {
                is HttpResponse -> n
                is IOException -> throw n
                else -> error("bad answer")
            }
        }
    }

    private val verifier = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk"

    private fun begun(stub: Stub, client: OidcClient = OidcClient(settings, stub) { 1000 }): PendingAuthorization =
        (client.begin(verifier, "st4te") as AuthResult.Ok).value

    @Test fun beginBuildsTheAuthorizationUrlWithS256() {
        val stub = Stub(discovery())
        val pending = begun(stub)
        assertEquals("$issuer/.well-known/openid-configuration", stub.requests.single().url)
        assertEquals(
            "$issuer/auth?response_type=code&client_id=ppm-mobile" +
                "&redirect_uri=io.github.projectportfoliomanagement.mobile%3A%2Foauth2redirect" +
                "&scope=openid%20offline_access&state=st4te" +
                "&code_challenge=E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM&code_challenge_method=S256",
            pending.url,
        )
    }

    @Test fun theIssuerMustEqualTheOneSet() {
        val result = OidcClient(settings, Stub(discovery(iss = "https://evil.example.org"))).begin()
        assertEquals(AuthResult.Failed(AuthFailure.IssuerMismatch("https://evil.example.org")), result)
    }

    @Test fun anHttpIssuerIsRefusedUnlessItIsADevelopmentAddress() {
        val stub = Stub()
        assertEquals(AuthResult.Failed(AuthFailure.Insecure), OidcClient(settings.copy(issuer = "http://id.example.org"), stub).discover())
        assertTrue(stub.requests.isEmpty())
    }

    @Test fun completeChecksStateThenExchangesTheCodeWithTheVerifier() {
        val stub = Stub(
            discovery(),
            HttpResponse(200, """{"access_token":"a1","refresh_token":"r1","id_token":"i1","expires_in":300,"token_type":"Bearer"}"""),
        )
        val client = OidcClient(settings, stub) { 1000 }
        val pending = begun(stub, client)
        val tokens = client.complete(pending, "${OidcSettings.DEFAULT_REDIRECT_URI}?code=c%2F1&state=st4te")
        assertEquals(AuthResult.Ok(Tokens("a1", "r1", "i1", 1300)), tokens)
        val exchange = stub.requests.last()
        assertEquals("POST", exchange.method)
        assertEquals("$issuer/token", exchange.url)
        assertEquals("application/x-www-form-urlencoded", exchange.headers["Content-Type"])
        assertEquals(
            "grant_type=authorization_code&code=c%2F1" +
                "&redirect_uri=io.github.projectportfoliomanagement.mobile%3A%2Foauth2redirect" +
                "&client_id=ppm-mobile&code_verifier=$verifier",
            exchange.body,
        )
    }

    @Test fun aWrongOrMissingStateIsRefusedBeforeAnyExchange() {
        val stub = Stub(discovery())
        val client = OidcClient(settings, stub)
        val pending = begun(stub, client)
        assertEquals(AuthResult.Failed(AuthFailure.StateMismatch), client.complete(pending, "x:/cb?code=c&state=other"))
        assertEquals(AuthResult.Failed(AuthFailure.StateMismatch), client.complete(pending, "x:/cb?code=c"))
        assertEquals(1, stub.requests.size)
    }

    @Test fun aProviderErrorIsReported() {
        val stub = Stub(discovery())
        val client = OidcClient(settings, stub)
        val pending = begun(stub, client)
        assertEquals(
            AuthResult.Failed(AuthFailure.Provider("access_denied", "The user cancelled")),
            client.complete(pending, "x:/cb?error=access_denied&error_description=The+user%20cancelled&state=st4te"),
        )
    }

    @Test fun refreshKeepsTheOldRefreshTokenWhenNoneIsSent() {
        val stub = Stub(discovery(), HttpResponse(200, """{"access_token":"a2","expires_in":60}"""))
        val result = OidcClient(settings, stub) { 2000 }.refresh(Tokens("a1", "r1", "i1", 1300))
        assertEquals(AuthResult.Ok(Tokens("a2", "r1", "i1", 2060)), result)
        assertEquals("grant_type=refresh_token&refresh_token=r1&client_id=ppm-mobile", stub.requests.last().body)
    }

    @Test fun endSessionUrlNeedsTheEndpoint() {
        assertEquals(
            "$issuer/logout?id_token_hint=i1&client_id=ppm-mobile" +
                "&post_logout_redirect_uri=io.github.projectportfoliomanagement.mobile%3A%2Foauth2redirect",
            OidcClient(settings, Stub(discovery())).endSessionUrl(Tokens("a", idToken = "i1")),
        )
        assertNull(OidcClient(settings, Stub(discovery(endSession = false))).endSessionUrl(Tokens("a")))
    }
}
