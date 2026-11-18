// Pure helpers for the funding ledger's writes (`/ledger`, EX-12, NC-1 slice 11): what recording an allocation sends
// and the checks the service makes on it, which allocations a budget line may draw on (its own source, its own
// currency), the checks on a cost code and a finance export, and each refusal's sentence.

import { ApiError } from "./api/client.js";
import type { StringKey } from "./i18n.svelte.js";
import type { AllocationBody, BudgetBoard } from "./api/ppm-types.js";
import type { AllocationTrace, FundingAllocation } from "./api/ppm.js";
import { currencyCode, toMinor } from "./money.js";

/** The service's limits: a reference, a cost code, and a finance export. */
export const REFERENCE_MAX = 120;
export const COST_CODE_MAX = 60;
export const EXPORT_MAX_BYTES = 1_000_000;

/**
 * The body to record an allocation, or every problem with it: a source, an amount above 0 in the reader's own number
 * format, a three-letter currency, a date not after `today`, and a reference of at most 120 characters.
 */
export function allocationBody(
  form: {
    source: string;
    amount: string;
    currency: string;
    allocatedOn: string;
    reference: string;
  },
  today: string,
  locale?: string,
): { body: AllocationBody } | { problems: StringKey[] } {
  const problems: StringKey[] = [];
  if (form.source === "") problems.push("p.ledger.problem_source");
  const currency = currencyCode(form.currency);
  if (!currency) problems.push("p.ledger.problem_currency");
  const minor = currency ? toMinor(form.amount, currency, locale) : null;
  if (minor === null || minor <= 0) problems.push("p.ledger.problem_amount");
  if (form.allocatedOn === "") problems.push("p.ledger.problem_date");
  else if (form.allocatedOn > today)
    problems.push("p.ledger.problem_date_future");
  const reference = form.reference.trim();
  if ([...reference].length > REFERENCE_MAX)
    problems.push("p.ledger.problem_reference");
  if (problems.length > 0) return { problems };
  return {
    body: {
      funding_source_pid: form.source,
      amount_minor: minor!,
      currency: currency!,
      allocated_on: form.allocatedOn,
      ...(reference ? { reference } : {}),
    },
  };
}

type Line = BudgetBoard["lines"][number];

/**
 * The allocations a line may draw on: those of the line's own funding source and currency, as the service requires.
 * A line with no funding source may draw on none until it is given one.
 */
export function allocationChoices(
  line: Line,
  allocations: readonly FundingAllocation[],
): FundingAllocation[] {
  if (!line.funding_source_pid) return [];
  return allocations.filter(
    (a) =>
      a.funding_source_pid === line.funding_source_pid &&
      a.currency === line.currency,
  );
}

/** The problem with a cost code to map, or `null`: 1 to 60 characters once trimmed, and a line chosen. */
export function costCodeProblem(
  code: string,
  linePid: string,
): StringKey | null {
  const n = [...code.trim()].length;
  if (n < 1 || n > COST_CODE_MAX) return "p.ledger.problem_code";
  if (linePid === "") return "p.ledger.problem_code_line";
  return null;
}

/** The problem with a finance export before it is sent, or `null`: not empty, under 1 MB, with the three headers. */
export function exportProblem(csv: string): StringKey | null {
  if (csv.trim() === "") return "p.ledger.problem_export_empty";
  if (new TextEncoder().encode(csv).length > EXPORT_MAX_BYTES)
    return "p.ledger.problem_export_large";
  const header = csv.split(/\r?\n/).find((l) => l.trim() !== "") ?? "";
  const names = header
    .split(",")
    .map((h) => h.trim().replace(/^"|"$/g, "").toLowerCase());
  if (
    !["cost_code", "amount_minor", "currency"].every((h) => names.includes(h))
  )
    return "p.ledger.problem_export_header";
  return null;
}

/** The sentence for a refused ledger write: a 422 means the file or the record did not pass the service's checks. */
export function ledgerRefusalKey(
  action: "allocation" | "line" | "code" | "reconcile",
  err: unknown,
): StringKey | null {
  if (!(err instanceof ApiError) || err.status !== 422) return null;
  if (action === "line") return "p.ledger.refused_line";
  if (action === "reconcile") return "p.ledger.refused_export";
  return null;
}

/** Why the trace's money is untraced, in the catalogue's words (NC-1 slice 12 part a): the sentence for the trace's
 * `untraced_reason_code`, a general one when money is untraced for a reason this page has no code for, and `null`
 * when every unit is traced. The service's English `untraced_reason` is never shown. */
export function untracedReasonKey(
  money: Pick<
    AllocationTrace["money"],
    "untraced_minor" | "untraced_reason" | "untraced_reason_code"
  >,
): StringKey | null {
  switch (money.untraced_reason_code) {
    case "no_readable_line":
      return "p.ledger.untraced_no_readable_line";
    case "not_on_a_line":
      return "p.ledger.untraced_not_on_a_line";
    case "lines_over_plan":
      return "p.ledger.untraced_lines_over_plan";
    default:
      return money.untraced_reason || money.untraced_minor !== 0
        ? "p.ledger.untraced_other"
        : null;
  }
}
