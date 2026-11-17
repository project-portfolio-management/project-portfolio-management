// Triage, thresholds and threshold decisions (EX-7, NC-1 slice 11): the service's rules (`domain/src/thresholds.rs`
// and `controllers/thresholds.rs`), said before anything is sent. The service still decides. Nothing here pauses,
// resets or stops a programme: a crossing opens a delivery exception, and a person records the decision.

import { MINOR_UNITS, currencyCode, toMinor } from "./money.js";
import type { DeliveryException } from "./api/ppm-types.js";

/** The triage outcomes, in the service's order. */
export const TRIAGE_OUTCOMES = ["finish", "fix", "pause", "stop"] as const;
export type TriageOutcome = (typeof TRIAGE_OUTCOMES)[number];
/** The threshold kinds, in the service's order. */
export const THRESHOLD_KINDS = [
  "confidence_red_packs",
  "forecast_past",
  "p80_cap",
] as const;
export type ThresholdKind = (typeof THRESHOLD_KINDS)[number];
/** The decisions that answer a crossing. */
export const THRESHOLD_ACTIONS = [
  "pause",
  "reset",
  "stop",
  "override",
] as const;
export type ThresholdAction = (typeof THRESHOLD_ACTIONS)[number];

/** The shortest reason, in characters after trimming, for a triage and for a threshold decision. */
export const REASON_MIN = 10;
/** The fewest and most issued packs at red in a row. */
export const RED_PACKS_MIN = 1;
export const RED_PACKS_MAX = 24;
/** The longest an override may last, in calendar days from today. */
export const OVERRIDE_MAX_DAYS = 366;

/** One threshold as the service answers it: only the parameter its kind uses is set. */
export interface Threshold {
  kind: ThresholdKind;
  count: number | null;
  date: string | null;
  cap_minor: number | null;
  currency: string | null;
}

/** One triage as the service answers it. */
export interface Triage {
  pid: string;
  triaged_on: string;
  outcome: TriageOutcome;
  reason: string;
}

/** What the form holds for one threshold kind, as typed. */
export interface ThresholdForm {
  /** Text, or a number once a number input has been typed in. */
  count: string | number;
  date: string;
  amount: string;
  currency: string;
}

const chars = (s: string) => [...s.trim()].length;

/** The catalogue key of what is wrong with a triage, or `null`. */
export function triageProblem(
  outcome: string,
  reason: string,
): "p.thresholds.problem_outcome" | "p.thresholds.problem_reason" | null {
  if (!(TRIAGE_OUTCOMES as readonly string[]).includes(outcome))
    return "p.thresholds.problem_outcome";
  return chars(reason) < REASON_MIN ? "p.thresholds.problem_reason" : null;
}

/** The body for `PUT …/thresholds`, or the catalogue key of what is wrong with the form for `kind`. */
export function thresholdBody(
  kind: ThresholdKind,
  form: ThresholdForm,
):
  | {
      body: {
        kind: ThresholdKind;
        count?: number;
        date?: string;
        cap_minor?: number;
        currency?: string;
      };
    }
  | {
      problem:
        | "p.thresholds.problem_count"
        | "p.thresholds.problem_date"
        | "p.thresholds.problem_cap"
        | "p.thresholds.problem_currency";
    } {
  if (kind === "confidence_red_packs") {
    // A number input binds a number, not text, once the reader has typed in it.
    const raw = String(form.count ?? "").trim();
    const n = Number(raw);
    return Number.isInteger(n) &&
      n >= RED_PACKS_MIN &&
      n <= RED_PACKS_MAX &&
      raw !== ""
      ? { body: { kind, count: n } }
      : { problem: "p.thresholds.problem_count" };
  }
  if (kind === "forecast_past") {
    return /^\d{4}-\d{2}-\d{2}$/.test(form.date)
      ? { body: { kind, date: form.date } }
      : { problem: "p.thresholds.problem_date" };
  }
  const currency = currencyCode(form.currency);
  if (!currency || !(currency in MINOR_UNITS))
    return { problem: "p.thresholds.problem_currency" };
  const minor = toMinor(form.amount, currency);
  return minor !== null && minor > 0
    ? { body: { kind, cap_minor: minor, currency } }
    : { problem: "p.thresholds.problem_cap" };
}

/** Whether `next` would set the threshold to what `current` already is. */
export function sameThreshold(
  current: Threshold | undefined,
  next: {
    count?: number;
    date?: string;
    cap_minor?: number;
    currency?: string;
  },
): boolean {
  if (!current) return false;
  return (
    (current.count ?? undefined) === next.count &&
    (current.date ?? undefined) === next.date &&
    (current.cap_minor ?? undefined) === next.cap_minor &&
    (current.currency ?? undefined) === next.currency
  );
}

/** Calendar days from `from` to `to`, both `YYYY-MM-DD`. */
function daysBetween(from: string, to: string): number {
  return Math.round(
    (Date.parse(`${to}T00:00:00Z`) - Date.parse(`${from}T00:00:00Z`)) /
      86_400_000,
  );
}

/** The catalogue keys of what is wrong with a threshold decision, in the service's order; empty when it can be sent. */
export function decisionProblems(
  action: string,
  reason: string,
  expiry: string,
  today: string,
): Array<
  | "p.thresholds.problem_action"
  | "p.thresholds.problem_reason"
  | "p.thresholds.problem_expiry_needed"
  | "p.thresholds.problem_expiry_past"
  | "p.thresholds.problem_expiry_long"
> {
  const out: ReturnType<typeof decisionProblems> = [];
  if (!(THRESHOLD_ACTIONS as readonly string[]).includes(action))
    out.push("p.thresholds.problem_action");
  if (chars(reason) < REASON_MIN) out.push("p.thresholds.problem_reason");
  if (action === "override") {
    if (!expiry) out.push("p.thresholds.problem_expiry_needed");
    else if (expiry <= today) out.push("p.thresholds.problem_expiry_past");
    else if (daysBetween(today, expiry) > OVERRIDE_MAX_DAYS)
      out.push("p.thresholds.problem_expiry_long");
  }
  return out;
}

/** An exception a threshold opened: it is closed only by a threshold decision, not by the ordinary one. */
export function isThresholdException(
  e: Pick<DeliveryException, "opened_value">,
): boolean {
  return e.opened_value === "threshold";
}
