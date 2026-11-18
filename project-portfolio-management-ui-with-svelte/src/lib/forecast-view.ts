// A plan's money read live (`/plans/[pid]/costs` and `/financials`, NC-1 slice 12 part f): the estimate at completion
// (`GET /api/plans/{pid}/financials/forecast`, and rolled up over the plans beneath one, `GET
// /api/financials/forecast`), the phased budget baselines (`GET`/`POST /api/plans/{pid}/budget-baselines`) and the
// release of a stage-gated budget line (`POST /api/plans/{pid}/budget-lines/{line}/release`). The service's rules
// (`controllers/financials.rs`, `domain/src/financials.rs`, `controllers/governance.rs`), said before anything is sent.
// The service still decides.
//
// A baseline is frozen once approved and never edited: a re-baseline is a new version, and every version after the
// first needs a reason (up to 1024 bytes). It has one currency and 1 to 256 periods, each a start, an end not before it
// and a planned amount not below zero. A budget line with a funding gate is held until the plan's stage reaches that
// gate; then it may be released, once, and only then can actual cost be recorded against it.

import { ApiError } from "./api/client.js";
import type {
  BaselinePeriod,
  BudgetBaseline,
  BudgetBaselineBody,
  EtcSource,
  GatedBudgetLine,
} from "./api/ppm-types.js";
import type { StringKey } from "./i18n.svelte.js";
import { isCivilDate } from "./locale-format.js";
import { currencyCode, toMinor } from "./money.js";

/** The deepest roll-up the service walks (`tba::MAX_ROLLUP_DEPTH`). */
export const DEPTH_MAX = 32;
/** The most periods one baseline holds (`validation::MAX_ARRAY_LEN`). */
export const PERIODS_MAX = 256;
/** The longest reason for a re-baseline, in bytes of UTF-8 (`validation::MAX_TEXT_LEN`; the service counts bytes). */
export const REASON_MAX_BYTES = 1024;
/** The phase gates in order (`domain/src/governance.rs`'s `GATES`). */
export const GATES = [
  "g0_concept",
  "g1_feasibility",
  "g2_definition",
  "g3_delivery",
  "g4_launch",
  "g5_benefits",
] as const;

/** Where an estimate to complete came from, in words. */
export const ETC_SOURCE_KEY: Record<EtcSource, StringKey> = {
  tpc: "p.forecast.etc_tpc",
  baseline_remaining: "p.forecast.etc_baseline",
};

const bytes = (s: string) => new TextEncoder().encode(s).length;

/** The problem with a roll-up depth (a number input's value: `null` when empty, which walks as deep as the service
 * allows). */
export function depthProblem(depth: number | null): StringKey | null {
  if (depth === null) return null;
  return Number.isInteger(depth) && depth >= 1 && depth <= DEPTH_MAX
    ? null
    : "p.forecast.problem_depth";
}

/** One period as typed: two dates and an amount in the baseline's currency, in the reader's number format. */
export interface PeriodForm {
  start: string;
  end: string;
  amount: string;
}

/** A new baseline as typed. */
export interface BaselineForm {
  currency: string;
  periods: PeriodForm[];
  reason: string;
}

/** A form to start from: the latest version's currency and periods (so a re-baseline changes only what moves), or one
 * empty period. */
export function baselineFormOf(
  latest: BudgetBaseline | null,
  toInput: (minor: number, currency: string) => string,
): BaselineForm {
  if (!latest)
    return {
      currency: "",
      periods: [{ start: "", end: "", amount: "" }],
      reason: "",
    };
  return {
    currency: latest.currency,
    periods: latest.periods.map((p) => ({
      start: p.period_start,
      end: p.period_end,
      amount: toInput(p.planned_minor, latest.currency),
    })),
    reason: "",
  };
}

const samePeriods = (a: BaselinePeriod[], b: BaselinePeriod[]) =>
  a.length === b.length &&
  a.every(
    (p, i) =>
      p.period_start === b[i]!.period_start &&
      p.period_end === b[i]!.period_end &&
      p.planned_minor === b[i]!.planned_minor,
  );

