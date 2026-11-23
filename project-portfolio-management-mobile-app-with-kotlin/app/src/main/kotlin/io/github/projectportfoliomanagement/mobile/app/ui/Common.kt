// Pieces every screen uses: the top bar, the MA-3 error alert, and a choice dialog (Material 3's, a native control).
package io.github.projectportfoliomanagement.mobile.app.ui

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.defaultMinSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.selection.selectable
import androidx.compose.foundation.selection.selectableGroup
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.RadioButton
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.unit.dp
import io.github.lilydesignsystem.compose.AlertKind
import io.github.lilydesignsystem.compose.LilyAlert
import io.github.lilydesignsystem.compose.LilyMinimumTouchTarget
import io.github.projectportfoliomanagement.mobile.api.ApiError
import io.github.projectportfoliomanagement.mobile.app.R

/** At least 48 by 48 dp (MA-4), on the control itself rather than in Material's invisible padding. */
val TapTarget: Modifier = Modifier.defaultMinSize(minWidth = LilyMinimumTouchTarget, minHeight = LilyMinimumTouchTarget)

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun AppBar(title: String, onBack: (() -> Unit)? = null, actions: @Composable () -> Unit = {}) {
    TopAppBar(
        title = { Text(title) },
        navigationIcon = {
            if (onBack != null) {
                IconButton(onClick = onBack, modifier = TapTarget) {
                    Icon(painterResource(R.drawable.ic_arrow_back), contentDescription = stringResource(R.string.back))
                }
            }
        },
        actions = { actions() },
    )
}

/** The words for an [ApiError] (MA-3): a heading and a message. 401 never reaches here; it signs out. */
@Composable
fun errorWords(error: ApiError): Pair<String, String> = when (error) {
    ApiError.Unauthorized -> stringResource(R.string.sign_in_title) to stringResource(R.string.session_ended)
    is ApiError.Forbidden -> stringResource(R.string.error_forbidden_heading) to
        (error.rule?.let { stringResource(R.string.error_forbidden_rule, it) } ?: stringResource(R.string.error_forbidden_no_rule))
    ApiError.NotFound -> stringResource(R.string.error_unexpected_heading) to stringResource(R.string.error_not_found)
    ApiError.Duplicate -> stringResource(R.string.there_is_a_problem) to stringResource(R.string.error_duplicate)
    is ApiError.Invalid -> stringResource(R.string.error_invalid_heading) to error.problems.joinToString("\n") { it.message }
    is ApiError.Unreachable -> stringResource(R.string.error_unreachable_heading) to when (error.reason) {
        ApiError.Unreachable.Reason.Network -> stringResource(R.string.error_unreachable_network)
        ApiError.Unreachable.Reason.Timeout -> stringResource(R.string.error_unreachable_timeout)
        ApiError.Unreachable.Reason.Server -> stringResource(R.string.error_unreachable_server)
    }
    is ApiError.Unexpected -> stringResource(R.string.error_unexpected_heading) to
        if (error.status == 0) stringResource(R.string.error_unreadable) else stringResource(R.string.error_unexpected, error.status)
}

/** An [ApiError] as a Lily alert, with "Try again" when [onRetry] is given (MA-3: no network, timeout, 5xx). */
@Composable
fun ErrorAlert(error: ApiError, modifier: Modifier = Modifier, extra: String? = null, onRetry: (() -> Unit)? = null) {
    val (heading, message) = errorWords(error)
    Column(modifier, verticalArrangement = Arrangement.spacedBy(8.dp)) {
        LilyAlert(
            message = listOfNotNull(extra, message).joinToString("\n"),
            heading = heading,
            kind = AlertKind.Error,
            modifier = Modifier.fillMaxWidth(),
        )
        if (onRetry != null) OutlinedButton(onClick = onRetry, modifier = TapTarget) { Text(stringResource(R.string.try_again)) }
    }
}

/** A dialog of radio choices; [label] gives each choice's words. Rows are at least 48 dp tall (MA-4). */
@Composable
fun <T> ChoiceDialog(
    title: String,
    choices: List<T>,
    initial: T?,
    label: @Composable (T) -> String,
    confirmLabel: String,
    onConfirm: (T) -> Unit,
    onDismiss: () -> Unit,
) {
    var selected by remember { mutableStateOf(initial) }
    AlertDialog(
        onDismissRequest = onDismiss,
        title = { Text(title) },
        text = {
            Column(Modifier.selectableGroup().verticalScroll(rememberScrollState())) {
                choices.forEach { choice ->
                    Row(
                        Modifier
                            .fillMaxWidth()
                            .defaultMinSize(minHeight = LilyMinimumTouchTarget)
                            .selectable(selected = choice == selected, role = Role.RadioButton, onClick = { selected = choice })
                            .padding(horizontal = 4.dp),
                        verticalAlignment = Alignment.CenterVertically,
                    ) {
                        RadioButton(selected = choice == selected, onClick = null)
                        Spacer(Modifier.width(12.dp))
                        Text(label(choice), style = MaterialTheme.typography.bodyLarge)
                    }
                }
            }
        },
        confirmButton = {
            TextButton(onClick = { selected?.let(onConfirm) }, enabled = selected != null && selected != initial, modifier = TapTarget) {
                Text(confirmLabel)
            }
        },
        dismissButton = { TextButton(onClick = onDismiss, modifier = TapTarget) { Text(stringResource(R.string.cancel)) } },
    )
}
