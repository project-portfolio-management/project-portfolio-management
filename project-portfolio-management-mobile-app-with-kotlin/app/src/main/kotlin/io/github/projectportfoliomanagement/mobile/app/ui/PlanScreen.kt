// MA-1.4 and MA-1.5: one plan's fields, its child plans and its tasks, and moving a task to another status. A failed move
// leaves the task in the status it had (MA-3).
package io.github.projectportfoliomanagement.mobile.app.ui

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.ListItem
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.pluralStringResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.heading
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.unit.dp
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import androidx.lifecycle.viewmodel.compose.viewModel
import io.github.lilydesignsystem.compose.AlertKind
import io.github.lilydesignsystem.compose.LilyAlert
import io.github.lilydesignsystem.compose.Politeness
import io.github.lilydesignsystem.compose.RecordList
import io.github.lilydesignsystem.compose.RecordListItem
import io.github.lilydesignsystem.compose.SummaryList
import io.github.lilydesignsystem.compose.SummaryListItem
import io.github.lilydesignsystem.compose.Tag
import io.github.lilydesignsystem.core.RecordListStatus
import io.github.projectportfoliomanagement.mobile.api.ApiError
import io.github.projectportfoliomanagement.mobile.api.ApiResult
import io.github.projectportfoliomanagement.mobile.api.Plan
import io.github.projectportfoliomanagement.mobile.api.PlanRef
import io.github.projectportfoliomanagement.mobile.api.Task
import io.github.projectportfoliomanagement.mobile.api.TaskStatuses
import io.github.projectportfoliomanagement.mobile.app.AppGraph
import io.github.projectportfoliomanagement.mobile.app.R
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext

class PlanViewModel(private val graph: AppGraph, private val pid: String) : ViewModel() {
    var plan by mutableStateOf<Plan?>(null)
        private set
    var error by mutableStateOf<ApiError?>(null)
        private set
    var children by mutableStateOf<List<PlanRef>?>(null)
        private set
    var childrenError by mutableStateOf<ApiError?>(null)
        private set
    var tasks by mutableStateOf<List<Task>?>(null)
        private set
    var tasksError by mutableStateOf<ApiError?>(null)
        private set

    /** The last move's outcome: the task as it was and the status asked for, and the error when it failed. */
    data class MoveOutcome(val task: Task, val to: String, val error: ApiError?)

    var moveOutcome by mutableStateOf<MoveOutcome?>(null)
        private set

    private fun <T> unauthorised(result: ApiResult<T>): Boolean =
        (result is ApiResult.Failed && result.error == ApiError.Unauthorized).also { if (it) graph.sessionExpired() }

    fun load() {
        viewModelScope.launch {
            val api = graph.api()
            val planResult = withContext(graph.io) { api.getPlan(pid) }
            if (unauthorised(planResult)) return@launch
            when (planResult) {
                is ApiResult.Ok -> { plan = planResult.value; error = null }
                is ApiResult.Failed -> { error = planResult.error; return@launch }
            }
            val childResult = withContext(graph.io) { api.childPlans(pid) }
            if (unauthorised(childResult)) return@launch
            when (childResult) {
                is ApiResult.Ok -> { children = childResult.value; childrenError = null }
                is ApiResult.Failed -> childrenError = childResult.error
            }
            val taskResult = withContext(graph.io) { api.tasks(pid) }
            if (unauthorised(taskResult)) return@launch
            when (taskResult) {
                is ApiResult.Ok -> { tasks = taskResult.value; tasksError = null }
                is ApiResult.Failed -> tasksError = taskResult.error
            }
        }
    }

    fun move(task: Task, to: String) {
        viewModelScope.launch {
            val api = graph.api()
            val result = withContext(graph.io) { api.moveTask(pid, task.pid, to) }
            if (unauthorised(result)) return@launch
            moveOutcome = when (result) {
                is ApiResult.Ok -> {
                    tasks = tasks?.map { if (it.pid == task.pid) result.value else it }
                    MoveOutcome(task, result.value.status, null)
                }
                // The task keeps the status it had: the list is not touched.
                is ApiResult.Failed -> MoveOutcome(task, to, result.error)
            }
        }
    }
}

