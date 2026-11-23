// MA-4: at twice the default text size, the plan screen and the new-plan form cut nothing off and keep their targets.
package io.github.projectportfoliomanagement.mobile.app

import androidx.compose.ui.test.hasClickAction
import androidx.compose.ui.test.hasText
import androidx.compose.ui.test.junit4.createComposeRule
import androidx.compose.ui.test.onNodeWithContentDescription
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import io.github.projectportfoliomanagement.mobile.app.ui.PpmApp
import io.github.projectportfoliomanagement.mobile.app.ui.PpmTheme
import io.github.projectportfoliomanagement.mobile.auth.InMemoryTokenStore
import io.github.projectportfoliomanagement.mobile.auth.Tokens
import kotlinx.coroutines.Dispatchers
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner
import org.robolectric.annotation.Config
import org.robolectric.annotation.GraphicsMode

@RunWith(RobolectricTestRunner::class)
@Config(sdk = [36], fontScale = 2.0f)
@GraphicsMode(GraphicsMode.Mode.NATIVE)
class LargeTextTest {
    @get:Rule val rule = createComposeRule()

    @Test fun thePlanScreenAndTheFormFitAtTwiceTheTextSize() {
        val base = "https://ppm.example.org"
        val service = RouteTransport().apply {
            on("GET", "$base/api/plans", 200, """[{"pid":"p1","name":"Alpha programme for the northern region"}]""")
            on("GET", "$base/api/plans/p1", 200, """{"name":"Alpha programme for the northern region","status":"OnHold"}""")
            on("GET", "$base/api/plans?parent=p1", 200, "[]")
            on("GET", "$base/api/plans/p1/tasks", 200, """[{"pid":"t1","title":"Agree the funding letter","status":"in_review"}]""")
        }
        val graph = AppGraph(
            InMemorySettingsStore(ServerSettings(base, "https://id.example.org", "c")),
            InMemoryTokenStore(Tokens("t")), service, { }, io = Dispatchers.Unconfined,
        )
        rule.setContent { PpmTheme { PpmApp(graph) } }
        rule.waitForIdle()
        rule.checkScreenSemantics()
        rule.onNodeWithText("Alpha programme for the northern region").performClick()
        rule.waitForIdle()
        rule.checkScreenSemantics()
        rule.onNodeWithContentDescription("Back").performClick()
        rule.waitForIdle()
        rule.onNode(hasText("New plan") and hasClickAction()).performClick()
        rule.waitForIdle()
        rule.checkScreenSemantics()
    }
}
