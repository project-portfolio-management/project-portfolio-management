// MA-6: the plans list, the plan screen, a move and a create against a stub service, each screen checked for MA-4 by
// checkScreenSemantics (tap targets, names, clipped text). Robolectric runs them on the JVM. The Accessibility Test
// Framework finds nothing under Robolectric, so it runs on a device instead: src/androidTest/AccessibilityChecksTest.kt.
package io.github.projectportfoliomanagement.mobile.app

import androidx.compose.ui.test.SemanticsNodeInteraction
import androidx.compose.ui.test.hasAnyAncestor
import androidx.compose.ui.test.hasClickAction
import androidx.compose.ui.test.hasContentDescription
import androidx.compose.ui.test.hasSetTextAction
import androidx.compose.ui.test.hasText
import androidx.compose.ui.test.isDialog
import androidx.compose.ui.test.junit4.createComposeRule
import androidx.compose.ui.test.onNodeWithContentDescription
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.performScrollTo
import androidx.compose.ui.test.performTextClearance
import androidx.compose.ui.test.performTextInput
import io.github.projectportfoliomanagement.mobile.app.ui.PpmApp
import io.github.projectportfoliomanagement.mobile.app.ui.PpmTheme
import io.github.projectportfoliomanagement.mobile.auth.InMemoryTokenStore
import io.github.projectportfoliomanagement.mobile.auth.Tokens
import kotlinx.coroutines.Dispatchers
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner
import org.robolectric.annotation.Config
import org.robolectric.annotation.GraphicsMode

@RunWith(RobolectricTestRunner::class)
@Config(sdk = [36])
@GraphicsMode(GraphicsMode.Mode.NATIVE)
class AppUiTest {
    @get:Rule val rule = createComposeRule()

    private val base = "https://ppm.example.org"
    private val service = RouteTransport()
    private val opened = mutableListOf<String>()
    private lateinit var graph: AppGraph

    @Before fun service() {
        service.on("GET", "$base/api/plans", 200, """[{"pid":"p1","name":"Alpha"},{"pid":"p2","name":"Beta"}]""")
        service.on("GET", "$base/api/plans/search?q=bet", 200, """[{"pid":"p2","name":"Beta"}]""")
        service.on(
            "GET", "$base/api/plans/p1", 200,
            """{"name":"Alpha","kind":"Project","code":"A-1","status":{"Custom":"Paused for audit"},
               "start_date":"2027-01-04","target_date":"2027-06-30","unknown_field":true}""",
        )
        service.on("GET", "$base/api/plans?parent=p1", 200, """[{"pid":"c1","name":"Alpha child"}]""")
        service.on(
            "GET", "$base/api/plans/p1/tasks", 200,
            """[{"pid":"t1","title":"Write it","status":"todo","points":3,"due_date":"2027-02-01","sprint_pid":null}]""",
        )
    }

    private fun start(signedIn: Boolean = true, url: String = base, issuer: String = "https://id.example.org") {
        graph = AppGraph(
            InMemorySettingsStore(ServerSettings(url, issuer, "ppm-mobile")),
            InMemoryTokenStore(if (signedIn) Tokens("tok") else null),
            service,
            { opened += it },
            io = Dispatchers.Unconfined,
        )
        rule.setContent { PpmTheme { PpmApp(graph) } }
        rule.waitForIdle()
    }

    private fun checkScreen() = rule.checkScreenSemantics()

    private fun field(label: String): SemanticsNodeInteraction =
        rule.onNode(hasSetTextAction() and hasContentDescription(label, substring = true))

    private fun dialogButton(text: String) = rule.onNode(hasText(text) and hasClickAction() and hasAnyAncestor(isDialog()))

    @Test fun thePlansListShowsTheActivePlansAndSearches() {
        start()
        rule.onNodeWithText("Alpha").assertExists()
        rule.onNodeWithText("Beta").assertExists()
        assertEquals("Bearer tok", service.sent("GET", "$base/api/plans")!!.headers["Authorization"])
        checkScreen()

        field("Search plans").performTextInput("bet")
        rule.mainClock.advanceTimeBy(400)
        rule.waitForIdle()
        assertNotNull(service.sent("GET", "$base/api/plans/search?q=bet"))
        rule.onNodeWithText("Alpha").assertDoesNotExist()
        rule.onNodeWithText("Beta").assertExists()
    }

    @Test fun thePlanScreenShowsItsFieldsChildrenAndTasks() {
        start()
        rule.onNodeWithText("Alpha").performClick()
        rule.waitForIdle()
        rule.onNodeWithText("Alpha child").assertExists()
        rule.onNodeWithText("Write it").assertExists()
        // A custom status is shown by its text; status is a word in a Tag, not a colour.
        rule.onNode(hasContentDescription("Paused for audit", substring = true), useUnmergedTree = true).assertExists()
        rule.onNode(hasContentDescription("To do", substring = true), useUnmergedTree = true).assertExists()
        rule.onNodeWithText("A-1").assertExists()
        checkScreen()
    }

