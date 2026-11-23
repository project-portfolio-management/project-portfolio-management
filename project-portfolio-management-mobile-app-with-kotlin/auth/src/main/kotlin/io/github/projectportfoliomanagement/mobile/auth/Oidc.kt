// The authorization-code flow with PKCE against any OIDC provider (MA-2): discovery, the authorization URL, the redirect,
// the code exchange, refresh and the end-session URL. All over [HttpTransport], so it is tested without a provider.
package io.github.projectportfoliomanagement.mobile.auth

import io.github.projectportfoliomanagement.mobile.api.Addresses
import io.github.projectportfoliomanagement.mobile.api.HttpRequest
import io.github.projectportfoliomanagement.mobile.api.HttpTransport
import io.github.projectportfoliomanagement.mobile.api.WireJson
import io.github.projectportfoliomanagement.mobile.api.percentEncode
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import java.io.IOException
import java.net.URI

/** The sign-in settings (MA-2). */
public data class OidcSettings(val issuer: String, val clientId: String, val redirectUri: String) {
    public companion object {
        public const val DEFAULT_REDIRECT_URI: String = "io.github.projectportfoliomanagement.mobile:/oauth2redirect"
        public const val SCOPE: String = "openid offline_access"
    }
}

/** The parts of `/.well-known/openid-configuration` the app uses. */
@Serializable
public data class Discovery(
    val issuer: String,
    @SerialName("authorization_endpoint") val authorizationEndpoint: String,
    @SerialName("token_endpoint") val tokenEndpoint: String,
    @SerialName("end_session_endpoint") val endSessionEndpoint: String? = null,
)

/** The tokens kept in the protected store. [expiresAt] is in epoch seconds, or null when the provider gave no lifetime. */
@Serializable
public data class Tokens(
    val accessToken: String,
    val refreshToken: String? = null,
    val idToken: String? = null,
    val expiresAt: Long? = null,
)

@Serializable
internal data class TokenAnswer(
    @SerialName("access_token") val accessToken: String,
    @SerialName("refresh_token") val refreshToken: String? = null,
    @SerialName("id_token") val idToken: String? = null,
    @SerialName("expires_in") val expiresIn: Long? = null,
)

/** Why sign-in or refresh failed. The words are the app's. */
public sealed interface AuthFailure {
    /** The issuer or an endpoint is not HTTPS (and not a development address). */
    public data object Insecure : AuthFailure
    /** The discovery document's `issuer` is not the one set. */
    public data class IssuerMismatch(val found: String) : AuthFailure
    /** The redirect's `state` is missing or not the one sent. */
    public data object StateMismatch : AuthFailure
    /** The provider answered the redirect with `error` (the person cancelled, or the client is not allowed). */
    public data class Provider(val error: String, val description: String?) : AuthFailure
    /** The redirect carried neither a code nor an error. */
    public data object NoCode : AuthFailure
    /** The provider could not be reached, or answered with something the app cannot read. */
    public data class Unreachable(val status: Int?) : AuthFailure
}

public sealed interface AuthResult<out T> {
    public data class Ok<T>(val value: T) : AuthResult<T>
    public data class Failed(val failure: AuthFailure) : AuthResult<Nothing>
}

/** A started sign-in: the URL to open in the browser, and what the redirect is checked against. Kept in memory only. */
public data class PendingAuthorization(
    val url: String,
    val state: String,
    val verifier: String,
    val discovery: Discovery,
)

