// Pure helpers for a plan's benefit classification and validation (`/plans/[pid]/benefits`, DP-9, NC-1 slice 11): the
// form a benefit fills, what saving its classification sends (all of it, since a field left out is cleared), the
// checks the service makes, a validation's body, and each refusal's sentence. A realized amount is a claim; only a
// validation by someone outside the programme validates it, and the latest one counts.

import { ApiError } from "./api/client.js";
import type { StringKey } from "./i18n.svelte.js";
import type { BenefitBoard, BenefitClassifyBody } from "./api/ppm-types.js";
import { minorToInput, toMinor } from "./money.js";

type Benefit = BenefitBoard["benefits"][number];

export interface ClassifyForm {
  recurrence: string;
  annual: string;
  classification: string;
  kpi: string;
}

/** The form filled from what the benefit has (a yearly amount in the reader's own number format). */
export function formOf(b: Benefit, locale?: string): ClassifyForm {
  return {
    recurrence: b.recurrence ?? "",
    annual:
      b.annualized_minor != null && b.currency
        ? minorToInput(b.annualized_minor, b.currency, locale)
        : "",
    classification: b.classification ?? "",
    kpi: b.kpi_pid ?? "",
  };
}

/**
 * The whole classification to send, or every problem with it: a yearly amount only for a recurring benefit with a
 * currency, and not negative. An empty choice is left out, which clears it.
 */
export function classifyBody(
  form: ClassifyForm,
  b: Benefit,
  locale?: string,
): { body: BenefitClassifyBody } | { problems: StringKey[] } {
  const problems: StringKey[] = [];
  const body: BenefitClassifyBody = {};
  if (form.recurrence === "recurring" || form.recurrence === "one_off")
    body.recurrence = form.recurrence;
  if (form.annual.trim() !== "") {
    if (form.recurrence !== "recurring")
      problems.push("p.benefits.problem_annual_recurring");
    else if (!b.currency) problems.push("p.benefits.problem_annual_financial");
    else {
      const minor = toMinor(form.annual, b.currency, locale);
      if (minor === null || minor < 0)
        problems.push("p.benefits.problem_annual_amount");
      else body.annualized_minor = minor;
    }
  }
  if (
    form.classification === "cash_releasing" ||
    form.classification === "non_cash_releasing" ||
    form.classification === "societal"
  )
    body.classification = form.classification;
  if (form.kpi) body.kpi_pid = form.kpi;
  return problems.length > 0 ? { problems } : { body };
}

/** A validation's body, or the problem: an amount validated to date, not negative, in the benefit's currency. */
export function validationBody(
  amount: string,
  currency: string,
  note: string,
  locale?: string,
): { body: { amount_minor: number; note?: string } } | { problem: StringKey } {
  const minor = toMinor(amount, currency, locale);
  if (minor === null || minor < 0)
    return { problem: "p.benefits.problem_validated_amount" };
  const n = note.trim();
  return { body: { amount_minor: minor, ...(n ? { note: n } : {}) } };
}

/** Each standing's catalogue key. */
export const STANDING_LABELS: Record<string, StringKey> = {
  not_claimed: "p.benefits.standing_not_claimed",
  claimed: "p.benefits.standing_claimed",
  partly_validated: "p.benefits.standing_partly_validated",
  validated: "p.benefits.standing_validated",
};

/** The sentence for a refused validation: a 403 is a validator who is a member of the programme. */
export function validationRefusalKey(err: unknown): StringKey | null {
  return err instanceof ApiError && err.status === 403
    ? "p.benefits.refused_member"
    : null;
}
