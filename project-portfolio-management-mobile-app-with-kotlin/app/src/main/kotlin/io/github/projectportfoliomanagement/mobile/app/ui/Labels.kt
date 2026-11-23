// The app's words for the service's values: statuses, kinds, dates and MA-3 errors. A value the app does not know is shown
// as the service sends it, never dropped.
package io.github.projectportfoliomanagement.mobile.app.ui

import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import io.github.projectportfoliomanagement.mobile.api.PlanStatus
import io.github.projectportfoliomanagement.mobile.api.TaskStatuses
import io.github.projectportfoliomanagement.mobile.app.R
import java.time.LocalDate
import java.time.format.DateTimeFormatter
import java.time.format.FormatStyle
import java.util.Locale

@Composable
fun taskStatusText(status: String): String = when (status) {
    TaskStatuses.TODO -> stringResource(R.string.task_status_todo)
    TaskStatuses.IN_PROGRESS -> stringResource(R.string.task_status_in_progress)
    TaskStatuses.IN_REVIEW -> stringResource(R.string.task_status_in_review)
    TaskStatuses.DONE -> stringResource(R.string.task_status_done)
    TaskStatuses.BLOCKED -> stringResource(R.string.task_status_blocked)
    else -> status
}

@Composable
fun planStatusText(status: PlanStatus): String = when (status) {
    is PlanStatus.Custom -> status.text
    is PlanStatus.Named -> when (status.name) {
        "Proposed" -> stringResource(R.string.plan_status_proposed)
        "Active" -> stringResource(R.string.plan_status_active)
        "OnHold" -> stringResource(R.string.plan_status_on_hold)
        "Completed" -> stringResource(R.string.plan_status_completed)
        "Cancelled" -> stringResource(R.string.plan_status_cancelled)
        else -> status.name
    }
}

@Composable
fun planKindText(kind: String): String = when (kind) {
    "Portfolio" -> stringResource(R.string.plan_kind_portfolio)
    "Project" -> stringResource(R.string.plan_kind_project)
    "Product" -> stringResource(R.string.plan_kind_product)
    "Program" -> stringResource(R.string.plan_kind_program)
    "Practice" -> stringResource(R.string.plan_kind_practice)
    "Process" -> stringResource(R.string.plan_kind_process)
    "Purpose" -> stringResource(R.string.plan_kind_purpose)
    "Pathway" -> stringResource(R.string.plan_kind_pathway)
    "Proposal" -> stringResource(R.string.plan_kind_proposal)
    else -> kind
}

/** An ISO calendar date (`YYYY-MM-DD`), or null when [text] is not one. No time zone is involved. */
fun isoDate(text: String): LocalDate? = runCatching { LocalDate.parse(text.trim(), DateTimeFormatter.ISO_LOCAL_DATE) }.getOrNull()

/** An ISO calendar date in the device's locale (MA-1); a value that is not a date is shown as sent. */
fun localDate(iso: String, locale: Locale = Locale.getDefault()): String =
    isoDate(iso)?.format(DateTimeFormatter.ofLocalizedDate(FormatStyle.MEDIUM).withLocale(locale)) ?: iso
