// MA-1.2 and MA-2: sign in with the authorization-code flow and PKCE in a Custom Tab; "Continue without signing in" only for
// a development address.
package io.github.projectportfoliomanagement.mobile.app.ui

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Button
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import androidx.lifecycle.viewmodel.compose.viewModel
import io.github.lilydesignsystem.compose.AlertKind
import io.github.lilydesignsystem.compose.InsetText
import io.github.lilydesignsystem.compose.LilyAlert
import io.github.lilydesignsystem.compose.Politeness
import io.github.projectportfoliomanagement.mobile.app.AppGraph
import io.github.projectportfoliomanagement.mobile.app.R
import io.github.projectportfoliomanagement.mobile.auth.AuthFailure
import io.github.projectportfoliomanagement.mobile.auth.AuthResult
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext

class SignInViewModel(private val graph: AppGraph) : ViewModel() {
    sealed interface State {
        data object Idle : State
        data object Waiting : State
        data class Failed(val failure: AuthFailure?) : State
    }

    var state by mutableStateOf<State>(State.Idle)
        private set

    fun signIn() {
        val oidc = graph.oidc() ?: return
        state = State.Waiting
        viewModelScope.launch {
            when (val begun = withContext(graph.io) { oidc.begin() }) {
                is AuthResult.Failed -> state = State.Failed(begun.failure)
                is AuthResult.Ok -> {
                    graph.pending = begun.value
                    graph.browser.open(begun.value.url)
                }
            }
        }
    }

    /** Completes the sign-in the redirect answers. A redirect with no sign-in under way (the end-session return) is ignored. */
    fun complete(redirect: String) {
        val pending = graph.pending ?: run {
            if (state == State.Waiting) state = State.Failed(null)
            return
        }
        val oidc = graph.oidc() ?: return
        graph.pending = null
        viewModelScope.launch {
            when (val done = withContext(graph.io) { oidc.complete(pending, redirect) }) {
                is AuthResult.Failed -> state = State.Failed(done.failure)
                is AuthResult.Ok -> {
                    state = State.Idle
                    graph.signedIn(done.value)
                }
            }
        }
    }
}

@Composable
fun authFailureText(failure: AuthFailure?): String = when (failure) {
    null -> stringResource(R.string.auth_interrupted)
    AuthFailure.Insecure -> stringResource(R.string.auth_insecure)
    is AuthFailure.IssuerMismatch -> stringResource(R.string.auth_issuer_mismatch, failure.found)
    AuthFailure.StateMismatch -> stringResource(R.string.auth_state_mismatch)
    is AuthFailure.Provider -> stringResource(R.string.auth_provider, failure.description ?: failure.error)
    AuthFailure.NoCode -> stringResource(R.string.auth_no_code)
    is AuthFailure.Unreachable -> stringResource(R.string.auth_unreachable)
}

@Composable
fun SignInScreen(graph: AppGraph, onChangeServer: () -> Unit) {
    val vm = viewModel(key = "sign-in") { SignInViewModel(graph) }
    val redirect = graph.redirect
    LaunchedEffect(redirect) {
        if (redirect != null) {
            graph.redirect = null
            vm.complete(redirect)
        }
    }
    Scaffold(topBar = { AppBar(stringResource(R.string.sign_in_title)) }) { padding ->
        Column(
            Modifier.fillMaxSize().padding(padding).verticalScroll(rememberScrollState()).padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(16.dp),
        ) {
            if (graph.sessionEnded) {
                LilyAlert(message = stringResource(R.string.session_ended), kind = AlertKind.Warning)
            }
            when (val state = vm.state) {
                SignInViewModel.State.Idle -> Unit
                SignInViewModel.State.Waiting -> LilyAlert(
                    message = stringResource(R.string.sign_in_waiting),
                    kind = AlertKind.Info,
                    politeness = Politeness.Polite,
                )
                is SignInViewModel.State.Failed -> LilyAlert(
                    heading = stringResource(R.string.auth_failed_heading),
                    message = authFailureText(state.failure),
                    kind = AlertKind.Error,
                )
            }
            InsetText { Text(stringResource(R.string.sign_in_intro)) }
            if (graph.oidc() != null) {
                Button(onClick = vm::signIn, modifier = TapTarget) { Text(stringResource(R.string.sign_in)) }
            } else {
                Text(stringResource(R.string.sign_in_no_issuer))
            }
            if (graph.mayContinueWithoutSignIn) {
                Text(stringResource(R.string.sign_in_without_note))
                OutlinedButton(onClick = graph::continueWithoutSignIn, modifier = TapTarget) { Text(stringResource(R.string.sign_in_without)) }
            }
            TextButton(onClick = onChangeServer, modifier = TapTarget) { Text(stringResource(R.string.sign_in_change_server)) }
        }
    }
}