@Composable
fun PlanScreen(graph: AppGraph, pid: String, onBack: () -> Unit, onOpen: (String) -> Unit) {
    val vm = viewModel(key = "plan-$pid") { PlanViewModel(graph, pid) }
    LaunchedEffect(pid) { vm.load() }
    var moving by remember { mutableStateOf<Task?>(null) }
    Scaffold(topBar = { AppBar(stringResource(R.string.plan_title), onBack) }) { padding ->
        Column(
            Modifier.fillMaxSize().padding(padding).verticalScroll(rememberScrollState()).padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(16.dp),
        ) {
            vm.error?.let { ErrorAlert(it, onRetry = vm::load) }
            val plan = vm.plan
            if (plan == null) {
                if (vm.error == null) Text(stringResource(R.string.loading))
                return@Column
            }
            Text(plan.name, style = MaterialTheme.typography.headlineSmall, modifier = Modifier.semantics { heading() })
            PlanDetails(plan)

            SectionHeading(stringResource(R.string.plan_children))
            vm.childrenError?.let { ErrorAlert(it, onRetry = vm::load) }
            vm.children?.let { children ->
                RecordList(
                    status = if (children.isEmpty()) RecordListStatus.NoneKnown else RecordListStatus.Known,
                    statusLabel = stringResource(if (children.isEmpty()) R.string.plan_no_children else R.string.plan_children),
                    label = stringResource(R.string.plan_children),
                ) {
                    val open = stringResource(R.string.plans_open)
                    children.forEach { child ->
                        ListItem(
                            headlineContent = { Text(child.name) },
                            modifier = Modifier.fillMaxWidth().clickable(onClickLabel = open) { onOpen(child.pid) },
                        )
                    }
                }
            }

            SectionHeading(stringResource(R.string.plan_tasks))
            vm.moveOutcome?.let { outcome ->
                if (outcome.error == null) {
                    LilyAlert(
                        message = stringResource(R.string.moved, outcome.task.title, taskStatusText(outcome.to)),
                        kind = AlertKind.Success,
                        politeness = Politeness.Polite,
                    )
                } else {
                    ErrorAlert(outcome.error, extra = stringResource(R.string.move_failed, taskStatusText(outcome.task.status)))
                }
            }
            vm.tasksError?.let { ErrorAlert(it, onRetry = vm::load) }
            vm.tasks?.let { tasks ->
                RecordList(
                    status = if (tasks.isEmpty()) RecordListStatus.NoneKnown else RecordListStatus.Known,
                    statusLabel = stringResource(if (tasks.isEmpty()) R.string.plan_no_tasks else R.string.plan_tasks),
                    label = stringResource(R.string.plan_tasks),
                ) {
                    tasks.forEach { task -> TaskRow(task, onMove = { moving = task }) }
                }
            }
        }
    }
    moving?.let { task ->
        val choices = (TaskStatuses.all + task.status).distinct()
        ChoiceDialog(
            title = stringResource(R.string.move_title),
            choices = choices,
            initial = task.status,
            label = { taskStatusText(it) },
            confirmLabel = stringResource(R.string.move_confirm),
            onConfirm = { to -> moving = null; vm.move(task, to) },
            onDismiss = { moving = null },
        )
    }
}

@Composable
private fun SectionHeading(text: String) {
    Text(text, style = MaterialTheme.typography.titleLarge, modifier = Modifier.semantics { heading() })
}

@Composable
private fun PlanDetails(plan: Plan) {
    val notSet = stringResource(R.string.not_set)
    SummaryList(label = stringResource(R.string.plan_details)) {
        SummaryListItem(term = stringResource(R.string.plan_kind)) { Text(plan.kind?.let { planKindText(it) } ?: notSet) }
        SummaryListItem(term = stringResource(R.string.plan_status)) {
            val status = plan.status
            if (status == null) Text(notSet) else Tag(text = planStatusText(status), label = stringResource(R.string.plan_status))
        }
        SummaryListItem(term = stringResource(R.string.plan_code)) { Text(plan.code ?: notSet) }
        SummaryListItem(term = stringResource(R.string.plan_start_date)) { Text(plan.startDate?.let(::localDate) ?: notSet) }
        SummaryListItem(term = stringResource(R.string.plan_target_date)) { Text(plan.targetDate?.let(::localDate) ?: notSet) }
        SummaryListItem(term = stringResource(R.string.plan_organisation)) { Text(plan.ownerOrgName ?: notSet) }
    }
}

@Composable
private fun TaskRow(task: Task, onMove: () -> Unit) {
    Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
        RecordListItem {
            Text(task.title, style = MaterialTheme.typography.titleMedium)
            // Status is a word, never a colour alone (MA-4).
            Tag(text = taskStatusText(task.status), label = stringResource(R.string.task_status))
            task.dueDate?.let { Text(stringResource(R.string.task_due, localDate(it))) }
            task.points?.let { Text(pluralStringResource(R.plurals.task_points, it, it)) }
            task.blockedDays?.let { Text(pluralStringResource(R.plurals.task_blocked_days, it, it)) }
        }
        val label = stringResource(R.string.task_move_label, task.title)
        OutlinedButton(onClick = onMove, modifier = TapTarget.semantics { contentDescription = label }) {
            Text(stringResource(R.string.task_move))
        }
    }
}
