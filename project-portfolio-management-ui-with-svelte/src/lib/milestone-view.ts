// Milestones: the words, symbols and sentences the milestone page shows. Pure
// and unit-tested.
//
// Every state has a word and a shape — never colour alone. A milestone with no
// forecast says so; it is never called "on track" by default.

import type {
  MilestoneScheduleState,
  MilestoneSchedule,
  MilestoneSlip,
  MilestoneStatus,
  MilestoneView,
} from "#lib/api/ppm.js";

export const STATUS_ORDER: readonly MilestoneStatus[] = [
  "late",
  "due_soon",
  "upcoming",
  "achieved",
];

export const STATUS_TEXT: Record<MilestoneStatus, string> = {
  late: "Late",
  due_soon: "Due soon",
  upcoming: "Upcoming",
  achieved: "Achieved",
};

/** A decorative shape per status (the word carries the meaning). */
export const STATUS_SYMBOL: Record<MilestoneStatus, string> = {
  late: "■",
  due_soon: "▲",
  upcoming: "○",
  achieved: "✓",
};

export const STATE_TEXT: Record<MilestoneScheduleState, string> = {
  on_track: "On track",
  at_risk: "At risk",
  forecast_late: "Forecast late",
  no_forecast: "No forecast",
};

const NO_FORECAST_REASON: Record<string, string> = {
  no_linked_tasks:
    "It is not linked to any task, so there is nothing to forecast from.",
  no_scheduled_tasks:
    "Its open tasks have no dates the schedule can place (give them a duration and a start or a predecessor).",
  cycle: "The plan's dependencies contain a cycle, so no forecast can be made.",
};

const businessDays = (n: number): string =>
  `${n} business day${Math.abs(n) === 1 ? "" : "s"}`;
const calendarDays = (n: number): string =>
  `${n} calendar day${Math.abs(n) === 1 ? "" : "s"}`;

/** The forecast as a sentence, or `null` when the milestone is achieved. */
export function forecastText(
  schedule: MilestoneSchedule,
  due: string,
): string | null {
  if (schedule.state === null) return null;
  if (schedule.state === "no_forecast" || schedule.forecast === null) {
    return NO_FORECAST_REASON[schedule.reason ?? ""] ?? "There is no forecast.";
  }
  const slack = schedule.slack_days ?? 0;
  const partial = schedule.partial
    ? " (some open tasks cannot be placed, so this may be early)"
    : "";
  if (slack < 0) {
    return `Forecast to finish ${schedule.forecast}, ${businessDays(-slack)} after it is due (${due})${partial}.`;
  }
  if (slack === 0)
    return `Forecast to finish on the due date, ${due}${partial}.`;
  return `Forecast to finish ${schedule.forecast}, ${businessDays(slack)} before it is due (${due})${partial}.`;
}

/** How far a milestone has moved, or `null` when it never has. */
export function slipText(slip: MilestoneSlip): string | null {
  if (slip.reschedules === 0) return null;
  const times = `rescheduled ${slip.reschedules} time${slip.reschedules === 1 ? "" : "s"}`;
  const later =
    slip.later_moves === 0 ? "" : `, ${slip.later_moves} of them later`;
  const net =
    slip.net_days === 0
      ? "back on its original date"
      : slip.net_days > 0
        ? `${calendarDays(slip.net_days)} later than the original ${slip.original_due}`
        : `${calendarDays(-slip.net_days)} earlier than the original ${slip.original_due}`;
  return `Originally due ${slip.original_due}; ${times}${later}; now ${net}.`;
}

/** Says how and when a milestone was achieved; `null` if it was not. */
export function achievedText(
  m: Pick<
    MilestoneView,
    "achieved" | "achieved_by" | "achieved_on" | "done_reason"
  >,
): string | null {
  if (!m.achieved) return null;
  const on = m.achieved_on ? ` on ${m.achieved_on}` : "";
  if (m.achieved_by === "tasks")
    return `Achieved${on}: every task it waits for is done.`;
  const why = m.done_reason ? ` Reason: ${m.done_reason}` : "";
  return `Marked done${on}.${why}`;
}

/** What is still open, for a linked milestone. */
export function openTasksText(
  m: Pick<
    MilestoneView,
    "completion_mode" | "linked_tasks" | "open_tasks" | "achieved"
  >,
): string | null {
  if (m.completion_mode !== "linked" || m.achieved) return null;
  return `${m.open_tasks} of ${m.linked_tasks} task${m.linked_tasks === 1 ? "" : "s"} still open`;
}

/** One line for a milestone row, readable aloud. */
export function summaryText(m: MilestoneView): string {
  const state = m.schedule.state
    ? `, ${STATE_TEXT[m.schedule.state].toLowerCase()}`
    : "";
  return `${m.name}: ${STATUS_TEXT[m.status].toLowerCase()}, due ${m.due}${state}`;
}

/** Whether completing it needs a reason: linked, with tasks still open. */
export function completionNeedsReason(
  m: Pick<MilestoneView, "completion_mode" | "open_tasks" | "achieved">,
): boolean {
  return !m.achieved && m.completion_mode === "linked" && m.open_tasks > 0;
}

/** Counts in the order a reader should look: late first. */
export function statusParts(
  by: Record<MilestoneStatus, number>,
): { status: MilestoneStatus; label: string; count: number }[] {
  return STATUS_ORDER.map((status) => ({
    status,
    label: STATUS_TEXT[status],
    count: by[status] ?? 0,
  }));
}
