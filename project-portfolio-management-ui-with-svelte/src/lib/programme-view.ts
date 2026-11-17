// Pure helpers for a plan's planned gates and programme status (`/plans/[pid]/programme`, DP-5, NC-1 slice 11): the
// changes a status allows, what saving gate dates sends, the checks before a decision is sent, and each refusal's
// sentence. A programme is active until a recorded decision pauses or stops it; stopped is final and keeps the record.

import { ApiError } from "./api/client.js";
import type { StringKey } from "./i18n.svelte.js";
import type { ProgrammeStatus, ProjectBufferBody } from "./api/ppm-types.js";

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

/** A buffer is 1 to 2000 business days (`set_buffer`). */
export const BUFFER_DAYS_MAX = 2000;

/**
 * What declaring a buffer sends, or the problem: a whole number of business days from 1 to 2000, or the rule (half the
 * baselined critical work, rounded up); a note only when written.
 */
export function bufferBody(form: {
  how: "days" | "rule";
  days: string;
  note: string;
}): { body: ProjectBufferBody } | { problem: StringKey } {
  const note = form.note.trim();
  const extra = note ? { note } : {};
  if (form.how === "rule")
    return { body: { rule: "half_of_critical_duration", ...extra } };
  const n = Number(form.days);
  if (!/^\d+$/.test(form.days.trim()) || n < 1 || n > BUFFER_DAYS_MAX)
    return { problem: "p.programme.problem_buffer_days" };
  return { body: { buffer_days: n, ...extra } };
}

/** The sentence for a refused declaration: by the rule, a 422 means there is no baseline with critical work to use. */
export function bufferRefusalKey(
  how: "days" | "rule",
  err: unknown,
): StringKey | null {
  if (!(err instanceof ApiError) || err.status !== 422) return null;
  return how === "rule"
    ? "p.programme.refused_buffer_rule"
    : "p.programme.refused_not_programme_buffer";
}
