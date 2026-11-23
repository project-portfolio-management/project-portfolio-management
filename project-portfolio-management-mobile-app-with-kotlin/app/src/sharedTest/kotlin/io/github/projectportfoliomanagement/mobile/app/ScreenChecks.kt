// MA-4 checks read from the semantics tree, which run anywhere Compose tests run (Robolectric included, where the
// Accessibility Test Framework finds nothing: see AGENTS.md). Every tappable node is at least 48 by 48 dp and has a name
// (text or a content description), and no text is cut off (height overflow or an ellipsis; Robolectric reports width
// overflow for text that fits, so that flag is not read).
package io.github.projectportfoliomanagement.mobile.app

import androidx.compose.ui.semantics.SemanticsActions
import androidx.compose.ui.semantics.SemanticsNode
import androidx.compose.ui.semantics.SemanticsProperties
import androidx.compose.ui.semantics.getOrNull
import androidx.compose.ui.test.isRoot
import androidx.compose.ui.test.junit4.ComposeTestRule
import androidx.compose.ui.text.TextLayoutResult
import org.junit.Assert.assertTrue

fun ComposeTestRule.checkScreenSemantics() {
    val problems = mutableListOf<String>()
    onAllNodes(isRoot()).fetchSemanticsNodes().forEach { root ->
        val density = density
        fun visit(node: SemanticsNode) {
            val c = node.config
            val name = c.getOrNull(SemanticsProperties.Text)?.joinToString(" ")?.ifBlank { null }
                ?: c.getOrNull(SemanticsProperties.ContentDescription)?.joinToString(" ")?.ifBlank { null }
            if (c.getOrNull(SemanticsActions.OnClick) != null && c.getOrNull(SemanticsProperties.Disabled) == null) {
                // The node's own size: the app gives every control a 48 dp minimum rather than rely on Material's
                // invisible padding (minimumInteractiveComponentSize), which the node's bounds do not include.
                val w = with(density) { node.size.width.toDp() }
                val h = with(density) { node.size.height.toDp() }
                if (w.value < 47.5f || h.value < 47.5f) problems += "tap target ${w.value}x${h.value} dp: ${name ?: node}"
                if (name == null) problems += "tap target with no name: $node"
            }
            c.getOrNull(SemanticsActions.GetTextLayoutResult)?.action?.let { get ->
                val results = mutableListOf<TextLayoutResult>()
                get(results)
                results.filter { it.didOverflowHeight || (it.lineCount > 0 && it.isLineEllipsized(it.lineCount - 1)) }
                    .forEach { problems += "text cut off: ${it.layoutInput.text}" }
            }
            node.children.forEach(::visit)
        }
        visit(root)
    }
    assertTrue(problems.joinToString("\n"), problems.isEmpty())
}
