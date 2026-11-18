// Recording time and reading it back (NC-1 slice 12 part h, LT-11, RM-10): a time entry on `/me/timesheet` (`POST
// /api/plans/{pid}/time-entries`, and the caller's own entries in a plan from `GET …/time-entries`), and the time
// report on `/reports` (`GET /api/time-reports`). Pure: the rules said and checked before anything is sent, the bodies
// and queries, the words for a group, and the refusal sentence for each action and status.
//
// The rules mirror the service (`controllers/effort.rs`, `controllers/timesheets.rs`, `domain/src/effort.rs`,
// `domain/src/timesheets.rs`): an entry is whole minutes, 1 to 1440 on one date, recorded against a `person:` or
// `worker:` reference (here the caller's own, as their week names it); it may name a task of the same plan; its category
// is capex, opex or unclassified (the default, not sent); it is billable or not (not, the default, not sent); an
// approved week is locked against new entries (409). There is no route to change or remove an entry, so recording one
// is final and is asked first. A report groups by person, plan, category, billable or month over the plans the caller
// can read (the others left out and counted), from a date to a date (the last 90 days to the deployment's today when
// left out; from not after to), with billable and not billable always apart and no entries said, never zero. Every 422
// is `unprocessable` with English text. The service decides.

import { ApiError } from "./api/client.js";
import type {
  EffortCategory,
  TimeEntry,
  TimeEntryBody,
  TimeReportGroupBy,
  TimeReportQuery,
} from "./api/ppm-types.js";
import { t, type StringKey } from "./i18n.svelte.js";
import { formatMonth } from "./locale-format.js";
import { isoWeekOf, personProblem, samePerson } from "./timesheet-view.js";

/** The most minutes one entry holds (a day). */
export const ENTRY_MAX_MINUTES = 1440;

/** The categories, the default first, each in words. */
export const CATEGORIES: readonly EffortCategory[] = [
  "unclassified",
  "capex",
  "opex",
];
export const CATEGORY_KEY: Record<EffortCategory, StringKey> = {
  unclassified: "p.timeentry.category_unclassified",
  capex: "p.timeentry.category_capex",
  opex: "p.timeentry.category_opex",
};

/** The words for a category the service sent; anything else reads as unclassified, as the service stores it. */
export function categoryKey(raw: string): StringKey {
  return CATEGORY_KEY[raw as EffortCategory] ?? CATEGORY_KEY.unclassified;
}

/** An entry as typed. Hours and minutes are number inputs' values (`null` or `undefined` when empty). */
export interface TimeEntryForm {
  planPid: string;
  taskPid: string;
  spentOn: string;
  hours: number | null | undefined;
  minutes: number | null | undefined;
  category: EffortCategory;
  billable: boolean;
  note: string;
}

/** An empty form on `today`. */
export function emptyEntryForm(today: string): TimeEntryForm {
  return {
    planPid: "",
    taskPid: "",
    spentOn: today,
    hours: null,
    minutes: null,
    category: "unclassified",
    billable: false,
    note: "",
  };
}

const realDate = (s: string): boolean => {
  const m = /^(\d{4})-(\d{2})-(\d{2})$/.exec(s.trim());
  if (!m) return false;
  const d = new Date(Date.UTC(Number(m[1]), Number(m[2]) - 1, Number(m[3])));
  return (
    d.getUTCFullYear() === Number(m[1]) &&
    d.getUTCMonth() === Number(m[2]) - 1 &&
    d.getUTCDate() === Number(m[3])
  );
};

/** The entry's whole minutes from its hours and minutes, or `null` when they are not whole numbers in range. */
export function entryMinutes(
  hours: number | null | undefined,
  minutes: number | null | undefined,
): number | null {
  const h = hours === null || hours === undefined ? 0 : Number(hours);
  const m = minutes === null || minutes === undefined ? 0 : Number(minutes);
  if (!Number.isInteger(h) || !Number.isInteger(m)) return null;
  if (h < 0 || h > 24 || m < 0 || m > 59) return null;
  const total = h * 60 + m;
  return total >= 1 && total <= ENTRY_MAX_MINUTES ? total : null;
}

/**
 * The plan and body for `POST /api/plans/{pid}/time-entries`, or every problem at once. `me` is the caller's own
 * person reference (from their week); `lockedWeeks` the ISO weeks the page knows are approved, so an entry in one is
 * refused before it is sent. The defaults (unclassified, not billable) and a blank note are left out.
 */
