// Pure helpers for a plan's delivery confidence (`/plans/[pid]/confidence`, DP-2, NC-1 slice 11): each value as a
// symbol as well as a word and a colour, the checks the service makes before an assessment is sent, and the history
// newest first. The stated value is a person's view, kept beside the derived RAG and never blended with it.

import { ApiError } from "./api/client.js";
import type { StringKey } from "./i18n.svelte.js";
import type { ConfidenceAssessment, ConfidenceValue } from "./api/ppm-types.js";

/** A rationale needs at least this many characters (`MIN_RATIONALE`). */
export const RATIONALE_MIN = 10;

/** Each value's symbol and tone, as the director pack draws them, so colour is never the only signal. */
export const CONFIDENCE_MARK: Record<
  ConfidenceValue | "not_assessed",
  { symbol: string; tone: string }
> = {
  green: { symbol: "●", tone: "green" },
  amber_green: { symbol: "◐", tone: "green" },
  amber: { symbol: "◆", tone: "amber" },
  amber_red: { symbol: "▲", tone: "red" },
  red: { symbol: "▲", tone: "red" },
  not_assessed: { symbol: "–", tone: "grey" },
};

/** Whether stating `value` opens an exception, or worsens the open one (`opens_exception`). */
export function opensException(value: string): boolean {
  return value === "amber_red" || value === "red";
}

/** Every problem with an assessment, as the service checks them. `today` is `YYYY-MM-DD` in the browser's zone. */
export function assessmentProblems(
  form: { value: string; kind: string; rationale: string; assessedOn: string },
  today: string,
): StringKey[] {
  const out: StringKey[] = [];
  if (form.value === "") out.push("p.confidence.problem_value");
  if (form.kind !== "self" && form.kind !== "independent")
    out.push("p.confidence.problem_kind");
  if ([...form.rationale.trim()].length < RATIONALE_MIN)
    out.push("p.confidence.problem_rationale");
  if (form.assessedOn !== "" && form.assessedOn > today)
    out.push("p.confidence.problem_future");
  return out;
}

/** The history newest first; the service sends it oldest first. */
export function newestFirst(
  history: readonly ConfidenceAssessment[],
): ConfidenceAssessment[] {
  return [...history].reverse();
}

/** The sentence for a refused assessment: a 403 is an independent one without the right; else `null`. */
export function assessRefusalKey(err: unknown): StringKey | null {
  return err instanceof ApiError && err.status === 403
    ? "p.confidence.refused_independent"
    : null;
}
