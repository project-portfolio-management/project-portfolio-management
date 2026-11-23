// MA-1.3: the active plans, with a search box. An empty box lists (`GET /api/plans`); text searches (`/search?q=`).
package io.github.projectportfoliomanagement.mobile.app.ui

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.ExtendedFloatingActionButton
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.ListItem
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.unit.dp
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import androidx.lifecycle.viewmodel.compose.viewModel
import io.github.projectportfoliomanagement.mobile.api.ApiError
import io.github.projectportfoliomanagement.mobile.api.ApiResult
import io.github.projectportfoliomanagement.mobile.api.PlanRef
import io.github.projectportfoliomanagement.mobile.app.AppGraph
import io.github.projectportfoliomanagement.mobile.app.R
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext

class PlansViewModel(private val graph: AppGraph) : ViewModel() {
    var query by mutableStateOf("")
    var plans by mutableStateOf<List<PlanRef>?>(null)
        private set
    var error by mutableStateOf<ApiError?>(null)
        private set
    private var job: Job? = null

    /** Loads the list for the current query; a newer load cancels an older one. */
    fun load() {
        job?.cancel()
        val q = query.trim()
        job = viewModelScope.launch {
            val api = graph.api()
            val result = withContext(graph.io) { if (q.isEmpty()) api.listPlans() else api.searchPlans(q) }
            when (result) {
                is ApiResult.Ok -> { plans = result.value; error = null }
                is ApiResult.Failed -> if (result.error == ApiError.Unauthorized) graph.sessionExpired() else error = result.error
            }
        }
    }
}

@Composable
fun PlansScreen(graph: AppGraph, onOpen: (String) -> Unit, onNew: () -> Unit, onSignedOut: () -> Unit) {
    val vm = viewModel(key = "plans") { PlansViewModel(graph) }
    val scope = rememberCoroutineScope()
    // Loads on entry, and again 300 ms after the person stops typing.
    var typed by remember { mutableStateOf(false) }
    LaunchedEffect(vm.query) {
        if (typed) delay(300)
        vm.load()
    }
    Scaffold(
        topBar = {
            AppBar(stringResource(R.string.plans_title)) {
                TextButton(onClick = {
                    scope.launch {
                        val url = withContext(graph.io) { graph.signOut() }
                        onSignedOut()
                        url?.let(graph.browser::open)
                    }
                }, modifier = TapTarget) { Text(stringResource(R.string.sign_out)) }
            }
        },
        floatingActionButton = {
            ExtendedFloatingActionButton(onClick = onNew) { Text(stringResource(R.string.plans_new)) }
        },
    ) { padding ->
        LazyColumn(
            Modifier.fillMaxSize().padding(padding),
            contentPadding = PaddingValues(bottom = 96.dp),
        ) {
            item {
                Column(Modifier.padding(16.dp), verticalArrangement = Arrangement.spacedBy(12.dp)) {
                    TextFieldRow(
                        label = stringResource(R.string.plans_search),
                        value = vm.query,
                        onValueChange = { typed = true; vm.query = it },
                        keyboardOptions = KeyboardOptions(imeAction = ImeAction.Search),
                        keyboardActions = KeyboardActions(onSearch = { vm.load() }),
                    )
                    vm.error?.let { ErrorAlert(it, onRetry = { vm.load() }) }
                    val plans = vm.plans
                    when {
                        plans == null && vm.error == null -> Text(stringResource(R.string.loading))
                        plans != null && plans.isEmpty() -> Text(
                            stringResource(if (vm.query.isBlank()) R.string.plans_none else R.string.plans_none_found),
                        )
                    }
                }
            }
            items(vm.plans.orEmpty(), key = { it.pid }) { plan ->
                val open = stringResource(R.string.plans_open)
                ListItem(
                    headlineContent = { Text(plan.name) },
                    modifier = Modifier.clickable(onClickLabel = open) { onOpen(plan.pid) },
                )
                HorizontalDivider()
            }
        }
    }
}