    @Test fun aMoveSendsThePatchAndShowsTheNewStatus() {
        service.on("PATCH", "$base/api/plans/p1/tasks/t1", 200, """{"pid":"t1","title":"Write it","status":"done"}""")
        start()
        rule.onNodeWithText("Alpha").performClick()
        rule.waitForIdle()
        rule.onNodeWithContentDescription("Move Write it").performScrollTo().performClick()
        rule.onNodeWithText("Done").performClick()
        dialogButton("Move").performClick()
        rule.waitForIdle()
        val patch = service.sent("PATCH", "$base/api/plans/p1/tasks/t1")!!
        assertEquals("""{"status":"done"}""", patch.body)
        assertEquals("1.0", patch.headers["Accepts-version"])
        rule.onNode(hasContentDescription("Write it moved to Done.", substring = true), useUnmergedTree = true).assertExists()
        rule.onNode(hasContentDescription("To do", substring = true), useUnmergedTree = true).assertDoesNotExist()
        checkScreen()
    }

    @Test fun aRefusedMoveLeavesTheTaskWhereItWasAndNamesTheRule() {
        service.on("PATCH", "$base/api/plans/p1/tasks/t1", 403, """{"status":403,"detail":"rule writers: access=write"}""")
        start()
        rule.onNodeWithText("Alpha").performClick()
        rule.waitForIdle()
        rule.onNodeWithContentDescription("Move Write it").performScrollTo().performClick()
        rule.onNodeWithText("In review").performClick()
        dialogButton("Move").performClick()
        rule.waitForIdle()
        rule.onNode(hasContentDescription("The task was not moved. It is still To do.", substring = true), useUnmergedTree = true)
            .assertExists()
        rule.onNode(hasContentDescription("rule writers: access=write", substring = true), useUnmergedTree = true).assertExists()
        rule.onNode(hasContentDescription("To do\n", substring = true), useUnmergedTree = true).assertExists()
        checkScreen()
    }

    @Test fun aCreatePostsThePlanAndOpensIt() {
        service.on("POST", "$base/api/plans", 201, """{"pid":"p9","name":"Gamma"}""")
        service.on("GET", "$base/api/plans/p9", 200, """{"name":"Gamma","code":"G-1"}""")
        service.on("GET", "$base/api/plans?parent=p9", 200, "[]")
        service.on("GET", "$base/api/plans/p9/tasks", 200, "[]")
        start()
        rule.onNodeWithText("New plan").performClick()
        rule.waitForIdle()
        checkScreen()
        field("Name").performTextInput("Gamma")
        field("Code").performTextInput("G-1")
        field("Start date").performTextInput("2027-03-01")
        rule.onNodeWithText("Create plan").performScrollTo().performClick()
        rule.waitForIdle()
        assertEquals(
            """{"name":"Gamma","code":"G-1","start_date":"2027-03-01"}""",
            service.sent("POST", "$base/api/plans")!!.body,
        )
        rule.onNodeWithText("Gamma").assertExists()
        rule.onNodeWithText("No tasks").assertExists()
        checkScreen()
    }

    @Test fun aCreateThatTheServiceRefusesShowsEveryProblemBesideItsField() {
        service.on(
            "POST", "$base/api/plans", 422,
            """{"status":422,"code":"validation","detail":"code: exceeds 4096 characters; in_language: \"x\" is not a valid BCP-47 tag"}""",
        )
        start()
        rule.onNodeWithText("New plan").performClick()
        rule.waitForIdle()
        field("Name").performTextInput("Gamma")
        rule.onNodeWithText("Create plan").performScrollTo().performClick()
        rule.waitForIdle()
        rule.onNodeWithText("There is a problem").assertExists()
        // In the summary (a button that focuses the field) and beside the field (the field's error).
        rule.onNode(hasText("code: exceeds 4096 characters") and hasClickAction()).assertExists()
        rule.onNode(hasText("in_language: \"x\" is not a valid BCP-47 tag") and hasClickAction()).assertExists()
        rule.onNode(hasSetTextAction() and hasContentDescription("Code", substring = true)).assertExists()
        checkScreen()
    }

    @Test fun aCreateWithNoNameIsStoppedBeforeTheService() {
        start()
        rule.onNodeWithText("New plan").performClick()
        rule.waitForIdle()
        rule.onNodeWithText("Create plan").performScrollTo().performClick()
        rule.waitForIdle()
        rule.onNode(hasText("Enter a name for the plan.") and hasClickAction()).assertExists()
        assertNull(service.sent("POST", "$base/api/plans"))
    }

    @Test fun aDuplicateCreateSaysAPlanLikeThisExists() {
        service.on("POST", "$base/api/plans", 409, """{"status":409,"code":"conflict","detail":"duplicates found"}""")
        start()
        rule.onNodeWithText("New plan").performClick()
        rule.waitForIdle()
        field("Name").performTextInput("Alpha")
        rule.onNodeWithText("Create plan").performScrollTo().performClick()
        rule.waitForIdle()
        rule.onNode(hasContentDescription("A plan like this already exists.", substring = true), useUnmergedTree = true)
            .assertExists()
    }

