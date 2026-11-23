// The app's screens and how one leads to another (MA-1). Which screen shows first follows the state: no settings, the server
// screen; not signed in, the sign-in screen; otherwise the plans, with a stack the system back gesture pops.
package io.github.projectportfoliomanagement.mobile.app.ui

import androidx.activity.compose.BackHandler
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateListOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.runtime.remember
import io.github.projectportfoliomanagement.mobile.app.AppGraph

sealed interface Screen {
    data object Plans : Screen
    data class Plan(val pid: String) : Screen
    data object NewPlan : Screen
}

class Navigator {
    val stack = mutableStateListOf<Screen>(Screen.Plans)
    val top: Screen get() = stack.last()
    val canPop: Boolean get() = stack.size > 1
    fun push(screen: Screen) { stack.add(screen) }
    fun pop() { if (canPop) stack.removeAt(stack.lastIndex) }
    fun replaceTop(screen: Screen) { stack[stack.lastIndex] = screen }
}

@Composable
fun PpmApp(graph: AppGraph) {
    val nav = remember { Navigator() }
    var editingServer by rememberSaveable { mutableStateOf(false) }
    val settings = graph.settings
    when {
        settings == null || editingServer -> {
            BackHandler(enabled = settings != null) { editingServer = false }
            ServerScreen(graph, onBack = if (settings != null) ({ editingServer = false }) else null, onSaved = { editingServer = false })
        }
        !graph.signedIn && !graph.withoutSignIn -> SignInScreen(graph, onChangeServer = { editingServer = true })
        else -> {
            BackHandler(enabled = nav.canPop) { nav.pop() }
            when (val screen = nav.top) {
                Screen.Plans -> PlansScreen(
                    graph,
                    onOpen = { nav.push(Screen.Plan(it)) },
                    onNew = { nav.push(Screen.NewPlan) },
                    onSignedOut = { nav.stack.retainAll { it == Screen.Plans } },
                )
                is Screen.Plan -> PlanScreen(graph, screen.pid, onBack = nav::pop, onOpen = { nav.push(Screen.Plan(it)) })
                Screen.NewPlan -> NewPlanScreen(graph, onBack = nav::pop, onCreated = { nav.replaceTop(Screen.Plan(it)) })
            }
        }
    }
}
