// The change-request screen (NC-1 slice 2, IR-4): turning what a person typed into the request body, checking the
// rules the service would otherwise refuse, and which steps a status allows. The rules mirror the service's
// `src/change_requests.rs` (a title, a description and a reason; a schedule impact in whole business days within
// ±3650; a cost with its currency and a currency only with a cost; a decision reason of at least 10 characters; an
// approver who is a `person:` or `worker:` reference and not the requester). The service decides; this only saves a
// round trip and lets the page say which field is wrong, in the reader's language.

import type { ChangeRequest, ChangeRequestBody } from "./api/ppm-types.js";
import { currencyCode, currentLocale, minorToInput, toMinor } from "./money.js";

export const CR_KINDS = [
  "scope",
  "schedule",
  "cost",
  "quality",
  "other",
] as const;
/** The service's limit on a stated schedule impact, in business days either way. */
export const MAX_SCHEDULE_DAYS = 3650;
/** The service's shortest decision reason, in characters. */
export const MIN_DECISION_REASON = 10;

/** What the draft form holds: every field a string, so an empty box is "". */
export interface ChangeRequestForm {
  title: string;
  description: string;
  reason: string;
  kind: ChangeRequest["kind"];
  scheduleDays: string;
  costAmount: string;
  currency: string;
  summary: string;
}

/** The first thing wrong with the draft form, or `null` when it can be sent. */
export type ChangeRequestProblem =
  | "title_required"
  | "description_required"
  | "reason_required"
  | "days_not_whole"
  | "days_out_of_range"
  | "cost_not_amount"
  | "cost_needs_currency"
  | "currency_needs_cost"
  | "currency_invalid"
  | null;

export function emptyChangeRequestForm(): ChangeRequestForm {
  return {
    title: "",
    description: "",
    reason: "",
    kind: "other",
    scheduleDays: "",
    costAmount: "",
    currency: "",
    summary: "",
  };
}

/** The form for editing `cr`; the cost is shown in its currency's own minor unit, in the reader's number format. */
export function formFromChangeRequest(
  cr: ChangeRequest,
  locale: string = currentLocale(),
): ChangeRequestForm {
  const {
    schedule_business_days: days,
    cost_minor: cost,
    cost_currency,
  } = cr.impact;
  return {
    title: cr.title,
    description: cr.description ?? "",
    reason: cr.reason ?? "",
    kind: cr.kind,
    scheduleDays: days === null ? "" : String(days),
    costAmount:
      cost === null ? "" : minorToInput(cost, cost_currency ?? "", locale),
    currency: cost_currency ?? "",
    summary: cr.impact.summary ?? "",
  };
}

const WHOLE = /^-?\d+$/;

/** The first thing wrong with the form. */
export function changeRequestProblem(
  f: ChangeRequestForm,
  locale: string = currentLocale(),
): ChangeRequestProblem {
  if (f.title.trim() === "") return "title_required";
  if (f.description.trim() === "") return "description_required";
  if (f.reason.trim() === "") return "reason_required";
  const days = f.scheduleDays.trim();
  if (days !== "") {
    if (!WHOLE.test(days)) return "days_not_whole";
    if (Math.abs(Number(days)) > MAX_SCHEDULE_DAYS) return "days_out_of_range";
  }
  const cost = f.costAmount.trim();
  const currency = f.currency.trim();
  if (cost !== "" && currency === "") return "cost_needs_currency";
  if (cost === "" && currency !== "") return "currency_needs_cost";
  if (cost !== "" && currencyCode(currency) === null) return "currency_invalid";
  if (cost !== "" && toMinor(cost, currency, locale) === null)
    return "cost_not_amount";
  return null;
}

const orNull = (s: string): string | null =>
  s.trim() === "" ? null : s.trim();

/**
 * The request body for the form. Every field is sent, because an edit replaces them all; an empty impact is null
 * (not assessed), never zero. `edits` are the what-if edits already on the request, sent back so an edit keeps them.
 */
export function changeRequestBody(
  f: ChangeRequestForm,
  edits: unknown[] | null = null,
  locale: string = currentLocale(),
): ChangeRequestBody {
  const days = f.scheduleDays.trim();
  const currency = currencyCode(f.currency);
  const cost =
    f.costAmount.trim() === "" || currency === null
      ? null
      : toMinor(f.costAmount, currency, locale);
  return {
    title: f.title.trim(),
    description: f.description.trim(),
    reason: f.reason.trim(),
    kind: f.kind,
    impact_schedule_days: days === "" ? null : Number(days),
    impact_cost_minor: cost,
    cost_currency: cost === null ? null : currency,
    impact_summary: orNull(f.summary),
    edits: edits && edits.length > 0 ? edits : null,
  };
}

/** A step a person takes on a change request from this screen. */
export type ChangeRequestStep =
  "edit" | "submit" | "review" | "decide" | "withdraw";

/**
 * The steps a status allows, in the service's state machine. Who may take each (the requester, the named approver,
 * an administrator) is the service's to decide; the page does not know who the caller is.
 */
export function stepsFor(status: ChangeRequest["status"]): ChangeRequestStep[] {
  switch (status) {
    case "draft":
      return ["edit", "submit", "withdraw"];
    case "submitted":
      return ["review", "decide", "withdraw"];
    case "under_review":
      return ["decide", "withdraw"];
    case "approved":
      return ["withdraw"];
    default:
      return [];
  }
}

/** The approver's identifier: a `person:` or `worker:` reference less its prefix, or a bare subject. */
export function identifier(reference: string): string {
  const r = reference.trim();
  const bare = r.startsWith("person:")
    ? r.slice(7)
    : r.startsWith("worker:")
      ? r.slice(7)
      : r;
  return bare.trim();
}

/** What is wrong with naming `approver` for a request made by `requestedBy`, or `null`. */
export function approverProblem(
  approver: string,
  requestedBy: string,
): "approver_not_reference" | "approver_is_requester" | null {
  const a = approver.trim();
  if (!/^(person|worker):\S/.test(a)) return "approver_not_reference";
  if (identifier(a).toLowerCase() === identifier(requestedBy).toLowerCase())
    return "approver_is_requester";
  return null;
}

/** Whether a decision reason is long enough for the service. */
export function decisionReasonShort(reason: string): boolean {
  return [...reason.trim()].length < MIN_DECISION_REASON;
}

/**
 * The catalogue sentence for a refusal of `step`, by HTTP status: 403 says who may take it, 409 that the request
 * has moved on, 422 what the service did not accept. `null` for anything else, so the page uses the general rule.
 */
export function refusalKey(
  step: ChangeRequestStep | "draft",
  status: number,
):
  | "p.plan_change_requests.refused_requester"
  | "p.plan_change_requests.refused_withdraw"
  | "p.plan_change_requests.refused_approver"
  | "p.plan_change_requests.refused_state"
  | "p.plan_change_requests.refused_invalid"
  | null {
  if (status === 409) return "p.plan_change_requests.refused_state";
  if (status === 422) return "p.plan_change_requests.refused_invalid";
  if (status === 403) {
    if (step === "review" || step === "decide")
      return "p.plan_change_requests.refused_approver";
    if (step === "withdraw") return "p.plan_change_requests.refused_withdraw";
    if (step === "edit" || step === "submit")
      return "p.plan_change_requests.refused_requester";
  }
  return null;
}
