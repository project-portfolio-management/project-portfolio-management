// MA-2: sign-in and sign-out pages open in the system browser as a Custom Tab (RFC 8252), never in a web view.
package io.github.projectportfoliomanagement.mobile.app

import android.content.Context
import android.content.Intent
import androidx.browser.customtabs.CustomTabsIntent
import androidx.core.net.toUri

fun interface Browser {
    fun open(url: String)
}

class CustomTabsBrowser(private val context: Context) : Browser {
    override fun open(url: String) {
        val tab = CustomTabsIntent.Builder().setShowTitle(true).build()
        // Opened from the application context, so the tab needs a task of its own; the redirect returns to MainActivity.
        tab.intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
        tab.launchUrl(context, url.toUri())
    }
}
