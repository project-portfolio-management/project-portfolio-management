// Critical-chain estimates and the frozen buffer plan (`/plans/[pid]/critical-chain`, NC-1 slice 12 part i, CC-2,
// CC-3): each task's aggressive (about even odds) and safe estimate (`GET /api/plans/{pid}/ccpm-estimates`), one
// recorded or cleared (`PUT /api/plans/{pid}/tasks/{t_pid}/ccpm-estimate`), and the latest buffer plan frozen on
// `/plans/[pid]/buffers` (`GET /api/plans/{pid}/buffer-plan`). The service's rules (`controllers/ccpm_estimates.rs`,
// `domain/src/ccpm_estimates.rs`), said and checked before anything is sent; the service still decides.
//
// Each figure is whole business days from 1 to 3650, the aggressive one not above the safe one. A figure shown is the
// one recorded; else the risk range's optimistic (aggressive) or pessimistic (safe); else, only under the
// deployment's rule, one derived from the task's duration; each says where it came from. A milestone and a task with
// no duration are not estimated. Clearing both removes what is recorded (the others are left). Every 422 is one
// `unprocessable_entity` with every problem in English (a service gap), so the page checks them all first.

import { ApiError } from "./api/client.js";
import type {
  CcpmEstimateBody,
  CcpmSource,
  CcpmState,
  CcpmTask,
} from "./api/ppm-types.js";
import type { StringKey } from "./i18n.svelte.js";

/** The largest estimate, in business days. */
export const CCPM_MAX_DAYS = 3650;

/** The deployment's rule, in words. */
export const RULE_KEY: Record<string, StringKey> = {
  explicit_only: "p.ccpm.rule_explicit_only",
  half_of_safe: "p.ccpm.rule_half_of_safe",
};

/** Where a figure came from. */
export const SOURCE_KEY: Record<CcpmSource, StringKey> = {
  recorded: "p.ccpm.source_recorded",
  risk_range: "p.ccpm.source_risk_range",
  rule: "p.ccpm.source_rule",
};

/** A task's standing. */
export const STATE_KEY: Record<CcpmState, StringKey> = {
  complete: "p.ccpm.state_complete",
  derived: "p.ccpm.state_derived",
  missing: "p.ccpm.state_missing",
  inconsistent: "p.ccpm.state_inconsistent",
  milestone: "p.ccpm.state_milestone",
  no_duration: "p.ccpm.state_no_duration",
};

/** A standing the page does not know is a general word. */
export function stateKey(state: string): StringKey {
  return (
    (STATE_KEY as Record<string, StringKey>)[state] ?? "p.ccpm.state_other"
  );
}

/** Whether a standing needs someone to act (a shape as well as the word). */
export function needsAttention(state: string): boolean {
  return state === "missing" || state === "inconsistent";
}

/** Whether a task takes estimates (a milestone and a task with no duration do not). */
export function estimable(task: Pick<CcpmTask, "state">): boolean {
  return task.state !== "milestone" && task.state !== "no_duration";
}

/** The two estimates as typed (a number input's value, `null` or `undefined` when empty). */
export interface EstimateForm {
  aggressive: number | null | undefined;
  safe: number | null | undefined;
}

/** The form filled from what is recorded (a figure from the risk range or the rule is not recorded). */
export function estimateFormOf(task: CcpmTask | null): EstimateForm {
  const rec = (f: CcpmTask["aggressive"]) =>
    f && f.source === "recorded" ? f.days : null;
  return {
    aggressive: rec(task?.aggressive ?? null),
    safe: rec(task?.safe ?? null),
  };
}

const value = (v: number | null | undefined): number | null =>
  v === null || v === undefined ? null : Number(v);

/**
 * The body to send and whether it clears what is recorded; every problem at once; or `null` when it is what is
 * recorded (nothing is sent).
 */
export function estimateBody(
  form: EstimateForm,
  task: CcpmTask,
):
  | { body: CcpmEstimateBody; clears: boolean }
  | { problems: StringKey[] }
  | null {
  const problems: StringKey[] = [];
  const a = value(form.aggressive);
  const s = value(form.safe);
  const ok = (v: number) =>
    Number.isSafeInteger(v) && v >= 1 && v <= CCPM_MAX_DAYS;
  if (a !== null && !ok(a)) problems.push("p.ccpm.problem_aggressive");
  if (s !== null && !ok(s)) problems.push("p.ccpm.problem_safe");
  if (a !== null && s !== null && ok(a) && ok(s) && a > s)
    problems.push("p.ccpm.problem_order");
  if (problems.length > 0) return { problems };
  const stored = estimateFormOf(task);
  if (a === stored.aggressive && s === stored.safe) return null;
  return {
    body: { aggressive_days: a, safe_days: s },
    clears: a === null && s === null,
  };
}

/** The catalogue sentence for a refused save, or `null` for the general rule. */
export function estimateRefusalKey(err: unknown): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  if (err.status === 404) return "p.ccpm.refused_gone";
  if (err.status === 422) return "p.ccpm.refused_invalid";
  return null;
}
