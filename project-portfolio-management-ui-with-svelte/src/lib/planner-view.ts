// The day's plan (`/me`, NC-1 slice 12 part h, PL-4): time blocked for a task on one local day, read with
// `GET /api/me/plan?day=&tz=`, added with `POST /api/me/planned-slots` and removed with its `…/delete` twin. Pure: the
// day moved back and forward, a typed time on the shown day turned into a UTC instant in the reader's IANA zone (the
// zone the day is read in), the rules of a slot said and checked before anything is sent, the clashes a slot is in, and
// the refusal sentence for each action and status.
//
// The rules mirror the service (`domain/src/planner.rs`, `controllers/planner.rs`): a slot ends after it starts and lasts
// at most 12 hours; it is its owner's alone and never changes the task's due date; overlapping slots are allowed and
// reported as clashes (back to back is not a clash); a day is built from local midnights, so it is 23 or 25 hours when
// the clocks change; a person holds at most 2,000 slots; a slot on a task the caller can no longer read is left out and
// counted. Every 422 is `unprocessable` with English text, so the rules are checked here first and a 422 is one
// sentence per action. The service decides.

import { ApiError } from "./api/client.js";
import type { PlannedSlotBody } from "./api/ppm-types.js";
import type { StringKey } from "./i18n.svelte.js";

/** The longest slot, in minutes (`MAX_SLOT`, 12 hours). */
export const SLOT_MAX_MINUTES = 12 * 60;
/** The most live slots one person may hold (`MAX_SLOTS`). */
export const SLOTS_MAX = 2000;

const DAY_MS = 86_400_000;
const CIVIL = /^(\d{4})-(\d{2})-(\d{2})$/;
const TIME = /^([01]\d|2[0-3]):([0-5]\d)(?::([0-5]\d))?$/;

/** A civil date's parts, or `null` when it is not a real date. */
function civilParts(civil: string): [number, number, number] | null {
  const m = CIVIL.exec(civil.trim());
  if (!m) return null;
  const [y, mo, d] = [Number(m[1]), Number(m[2]), Number(m[3])];
  const back = new Date(Date.UTC(y, mo - 1, d));
  if (
    back.getUTCFullYear() !== y ||
    back.getUTCMonth() !== mo - 1 ||
    back.getUTCDate() !== d
  )
    return null;
  return [y, mo, d];
}

/** Whether `civil` is a real calendar date (`YYYY-MM-DD`). */
export function isDay(civil: string): boolean {
  return civilParts(civil) !== null;
}

/** The day `delta` days after `day` (before, when negative), or `null` when `day` is not a date. */
export function shiftDay(day: string, delta: number): string | null {
  const p = civilParts(day);
  if (!p) return null;
  return new Date(Date.UTC(p[0], p[1] - 1, p[2]) + delta * DAY_MS)
    .toISOString()
    .slice(0, 10);
}

/** The zone's offset from UTC at instant `ms`, in milliseconds (local wall clock minus UTC), or `null` for an unknown zone. */
function offsetAt(tz: string, ms: number): number | null {
  let parts: Intl.DateTimeFormatPart[];
  try {
    parts = new Intl.DateTimeFormat("en-US", {
      timeZone: tz,
      hourCycle: "h23",
      year: "numeric",
      month: "2-digit",
      day: "2-digit",
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit",
    }).formatToParts(new Date(ms));
  } catch {
    return null;
  }
  const get = (type: string) =>
    Number(parts.find((p) => p.type === type)?.value ?? "0");
  const hour = get("hour") === 24 ? 0 : get("hour");
  const wall = Date.UTC(
    get("year"),
    get("month") - 1,
    get("day"),
    hour,
    get("minute"),
    get("second"),
  );
  return wall - Math.floor(ms / 1000) * 1000;
}

/**
 * A time of day (`HH:MM`, as a time input gives it) on a civil `day`, read in the IANA zone `tz`, as a UTC instant
 * (`2026-11-18T09:00:00Z`). `"skipped"` when the clocks jump over that time on that day; when they go back and the
 * time happens twice, the first (as the service reads a repeated midnight). `null` when the day or time is not one.
 */
