// Changing a risk on the governance page (NC-1 slice 12 part c, `PUT /api/plans/{pid}/risks/{risk_pid}`): the service's
// rules (`controllers/governance.rs` `update_risk`, `domain/src/governance.rs`), said before anything is sent. The
// service still decides. A field left out of the body keeps what is stored, so only the fields that changed are sent,
// and nothing at all for no change. The likelihood and the impact are whole numbers from 1 to 5 (the exposure is their
// product); the status is one of open, mitigating, closed or materialized; the mitigation is at most 1024 bytes as the
// service counts them. A review date cannot be removed once set (a missing one keeps the old). Recording that a risk
// has happened is the Escalate button, which also stamps when, so this form does not offer "materialized" unless the
// risk already is.

import { ApiError } from "./api/client.js";
import type { Risk, RiskUpdateBody } from "./api/ppm-types.js";
import type { StringKey } from "./i18n.svelte.js";

/** The statuses the service knows, in its order. */
export const RISK_STATUSES = [
  "open",
  "mitigating",
  "closed",
  "materialized",
] as const;
export type RiskStatus = (typeof RISK_STATUSES)[number];
/** The lowest and highest score for likelihood and impact. */
export const SCORE_MIN = 1;
export const SCORE_MAX = 5;
/** The longest mitigation, in bytes of UTF-8 (the service's `MAX_TEXT_LEN`). */
export const MITIGATION_MAX_BYTES = 1024;

/** What the form holds. A number input binds a number once typed in and a string before. */
export interface RiskForm {
  status: string;
  probability: string | number;
  impact: string | number;
  mitigation: string;
  reviewDate: string;
}

export function formFromRisk(r: Risk): RiskForm {
  return {
    status: r.status,
    probability: r.probability,
    impact: r.impact,
    mitigation: r.mitigation ?? "",
    reviewDate: r.review_date ?? "",
  };
}

/** The statuses the form offers for a risk now in `current`: "materialized" only when it already is (Escalate records
 * it, with the time). A status the service does not list stays shown until it is changed. */
export function statusChoices(current: string): string[] {
  const offered: string[] = RISK_STATUSES.filter(
    (s) => s !== "materialized" || current === "materialized",
  );
  return offered.includes(current) ? offered : [current, ...offered];
}

/** A whole number from a form value, or `null`. */
function whole(v: string | number): number | null {
  const raw = String(v ?? "").trim();
  if (raw === "") return null;
  const n = Number(raw);
  return Number.isSafeInteger(n) ? n : null;
}

const bytes = (s: string) => new TextEncoder().encode(s).length;

/** The problems with the form, in the order the form shows its fields, or none. */
export function riskProblems(f: RiskForm, current: Risk): StringKey[] {
  const problems: StringKey[] = [];
  if (
    !(RISK_STATUSES as readonly string[]).includes(f.status) &&
    f.status !== current.status
  )
    problems.push("p.risk_edit.problem_status");
  for (const v of [f.probability, f.impact]) {
    const n = whole(v);
    if (n === null || n < SCORE_MIN || n > SCORE_MAX) {
      problems.push("p.risk_edit.problem_score");
      break;
    }
  }
  if (bytes(f.mitigation) > MITIGATION_MAX_BYTES)
    problems.push("p.risk_edit.problem_mitigation");
  if (f.reviewDate === "" && current.review_date)
    problems.push("p.risk_edit.problem_review_cleared");
  else if (f.reviewDate !== "" && !/^\d{4}-\d{2}-\d{2}$/.test(f.reviewDate))
    problems.push("p.risk_edit.problem_review_date");
  return problems;
}

/** The body for the fields that changed, every problem, or `null` when nothing changed (nothing is sent). */
export function riskBody(
  f: RiskForm,
  current: Risk,
): { body: RiskUpdateBody } | { problems: StringKey[] } | null {
  const problems = riskProblems(f, current);
  if (problems.length > 0) return { problems };
  const body: RiskUpdateBody = {};
  if (f.status !== current.status) body.status = f.status;
  const p = whole(f.probability) as number;
  const i = whole(f.impact) as number;
  if (p !== current.probability) body.probability = p;
  if (i !== current.impact) body.impact = i;
  if (f.mitigation.trim() !== (current.mitigation ?? "").trim())
    body.mitigation = f.mitigation.trim();
  if (f.reviewDate !== (current.review_date ?? ""))
    body.review_date = f.reviewDate;
  return Object.keys(body).length === 0 ? null : { body };
}

/** The sentence for a refused change, or `null` for the general rule. Every 422 here has the one code `validation`
 * with English text, whichever rule it is; the page checks them all first, so a 422 after that is said as one. */
export function riskRefusalKey(err: unknown): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  if (err.status === 404) return "p.risk_edit.refused_gone";
  if (err.status === 422) return "p.risk_edit.refused_invalid";
  return null;
}
