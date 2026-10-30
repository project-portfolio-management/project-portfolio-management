// The what-if page: building edits from what the form holds, saying each in
// words, and summarizing a preview. Pure and tested.

import type { SandboxEdit, SandboxPreview } from "#lib/api/ppm.js";
import { t, tn } from "./i18n.svelte.js";
import { signedMoney } from "./money.js";

export type FieldValue = string | number | null | undefined;
const str = (v: FieldValue) => (v === null || v === undefined ? "" : String(v));
const blank = (v: FieldValue) => str(v).trim() === "";

export const MAX_EDITS = 100;

export type EditKind =
  "move" | "duration" | "constraint" | "add_link" | "remove_link";

export const EDIT_TEXT: Record<EditKind, string> = {
  get move() {
    return t("l.whatif_view.move_a_task_set_its");
  },
  get duration() {
    return t("l.whatif_view.change_a_task_s_duration");
  },
  get constraint() {
    return t("l.whatif_view.change_a_task_s_constraint");
  },
  get add_link() {
    return t("l.whatif_view.add_a_link");
  },
  get remove_link() {
    return t("l.whatif_view.remove_a_link");
  },
};

export const CONSTRAINTS: Record<string, string> = {
  get snet() {
    return t("p.plans.pid.critical_path.start_no_earlier_than");
  },
  get snlt() {
    return t("p.plans.pid.critical_path.start_no_later_than");
  },
  get fnet() {
    return t("p.plans.pid.critical_path.finish_no_earlier_than");
  },
  get fnlt() {
    return t("p.plans.pid.critical_path.finish_no_later_than");
  },
  get mso() {
    return t("l.whatif_view.must_start_on_hard");
  },
  get mfo() {
    return t("l.whatif_view.must_finish_on_hard");
  },
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
      if (blank(f.task)) return { error: t("l.whatif_view.choose_a_task") };
      if (!isDate(str(f.start)))
        return { error: t("l.whatif_view.give_the_new_start_date") };
      return { op: "move", task_pid: str(f.task), start_date: str(f.start) };
    case "duration": {
      if (blank(f.task)) return { error: t("l.whatif_view.choose_a_task") };
      const n = Number(str(f.days));
      if (blank(f.days) || !Number.isInteger(n) || n < 0 || n > 3650)
        return {
          error: t("l.whatif_view.duration_must_be_a_whole"),
        };
      return { op: "duration", task_pid: str(f.task), duration_days: n };
    }
    case "constraint": {
      if (blank(f.task)) return { error: t("l.whatif_view.choose_a_task") };
      if (blank(f.constraint))
        return {
          op: "constraint",
          task_pid: str(f.task),
          constraint_kind: null,
          constraint_date: null,
        };
      const kind = str(f.constraint);
      if (!(kind in CONSTRAINTS))
        return { error: t("l.whatif_view.choose_a_constraint") };
      if (!isDate(str(f.constraintDate)))
        return { error: t("l.whatif_view.a_constraint_needs_a_date") };
      const hard = kind === "mso" || kind === "mfo";
      if (hard && blank(f.reason))
        return { error: t("l.whatif_view.a_hard_constraint_needs_a") };
      return {
        op: "constraint",
        task_pid: str(f.task),
        constraint_kind: kind,
        constraint_date: str(f.constraintDate),
        ...(hard ? { reason: str(f.reason).trim() } : {}),
      };
    }
    case "add_link": {
      if (blank(f.from) || blank(f.to))
        return { error: t("l.whatif_view.choose_both_tasks") };
      if (str(f.from) === str(f.to))
        return { error: t("l.whatif_view.a_task_cannot_depend_on") };
      const lag = blank(f.lag) ? 0 : Number(str(f.lag));
      if (!Number.isInteger(lag) || lag < -3650 || lag > 3650)
        return {
          error: t("l.whatif_view.lag_must_be_a_whole"),
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
      if (blank(f.link)) return { error: t("l.whatif_view.choose_a_link") };
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
      return t("l.whatif_view.start_on", {
        name: name(e.task_pid),
        start_date: e.start_date,
      });
    case "duration":
      return e.duration_days === 0
        ? t("l.whatif_view.make_a_milestone", { name: name(e.task_pid) })
        : tn("l.whatif_view.set_duration", e.duration_days, {
            name: name(e.task_pid),
            duration_days: e.duration_days,
          });
    case "constraint":
      return e.constraint_kind
        ? t("l.whatif_view.for", {
            constraint_kind:
              CONSTRAINTS[e.constraint_kind] ?? e.constraint_kind,
            constraint_date: e.constraint_date,
            name: name(e.task_pid),
          })
        : t("l.whatif_view.clear_the_constraint_on", {
            name: name(e.task_pid),
          });
    case "add_link":
      return t("l.whatif_view.follows", {
        name: name(e.successor_pid),
        name2: name(e.predecessor_pid),
        toUpperCase: (e.kind ?? "fs").toUpperCase(),
        v: e.lag_days ? t("l.whatif_view.lag", { lag_days: e.lag_days }) : "",
      });
    case "remove_link":
      return t("l.whatif_view.remove_the_link", {
        slice: e.link_pid.slice(0, 8),
      });
  }
}

/** The headline of a preview, in a sentence. */
export function finishText(p: Pick<SandboxPreview, "finish">): string {
  const { before, after } = p.finish;
  if (!before && !after) return t("l.whatif_view.the_plan_has_no_finish");
  if (before === after) return t("l.whatif_view.the_finish_stays", { after });
  if (!before) return t("l.whatif_view.the_finish_becomes", { after });
  if (!after) return t("l.whatif_view.the_plan_no_longer_has", { before });
  return t("l.whatif_view.the_finish_moves_from_to", { before, after });
}

/** Money in minor units with the sign shown ("+£120.00"), in the reader's locale and the currency's exponent. */
export function deltaText(minor: number, currency: string): string {
  return signedMoney(minor, currency);
}

/** A sentence for why cost is absent; empty when it is present. */
export function costReasonText(c: SandboxPreview["cost_impact"]): string {
  if (c.available) return "";
  return c.reason === "no_finance_access"
    ? t("l.whatif_view.cost_impact_is_finance_data")
    : t("l.whatif_view.no_task_with_work_has");
}

/** Whether a failed commit means "the plan changed" (409). */
export function isStale(status: number): boolean {
  return status === 409;
}