export function timeEntryBody(
  form: TimeEntryForm,
  me: string,
  lockedWeeks: ReadonlySet<string> = new Set(),
): { planPid: string; body: TimeEntryBody } | { problems: StringKey[] } {
  const problems: StringKey[] = [];
  if (!form.planPid) problems.push("p.timeentry.problem_plan");
  const day = form.spentOn.trim();
  if (!realDate(day)) problems.push("p.timeentry.problem_date");
  else {
    const week = isoWeekOf(day);
    if (week && lockedWeeks.has(week))
      problems.push("p.timeentry.problem_locked");
  }
  const minutes = entryMinutes(form.hours, form.minutes);
  if (minutes === null) problems.push("p.timeentry.problem_duration");
  if (problems.length > 0 || minutes === null) return { problems };
  const note = form.note.trim();
  return {
    planPid: form.planPid,
    body: {
      actor_ref: me,
      spent_on: day,
      minutes,
      ...(form.taskPid ? { task_pid: form.taskPid } : {}),
      ...(form.category !== "unclassified" ? { category: form.category } : {}),
      ...(form.billable ? { billable: true } : {}),
      ...(note ? { note } : {}),
    },
  };
}

/** The caller's own entries among a plan's (the service lists everyone's), newest first as it sent them. */
export function ownEntries(entries: TimeEntry[], me: string): TimeEntry[] {
  return entries.filter((e) => samePerson(e.actor_ref, me));
}

/** What the time report groups by, in the order offered, each in words. */
export const GROUP_BYS: readonly TimeReportGroupBy[] = [
  "plan",
  "person",
  "category",
  "billable",
  "month",
];
export const GROUP_BY_KEY: Record<TimeReportGroupBy, StringKey> = {
  plan: "p.timereport.by_plan",
  person: "p.timereport.by_person",
  category: "p.timereport.by_category",
  billable: "p.timereport.by_billable",
  month: "p.timereport.by_month",
};

/** The time report as asked. Dates and the person are `""` when left out; the plan `""` for every plan. */
export interface TimeReportForm {
  groupBy: TimeReportGroupBy;
  from: string;
  to: string;
  plan: string;
  person: string;
}

/** The query for `GET /api/time-reports`, or every problem at once. A left-out date is the service's default. */
export function timeReportQuery(
  form: TimeReportForm,
): { query: TimeReportQuery } | { problems: StringKey[] } {
  const problems: StringKey[] = [];
  const from = form.from.trim();
  const to = form.to.trim();
  if (from && !realDate(from)) problems.push("p.timereport.problem_from");
  if (to && !realDate(to)) problems.push("p.timereport.problem_to");
  if (from && to && realDate(from) && realDate(to) && from > to)
    problems.push("p.timereport.problem_order");
  const person = form.person.trim();
  if (person && personProblem(person))
    problems.push("p.timereport.problem_person");
  if (problems.length > 0) return { problems };
  return {
    query: {
      groupBy: form.groupBy,
      ...(from ? { from } : {}),
      ...(to ? { to } : {}),
      ...(form.plan ? { plan: form.plan } : {}),
      ...(person ? { person } : {}),
    },
  };
}

/**
 * A group's key in words: a plan by its name (or the start of its reference, its name not shown), a category and
 * billable in the catalogue's words, a month in the reader's locale, a person by the reference the service holds.
 */
export function groupText(
  by: string,
  key: string,
  planNames: Readonly<Record<string, string>>,
): string {
  if (by === "plan")
    return (
      planNames[key] ?? t("p.timereport.plan_unnamed", { ref: key.slice(0, 8) })
    );
  if (by === "category") return t(categoryKey(key));
  if (by === "billable")
    return t(
      key === "billable"
        ? "p.timereport.billable"
        : "p.timereport.not_billable",
    );
  if (by === "month") return formatMonth(key, "long");
  return key;
}

/** An action on recorded time. */
export type TimeAction = "record" | "entries" | "report";

/**
 * The catalogue sentence for a refusal, or `null` for the general rule. Recording: 409 the week is approved and
 * locked; 422 the entry out of the rules (checked first) or a task not of the plan; 404 the plan gone or not
 * readable; 403 the policy refusing it. Reading a plan's entries: 404 the plan gone. The report: 422 a query the
 * service would not take (checked first).
 */
export function timeRefusalKey(
  action: TimeAction,
  err: unknown,
): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  if (action === "record") {
    if (err.status === 409) return "p.timeentry.refused_locked";
    if (err.status === 422) return "p.timeentry.refused_invalid";
    if (err.status === 404) return "p.timeentry.refused_plan";
    if (err.status === 403) return "p.timeentry.refused_forbidden";
    return null;
  }
  if (action === "entries")
    return err.status === 404 ? "p.timeentry.refused_plan" : null;
  return err.status === 422 ? "p.timereport.refused" : null;
}
