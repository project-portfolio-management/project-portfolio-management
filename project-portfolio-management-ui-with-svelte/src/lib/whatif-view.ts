// The what-if page: building edits from what the form holds, saying each in
// words, and summarizing a preview. Pure and tested.

import type { SandboxEdit, SandboxPreview } from "#lib/api/ppm.js";

export type FieldValue = string | number | null | undefined;
const str = (v: FieldValue) => (v === null || v === undefined ? "" : String(v));
const blank = (v: FieldValue) => str(v).trim() === "";

export const MAX_EDITS = 100;

export type EditKind =
  "move" | "duration" | "constraint" | "add_link" | "remove_link";

export const EDIT_TEXT: Record<EditKind, string> = {
  move: "Move a task (set its start)",
  duration: "Change a task's duration",
  constraint: "Change a task's constraint",
  add_link: "Add a link",
  remove_link: "Remove a link",
};

export const CONSTRAINTS: Record<string, string> = {
  snet: "Start no earlier than",
  snlt: "Start no later than",
  fnet: "Finish no earlier than",
  fnlt: "Finish no later than",
  mso: "Must start on (hard)",
  mfo: "Must finish on (hard)",
};

/** What the edit form holds. */
export interface EditForm {
  kind: EditKind;
  task: FieldValue;
  start: FieldValue;
  /** Duration in business days (for a duration edit). */
  days: FieldValue;
  constraint: FieldValue;
  constraintDate: FieldValue;
  reason: FieldValue;
  from: FieldValue;
  to: FieldValue;
  linkKind: FieldValue;
  lag: FieldValue;
  link: FieldValue;
}

const isDate = (s: string) => /^\d{4}-\d{2}-\d{2}$/.test(s);

/** The edit, or the first thing wrong with the form. */
export function buildEdit(f: EditForm): SandboxEdit | { error: string } {
  switch (f.kind) {
    case "move":
      if (blank(f.task)) return { error: "Choose a task." };
      if (!isDate(str(f.start))) return { error: "Give the new start date." };
      return { op: "move", task_pid: str(f.task), start_date: str(f.start) };
    case "duration": {
      if (blank(f.task)) return { error: "Choose a task." };
      const n = Number(str(f.days));
      if (blank(f.days) || !Number.isInteger(n) || n < 0 || n > 3650)
        return {
          error:
            "Duration must be a whole number of business days from 0 to 3650.",
        };
      return { op: "duration", task_pid: str(f.task), duration_days: n };
    }
    case "constraint": {
      if (blank(f.task)) return { error: "Choose a task." };
      if (blank(f.constraint))
        return {
          op: "constraint",
          task_pid: str(f.task),
          constraint_kind: null,
          constraint_date: null,
        };
      const kind = str(f.constraint);
      if (!(kind in CONSTRAINTS)) return { error: "Choose a constraint." };
      if (!isDate(str(f.constraintDate)))
        return { error: "A constraint needs a date." };
      const hard = kind === "mso" || kind === "mfo";
      if (hard && blank(f.reason))
        return { error: "A hard constraint needs a reason." };
      return {
        op: "constraint",
        task_pid: str(f.task),
        constraint_kind: kind,
        constraint_date: str(f.constraintDate),
        ...(hard ? { reason: str(f.reason).trim() } : {}),
      };
    }
    case "add_link": {
      if (blank(f.from) || blank(f.to)) return { error: "Choose both tasks." };
      if (str(f.from) === str(f.to))
        return { error: "A task cannot depend on itself." };
      const lag = blank(f.lag) ? 0 : Number(str(f.lag));
      if (!Number.isInteger(lag) || lag < -3650 || lag > 3650)
        return {
          error:
            "Lag must be a whole number of business days (negative is a lead).",
        };
      return {
        op: "add_link",
        predecessor_pid: str(f.from),
        successor_pid: str(f.to),
        kind: blank(f.linkKind) ? "fs" : str(f.linkKind),
        lag_days: lag,
      };
    }
    case "remove_link":
      if (blank(f.link)) return { error: "Choose a link." };
      return { op: "remove_link", link_pid: str(f.link) };
  }
}

/** One edit in words. `name` turns a pid into a title. */
export function describeEdit(
  e: SandboxEdit,
  name: (pid: string) => string,
): string {
  switch (e.op) {
    case "move":
      return `Start “${name(e.task_pid)}” on ${e.start_date}`;
    case "duration":
      return e.duration_days === 0
        ? `Make “${name(e.task_pid)}” a milestone`
        : `Set “${name(e.task_pid)}” to ${e.duration_days} business day${e.duration_days === 1 ? "" : "s"}`;
    case "constraint":
      return e.constraint_kind
        ? `${CONSTRAINTS[e.constraint_kind] ?? e.constraint_kind} ${e.constraint_date} for “${name(e.task_pid)}”`
        : `Clear the constraint on “${name(e.task_pid)}”`;
    case "add_link":
      return `“${name(e.successor_pid)}” follows “${name(e.predecessor_pid)}” (${(e.kind ?? "fs").toUpperCase()}${e.lag_days ? `, lag ${e.lag_days}` : ""})`;
    case "remove_link":
      return `Remove the link ${e.link_pid.slice(0, 8)}`;
  }
}

/** The headline of a preview, in a sentence. */
export function finishText(p: Pick<SandboxPreview, "finish">): string {
  const { before, after } = p.finish;
  if (!before && !after) return "The plan has no finish date yet.";
  if (before === after) return `The finish stays ${after}.`;
  if (!before) return `The finish becomes ${after}.`;
  if (!after) return `The plan no longer has a finish date (was ${before}).`;
  return `The finish moves from ${before} to ${after}.`;
}

/** Money in minor units with the sign shown ("+120.00 GBP"). */
export function deltaText(minor: number, currency: string): string {
  const sign = minor > 0 ? "+" : minor < 0 ? "-" : "";
  const abs = Math.abs(minor);
  return `${sign}${Math.floor(abs / 100).toLocaleString("en-GB")}.${String(abs % 100).padStart(2, "0")} ${currency}`;
}

/** A sentence for why cost is absent; empty when it is present. */
export function costReasonText(c: SandboxPreview["cost_impact"]): string {
  if (c.available) return "";
  return c.reason === "no_finance_access"
    ? "Cost impact is finance data, so it is not shown to you."
    : "No task with work has a rate, so there is no cost to compare.";
}

/** Whether a failed commit means "the plan changed" (409). */
export function isStale(status: number): boolean {
  return status === 409;
}
