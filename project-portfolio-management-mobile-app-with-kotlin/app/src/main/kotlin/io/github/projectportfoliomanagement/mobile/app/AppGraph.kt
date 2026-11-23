// What the screens share: the settings, the session, the transport and the browser. The app builds one in PpmApplication;
// a test builds one with a stub transport and in-memory stores.
package io.github.projectportfoliomanagement.mobile.app

import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import io.github.projectportfoliomanagement.mobile.api.Addresses
import io.github.projectportfoliomanagement.mobile.api.ApiClient
import io.github.projectportfoliomanagement.mobile.api.HttpTransport
import io.github.projectportfoliomanagement.mobile.auth.OidcClient
import io.github.projectportfoliomanagement.mobile.auth.PendingAuthorization
import io.github.projectportfoliomanagement.mobile.auth.Session
import io.github.projectportfoliomanagement.mobile.auth.TokenStore
import kotlinx.coroutines.Dispatchers
import kotlin.coroutines.CoroutineContext

class AppGraph(
    private val settingsStore: SettingsStore,
    tokenStore: TokenStore,
    val transport: HttpTransport,
    val browser: Browser,
    /** Where calls run; a test passes Dispatchers.Unconfined to run them in step. */
    val io: CoroutineContext = Dispatchers.IO,
) {
    var settings: ServerSettings? by mutableStateOf(settingsStore.load())
        private set

    var signedIn: Boolean by mutableStateOf(tokenStore.load() != null)
        private set

    /** "Continue without signing in" was chosen (development addresses only). Kept in memory only. */
    var withoutSignIn: Boolean by mutableStateOf(false)
        private set

    /** The service answered 401, or a refresh failed: the sign-in screen says the session ended. */
    var sessionEnded: Boolean by mutableStateOf(false)
        private set

    /** The last redirect the activity received; the sign-in screen completes it. */
    var redirect: String? by mutableStateOf(null)

    /** The sign-in under way. Kept in memory only, so a process killed while the browser is open starts again. */
    @Volatile var pending: PendingAuthorization? = null

    private var oidcClient: OidcClient? = null

    val session = Session(tokenStore, { oidc() }, onSignedOut = { signedIn = false })

    /** The OIDC client for the current settings, or null when no issuer is set. */
    @Synchronized
    fun oidc(): OidcClient? {
        val wanted = settings?.oidc ?: return null
        oidcClient?.takeIf { it.settings == wanted }?.let { return it }
        return OidcClient(wanted, transport).also { oidcClient = it }
    }

    fun api(): ApiClient = ApiClient(settings?.serviceUrl.orEmpty(), transport, session)

    /** MA-2: "Continue without signing in" is offered only for a development address. */
    val mayContinueWithoutSignIn: Boolean get() = settings?.let { Addresses.isDevelopment(it.serviceUrl) } == true

    /** Saves new settings. A change of service, issuer or client forgets the tokens: they belong to the old one. */
    fun saveSettings(new: ServerSettings) {
        val old = settings
        if (old == null || old.serviceUrl != new.serviceUrl || old.issuer != new.issuer || old.clientId != new.clientId) {
            session.signOut()
            withoutSignIn = false
        }
        settingsStore.save(new)
        settings = new
    }

    fun signedIn(tokens: io.github.projectportfoliomanagement.mobile.auth.Tokens) {
        session.signedIn(tokens)
        sessionEnded = false
        signedIn = true
    }

    fun continueWithoutSignIn() {
        sessionEnded = false
        withoutSignIn = true
    }

    /** MA-3 401: forget the tokens and ask the person to sign in again. */
    fun sessionExpired() {
        session.signOut()
        withoutSignIn = false
        sessionEnded = true
    }

    /** Sign out (MA-2): forgets the tokens and returns the provider's end-session URL, when it has one. Does network I/O. */
    fun signOut(): String? {
        val tokens = session.signOut()
        withoutSignIn = false
        sessionEnded = false
        return runCatching { oidc()?.endSessionUrl(tokens) }.getOrNull()
    }
}
