// MA-1.6: a new plan: name (required), kind, status, code, start and target dates. A 422 shows every problem the service
// lists in Lily's ErrorSummary and beside the field it names; a 409 says a plan like this already exists (MA-3).
package io.github.projectportfoliomanagement.mobile.app.ui

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Button
import androidx.compose.material3.DatePicker
import androidx.compose.material3.DatePickerDialog
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.material3.rememberDatePickerState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import androidx.lifecycle.viewmodel.compose.viewModel
import io.github.lilydesignsystem.compose.ErrorSummary
import io.github.lilydesignsystem.compose.Field
import io.github.lilydesignsystem.compose.Problem
import io.github.projectportfoliomanagement.mobile.api.ApiError
import io.github.projectportfoliomanagement.mobile.api.ApiResult
import io.github.projectportfoliomanagement.mobile.api.FieldProblem
import io.github.projectportfoliomanagement.mobile.api.NewPlan
import io.github.projectportfoliomanagement.mobile.api.PlanKinds
import io.github.projectportfoliomanagement.mobile.api.PlanStatus
import io.github.projectportfoliomanagement.mobile.app.AppGraph
import io.github.projectportfoliomanagement.mobile.app.R
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import java.time.Instant
import java.time.ZoneOffset

/** A form problem: the wire field it belongs to (or null), and its words: the app's ([res]) or the service's ([text]). */
data class FormProblem(val field: String?, val res: Int? = null, val text: String? = null)

class NewPlanViewModel(private val graph: AppGraph) : ViewModel() {
    var name by mutableStateOf("")
    var kind by mutableStateOf<String?>(null)
    var status by mutableStateOf<PlanStatus?>(null)
    var code by mutableStateOf("")
    var startDate by mutableStateOf("")
    var targetDate by mutableStateOf("")

    var problems by mutableStateOf<List<FormProblem>>(emptyList())
        private set
    var error by mutableStateOf<ApiError?>(null)
        private set
    var busy by mutableStateOf(false)
        private set

    private fun validate(): List<FormProblem> = buildList {
        if (name.isBlank()) add(FormProblem("name", R.string.new_plan_name_required))
        if (startDate.isNotBlank() && isoDate(startDate) == null) add(FormProblem("start_date", R.string.new_plan_date_format))
        if (targetDate.isNotBlank() && isoDate(targetDate) == null) add(FormProblem("target_date", R.string.new_plan_date_format))
    }

    fun submit(onCreated: (String) -> Unit) {
        if (busy) return
        error = null
        problems = validate()
        if (problems.isNotEmpty()) return
        val plan = NewPlan(
            name = name.trim(),
            kind = kind,
            status = status,
            code = code.trim().ifEmpty { null },
            startDate = startDate.trim().ifEmpty { null },
            targetDate = targetDate.trim().ifEmpty { null },
        )
        busy = true
        viewModelScope.launch {
            val api = graph.api()
            val result = withContext(graph.io) { api.createPlan(plan) }
            busy = false
            when (result) {
                is ApiResult.Ok -> onCreated(result.value.pid)
                is ApiResult.Failed -> when (val e = result.error) {
                    ApiError.Unauthorized -> graph.sessionExpired()
                    is ApiError.Invalid -> problems = e.problems.map { FormProblem(it.field.takeIf { f -> f in FIELDS }, text = it.message) }
                        .ifEmpty { listOf(FormProblem(null, text = "")) }
                    else -> error = e
                }
            }
        }
    }

    companion object {
        val FIELDS = setOf("name", "kind", "status", "code", "start_date", "target_date")
    }
}

@Composable
private fun FormProblem.words(): String = res?.let { stringResource(it) } ?: text?.takeIf { it.isNotBlank() }
    ?: stringResource(R.string.error_invalid_heading)

