// Pure helpers for the delivery exceptions register (`/exceptions`, DP-3, NC-1 slice 11): what an exception still
// needs in the reader's words, what a completion would send (only what changed), the checks the service makes, and
// the sentence for each refusal.

import { ApiError } from "./api/client.js";
import type { StringKey } from "./i18n.svelte.js";
import type {
  DeliveryException,
  ExceptionCompleteBody,
} from "./api/ppm-types.js";

/** A decision needs at least this many characters; each text part needs three to count as there (`missing`). */
export const DECISION_MIN = 10;
export const PART_MIN = 3;

/** Each missing part's catalogue key, in the service's order. */
export const MISSING_LABELS: Record<
  DeliveryException["missing"][number],
  StringKey
> = {
  cause: "p.exceptions.part_cause",
  decision_needed: "p.exceptions.part_decision_needed",
  owner: "p.exceptions.part_owner",
  decide_by: "p.exceptions.part_decide_by",
};

export interface CompleteForm {
  cause: string;
  decisionNeeded: string;
  ownerRef: string;
  ownerRole: string;
  decideBy: string;
}

/** The form filled from what the exception already has. */
export function formOf(e: DeliveryException): CompleteForm {
  return {
    cause: e.cause ?? "",
    decisionNeeded: e.decision_needed ?? "",
    ownerRef: e.owner_ref ?? "",
    ownerRole: e.owner_role ?? "",
    decideBy: e.decide_by ?? "",
  };
}

/**
 * What saving would send: each part trimmed, only those that changed and are not blank (the service keeps a blank
 * one as it was, so blanking cannot be sent). `null` when nothing changed.
 */
export function completeBody(
  form: CompleteForm,
  e: DeliveryException,
): ExceptionCompleteBody | null {
  const out: ExceptionCompleteBody = {};
  const put = (
    key: keyof ExceptionCompleteBody,
    value: string,
    before: string | null,
  ) => {
    const v = value.trim();
    if (v !== "" && v !== (before ?? "")) out[key] = v;
  };
  put("cause", form.cause, e.cause);
  put("decision_needed", form.decisionNeeded, e.decision_needed);
  put("owner_ref", form.ownerRef, e.owner_ref);
  put("owner_role", form.ownerRole, e.owner_role);
  put("decide_by", form.decideBy, e.decide_by);
  return Object.keys(out).length > 0 ? out : null;
}

/** Problems with a completion before it is sent: a text part of one or two characters does not count as there. */
export function completeProblems(form: CompleteForm): StringKey[] {
  const short = (s: string) => {
    const n = [...s.trim()].length;
    return n > 0 && n < PART_MIN;
  };
  return short(form.cause) || short(form.decisionNeeded) || short(form.ownerRef)
    ? ["p.exceptions.problem_part_short"]
    : [];
}

/** The problem with a decision, or `null`: at least ten characters once trimmed. */
export function decisionProblem(decision: string): StringKey | null {
  return [...decision.trim()].length < DECISION_MIN
    ? "p.exceptions.problem_decision"
    : null;
}

/**
 * The sentence for a refusal. A 409 is an exception already closed. A 422 on a decision the page has already checked
 * for length can only be one a threshold opened (EX-7), which closes another way; the register does not say which
 * those are.
 */
export function exceptionRefusalKey(
  action: "complete" | "decide",
  err: unknown,
): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  if (err.status === 409) return "p.exceptions.conflict_closed";
  if (err.status === 422 && action === "decide")
    return "p.exceptions.refused_threshold";
  return null;
}
