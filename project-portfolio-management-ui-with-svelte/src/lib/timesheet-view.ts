// Timesheets (NC-1 slice 6, LT-11): one person's week by day and plan, submitting it, and an approver's decision.
// Pure: which week is "this week" in the reader's time zone, moving a week back and forward, checking a typed week
// and a person reference, the time in words, the steps a status allows, the request bodies, and the refusal sentence
// for each step.
//
// The rules mirror the service: `src/timesheets.rs` (weeks are ISO 8601 weeks, Monday to Sunday, named such as
// `2026-W46`; whole minutes; billable and non-billable always apart; an empty week has no total and the reason
// `no_entries`; submit from open or rejected, and only a week with entries; decide only a submitted week; a rejection
// must say why; approved is final) and `src/controllers/timesheets.rs` (a person is a `person:` or `worker:` reference
// whose id is a UUID; only the owner submits; only an approver decides, never on their own week). The service decides;
// this only saves a round trip and lets the page say what is wrong, in the reader's language.
//
// The week is the service's ISO week whatever the person's first day of the week (WW-1.4): a timesheet week that
// started on a Sunday would be a different set of days from the one the service approves and locks.

import { t, tn, type StringKey } from "./i18n.svelte.js";
import { DASH, formatInteger } from "./locale-format.js";
import type {
  TimesheetDecideBody,
  TimesheetStatus,
  TimesheetSubmitBody,
} from "./api/ppm-types.js";

const DAY_MS = 86_400_000;

/** A civil date (`YYYY-MM-DD`) as a UTC midnight count of days, or `null` when it is not a real date. */
function dayCount(civil: string): number | null {
  const m = /^(\d{4})-(\d{2})-(\d{2})$/.exec(civil);
  if (!m) return null;
  const [y, mo, d] = [Number(m[1]), Number(m[2]), Number(m[3])];
  const ms = Date.UTC(y, mo - 1, d);
  const back = new Date(ms);
  if (
    back.getUTCFullYear() !== y ||
    back.getUTCMonth() !== mo - 1 ||
    back.getUTCDate() !== d
  )
    return null;
  return ms / DAY_MS;
}

function civilOf(days: number): string {
  const d = new Date(days * DAY_MS);
  const pad = (n: number, w = 2) => String(n).padStart(w, "0");
  return `${pad(d.getUTCFullYear(), 4)}-${pad(d.getUTCMonth() + 1)}-${pad(d.getUTCDate())}`;
}

/** The Monday (as a day count) of ISO week 1 of `year`: the week holding 4 January. */
function week1Monday(year: number): number {
  const jan4 = Date.UTC(year, 0, 4) / DAY_MS;
  const weekday = (new Date(jan4 * DAY_MS).getUTCDay() + 6) % 7; // Monday 0 … Sunday 6
  return jan4 - weekday;
}

/** The ISO week (`2026-W46`) holding a civil date, or `null` when the date is not real. */
export function isoWeekOf(civil: string): string | null {
  const days = dayCount(civil);
  if (days === null) return null;
  const weekday = (new Date(days * DAY_MS).getUTCDay() + 6) % 7;
  // The ISO year is the year of the week's Thursday.
  const thursday = days - weekday + 3;
  const year = new Date(thursday * DAY_MS).getUTCFullYear();
  const week = Math.floor((thursday - week1Monday(year)) / 7) + 1;
  return `${String(year).padStart(4, "0")}-W${String(week).padStart(2, "0")}`;
}

/**
 * A typed week as the service names it (`2026-W46`), or `null` when it is not one: the service's `week_start` refuses
 * week 0, a 53rd week in a year that has 52, and anything else. Spaces around it and a lower-case `w` are accepted.
 */
export function parseWeek(raw: string): string | null {
  const m = /^(\d{4})-[Ww](\d{1,2})$/.exec(raw.trim());
  if (!m) return null;
  const [year, week] = [Number(m[1]), Number(m[2])];
  if (week < 1 || week > 53) return null;
  const name = `${m[1]}-W${String(week).padStart(2, "0")}`;
  const monday = week1Monday(year) + (week - 1) * 7;
  return isoWeekOf(civilOf(monday)) === name ? name : null;
}

/** The Monday of an ISO week, as a civil date, or `null` when the week is not one. */
export function weekMonday(week: string): string | null {
  const name = parseWeek(week);
  if (!name) return null;
  const [year, w] = [Number(name.slice(0, 4)), Number(name.slice(6))];
  return civilOf(week1Monday(year) + (w - 1) * 7);
}

/** The week `delta` weeks after `week` (before, when negative), or `null` when `week` is not one. */
export function shiftWeek(week: string, delta: number): string | null {
  const monday = weekMonday(week);
  if (!monday) return null;
  const days = dayCount(monday);
  return days === null ? null : isoWeekOf(civilOf(days + delta * 7));
}

/** Today's civil date in the IANA zone `tz` (the reader's own, from the browser), falling back to UTC. */
export function todayIn(tz: string, now: Date = new Date()): string {
  let parts: Intl.DateTimeFormatPart[];
  try {
    parts = new Intl.DateTimeFormat("en-CA", {
      timeZone: tz,
      year: "numeric",
      month: "2-digit",
      day: "2-digit",
    }).formatToParts(now);
  } catch {
    return civilOf(Math.floor(now.getTime() / DAY_MS));
  }
  const part = (type: string) =>
    parts.find((p) => p.type === type)?.value ?? "";
  return `${part("year")}-${part("month")}-${part("day")}`;
}

