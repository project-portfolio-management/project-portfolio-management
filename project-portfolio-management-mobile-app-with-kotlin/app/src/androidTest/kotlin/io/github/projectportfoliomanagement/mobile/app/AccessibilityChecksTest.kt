// MA-4 and MA-6: Google's Accessibility Test Framework over every screen of the first slice, on a device or emulator
// (API 34 or later), against the stub service. enableAccessibilityChecks() runs ATF before each action, and
// tryPerformAccessibilityChecks() checks each screen as shown; checkScreenSemantics() adds the app's own checks (no text cut
// off, 48 dp targets, a name on every control). Run it at font scale 1.0 and 2.0:
//   adb shell settings put system font_scale 2.0 && ./gradlew :app:connectedDebugAndroidTest
// ATF finds nothing under Robolectric, which is why this is an instrumented test.
package io.github.projectportfoliomanagement.mobile.app

import androidx.compose.ui.test.hasAnyAncestor
import androidx.compose.ui.test.hasClickAction
import androidx.compose.ui.test.hasContentDescription
import androidx.compose.ui.test.hasSetTextAction
import androidx.compose.ui.test.hasText
import androidx.compose.ui.test.isDialog
import androidx.compose.ui.test.isRoot
import androidx.compose.ui.test.junit4.accessibility.enableAccessibilityChecks
import androidx.compose.ui.test.junit4.createComposeRule
import androidx.compose.ui.test.onNodeWithContentDescription
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.performScrollTo
import androidx.compose.ui.test.performTextInput
import androidx.compose.ui.test.tryPerformAccessibilityChecks
import androidx.test.ext.junit.runners.AndroidJUnit4
import io.github.projectportfoliomanagement.mobile.app.ui.PpmApp
import io.github.projectportfoliomanagement.mobile.app.ui.PpmTheme
import io.github.projectportfoliomanagement.mobile.auth.InMemoryTokenStore
import io.github.projectportfoliomanagement.mobile.auth.Tokens
import kotlinx.coroutines.Dispatchers
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class AccessibilityChecksTest {
    @get:Rule val rule = createComposeRule()

    private val base = "https://ppm.example.org"
    private val service = RouteTransport().apply {
        on("GET", "$base/api/plans", 200, """[{"pid":"p1","name":"Alpha programme for the northern region"},{"pid":"p2","name":"Beta"}]""")
        on(
            "GET", "$base/api/plans/p1", 200,
            """{"name":"Alpha programme for the northern region","kind":"Program","code":"A-1","status":{"Custom":"Paused for audit"},
               "start_date":"2027-01-04","target_date":"2027-06-30","owner_org_name":"Operations"}""",
        )
        on("GET", "$base/api/plans?parent=p1", 200, """[{"pid":"c1","name":"Alpha child"}]""")
        on(
            "GET", "$base/api/plans/p1/tasks", 200,
            """[{"pid":"t1","title":"Agree the funding letter","status":"blocked","points":3,"due_date":"2027-02-01","blocked_days":4},
               {"pid":"t2","title":"Write it","status":"todo"}]""",
        )
        on("PATCH", "$base/api/plans/p1/tasks/t2", 403, """{"status":403,"detail":"rule writers: access=write"}""")
        on("POST", "$base/api/plans", 422, """{"status":422,"detail":"code: exceeds 4096 characters"}""")
    }

    @Before fun enable() {
        rule.enableAccessibilityChecks()
    }

    private fun start(signedIn: Boolean) {
        val graph = AppGraph(
            InMemorySettingsStore(ServerSettings("http://10.0.2.2:5150".takeIf { !signedIn } ?: base, "https://id.example.org", "c")),
            InMemoryTokenStore(if (signedIn) Tokens("t") else null),
            service, { }, io = Dispatchers.Unconfined,
        )
        rule.setContent { PpmTheme { PpmApp(graph) } }
        rule.waitForIdle()
    }

    private fun checkAll() {
        rule.onAllNodes(isRoot()).fetchSemanticsNodes().indices.forEach { rule.onAllNodes(isRoot())[it].tryPerformAccessibilityChecks() }
        rule.checkScreenSemantics()
    }

    @Test fun thePlansThePlanAMoveAndTheFormPassTheChecks() {
        start(signedIn = true)
        checkAll()
        rule.onNodeWithText("Alpha programme for the northern region").performClick()
        rule.waitForIdle()
        checkAll()
        rule.onNodeWithContentDescription("Move Write it").performScrollTo().performClick()
        rule.waitForIdle()
        checkAll()
        rule.onNodeWithText("Done").performClick()
        rule.onNode(hasText("Move") and hasClickAction() and hasAnyAncestor(isDialog())).performClick()
        rule.waitForIdle()
        checkAll()
        rule.onNodeWithContentDescription("Back").performClick()
        rule.waitForIdle()
        rule.onNode(hasText("New plan") and hasClickAction()).performClick()
        rule.waitForIdle()
        checkAll()
        rule.onNode(hasSetTextAction() and hasContentDescription("Name", substring = true)).performTextInput("Gamma")
        rule.onNodeWithText("Create plan").performScrollTo().performClick()
        rule.waitForIdle()
        checkAll()
    }

    @Test fun theSignInScreenPassesTheChecks() {
        start(signedIn = false)
        checkAll()
        rule.onNodeWithText("Change server").performClick()
        rule.waitForIdle()
        checkAll()
    }
}
