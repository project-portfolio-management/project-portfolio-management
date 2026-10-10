// My work (LT-3) + float for the worker (SC-10): the plain sentence shown under
// a task about its schedule. Pure, so it is unit-testable without a component.
//
// A task with no schedule data shows nothing at all — never "0 business days of float".
// Negative float is said in words, not just colour, and the text is ordinary
// text, so a screen reader reads it with the rest of the row.

import type { MyTaskSchedule } from "#lib/api/ppm.js";

export type FloatKind = "late" | "critical" | "float";

export interface FloatText {
  kind: FloatKind;
  /** The sentence, ready to render. */
  text: string;
}

const days = (n: number): string => `${n} business day${n === 1 ? "" : "s"}`;

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
      text: `This is already late for the plan (${days(-f)} behind).`,
    };
  }
  if (f === 0 || schedule.critical) {
    return { kind: "critical", text: "On the critical path" };
  }
  return {
    kind: "float",
    text: `${days(f)} of float`,
  };
}
