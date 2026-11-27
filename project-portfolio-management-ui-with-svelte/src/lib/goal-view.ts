// The rules of a plan's goals page (`/plans/[pid]/goals`, FR-12, CF-2; NC-1 slice 12 part n), kept out of the
// component so they are unit-tested: what a goal form sends, the limits the service checks (a title neither blank
// nor over 512 characters, at most 256 goals), whether an edit changed anything, and which sentence a refusal is.

import type { Goal, GoalStatus } from "#lib/api/types.js";

/** The service's limit on a goal title, in characters (`validation::MAX_ITEM_LEN`). */
export const TITLE_MAX = 512;
/** The most goals a plan holds (`validation::MAX_ARRAY_LEN`). */
export const GOALS_MAX = 256;
/** The named statuses; any other is a custom one, named by the person. */
export const STATUSES = [
  "NotStarted",
  "InProgress",
  "Achieved",
  "Missed",
] as const;
export type NamedStatus = (typeof STATUSES)[number];
/** What the status picker holds: none, a named status, or a custom one. */
export type StatusChoice = "" | NamedStatus | "Custom";

/** A goal as its form holds it: every field a string. */
export interface GoalForm {
  title: string;
  description: string;
  target_date: string;
  status: StatusChoice;
  custom: string;
}

export const EMPTY_FORM: GoalForm = {
  title: "",
  description: "",
  target_date: "",
  status: "",
  custom: "",
};

/** The form for a stored goal. */
export function formOf(g: Goal): GoalForm {
  const s = g.status ?? null;
  return {
    title: g.title,
    description: g.description ?? "",
    target_date: g.target_date ?? "",
    status: s === null ? "" : typeof s === "string" ? s : "Custom",
    custom: s !== null && typeof s === "object" ? s.Custom : "",
  };
}

/** Whether `YYYY-MM-DD` names a day on the calendar. */
function realDay(s: string): boolean {
  const m = /^(\d{4})-(\d{2})-(\d{2})$/.exec(s);
  if (!m) return false;
  const [y, mo, d] = [Number(m[1]), Number(m[2]), Number(m[3])];
  const at = new Date(Date.UTC(y, mo - 1, d));
  return (
    at.getUTCFullYear() === y &&
    at.getUTCMonth() === mo - 1 &&
    at.getUTCDate() === d
  );
}

/** The body a form sends, or the catalogue key of the first problem with it. */
export function goalBody(f: GoalForm): { body: Goal } | { problem: string } {
  const title = f.title.trim();
  if (title === "") return { problem: "p.goals.title_required" };
  if ([...title].length > TITLE_MAX)
    return { problem: "p.goals.title_too_long" };
  const target = f.target_date.trim();
  if (target !== "" && !realDay(target)) return { problem: "p.goals.date_bad" };
  let status: GoalStatus | null = null;
  if (f.status === "Custom") {
    const name = f.custom.trim();
    if (name === "") return { problem: "p.goals.custom_required" };
    status = { Custom: name };
  } else if (f.status !== "") status = f.status;
  const description = f.description.trim();
  return {
    body: {
      title,
      description: description === "" ? null : description,
      target_date: target === "" ? null : target,
      status,
    },
  };
}

/** The status as a comparable string (`""` for none). */
function statusText(s: GoalStatus | null | undefined): string {
  if (s === null || s === undefined) return "";
  return typeof s === "string" ? s : `Custom:${s.Custom}`;
}

/** Whether two goals say the same (a missing field and `null` are the same), so an unchanged edit sends nothing. */
export function sameGoal(a: Goal, b: Goal): boolean {
  return (
    a.title === b.title &&
    (a.description ?? null) === (b.description ?? null) &&
    (a.target_date ?? null) === (b.target_date ?? null) &&
    statusText(a.status) === statusText(b.status)
  );
}

/** How a status reads: a catalogue key for a named one, the person's own words for a custom one, or nothing. */
export function statusLabel(
  s: GoalStatus | null | undefined,
): { key: string } | { custom: string } | null {
  if (s === null || s === undefined) return null;
  if (typeof s === "string") return { key: `p.goals.st.${s}` };
  return { custom: s.Custom };
}

/**
 * The sentence for a refused write: 412 is a goal list that changed since it was read (CF-2), 404 a goal no longer
 * there; both are followed by a fresh read. Anything else is the plain "could not" sentence with the service's reason.
 */
export function refusalKey(action: "save" | "remove", status: number): string {
  if (status === 412) return "p.goals.changed";
  if (status === 404) return "p.goals.gone";
  return `p.goals.could_not_${action}`;
}
