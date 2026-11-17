// Pure helpers for a plan's planned gates and programme status (`/plans/[pid]/programme`, DP-5, NC-1 slice 11): the
// changes a status allows, what saving gate dates sends, the checks before a decision is sent, and each refusal's
// sentence. A programme is active until a recorded decision pauses or stops it; stopped is final and keeps the record.

import { ApiError } from "./api/client.js";
import type { StringKey } from "./i18n.svelte.js";
import type { ProgrammeStatus } from "./api/ppm-types.js";

/** A reason needs at least this many characters (`MIN_REASON`). */
export const REASON_MIN = 10;

/** The statuses a programme may move to from `current`: none once stopped, never the one it has. */
export function statusChoices(current: ProgrammeStatus): ProgrammeStatus[] {
  if (current === "stopped") return [];
  return (["active", "paused", "stopped"] as const).filter(
    (s) => s !== current,
  );
}

/** Each change's catalogue key: resuming, pausing or stopping. */
export const CHANGE_LABELS: Record<ProgrammeStatus, StringKey> = {
  active: "p.programme.do_resume",
  paused: "p.programme.do_pause",
  stopped: "p.programme.do_stop",
};

/** The gate dates that changed and are set, as `[gate, date]`; an emptied date cannot be sent (no route clears one). */
export function changedGates(
  form: Readonly<Record<string, string>>,
  stored: ReadonlyArray<{ gate: string; planned_on: string }>,
): Array<[string, string]> {
  const before = new Map(stored.map((g) => [g.gate, g.planned_on]));
  return Object.entries(form).filter(
    ([gate, date]) => date !== "" && date !== before.get(gate),
  );
}

/** Every problem with a status decision before it is sent. */
export function decisionProblems(form: {
  status: string;
  reason: string;
}): StringKey[] {
  const out: StringKey[] = [];
  if (form.status === "") out.push("p.programme.problem_status");
  if ([...form.reason.trim()].length < REASON_MIN)
    out.push("p.programme.problem_reason");
  return out;
}

/** The sentence for a refused decision: the right it needs, or what a 409 or 422 means here; else `null`. */
export function programmeRefusalKey(err: unknown): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  if (err.status === 403) return "p.programme.refused_decider";
  if (err.status === 409) return "p.programme.conflict_status";
  if (err.status === 422) return "p.programme.refused_not_programme";
  return null;
}
