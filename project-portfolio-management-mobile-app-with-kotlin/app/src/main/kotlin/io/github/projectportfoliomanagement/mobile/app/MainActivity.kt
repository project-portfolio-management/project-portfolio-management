// The one activity: edge to edge, light or dark with the system, and the target of the sign-in redirect.
package io.github.projectportfoliomanagement.mobile.app

import android.content.Intent
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import io.github.projectportfoliomanagement.mobile.app.ui.PpmApp
import io.github.projectportfoliomanagement.mobile.app.ui.PpmTheme

class MainActivity : ComponentActivity() {
    private val graph get() = (application as PpmApplication).graph

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        if (savedInstanceState == null) receive(intent)
        setContent { PpmTheme { PpmApp(graph) } }
    }

    override fun onNewIntent(intent: Intent) {
        super.onNewIntent(intent)
        receive(intent)
    }

    private fun receive(intent: Intent?) {
        val data = intent?.data ?: return
        if (intent.action == Intent.ACTION_VIEW && data.scheme == ServerSettings.REDIRECT_SCHEME) graph.redirect = data.toString()
    }
}
