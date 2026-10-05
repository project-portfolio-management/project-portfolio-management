// Assignments, work contours and cost rates: what the forms hold, what they send,
// and the words the pages use. Pure and tested.
//
// Work is stored in minutes; people think in hours. Money is stored in minor
// units; people type major units. Both conversions are strict: text that is not
// a clean amount is an error, never a silent zero.

import type { AssignmentBody, AssignmentResult, SchedulingRule } from "#lib/api/ppm.js";

export type FieldValue = string | number | null | undefined;
const str = (v: FieldValue) => (v === null || v === undefined ? "" : String(v));
const blank = (v: FieldValue) => str(v).trim() === "";

export const RULE_TEXT: Record<SchedulingRule, string> = {
  fixed_units: "Fixed units",
  fixed_duration: "Fixed duration",
  fixed_work: "Fixed work",
};

export const RULE_HELP: Record<SchedulingRule, string> = {
  fixed_units: "Changing the work changes how long it takes; the share of the day stays.",
  fixed_duration: "Changing the work changes the share of the day; the duration stays.",
  fixed_work: "Changing the duration or the share of the day changes the other; the work stays.",
};

/** 90 → "1.5 h"; 0 → "0 h". */
export function hoursText(minutes: number): string {
  const h = minutes / 60;
  return `${Number.isInteger(h) ? h : h.toFixed(2).replace(/0$/, "")} h`;
}

/** Hours as typed → whole minutes, or null when it is not a positive number of hours. */
export function minutesFromHours(text: FieldValue): number | null {
  const n = Number(str(text));
  if (blank(text) || !Number.isFinite(n) || n <= 0) return null;
  return Math.round(n * 60);
}

/** Money as typed (major units) → minor units, or null when not a non-negative amount. */
export function minorFromMajor(text: FieldValue, allowZero = false): number | null {
  const n = Number(str(text));
  if (blank(text) || !Number.isFinite(n) || n < 0 || (!allowZero && n === 0)) return null;
  return Math.round(n * 100);
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
export function assignmentBody(f: AssignmentForm): AssignmentBody | { error: string } {
  const body: AssignmentBody = { rule: f.rule };
  const days = Number(str(f.duration));
  const units = Number(str(f.units));
  const work = minutesFromHours(f.hours);
  const wantDays = f.first || f.changing === "duration";
  const wantUnits = f.first || f.changing === "units";
  const wantWork = f.changing === "work";
  if (f.first && f.changing === "work") {
    // first assignment with work: give work and units (duration derived)
    if (work === null) return { error: "Work must be a number of hours above zero." };
  }
  if (wantDays && !(f.first && f.changing === "work")) {
    if (blank(f.duration) || !Number.isInteger(days) || days < 1 || days > 3650)
      return { error: "Duration must be a whole number of working days from 1 to 3650." };
    body.duration_days = days;
  }
  if (wantUnits) {
    if (blank(f.units) || !Number.isInteger(units) || units < 1 || units > 1000)
      return { error: "Units must be a whole percent of a day from 1 to 1000." };
    body.units_percent = units;
  }
  if (wantWork) {
    if (work === null) return { error: "Work must be a number of hours above zero." };
    body.work_minutes = work;
  }
  if (f.first && f.changing === "duration" && !wantUnits) return { error: "Give the units too." };
  return body;
}

/** A sentence saying what the rule did. */
export function resultText(r: AssignmentResult): string {
  const parts = `${r.duration_days} working days at ${r.units_percent}% is ${hoursText(r.work_minutes)}`;
  const what =
    r.derived === "duration"
      ? "The duration was worked out."
      : r.derived === "units"
        ? "The share of the day was worked out."
        : r.derived === "work"
          ? "The work was worked out."
          : "";
  const rounded = r.exact === false ? " Rounded: the three do not match to the minute." : "";
  return `${parts}. ${what}${rounded}`.replace(/\s+/g, " ").trim();
}

/** A sentence for the capacity check; empty when there is nothing to say. */
export function capacityText(c: AssignmentResult["capacity"]): string {
  if (!c.checked) return c.reason === "unscheduled" ? "Capacity not checked: the task has no dates yet." : "";
  if (c.over_capacity) return `Over capacity on ${c.days_over} day${c.days_over === 1 ? "" : "s"} (${c.from} to ${c.to}): this asks more than the person can give.`;
  return "Within the person's capacity.";
}

export const CONTOUR_TEXT: Record<string, string> = {
  flat: "Flat: the same each day",
  front_loaded: "Front loaded: more at the start",
  back_loaded: "Back loaded: more at the end",
  bell: "Bell: most in the middle",
  turtle: "Turtle: ramps up, holds, ramps down",
  custom: "Custom percentages",
};

/** "10, 20, 70" → [10,20,70]; an error when they are not whole percentages summing to 100. */
export function customWeights(text: string): number[] | { error: string } {
  const parts = text.split(/[,\s]+/).filter(Boolean);
  const nums = parts.map(Number);
  if (nums.length === 0 || nums.length > 40 || nums.some((n) => !Number.isInteger(n) || n < 0))
    return { error: "Give 1 to 40 whole percentages, separated by commas." };
  const sum = nums.reduce((a, b) => a + b, 0);
  if (sum !== 100) return { error: `The percentages add up to ${sum}, not 100.` };
  return nums;
}

export const UNPLACED_TEXT: Record<string, string> = {
  unscheduled: "has no dates yet",
  no_working_days: "has no working day to fall on",
  no_weight: "has a contour that gives it no weight",
};

export const NO_RATE_TEXT: Record<string, string> = {
  no_rate: "no rate applies",
  no_rate_on_some_days: "no rate applied on some days",
  no_assignee: "has nobody assigned",
};
