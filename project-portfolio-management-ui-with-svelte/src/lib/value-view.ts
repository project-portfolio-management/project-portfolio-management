// A plan's realized value (`/plans/[pid]/benefits` and `/plans/[pid]/costs`, NC-1 slice 12 part f): the realized-gains
// reading (`GET /api/plans/{pid}/value-realization`), an observed value (`POST /api/plans/{pid}/value-points`) and a
// promised business-case target (`POST /api/plans/{pid}/business-case`). The service's rules (`controllers/value.rs`,
// `domain/src/value.rs`), said before anything is sent. The service still decides.
//
// Every figure is derived when read and nothing is stored, so a new value point changes every figure resting on it.
// An absent figure is said with its reason, never as zero. The return on investment adds every value point and sets
// it against the actual cost recorded; the service converts no currency, so it is withheld when the budget lines have
// more than one. A value point is measured, estimated or asserted (anything else is asserted). One value point per
// plan may be the first measurable one, which stops the time-to-value clock that the first business-case target
// started; neither can be moved. A target's metric is required, and it was promised in the charter or at a gate
// review.

import { ApiError } from "./api/client.js";
import type {
  BusinessCaseBody,
  ValueAbsent,
  ValueMethod,
  ValuePointBody,
} from "./api/ppm-types.js";
import type { StringKey } from "./i18n.svelte.js";
import { currencyCode, toMinor } from "./money.js";
import { parseNumber } from "./locale-format.js";

/** How a value point was arrived at, strongest first. */
export const METHODS: readonly ValueMethod[] = [
  "measured",
  "estimated",
  "asserted",
];

/** Each method's words. */
export const METHOD_KEY: Record<ValueMethod, StringKey> = {
  measured: "p.value.method_measured",
  estimated: "p.value.method_estimated",
  asserted: "p.value.method_asserted",
};

/** Why a figure is absent, in words. */
export const ABSENT_KEY: Record<ValueAbsent, StringKey> = {
  unrealized: "p.value.absent_unrealized",
  no_denominator: "p.value.absent_no_denominator",
  mixed_currency: "p.value.absent_mixed_currency",
  no_baseline: "p.value.absent_no_baseline",
  no_responses: "p.value.absent_no_responses",
  overflow: "p.value.absent_overflow",
};

/** Where a business-case target was promised. */
export const SOURCES = ["charter", "gate_review"] as const;
export type TargetSource = (typeof SOURCES)[number];

/** Each source's words. */
export const SOURCE_KEY: Record<TargetSource, StringKey> = {
  charter: "p.value.source_charter",
  gate_review: "p.value.source_gate_review",
};

/** The words for a source the service sent; one this page does not know is a general word. */
export function sourceKey(source: string): StringKey {
  return (
    (SOURCE_KEY as Record<string, StringKey>)[source] ?? "p.value.source_other"
  );
}

/** The words for an absent figure; a reason this page does not know is a general sentence. */
export function absentKey(absent: string | null | undefined): StringKey {
  return (
    (absent && (ABSENT_KEY as Record<string, StringKey>)[absent]) ||
    "p.value.absent_other"
  );
}

/** An observed value as typed. */
export interface ValuePointForm {
  /** The value, in the reader's number format: money when a currency is given, else a whole number. */
  value: string;
  currency: string;
  method: ValueMethod;
  first: boolean;
  evidence: string;
}

/** The value point to record, or every problem with it. A blank currency or evidence is not sent; the first-measurable
 * flag is sent only when set. */
export function valuePointBody(
  form: ValuePointForm,
  locale?: string,
): { body: ValuePointBody } | { problems: StringKey[] } {
  const problems: StringKey[] = [];
  const typedCurrency = form.currency.trim();
  const currency = typedCurrency ? currencyCode(typedCurrency) : null;
  if (typedCurrency && currency === null)
    problems.push("p.value.problem_currency");
  let value: number | null = null;
  if (currency) value = toMinor(form.value, currency, locale);
  else if (!typedCurrency) {
    const n = parseNumber(form.value, locale);
    value = n !== null && Number.isSafeInteger(n) ? n : null;
  }
  if (value === null && !(typedCurrency && currency === null))
    problems.push(
      currency ? "p.value.problem_amount" : "p.value.problem_whole",
    );
  if (!METHODS.includes(form.method)) problems.push("p.value.problem_method");
  if (problems.length > 0) return { problems };
  const evidence = form.evidence.trim();
  return {
    body: {
      value: value!,
      method: form.method,
      ...(currency ? { currency } : {}),
      ...(form.first ? { is_first_measurable: true } : {}),
      ...(evidence ? { evidence_ref: evidence } : {}),
    },
  };
}

/** The sentence for a refused value point, or `null` for the generic one. The only rule the page cannot check is that
 * the plan has a first-measurable point already. */
export function valuePointRefusalKey(
  err: unknown,
  first: boolean,
): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  if (err.status === 404) return "p.value.refused_gone";
  if (err.status === 422)
    return first ? "p.value.refused_first_taken" : "p.value.refused_value";
  return null;
}

/** A business-case target as typed: the two values are a number input's (`null` when empty). */
export interface BusinessCaseForm {
  metric: string;
  baseline: number | null;
  target: number | null;
  unit: string;
  source: TargetSource;
}

/** The target to record, or every problem with it: a metric, two whole numbers in its unit, and where it was promised.
 * A blank unit is not sent. */
export function businessCaseBody(
  form: BusinessCaseForm,
): { body: BusinessCaseBody } | { problems: StringKey[] } {
  const problems: StringKey[] = [];
  const metric = form.metric.trim();
  if (!metric) problems.push("p.value.problem_metric");
  const whole = (n: number | null) => n !== null && Number.isSafeInteger(n);
  if (!whole(form.baseline) || !whole(form.target))
    problems.push("p.value.problem_target_values");
  const unit = form.unit.trim();
  if (!SOURCES.includes(form.source)) problems.push("p.value.problem_source");
  if (problems.length > 0) return { problems };
  return {
    body: {
      metric,
      baseline_value: form.baseline!,
      target_value: form.target!,
      ...(unit ? { unit } : {}),
      source: form.source,
    },
  };
}

/** The sentence for a refused target, or `null` for the generic one. */
export function businessCaseRefusalKey(err: unknown): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  if (err.status === 404) return "p.value.refused_gone";
  if (err.status === 422) return "p.value.refused_target";
  return null;
}