/** This week, in the reader's time zone `tz`. */
export function thisWeek(tz: string, now: Date = new Date()): string {
  return isoWeekOf(todayIn(tz, now)) ?? "1970-W01";
}

/** What is wrong with a typed person reference, or `null`: the service takes `person:` or `worker:` and a UUID. */
export function personProblem(raw: string): "person_not_reference" | null {
  return /^(person|worker):[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(
    raw.trim(),
  )
    ? null
    : "person_not_reference";
}

/** The UUID a person reference names, lower case, or `null`. */
function idOf(ref: string): string | null {
  const m = /^(?:person|worker):(.+)$/i.exec(ref.trim());
  return m ? m[1]!.toLowerCase() : null;
}

/** Whether two person references name the same person (the service compares the id, ignoring case and the prefix). */
export function samePerson(a: string, b: string): boolean {
  const x = idOf(a);
  return x !== null && x === idOf(b);
}

/**
 * Whole minutes as hours and minutes in words ("7 hrs 30 mins", "7 awr 30 munud"); `null` (no evidence) is "—",
 * never 0. The units are catalogue plurals, not `Intl` units: some browsers' locale data has no Welsh units and
 * would show English.
 */
export function durationText(minutes: number | null | undefined): string {
  if (minutes === null || minutes === undefined || !Number.isFinite(minutes))
    return DASH;
  const whole = Math.abs(Math.round(minutes));
  const h = Math.floor(whole / 60);
  const m = whole % 60;
  const hours = () => tn("p.timesheet.hours", h, { n: formatInteger(h) });
  const mins = () => tn("p.timesheet.minutes", m, { n: formatInteger(m) });
  if (h === 0) return mins();
  if (m === 0) return hours();
  return t("p.timesheet.duration_hm", { h: hours(), m: mins() });
}

/** The statuses, each with its words and a shape, so it is never told by colour alone. */
export const STATUS_KEY: Record<TimesheetStatus, StringKey> = {
  open: "p.timesheet.status_open",
  submitted: "p.timesheet.status_submitted",
  approved: "p.timesheet.status_approved",
  rejected: "p.timesheet.status_rejected",
};
export const STATUS_SYMBOL: Record<TimesheetStatus, string> = {
  open: "○",
  submitted: "◔",
  approved: "✓",
  rejected: "✗",
};

/** A status as the service sent it, narrowed; anything unknown reads as open (the service's own default). */
export function statusOf(raw: string): TimesheetStatus {
  return raw === "submitted" || raw === "approved" || raw === "rejected"
    ? raw
    : "open";
}

/** Whether the owner may submit the week: open or rejected, and it has time in it. */
export function canSubmit(
  status: TimesheetStatus,
  total: number | null,
): boolean {
  return (status === "open" || status === "rejected") && total !== null;
}

/** Whether an approver may decide the week: only once it is submitted. */
export function canDecide(status: TimesheetStatus): boolean {
  return status === "submitted";
}

/** The submit body: the caller's own week (the service takes the person from the token). */
export function submitBody(week: string): TimesheetSubmitBody {
  return { week };
}

/** What is wrong with a decision before it is sent, or `null`. */
export function decisionProblem(
  decision: "approved" | "rejected",
  comment: string,
  person: string,
  own: string | null,
): "own_week" | "comment_required" | null {
  if (own !== null && samePerson(person, own)) return "own_week";
  if (decision === "rejected" && comment.trim() === "")
    return "comment_required";
  return null;
}

/** The decide body. A blank comment on an approval is left out. */
export function decideBody(
  person: string,
  week: string,
  decision: "approved" | "rejected",
  comment: string,
): TimesheetDecideBody {
  const body: TimesheetDecideBody = {
    person: person.trim(),
    week,
    decision,
  };
  if (comment.trim() !== "") body.comment = comment.trim();
  return body;
}

/** A step a person takes from this screen. */
export type TimesheetStep = "read_own" | "read_other" | "submit" | "decide";

/**
 * The catalogue sentence for a refusal of `step`, by HTTP status: 403 that the caller may not (another's week without
 * the approver right, submitting another's week, deciding without the right or on their own week), 409 that the week
 * has moved on (already submitted or approved, or not submitted), 422 what was not accepted (an empty week, a week or
 * person the service does not read, a rejection with no reason, or a sign-in identity with no person reference).
 * `null` for anything else, so the page uses the general rule.
 */
export function timesheetRefusalKey(
  step: TimesheetStep,
  status: number,
):
  | "p.timesheet.refused_read_forbidden"
  | "p.timesheet.refused_submit_forbidden"
  | "p.timesheet.refused_decide_forbidden"
  | "p.timesheet.refused_submit_conflict"
  | "p.timesheet.refused_decide_conflict"
  | "p.timesheet.refused_not_addressable"
  | "p.timesheet.refused_read_invalid"
  | "p.timesheet.refused_submit_invalid"
  | "p.timesheet.refused_decide_invalid"
  | null {
  if (status === 403) {
    if (step === "submit") return "p.timesheet.refused_submit_forbidden";
    if (step === "decide") return "p.timesheet.refused_decide_forbidden";
    return "p.timesheet.refused_read_forbidden";
  }
  if (status === 409) {
    if (step === "submit") return "p.timesheet.refused_submit_conflict";
    if (step === "decide") return "p.timesheet.refused_decide_conflict";
    return null;
  }
  if (status === 422) {
    if (step === "read_own") return "p.timesheet.refused_not_addressable";
    if (step === "read_other") return "p.timesheet.refused_read_invalid";
    if (step === "submit") return "p.timesheet.refused_submit_invalid";
    return "p.timesheet.refused_decide_invalid";
  }
  return null;
}
