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
import { businessDays, calendarDays } from "./units.js";
import { t, tn } from "./i18n.svelte.js";
import { formatDate } from "#lib/locale-format.js";

export const STATUS_ORDER: readonly MilestoneStatus[] = [
  "late",
  "due_soon",
  "upcoming",
  "achieved",
];

export const STATUS_TEXT: Record<MilestoneStatus, string> = {
  get late() {
    return t("p.milestones.late");
  },
  get due_soon() {
    return t("p.milestones.due_soon");
  },
  get upcoming() {
    return t("p.milestones.upcoming");
  },
  get achieved() {
    return t("p.milestones.achieved");
  },
};

/** A decorative shape per status (the word carries the meaning). */
export const STATUS_SYMBOL: Record<MilestoneStatus, string> = {
  late: "■",
  due_soon: "▲",
  upcoming: "○",
  achieved: "✓",
};

export const STATE_TEXT: Record<MilestoneScheduleState, string> = {
  get on_track() {
    return t("l.milestone_view.on_track");
  },
  get at_risk() {
    return t("l.milestone_view.at_risk");
  },
  get forecast_late() {
    return t("l.milestone_view.forecast_late");
  },
  get no_forecast() {
    return t("l.milestone_view.no_forecast");
  },
};

const NO_FORECAST_REASON: Record<string, string> = {
  get no_linked_tasks() {
    return t("l.milestone_view.it_is_not_linked_to");
  },
  get no_scheduled_tasks() {
    return t("l.milestone_view.its_open_tasks_have_no");
  },
  get cycle() {
    return t("l.milestone_view.the_plan_s_dependencies_contain");
  },
};

/** The forecast as a sentence, or `null` when the milestone is achieved. */
export function forecastText(
  schedule: MilestoneSchedule,
  due: string,
): string | null {
  if (schedule.state === null) return null;
  if (schedule.state === "no_forecast" || schedule.forecast === null) {
    return (
      NO_FORECAST_REASON[schedule.reason ?? ""] ??
      t("l.milestone_view.there_is_no_forecast")
    );
  }
  const slack = schedule.slack_days ?? 0;
  const partial = schedule.partial
    ? t("l.milestone_view.some_open_tasks_cannot_be")
    : "";
  if (slack < 0) {
    return t("l.milestone_view.forecast_to_finish_after_it", {
      forecast: schedule.forecast,
      businessDays: businessDays(-slack),
      due,
      partial,
    });
  }
  if (slack === 0)
    return t("l.milestone_view.forecast_to_finish_on_the", { due, partial });
  return t("l.milestone_view.forecast_to_finish_before_it", {
    forecast: schedule.forecast,
    businessDays: businessDays(slack),
    due,
    partial,
  });
}

/** How far a milestone has moved, or `null` when it never has. */
export function slipText(slip: MilestoneSlip): string | null {
  if (slip.reschedules === 0) return null;
  const times = tn("l.milestone_view.rescheduled", slip.reschedules, {
    count: slip.reschedules,
  });
  const later =
    slip.later_moves === 0
      ? ""
      : t("l.milestone_view.of_them_later", { later_moves: slip.later_moves });
  const net =
    slip.net_days === 0
      ? t("l.milestone_view.back_on_its_original_date")
      : slip.net_days > 0
        ? t("l.milestone_view.later_than_the_original", {
            calendarDays: calendarDays(slip.net_days),
            original_due: slip.original_due,
          })
        : t("l.milestone_view.earlier_than_the_original", {
            calendarDays: calendarDays(-slip.net_days),
            original_due: slip.original_due,
          });
  return t("l.milestone_view.originally_due_now", {
    original_due: slip.original_due,
    times,
    later,
    net,
  });
}

/** Says how and when a milestone was achieved; `null` if it was not. */
export function achievedText(
  m: Pick<
    MilestoneView,
    "achieved" | "achieved_by" | "achieved_on" | "done_reason"
  >,
): string | null {
  if (!m.achieved) return null;
  const on = m.achieved_on
    ? t("l.milestone_view.on", { achieved_on: formatDate(m.achieved_on) })
    : "";
  if (m.achieved_by === "tasks")
    return t("l.milestone_view.achieved_every_task_it_waits", { on });
  const why = m.done_reason
    ? t("l.milestone_view.reason", { done_reason: m.done_reason })
    : "";
  return t("l.milestone_view.marked_done", { on, why });
}

/** What is still open, for a linked milestone. */
export function openTasksText(
  m: Pick<
    MilestoneView,
    "completion_mode" | "linked_tasks" | "open_tasks" | "achieved"
  >,
): string | null {
  if (m.completion_mode !== "linked" || m.achieved) return null;
  return tn("l.milestone_view.open_of_linked", m.linked_tasks, {
    open_tasks: m.open_tasks,
    linked_tasks: m.linked_tasks,
  });
}

/** One line for a milestone row, readable aloud. */
export function summaryText(m: MilestoneView): string {
  const state = m.schedule.state
    ? `, ${STATE_TEXT[m.schedule.state].toLowerCase()}`
    : "";
  return t("l.milestone_view.due", {
    name: m.name,
    toLowerCase: STATUS_TEXT[m.status].toLowerCase(),
    due: m.due,
    state,
  });
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
