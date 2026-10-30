// Assignments, work contours and cost rates: what the forms hold, what they send,
// and the words the pages use. Pure and tested.
//
// Work is stored in minutes; people think in hours. Money is stored in minor
// units; people type major units. Both conversions are strict: text that is not
// a clean amount is an error, never a silent zero.

import type {
  AssignmentBody,
  AssignmentResult,
  SchedulingRule,
} from "#lib/api/ppm.js";
import { t, tn } from "./i18n.svelte.js";
import { formatNumber } from "#lib/locale-format.js";
import { currentLocale, toMinor } from "./money.js";

export type FieldValue = string | number | null | undefined;
const str = (v: FieldValue) => (v === null || v === undefined ? "" : String(v));
const blank = (v: FieldValue) => str(v).trim() === "";

export const RULE_TEXT: Record<SchedulingRule, string> = {
  get fixed_units() {
    return t("l.work_view.fixed_units");
  },
  get fixed_duration() {
    return t("l.work_view.fixed_duration");
  },
  get fixed_work() {
    return t("l.work_view.fixed_work");
  },
};

export const RULE_HELP: Record<SchedulingRule, string> = {
  get fixed_units() {
    return t("l.work_view.changing_the_work_changes_how");
  },
  get fixed_duration() {
    return t("l.work_view.changing_the_work_changes_the");
  },
  get fixed_work() {
    return t("l.work_view.changing_the_duration_or_the");
  },
};

/** 90 → "1.5 h"; 0 → "0 h". */
export function hoursText(minutes: number): string {
  const h = minutes / 60;
  return `${formatNumber(h, { maximumFractionDigits: 2 })} h`;
}

/** Hours as typed → whole minutes, or null when it is not a positive number of hours. */
export function minutesFromHours(text: FieldValue): number | null {
  const n = Number(str(text));
  if (blank(text) || !Number.isFinite(n) || n <= 0) return null;
  return Math.round(n * 60);
}

/** Money as typed (major units, in the reader's locale) → minor units of `currency`, or null when not a non-negative amount. */
export function minorFromMajor(
  text: FieldValue,
  currency: string,
  allowZero = false,
  locale: string = currentLocale(),
): number | null {
  if (blank(text)) return null;
  const n = toMinor(str(text), currency, locale);
  if (n === null || n < 0 || (!allowZero && n === 0)) return null;
  return n;
}

/** What the assignment form holds. Exactly one of the three is the field being changed. */
export interface AssignmentForm {
  rule: SchedulingRule;
  changing: "duration" | "units" | "work";
  duration: FieldValue;
  units: FieldValue;
  hours: FieldValue;
  /** True for a first assignment: two quantities are given and the third derived. */
  first: boolean;
}

/** The request body, or the first thing wrong. */
export function assignmentBody(
  f: AssignmentForm,
): AssignmentBody | { error: string } {
  const body: AssignmentBody = { rule: f.rule };
  const days = Number(str(f.duration));
  const units = Number(str(f.units));
  const work = minutesFromHours(f.hours);
  const wantDays = f.first || f.changing === "duration";
  const wantUnits = f.first || f.changing === "units";
  const wantWork = f.changing === "work";
  if (f.first && f.changing === "work") {
    // first assignment with work: give work and units (duration derived)
    if (work === null) return { error: t("l.work_view.work_must_be_a_number") };
  }
  if (wantDays && !(f.first && f.changing === "work")) {
    if (blank(f.duration) || !Number.isInteger(days) || days < 1 || days > 3650)
      return {
        error: t("l.work_view.duration_must_be_a_whole"),
      };
    body.duration_days = days;
  }
  if (wantUnits) {
    if (blank(f.units) || !Number.isInteger(units) || units < 1 || units > 1000)
      return {
        error: t("l.work_view.units_must_be_a_whole"),
      };
    body.units_percent = units;
  }
  if (wantWork) {
    if (work === null) return { error: t("l.work_view.work_must_be_a_number") };
    body.work_minutes = work;
  }
  if (f.first && f.changing === "duration" && !wantUnits)
    return { error: t("l.work_view.give_the_units_too") };
  return body;
}

/** A sentence saying what the rule did. */
export function resultText(r: AssignmentResult): string {
  const parts = t("l.work_view.business_days_at_is", {
    duration_days: r.duration_days,
    units_percent: r.units_percent,
    hoursText: hoursText(r.work_minutes),
  });
  const what =
    r.derived === "duration"
      ? t("l.work_view.the_duration_was_worked_out")
      : r.derived === "units"
        ? t("l.work_view.the_share_of_the_day")
        : r.derived === "work"
          ? t("l.work_view.the_work_was_worked_out")
          : "";
  const rounded =
    r.exact === false ? t("l.work_view.rounded_the_three_do_not") : "";
  return `${parts}. ${what}${rounded}`.replace(/\s+/g, " ").trim();
}

/** A sentence for the capacity check; empty when there is nothing to say. */
export function capacityText(c: AssignmentResult["capacity"]): string {
  if (!c.checked)
    return c.reason === "unscheduled"
      ? t("l.work_view.capacity_not_checked_the_task")
      : "";
  if (c.over_capacity)
    return tn("l.work_view.over_capacity", c.days_over ?? 0, {
      days_over: c.days_over,
      from: c.from,
      to: c.to,
    });
  return t("l.work_view.within_the_person_s_capacity");
}

export const CONTOUR_TEXT: Record<string, string> = {
  get flat() {
    return t("l.work_view.flat_the_same_each_day");
  },
  get front_loaded() {
    return t("l.work_view.front_loaded_more_at_the");
  },
  get back_loaded() {
    return t("l.work_view.back_loaded_more_at_the");
  },
  get bell() {
    return t("l.work_view.bell_most_in_the_middle");
  },
  get turtle() {
    return t("l.work_view.turtle_ramps_up_holds_ramps");
  },
  get custom() {
    return t("l.work_view.custom_percentages");
  },
};

/** "10, 20, 70" → [10,20,70]; an error when they are not whole percentages summing to 100. */
export function customWeights(text: string): number[] | { error: string } {
  const parts = text.split(/[,\s]+/).filter(Boolean);
  const nums = parts.map(Number);
  if (
    nums.length === 0 ||
    nums.length > 40 ||
    nums.some((n) => !Number.isInteger(n) || n < 0)
  )
    return { error: t("l.work_view.give_1_to_40_whole") };
  const sum = nums.reduce((a, b) => a + b, 0);
  if (sum !== 100)
    return { error: t("l.work_view.the_percentages_add_up_to", { sum }) };
  return nums;
}

export const UNPLACED_TEXT: Record<string, string> = {
  get unscheduled() {
    return t("l.work_view.has_no_dates_yet");
  },
  get no_working_days() {
    return t("l.work_view.has_no_business_day_to");
  },
  get no_weight() {
    return t("l.work_view.has_a_contour_that_gives");
  },
};

export const NO_RATE_TEXT: Record<string, string> = {
  get no_rate() {
    return t("l.work_view.no_rate_applies");
  },
  get no_rate_on_some_days() {
    return t("l.work_view.no_rate_applied_on_some");
  },
  get no_assignee() {
    return t("l.work_view.has_nobody_assigned");
  },
};
