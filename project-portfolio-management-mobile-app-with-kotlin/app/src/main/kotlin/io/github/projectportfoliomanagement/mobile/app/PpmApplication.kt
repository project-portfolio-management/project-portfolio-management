package io.github.projectportfoliomanagement.mobile.app

import android.app.Application
import io.github.projectportfoliomanagement.mobile.api.OkHttpTransport

class PpmApplication : Application() {
    val graph: AppGraph by lazy {
        AppGraph(
            settingsStore = PreferencesSettingsStore(this),
            tokenStore = KeystoreTokenStore(this),
            transport = OkHttpTransport(),
            browser = CustomTabsBrowser(this),
        )
    }
}