/**
 * The baseline to approve, or every problem with it. A period with nothing typed is left out. `latest` is the newest
 * version, or `null` for the first: a later one needs a reason, and one the same as the latest is not sent.
 */
export function baselineBody(
  form: BaselineForm,
  latest: BudgetBaseline | null,
  locale?: string,
): { body: BudgetBaselineBody } | { problems: StringKey[] } {
  const problems = new Set<StringKey>();
  const currency = currencyCode(form.currency);
  if (currency === null) problems.add("p.forecast.problem_currency");
  const typed = form.periods.filter(
    (p) => p.start.trim() || p.end.trim() || p.amount.trim(),
  );
  if (typed.length === 0) problems.add("p.forecast.problem_no_periods");
  if (typed.length > PERIODS_MAX) problems.add("p.forecast.problem_too_many");
  const periods: BaselinePeriod[] = [];
  for (const p of typed) {
    if (!isCivilDate(p.start) || !isCivilDate(p.end)) {
      problems.add("p.forecast.problem_dates");
      continue;
    }
    if (p.end < p.start) problems.add("p.forecast.problem_order");
    const minor = currency ? toMinor(p.amount, currency, locale) : null;
    if (currency && (minor === null || minor < 0))
      problems.add("p.forecast.problem_amount");
    periods.push({
      period_start: p.start,
      period_end: p.end,
      planned_minor: minor ?? 0,
    });
  }
  const reason = form.reason.trim();
  if (latest && !reason) problems.add("p.forecast.problem_reason");
  if (bytes(reason) > REASON_MAX_BYTES)
    problems.add("p.forecast.problem_reason_long");
  if (
    problems.size === 0 &&
    latest &&
    latest.currency === currency &&
    samePeriods(periods, latest.periods)
  )
    problems.add("p.forecast.problem_same");
  if (problems.size > 0) return { problems: [...problems] };
  return {
    body: {
      currency: currency!,
      periods,
      ...(reason ? { reason } : {}),
    },
  };
}

/** Whether a plan at `stage` has reached `gate` (the service's `gate_reached`: a stage at or past the gate). */
export function gateReached(stage: string | null, gate: string): boolean {
  const g = (GATES as readonly string[]).indexOf(gate);
  const s = stage === null ? -1 : (GATES as readonly string[]).indexOf(stage);
  return g >= 0 && s >= 0 && s >= g;
}

/** Where a gated budget line stands: released, held until its gate passes, or ready to release. A line with no gate
 * is not a tranche and is `null`. */
export function releaseState(
  line: GatedBudgetLine,
  stage: string | null,
): "released" | "held" | "ready" | null {
  if (!line.gate) return null;
  if (line.released_at) return "released";
  return gateReached(stage, line.gate) ? "ready" : "held";
}

const codeOf = (err: ApiError): string | null => {
  const body = err.body as { code?: unknown } | undefined;
  return typeof body?.code === "string" ? body.code : null;
};

/** The sentence for a refused roll-up, or `null` for the generic one: a 404 is a plan gone or one the reader cannot
 * read (the service says the same for both); the page checks the depth first. */
export function rollupRefusalKey(err: unknown): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  return err.status === 404 ? "p.forecast.refused_plan_gone" : null;
}

/** The sentence for a refused approval, or `null` for the generic one. The page checks every rule first, so a 422
 * after that is one it could not see (a version approved meanwhile, which needs a reason). */
export function baselineRefusalKey(err: unknown): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  if (err.status === 404) return "p.forecast.refused_gone";
  if (err.status === 422) return "p.forecast.refused_baseline";
  return null;
}

/** The sentence for a refused release, or `null` for the generic one: the programme's register incomplete
 * (`register_incomplete`), the line no longer held or its gate not passed (read again), or the line or plan gone. */
export function releaseRefusalKey(err: unknown): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  if (err.status === 404) return "p.forecast.refused_line_gone";
  if (err.status === 422)
    return codeOf(err) === "register_incomplete"
      ? "p.forecast.refused_register"
      : "p.forecast.refused_release";
  return null;
}