export function localToUtc(
  day: string,
  time: string,
  tz: string,
): string | "skipped" | null {
  const p = civilParts(day);
  const m = TIME.exec(time.trim());
  if (!p || !m) return null;
  const wall = Date.UTC(
    p[0],
    p[1] - 1,
    p[2],
    Number(m[1]),
    Number(m[2]),
    Number(m[3] ?? "0"),
  );
  const found = new Set<number>();
  for (const probe of [wall - DAY_MS / 2, wall, wall + DAY_MS / 2]) {
    const o = offsetAt(tz, probe);
    if (o === null) return null;
    const at = wall - o;
    if (offsetAt(tz, at) === o) found.add(at);
  }
  if (found.size === 0) return "skipped";
  return new Date(Math.min(...found)).toISOString().replace(".000Z", "Z");
}

/** A slot as typed: the task, and the start and end as times of day on the shown day. */
export interface SlotForm {
  taskPid: string;
  start: string;
  end: string;
}

/** What may be wrong with a slot. */
export type SlotProblem =
  | "task_needed"
  | "start_needed"
  | "end_needed"
  | "start_skipped"
  | "end_skipped"
  | "end_not_after_start"
  | "too_long";

/** The sentence for each problem. */
export const SLOT_PROBLEM_KEY: Record<SlotProblem, StringKey> = {
  task_needed: "p.planner.problem_task",
  start_needed: "p.planner.problem_start",
  end_needed: "p.planner.problem_end",
  start_skipped: "p.planner.problem_start_skipped",
  end_skipped: "p.planner.problem_end_skipped",
  end_not_after_start: "p.planner.problem_order",
  too_long: "p.planner.problem_too_long",
};

/**
 * The body for `POST /api/me/planned-slots`, or every problem at once. Both times are on the shown `day` in `tz`; a
 * slot that runs past midnight is not offered (it would need the next day's date).
 */
export function slotBody(
  form: SlotForm,
  day: string,
  tz: string,
): { body: PlannedSlotBody } | { problems: SlotProblem[] } {
  const problems: SlotProblem[] = [];
  if (!form.taskPid) problems.push("task_needed");
  const a = form.start.trim() ? localToUtc(day, form.start, tz) : null;
  const b = form.end.trim() ? localToUtc(day, form.end, tz) : null;
  if (a === null) problems.push("start_needed");
  else if (a === "skipped") problems.push("start_skipped");
  if (b === null) problems.push("end_needed");
  else if (b === "skipped") problems.push("end_skipped");
  if (a && b && a !== "skipped" && b !== "skipped") {
    const span = (Date.parse(b) - Date.parse(a)) / 60_000;
    if (span <= 0) problems.push("end_not_after_start");
    else if (span > SLOT_MAX_MINUTES) problems.push("too_long");
  }
  if (problems.length > 0 || !a || !b || a === "skipped" || b === "skipped")
    return { problems };
  return { body: { task_pid: form.taskPid, starts_at: a, ends_at: b } };
}

/** How many other slots `slotPid` overlaps, from the service's clash groups (0 when it clashes with none). */
export function clashCount(slotPid: string, clashes: string[][]): number {
  const group = clashes.find((g) => g.includes(slotPid));
  return group ? Math.max(0, group.length - 1) : 0;
}

/** An action on the day's plan. */
export type PlannerAction = "read" | "plan" | "remove";

/**
 * The catalogue sentence for a refusal, or `null` for the general rule. Reading: a 422 is a time zone the service
 * refused. Planning: a 422 is a slot out of the rules (checked first) or one past the 2,000 a person may hold; a 404
 * a task that is gone or in a plan the caller cannot read. Removing: a 404 a slot already gone.
 */
export function plannerRefusalKey(
  action: PlannerAction,
  err: unknown,
): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  if (action === "read")
    return err.status === 422 ? "p.planner.refused_zone" : null;
  if (action === "plan") {
    if (err.status === 422) return "p.planner.refused_slot";
    if (err.status === 404) return "p.planner.refused_task";
    return null;
  }
  return err.status === 404 ? "p.planner.refused_gone" : null;
}
