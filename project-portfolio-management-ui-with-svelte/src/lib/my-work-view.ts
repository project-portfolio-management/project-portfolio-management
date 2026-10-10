// My work (LT-3) + float for the worker (SC-10): the plain sentence shown under
// a task about its schedule. Pure, so it is unit-testable without a component.
//
// A task with no schedule data shows nothing at all — never "0 business days of float".
// Negative float is said in words, not just colour, and the text is ordinary
// text, so a screen reader reads it with the rest of the row.

import type { MyTaskSchedule } from "#lib/api/ppm.js";
import { businessDays } from "./units.js";
import { t } from "./i18n.svelte.js";

export type FloatKind = "late" | "critical" | "float";

export interface FloatText {
  kind: FloatKind;
  /** The sentence, ready to render. */
  text: string;
}

const days = businessDays;

/**
 * The sentence for a task's schedule, or `null` when there is nothing to say
 * (no schedule data).
 */
export function floatText(
  schedule: MyTaskSchedule | null | undefined,
): FloatText | null {
  if (!schedule || !Number.isFinite(schedule.total_float)) return null;
  const f = schedule.total_float;
  if (f < 0) {
    return {
      kind: "late",
      text: t("l.my_work_view.this_is_already_late_for", { days: days(-f) }),
    };
  }
  if (f === 0 || schedule.critical) {
    return { kind: "critical", text: t("l.my_work_view.on_the_critical_path") };
  }
  return {
    kind: "float",
    text: t("l.my_work_view.of_float", { days: days(f) }),
  };
}