    @Test fun a401SignsOutAndAsksForSignIn() {
        service.on("GET", "$base/api/plans", 401, """{"status":401}""")
        start()
        rule.waitForIdle()
        rule.onNode(hasContentDescription("Your session has ended. Sign in again.", substring = true), useUnmergedTree = true)
            .assertExists()
        assertTrue(!graph.signedIn)
        checkScreen()
    }

    @Test fun aServerThatCannotBeReachedOffersToTryAgain() {
        service.on("GET", "$base/api/plans", 503, "")
        start()
        rule.onNode(hasContentDescription("The service could not be reached", substring = true), useUnmergedTree = true)
            .assertExists()
        service.on("GET", "$base/api/plans", 200, """[{"pid":"p1","name":"Alpha"}]""")
        rule.onNodeWithText("Try again").performClick()
        rule.waitForIdle()
        rule.onNodeWithText("Alpha").assertExists()
    }

    @Test fun signInOpensTheBrowserWithPkceAndTheRedirectCompletesIt() {
        val id = "https://id.example.org"
        service.on(
            "GET", "$id/.well-known/openid-configuration", 200,
            """{"issuer":"$id","authorization_endpoint":"$id/auth","token_endpoint":"$id/token"}""",
        )
        service.on("POST", "$id/token", 200, """{"access_token":"fresh","expires_in":300}""")
        start(signedIn = false)
        // Not a development address: no way round signing in.
        rule.onNodeWithText("Continue without signing in").assertDoesNotExist()
        checkScreen()
        rule.onNode(hasText("Sign in") and hasClickAction()).performClick()
        rule.waitForIdle()
        val url = opened.single()
        assertTrue(url, url.startsWith("$id/auth?response_type=code&client_id=ppm-mobile&"))
        assertTrue(url, "code_challenge_method=S256" in url)
        val state = graph.pending!!.state
        graph.redirect = "${ServerSettings.REDIRECT_SCHEME}:/oauth2redirect?code=abc&state=$state"
        rule.waitForIdle()
        assertTrue("code_verifier=" in service.sent("POST", "$id/token")!!.body!!)
        rule.onNodeWithText("Alpha").assertExists()
        assertEquals("Bearer fresh", service.sent("GET", "$base/api/plans")!!.headers["Authorization"])
    }

    @Test fun aRedirectWithTheWrongStateIsRefused() {
        val id = "https://id.example.org"
        service.on(
            "GET", "$id/.well-known/openid-configuration", 200,
            """{"issuer":"$id","authorization_endpoint":"$id/auth","token_endpoint":"$id/token"}""",
        )
        start(signedIn = false)
        rule.onNode(hasText("Sign in") and hasClickAction()).performClick()
        rule.waitForIdle()
        graph.redirect = "${ServerSettings.REDIRECT_SCHEME}:/oauth2redirect?code=abc&state=forged"
        rule.waitForIdle()
        rule.onNode(hasContentDescription("did not match this sign-in", substring = true), useUnmergedTree = true).assertExists()
        assertNull(service.sent("POST", "$id/token"))
        assertTrue(!graph.signedIn)
    }

    @Test fun aDevelopmentAddressOffersToContinueWithoutSigningIn() {
        val dev = "http://10.0.2.2:5150"
        service.on("GET", "$dev/api/plans", 200, """[{"pid":"p1","name":"Local plan"}]""")
        start(signedIn = false, url = dev, issuer = "")
        rule.onNodeWithText("Continue without signing in").performClick()
        rule.waitForIdle()
        rule.onNodeWithText("Local plan").assertExists()
        assertNull(service.sent("GET", "$dev/api/plans")!!.headers["Authorization"])
    }

    @Test fun theServerScreenChecksTheAddressAndRefusesPlainHttp() {
        service.on("GET", "https://other.example.org/_health/ready", 200, """{"ok":true}""")
        graph = AppGraph(InMemorySettingsStore(), InMemoryTokenStore(), service, { }, io = Dispatchers.Unconfined)
        rule.setContent { PpmTheme { PpmApp(graph) } }
        rule.waitForIdle()
        checkScreen()
        field("Service address").performTextInput("http://other.example.org")
        rule.onNodeWithText("Save").performScrollTo().performClick()
        rule.waitForIdle()
        rule.onNodeWithText("There is a problem").assertExists()
        assertNull(graph.settings)
        checkScreen()
        field("Service address").performTextInput("")
        field("Service address").performTextClearanceAndInput("https://other.example.org")
        rule.onNodeWithText("Check address").performScrollTo().performClick()
        rule.waitForIdle()
        rule.onNode(hasContentDescription("The service answered and is ready.", substring = true), useUnmergedTree = true)
            .assertExists()
        field("Sign-in issuer").performTextInput("https://id.example.org")
        field("Client ID").performTextInput("ppm-mobile")
        rule.onNodeWithText("Save").performScrollTo().performClick()
        rule.waitForIdle()
        assertEquals(ServerSettings("https://other.example.org", "https://id.example.org", "ppm-mobile"), graph.settings)
        rule.onNode(hasText("Sign in") and hasClickAction()).assertExists()
    }

    private fun SemanticsNodeInteraction.performTextClearanceAndInput(text: String) {
        performTextClearance()
        performTextInput(text)
    }
}
