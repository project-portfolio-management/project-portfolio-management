// The checks must fail on a screen that breaks them; otherwise a passing UI test proves nothing.
package io.github.projectportfoliomanagement.mobile.app

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.text.BasicText
import androidx.compose.ui.Modifier
import androidx.compose.ui.test.junit4.createComposeRule
import androidx.compose.ui.unit.dp
import org.junit.Assert.assertTrue
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner
import org.robolectric.annotation.Config
import org.robolectric.annotation.GraphicsMode

@RunWith(RobolectricTestRunner::class)
@Config(sdk = [36])
@GraphicsMode(GraphicsMode.Mode.NATIVE)
class ScreenChecksTest {
    @get:Rule val rule = createComposeRule()

    private fun fails(): String = try {
        rule.checkScreenSemantics()
        ""
    } catch (e: AssertionError) {
        e.message.orEmpty()
    }

    @Test fun aSmallUnnamedTargetFails() {
        rule.setContent { Box(Modifier.size(10.dp).clickable { }) }
        val message = fails()
        assertTrue(message, "tap target 10" in message && "no name" in message)
    }

    @Test fun clippedTextFails() {
        rule.setContent { BasicText("A long plan name that cannot fit", maxLines = 1, modifier = Modifier.width(20.dp)) }
        assertTrue("text cut off" in fails())
    }
}
