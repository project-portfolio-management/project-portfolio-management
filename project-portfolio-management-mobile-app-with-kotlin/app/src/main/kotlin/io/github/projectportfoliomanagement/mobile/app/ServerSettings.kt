// MA-1.1 and MA-2: the service's base URL and the sign-in settings. These are not secrets and are kept in plain
// SharedPreferences; the tokens are not (KeystoreTokenStore).
package io.github.projectportfoliomanagement.mobile.app

import android.content.Context
import androidx.core.content.edit
import io.github.projectportfoliomanagement.mobile.auth.OidcSettings

data class ServerSettings(
    val serviceUrl: String,
    val issuer: String = "",
    val clientId: String = "",
    val redirectUri: String = OidcSettings.DEFAULT_REDIRECT_URI,
) {
    /** The sign-in settings, or null when no issuer is set (a development service with enforcement off). */
    val oidc: OidcSettings? get() =
        if (issuer.isBlank() || clientId.isBlank()) null else OidcSettings(issuer.trim(), clientId.trim(), redirectUri.trim())

    companion object {
        /** The scheme the manifest's intent filter registers; a redirect URI with another scheme never comes back. */
        const val REDIRECT_SCHEME: String = "io.github.projectportfoliomanagement.mobile"
    }
}

interface SettingsStore {
    fun load(): ServerSettings?
    fun save(settings: ServerSettings)
}

class InMemorySettingsStore(private var settings: ServerSettings? = null) : SettingsStore {
    override fun load(): ServerSettings? = settings
    override fun save(settings: ServerSettings) { this.settings = settings }
}

class PreferencesSettingsStore(context: Context) : SettingsStore {
    private val prefs = context.getSharedPreferences("server_settings", Context.MODE_PRIVATE)

    override fun load(): ServerSettings? {
        val url = prefs.getString("service_url", null) ?: return null
        return ServerSettings(
            serviceUrl = url,
            issuer = prefs.getString("issuer", "").orEmpty(),
            clientId = prefs.getString("client_id", "").orEmpty(),
            redirectUri = prefs.getString("redirect_uri", null) ?: OidcSettings.DEFAULT_REDIRECT_URI,
        )
    }

    override fun save(settings: ServerSettings) {
        prefs.edit {
            putString("service_url", settings.serviceUrl)
            putString("issuer", settings.issuer)
            putString("client_id", settings.clientId)
            putString("redirect_uri", settings.redirectUri)
        }
    }
}
