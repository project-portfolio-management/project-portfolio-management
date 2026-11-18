// The working day and time away (`/plans/[pid]/calendar`, NC-1 slice 12 part i): a working day and week (`POST
// /api/working-time`) and a person's leave or other time away from project work (`POST /api/non-working`), which
// together stop somebody on leave reading as idle in utilization. The service's rules (`controllers/effort.rs`), said
// and checked before anything is sent; the service still decides.
//
// A working day is 1 to 1440 minutes and a working week 1 to 7 days, for this plan (`plan:{pid}`) or as the
// organization's default (no scope). Each is a new record, not a change: a second one for the same plan is refused
// (409), and there is no route to change or remove one, so it is asked first. Time away names a person (`person:` or
// `worker:` and an id), a first and a last day (the last not before the first), and a kind: leave, a holiday, study
// leave or a duty outside the project; a blank note is not sent. It cannot be changed or removed once recorded, so it
// is asked first. Every 422 is `unprocessable` with English text (a service gap).

import { ApiError } from "./api/client.js";
import type {
  NonWorkingBody,
  NonWorkingKind,
  WorkingTimeBody,
} from "./api/ppm-types.js";
import type { StringKey } from "./i18n.svelte.js";
import { personProblem } from "./timesheet-view.js";

/** A real calendar date (`YYYY-MM-DD`; 30 February is not one). */
function isCivilDate(s: string): boolean {
  const m = /^(\d{4})-(\d{2})-(\d{2})$/.exec(s);
  if (!m) return false;
  const d = new Date(Date.UTC(Number(m[1]), Number(m[2]) - 1, Number(m[3])));
  return (
    d.getUTCFullYear() === Number(m[1]) &&
    d.getUTCMonth() === Number(m[2]) - 1 &&
    d.getUTCDate() === Number(m[3])
  );
}

/** The kinds of time away, in the service's order. */
export const NON_WORKING_KINDS: readonly NonWorkingKind[] = [
  "leave",
  "holiday",
  "study_leave",
  "non_project_duty",
];

export const NON_WORKING_KEY: Record<NonWorkingKind, StringKey> = {
  leave: "p.workingtime.kind_leave",
  holiday: "p.workingtime.kind_holiday",
  study_leave: "p.workingtime.kind_study_leave",
  non_project_duty: "p.workingtime.kind_non_project_duty",
};

/** The working day and week as typed. */
export interface WorkingTimeForm {
  scope: "plan" | "default";
  hours: number | null | undefined;
  minutes: number | null | undefined;
  days: number | null | undefined;
}

export function emptyWorkingTimeForm(): WorkingTimeForm {
  return { scope: "plan", hours: 7, minutes: 30, days: 5 };
}

const n = (v: number | null | undefined): number =>
  v === null || v === undefined ? 0 : Number(v);

/** The body to send, or every problem at once. */
export function workingTimeBody(
  form: WorkingTimeForm,
  planPid: string,
): { body: WorkingTimeBody } | { problems: StringKey[] } {
  const problems: StringKey[] = [];
  const h = n(form.hours);
  const m = n(form.minutes);
  if (!Number.isSafeInteger(h) || h < 0 || h > 24)
    problems.push("p.workingtime.problem_hours");
  if (!Number.isSafeInteger(m) || m < 0 || m > 59)
    problems.push("p.workingtime.problem_minutes");
  const minutes = h * 60 + m;
  if (problems.length === 0 && (minutes < 1 || minutes > 1440))
    problems.push("p.workingtime.problem_day");
  const days =
    form.days === null || form.days === undefined ? NaN : Number(form.days);
  if (!Number.isSafeInteger(days) || days < 1 || days > 7)
    problems.push("p.workingtime.problem_week");
  if (problems.length > 0) return { problems };
  const body: WorkingTimeBody = {
    minutes_per_day: minutes,
    working_days_per_week: days,
  };
  if (form.scope === "plan") body.scope_ref = `plan:${planPid}`;
  return { body };
}

/** Time away as typed. */
export interface NonWorkingForm {
  person: string;
  startsOn: string;
  endsOn: string;
  kind: NonWorkingKind;
  note: string;
}

export function emptyNonWorkingForm(): NonWorkingForm {
  return { person: "", startsOn: "", endsOn: "", kind: "leave", note: "" };
}

/** The body to send, or every problem at once. */
export function nonWorkingBody(
  form: NonWorkingForm,
): { body: NonWorkingBody } | { problems: StringKey[] } {
  const problems: StringKey[] = [];
  const person = form.person.trim();
  if (personProblem(person)) problems.push("p.workingtime.problem_person");
  const from = form.startsOn.trim();
  const to = form.endsOn.trim();
  if (!isCivilDate(from)) problems.push("p.workingtime.problem_starts");
  if (!isCivilDate(to)) problems.push("p.workingtime.problem_ends");
  if (isCivilDate(from) && isCivilDate(to) && to < from)
    problems.push("p.workingtime.problem_order");
  if (problems.length > 0) return { problems };
  const body: NonWorkingBody = {
    person_ref: person,
    starts_on: from,
    ends_on: to,
    kind: form.kind,
  };
  const note = form.note.trim();
  if (note) body.note = note;
  return { body };
}

/** The catalogue sentence for a refusal, or `null` for the general rule. */
export function workingTimeRefusalKey(
  action: "working_time" | "non_working",
  err: unknown,
): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  if (action === "working_time") {
    if (err.status === 409) return "p.workingtime.refused_exists";
    if (err.status === 422) return "p.workingtime.refused_working";
    if (err.status === 403) return "p.workingtime.refused_forbidden";
    return null;
  }
  if (err.status === 422) return "p.workingtime.refused_non_working";
  if (err.status === 403) return "p.workingtime.refused_forbidden";
  return null;
}