public class OidcClient(
    public val settings: OidcSettings,
    private val transport: HttpTransport,
    private val now: () -> Long = { System.currentTimeMillis() / 1000 },
) {
    private var discovered: Discovery? = null

    /** Fetches and checks the discovery document, once per client. */
    public fun discover(): AuthResult<Discovery> {
        discovered?.let { return AuthResult.Ok(it) }
        if (!Addresses.isAllowed(settings.issuer)) return AuthResult.Failed(AuthFailure.Insecure)
        val url = Addresses.normalise(settings.issuer) + "/.well-known/openid-configuration"
        val response = try {
            transport.execute(HttpRequest("GET", url, mapOf("Accept" to "application/json")))
        } catch (_: IOException) {
            return AuthResult.Failed(AuthFailure.Unreachable(null))
        }
        if (response.status != 200) return AuthResult.Failed(AuthFailure.Unreachable(response.status))
        val doc = runCatching { WireJson.decodeFromString(Discovery.serializer(), response.body) }.getOrNull()
            ?: return AuthResult.Failed(AuthFailure.Unreachable(response.status))
        if (doc.issuer != settings.issuer.trim()) return AuthResult.Failed(AuthFailure.IssuerMismatch(doc.issuer))
        val endpoints = listOfNotNull(doc.authorizationEndpoint, doc.tokenEndpoint, doc.endSessionEndpoint)
        if (!endpoints.all(Addresses::isAllowed)) return AuthResult.Failed(AuthFailure.Insecure)
        discovered = doc
        return AuthResult.Ok(doc)
    }

    /** Starts a sign-in: a new verifier and state, and the authorization URL to open in the system browser. */
    public fun begin(verifier: String = Pkce.newVerifier(), state: String = Pkce.newState()): AuthResult<PendingAuthorization> =
        when (val d = discover()) {
            is AuthResult.Failed -> d
            is AuthResult.Ok -> {
                val query = listOf(
                    "response_type" to "code",
                    "client_id" to settings.clientId,
                    "redirect_uri" to settings.redirectUri,
                    "scope" to OidcSettings.SCOPE,
                    "state" to state,
                    "code_challenge" to Pkce.challenge(verifier),
                    "code_challenge_method" to Pkce.METHOD,
                )
                AuthResult.Ok(PendingAuthorization(withQuery(d.value.authorizationEndpoint, query), state, verifier, d.value))
            }
        }

    /** Checks the redirect against [pending] and exchanges its code for tokens. */
    public fun complete(pending: PendingAuthorization, redirect: String): AuthResult<Tokens> {
        val params = queryOf(redirect)
        if (!Pkce.stateMatches(pending.state, params["state"])) return AuthResult.Failed(AuthFailure.StateMismatch)
        params["error"]?.let { return AuthResult.Failed(AuthFailure.Provider(it, params["error_description"])) }
        val code = params["code"] ?: return AuthResult.Failed(AuthFailure.NoCode)
        return tokenRequest(
            pending.discovery.tokenEndpoint,
            listOf(
                "grant_type" to "authorization_code",
                "code" to code,
                "redirect_uri" to settings.redirectUri,
                "client_id" to settings.clientId,
                "code_verifier" to pending.verifier,
            ),
            previous = null,
        )
    }

    /** Exchanges a refresh token for new tokens; a provider that sends no new refresh token keeps the old one. */
    public fun refresh(tokens: Tokens): AuthResult<Tokens> {
        val refreshToken = tokens.refreshToken ?: return AuthResult.Failed(AuthFailure.NoCode)
        return when (val d = discover()) {
            is AuthResult.Failed -> d
            is AuthResult.Ok -> tokenRequest(
                d.value.tokenEndpoint,
                listOf("grant_type" to "refresh_token", "refresh_token" to refreshToken, "client_id" to settings.clientId),
                previous = tokens,
            )
        }
    }

    /** The provider's end-session URL for [tokens], or null when the provider has no `end_session_endpoint`. */
    public fun endSessionUrl(tokens: Tokens?): String? {
        val endpoint = (discover() as? AuthResult.Ok)?.value?.endSessionEndpoint ?: return null
        val query = listOfNotNull(
            tokens?.idToken?.let { "id_token_hint" to it },
            "client_id" to settings.clientId,
            "post_logout_redirect_uri" to settings.redirectUri,
        )
        return withQuery(endpoint, query)
    }

    private fun tokenRequest(endpoint: String, form: List<Pair<String, String>>, previous: Tokens?): AuthResult<Tokens> {
        val request = HttpRequest(
            "POST",
            endpoint,
            mapOf("Accept" to "application/json", "Content-Type" to "application/x-www-form-urlencoded"),
            form.joinToString("&") { (k, v) -> "${percentEncode(k)}=${percentEncode(v)}" },
        )
        val response = try {
            transport.execute(request)
        } catch (_: IOException) {
            return AuthResult.Failed(AuthFailure.Unreachable(null))
        }
        if (response.status != 200) return AuthResult.Failed(AuthFailure.Unreachable(response.status))
        val answer = runCatching { WireJson.decodeFromString(TokenAnswer.serializer(), response.body) }.getOrNull()
            ?: return AuthResult.Failed(AuthFailure.Unreachable(response.status))
        return AuthResult.Ok(
            Tokens(
                accessToken = answer.accessToken,
                refreshToken = answer.refreshToken ?: previous?.refreshToken,
                idToken = answer.idToken ?: previous?.idToken,
                expiresAt = answer.expiresIn?.let { now() + it },
            ),
        )
    }

    internal companion object {
        fun withQuery(endpoint: String, query: List<Pair<String, String>>): String {
            val joiner = if ('?' in endpoint) '&' else '?'
            return endpoint + joiner + query.joinToString("&") { (k, v) -> "${percentEncode(k)}=${percentEncode(v)}" }
        }

        fun queryOf(uri: String): Map<String, String> {
            val raw = runCatching { URI(uri).rawQuery }.getOrNull() ?: return emptyMap()
            return raw.split('&').filter { it.isNotEmpty() }.associate { part ->
                val key = part.substringBefore('=')
                val value = part.substringAfter('=', "")
                decode(key) to decode(value)
            }
        }

        private fun decode(s: String): String = java.net.URLDecoder.decode(s, Charsets.UTF_8)
    }
}