@Composable
fun NewPlanScreen(graph: AppGraph, onBack: () -> Unit, onCreated: (String) -> Unit) {
    val vm = viewModel(key = "new-plan") { NewPlanViewModel(graph) }
    val focus = remember { NewPlanViewModel.FIELDS.associateWith { FocusRequester() } }
    var choosing by remember { mutableStateOf<String?>(null) }
    @Composable
    fun errorFor(field: String): String? = vm.problems.filter { it.field == field }.map { it.words() }
        .takeIf { it.isNotEmpty() }?.joinToString("\n")

    Scaffold(topBar = { AppBar(stringResource(R.string.new_plan_title), onBack) }) { padding ->
        Column(
            Modifier.fillMaxSize().padding(padding).verticalScroll(rememberScrollState()).padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(16.dp),
        ) {
            if (vm.problems.isNotEmpty()) {
                ErrorSummary(
                    title = stringResource(R.string.there_is_a_problem),
                    problems = vm.problems.map { p -> Problem(p.words()) { p.field?.let { focus[it] }?.requestFocus() } },
                )
            }
            vm.error?.let { ErrorAlert(it, onRetry = if (it is ApiError.Unreachable) ({ vm.submit(onCreated) }) else null) }
            TextFieldRow(
                label = stringResource(R.string.plan_name),
                value = vm.name,
                onValueChange = { vm.name = it },
                error = errorFor("name"),
                required = true,
                focusRequester = focus["name"],
            )
            ChoiceField(
                label = stringResource(R.string.plan_kind),
                value = vm.kind?.let { planKindText(it) },
                error = errorFor("kind"),
                focusRequester = focus.getValue("kind"),
                onClick = { choosing = "kind" },
            )
            ChoiceField(
                label = stringResource(R.string.plan_status),
                value = vm.status?.let { planStatusText(it) },
                error = errorFor("status"),
                focusRequester = focus.getValue("status"),
                onClick = { choosing = "status" },
            )
            TextFieldRow(
                label = stringResource(R.string.plan_code),
                value = vm.code,
                onValueChange = { vm.code = it },
                error = errorFor("code"),
                focusRequester = focus["code"],
            )
            DateField(
                label = stringResource(R.string.plan_start_date),
                value = vm.startDate,
                onValueChange = { vm.startDate = it },
                error = errorFor("start_date"),
                focusRequester = focus.getValue("start_date"),
            )
            DateField(
                label = stringResource(R.string.plan_target_date),
                value = vm.targetDate,
                onValueChange = { vm.targetDate = it },
                error = errorFor("target_date"),
                focusRequester = focus.getValue("target_date"),
            )
            Button(onClick = { vm.submit(onCreated) }, enabled = !vm.busy, modifier = TapTarget) { Text(stringResource(R.string.new_plan_create)) }
        }
    }
    when (choosing) {
        "kind" -> ChoiceDialog(
            title = stringResource(R.string.choose_kind),
            choices = PlanKinds.all,
            initial = vm.kind,
            label = { planKindText(it) },
            confirmLabel = stringResource(R.string.choose),
            onConfirm = { vm.kind = it; choosing = null },
            onDismiss = { choosing = null },
        )
        "status" -> ChoiceDialog(
            title = stringResource(R.string.choose_status),
            choices = PlanStatus.units,
            initial = vm.status,
            label = { planStatusText(it) },
            confirmLabel = stringResource(R.string.choose),
            onConfirm = { vm.status = it; choosing = null },
            onDismiss = { choosing = null },
        )
    }
}

/** A choice shown as a button that opens a [ChoiceDialog]; Field gives it its label and error. */
@Composable
private fun ChoiceField(label: String, value: String?, error: String?, focusRequester: FocusRequester, onClick: () -> Unit) {
    Field(label = label, error = error, modifier = Modifier.fillMaxWidth()) { fieldModifier ->
        OutlinedButton(onClick = onClick, modifier = fieldModifier.then(TapTarget).fillMaxWidth().focusRequester(focusRequester)) {
            Text(value ?: stringResource(R.string.not_set))
        }
    }
}

/** An ISO date typed as text, or chosen with Material 3's date picker (which works in UTC days: no zone conversion). */
@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun DateField(label: String, value: String, onValueChange: (String) -> Unit, error: String?, focusRequester: FocusRequester) {
    var picking by remember { mutableStateOf(false) }
    Column(verticalArrangement = Arrangement.spacedBy(4.dp)) {
        TextFieldRow(
            label = label,
            description = stringResource(R.string.date_hint),
            value = value,
            onValueChange = onValueChange,
            error = error,
            focusRequester = focusRequester,
            keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
        )
        TextButton(onClick = { picking = true }, modifier = TapTarget) { Text(stringResource(R.string.choose_date)) }
    }
    if (picking) {
        val initial = isoDate(value)?.atStartOfDay(ZoneOffset.UTC)?.toInstant()?.toEpochMilli()
        val state = rememberDatePickerState(initialSelectedDateMillis = initial)
        DatePickerDialog(
            onDismissRequest = { picking = false },
            confirmButton = {
                TextButton(onClick = {
                    state.selectedDateMillis?.let {
                        onValueChange(Instant.ofEpochMilli(it).atZone(ZoneOffset.UTC).toLocalDate().toString())
                    }
                    picking = false
                }, modifier = TapTarget) { Text(stringResource(R.string.ok)) }
            },
            dismissButton = { TextButton(onClick = { picking = false }, modifier = TapTarget) { Text(stringResource(R.string.cancel)) } },
        ) { DatePicker(state = state) }
    }
}
