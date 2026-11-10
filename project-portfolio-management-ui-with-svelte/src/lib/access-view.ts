// Access review and break-glass access (NC-1 slice 7, RG-9, RG-10). Pure: the checks on a typed plan id, review age,
// reason, length of time and person to tell; the request bodies; a grant's state in words and a shape; and the
// refusal sentence for each step.
//
// The rules mirror the service: `src/access_review.rs` and `src/controllers/access_review.rs` (administrators only;
// `older_than_days` a whole number of calendar days, not negative, 365 when left out) and `src/break_glass.rs` with
// `src/controllers/break_glass.rs` (a reason of at least 20 characters once trimmed and at most 1000; 1 to 240
// minutes, 60 when left out; the person to tell a `person:` or `worker:` reference, never the caller; the plan must be
// private; one live grant per plan and person; five a day). The service decides; this only saves a round trip and
// lets the page say what is wrong, in the reader's language.
//
// Two checks here are stricter than the service, never looser. The person to tell must be a `person:` or `worker:`
// reference with a UUID, because that is the only form the service can deliver a notice to and check the person's
// choices for (`notification_prefs::recipient_sub`); the service itself accepts any text after the prefix. And the
// reason's length is counted in characters as the service counts them, but the text box also stops at 1000 UTF-16
// units, which is never more than the service allows.

import type {
  AccessReviewQuery,
  BreakGlassGrant,
  BreakGlassOpenBody,
} from "./api/ppm-types.js";

/** The service's limits (`src/break_glass.rs`, `src/access_review.rs`). */
export const MIN_REASON = 20;
export const MAX_REASON = 1000;
export const MIN_MINUTES = 1;
export const MAX_MINUTES = 240;
export const DEFAULT_MINUTES = 60;
export const DEFAULT_REVIEW_DAYS = 365;

const UUID = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;

/** Whether `raw` (trimmed) is a plan id the service reads. */
export function isPlanId(raw: string): boolean {
  return UUID.test(raw.trim());
}

/** Whether `raw` (trimmed) is a `person:` or `worker:` reference with a UUID, the form a notice can be delivered to. */
export function isPersonRef(raw: string): boolean {
  const m = /^(?:person|worker):(.+)$/.exec(raw.trim());
  return m !== null && UUID.test(m[1]!);
}

/** A reason's length in characters as the service counts them (Unicode scalar values), once trimmed. */
export function reasonLength(reason: string): number {
  return [...reason.trim()].length;
}

/** What is wrong with the access review's filters, or `null`. A blank plan covers every plan; a blank age is 365. */
export function reviewProblem(
  plan: string,
  days: string,
): "plan_id" | "days" | null {
  if (plan.trim() !== "" && !isPlanId(plan)) return "plan_id";
  if (days.trim() !== "" && !/^\d{1,5}$/.test(days.trim())) return "days";
  return null;
}

/** The access review's query from the filters (assumed checked): blanks are left out, so the service's defaults apply. */
export function reviewQuery(plan: string, days: string): AccessReviewQuery {
  const q: AccessReviewQuery = {};
  if (plan.trim() !== "") q.plan = plan.trim().toLowerCase();
  if (days.trim() !== "") q.older_than_days = Number(days.trim());
  return q;
}

/** One problem with a break-glass request. */
export type OpenProblem =
  | "plan_id"
  | "reason_short"
  | "reason_long"
  | "minutes"
  | "notify_ref"
  | "acknowledged";

/** Everything wrong with a break-glass request before it is sent, in a fixed order; empty means it can be sent. */
export function openProblems(f: {
  plan: string;
  reason: string;
  minutes: string;
  notify: string;
  acknowledged: boolean;
}): OpenProblem[] {
  const out: OpenProblem[] = [];
  if (!isPlanId(f.plan)) out.push("plan_id");
  if (reasonLength(f.reason) < MIN_REASON) out.push("reason_short");
  if ([...f.reason].length > MAX_REASON) out.push("reason_long");
  const m = f.minutes.trim();
  if (
    !/^\d{1,3}$/.test(m) ||
    Number(m) < MIN_MINUTES ||
    Number(m) > MAX_MINUTES
  )
    out.push("minutes");
  if (!isPersonRef(f.notify)) out.push("notify_ref");
  if (!f.acknowledged) out.push("acknowledged");
  return out;
}

/** The open body (assumed checked). The minutes are always sent, so the page never leans on the service's default. */
export function openBody(f: {
  plan: string;
  reason: string;
  minutes: string;
  notify: string;
}): BreakGlassOpenBody {
  return {
    plan_pid: f.plan.trim().toLowerCase(),
    reason: f.reason.trim(),
    minutes: Number(f.minutes.trim()),
    notify_ref: f.notify.trim(),
  };
}

/** Where a grant stands. */
export type GrantState = "live" | "revoked" | "expired";

/** A grant's state: the service's `live` decides whether it is in force; otherwise it was ended early or ran out. */
export function grantState(
  g: Pick<BreakGlassGrant, "live" | "revoked_at">,
): GrantState {
  if (g.live) return "live";
  return g.revoked_at ? "revoked" : "expired";
}

/** Each state with a shape, so it is never told by colour alone. */
export const GRANT_SYMBOL: Record<GrantState, string> = {
  live: "●",
  revoked: "■",
  expired: "○",
};

/** A step a person takes from this screen. */
export type AccessStep = "review" | "list" | "open" | "revoke";

/**
 * The catalogue sentence for a refusal of `step`, by HTTP status, or `null` for the general rule. The page checks
 * the request first, so a 422 on opening is one of the service's own checks the page cannot make: the plan is not
 * private, the person to tell is the caller, or that person cannot be told.
 */
export function accessRefusalKey(
  step: AccessStep,
  status: number,
):
  | "p.access.refused_sign_in"
  | "p.access.refused_review_forbidden"
  | "p.access.refused_review_invalid"
  | "p.access.refused_open_forbidden"
  | "p.access.refused_open_not_found"
  | "p.access.refused_open_conflict"
  | "p.access.refused_open_invalid"
  | "p.access.refused_open_limit"
  | "p.access.refused_revoke_not_found"
  | null {
  if (status === 401) return "p.access.refused_sign_in";
  if (step === "review") {
    if (status === 403) return "p.access.refused_review_forbidden";
    if (status === 400 || status === 422)
      return "p.access.refused_review_invalid";
    return null;
  }
  if (step === "open") {
    if (status === 403) return "p.access.refused_open_forbidden";
    if (status === 404) return "p.access.refused_open_not_found";
    if (status === 409) return "p.access.refused_open_conflict";
    if (status === 422) return "p.access.refused_open_invalid";
    if (status === 429) return "p.access.refused_open_limit";
    return null;
  }
  if (step === "revoke" && status === 404)
    return "p.access.refused_revoke_not_found";
  return null;
}
