// MA-1.1: the service's base URL and the sign-in settings, with a check of the address (GET /_health/ready).
package io.github.projectportfoliomanagement.mobile.app.ui

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Button
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import androidx.lifecycle.viewmodel.compose.viewModel
import io.github.lilydesignsystem.compose.AlertKind
import io.github.lilydesignsystem.compose.ErrorSummary
import io.github.lilydesignsystem.compose.InsetText
import io.github.lilydesignsystem.compose.LilyAlert
import io.github.lilydesignsystem.compose.Politeness
import io.github.lilydesignsystem.compose.Problem
import io.github.projectportfoliomanagement.mobile.api.Addresses
import io.github.projectportfoliomanagement.mobile.api.ApiClient
import io.github.projectportfoliomanagement.mobile.api.ApiError
import io.github.projectportfoliomanagement.mobile.api.ApiResult
import io.github.projectportfoliomanagement.mobile.app.AppGraph
import io.github.projectportfoliomanagement.mobile.app.R
import io.github.projectportfoliomanagement.mobile.app.ServerSettings
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext

class ServerViewModel(private val graph: AppGraph) : ViewModel() {
    private val initial = graph.settings ?: ServerSettings("")
    var serviceUrl by mutableStateOf(initial.serviceUrl)
    var issuer by mutableStateOf(initial.issuer)
    var clientId by mutableStateOf(initial.clientId)
    var redirectUri by mutableStateOf(initial.redirectUri)

    /** Field key ("url", "issuer", "client", "redirect") to the string resource of its problem. */
    var problems by mutableStateOf<List<Pair<String, Int>>>(emptyList())
        private set
    var check by mutableStateOf<CheckState>(CheckState.None)
        private set

    sealed interface CheckState {
        data object None : CheckState
        data object Checking : CheckState
        data object Ready : CheckState
        data class Failed(val error: ApiError) : CheckState
    }

    private fun validate(): List<Pair<String, Int>> = buildList {
        val dev = Addresses.isDevelopment(serviceUrl)
        if (!Addresses.isAllowed(serviceUrl)) add("url" to R.string.server_error_url)
        if (issuer.isBlank()) {
            if (!dev) add("issuer" to R.string.server_error_issuer_required)
        } else {
            if (!Addresses.isAllowed(issuer)) add("issuer" to R.string.server_error_issuer)
            if (clientId.isBlank()) add("client" to R.string.server_error_client_id)
        }
        if (!redirectUri.trim().startsWith(ServerSettings.REDIRECT_SCHEME + ":")) add("redirect" to R.string.server_error_redirect)
    }

    fun checkAddress() {
        val url = serviceUrl
        if (!Addresses.isAllowed(url)) {
            problems = listOf("url" to R.string.server_error_url)
            return
        }
        check = CheckState.Checking
        viewModelScope.launch {
            val result = withContext(graph.io) { ApiClient(url, graph.transport).checkReady() }
            check = when (result) {
                is ApiResult.Ok -> CheckState.Ready
                is ApiResult.Failed -> CheckState.Failed(result.error)
            }
        }
    }

    fun save(): Boolean {
        problems = validate()
        if (problems.isNotEmpty()) return false
        graph.saveSettings(
            ServerSettings(Addresses.normalise(serviceUrl), issuer.trim(), clientId.trim(), redirectUri.trim()),
        )
        return true
    }
}

@Composable
fun ServerScreen(graph: AppGraph, onBack: (() -> Unit)?, onSaved: () -> Unit) {
    val vm = viewModel(key = "server") { ServerViewModel(graph) }
    val focus = remember { mapOf("url" to FocusRequester(), "issuer" to FocusRequester(), "client" to FocusRequester(), "redirect" to FocusRequester()) }
    fun errorFor(key: String): Int? = vm.problems.firstOrNull { it.first == key }?.second
    Scaffold(topBar = { AppBar(stringResource(R.string.server_title), onBack) }) { padding ->
        Column(
            Modifier.fillMaxSize().padding(padding).verticalScroll(rememberScrollState()).padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(16.dp),
        ) {
            if (vm.problems.isNotEmpty()) {
                ErrorSummary(
                    title = stringResource(R.string.there_is_a_problem),
                    problems = vm.problems.map { (key, res) -> Problem(stringResource(res)) { focus[key]?.requestFocus() } },
                )
            }
            InsetText { Text(stringResource(R.string.server_intro)) }
            TextFieldRow(
                label = stringResource(R.string.server_url),
                description = stringResource(R.string.server_url_hint),
                value = vm.serviceUrl,
                onValueChange = { vm.serviceUrl = it },
                error = errorFor("url")?.let { stringResource(it) },
                required = true,
                focusRequester = focus["url"],
                keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Uri),
            )
            OutlinedButton(onClick = vm::checkAddress, modifier = TapTarget) { Text(stringResource(R.string.server_check)) }
            when (val check = vm.check) {
                ServerViewModel.CheckState.None -> Unit
                ServerViewModel.CheckState.Checking -> Text(stringResource(R.string.loading))
                ServerViewModel.CheckState.Ready -> LilyAlert(
                    message = stringResource(R.string.server_ready),
                    kind = AlertKind.Success,
                    politeness = Politeness.Polite,
                )
                is ServerViewModel.CheckState.Failed -> ErrorAlert(check.error)
            }
            TextFieldRow(
                label = stringResource(R.string.server_issuer),
                description = stringResource(R.string.server_issuer_hint),
                value = vm.issuer,
                onValueChange = { vm.issuer = it },
                error = errorFor("issuer")?.let { stringResource(it) },
                focusRequester = focus["issuer"],
                keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Uri),
            )
            TextFieldRow(
                label = stringResource(R.string.server_client_id),
                value = vm.clientId,
                onValueChange = { vm.clientId = it },
                error = errorFor("client")?.let { stringResource(it) },
                focusRequester = focus["client"],
            )
            TextFieldRow(
                label = stringResource(R.string.server_redirect),
                description = stringResource(R.string.server_redirect_hint),
                value = vm.redirectUri,
                onValueChange = { vm.redirectUri = it },
                error = errorFor("redirect")?.let { stringResource(it) },
                focusRequester = focus["redirect"],
                keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Uri),
            )
            Button(onClick = { if (vm.save()) onSaved() }, modifier = TapTarget) { Text(stringResource(R.string.server_save)) }
        }
    }
}
