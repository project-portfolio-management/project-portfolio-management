// The signed-in session: the tokens in the protected store, refreshed near expiry, forgotten on sign-out (MA-2).
package io.github.projectportfoliomanagement.mobile.auth

import io.github.projectportfoliomanagement.mobile.api.AccessTokenSource

/** Where the tokens are kept. On Android, values encrypted with an Android Keystore key (`:app` KeystoreTokenStore). */
public interface TokenStore {
    public fun load(): Tokens?
    public fun save(tokens: Tokens)
    public fun clear()
}

/** A store that keeps nothing on disk; for tests and for a service with enforcement off. */
public class InMemoryTokenStore(private var tokens: Tokens? = null) : TokenStore {
    override fun load(): Tokens? = tokens
    override fun save(tokens: Tokens) { this.tokens = tokens }
    override fun clear() { tokens = null }
}

/**
 * Gives the access token for each request. A token within [refreshMarginSeconds] of expiry is refreshed first; when the
 * refresh fails, or there is no refresh token, the tokens are forgotten and [onSignedOut] runs.
 */
public class Session(
    private val store: TokenStore,
    private val client: () -> OidcClient?,
    private val now: () -> Long = { System.currentTimeMillis() / 1000 },
    private val refreshMarginSeconds: Long = 60,
    private val onSignedOut: () -> Unit = {},
) : AccessTokenSource {
    @Synchronized
    override fun accessToken(): String? {
        val tokens = store.load() ?: return null
        val expiresAt = tokens.expiresAt ?: return tokens.accessToken
        if (expiresAt - now() > refreshMarginSeconds) return tokens.accessToken
        val refreshed = client()?.refresh(tokens)
        if (refreshed is AuthResult.Ok) {
            store.save(refreshed.value)
            return refreshed.value.accessToken
        }
        signOut()
        return null
    }

    public val isSignedIn: Boolean @Synchronized get() = store.load() != null

    @Synchronized
    public fun signedIn(tokens: Tokens) {
        store.save(tokens)
    }

    /** Forgets the tokens; returns the ones forgotten (for the end-session call). */
    @Synchronized
    public fun signOut(): Tokens? {
        val tokens = store.load()
        store.clear()
        onSignedOut()
        return tokens
    }
}
