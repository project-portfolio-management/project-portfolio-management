// Pure helpers for funding sources (`/funding`, DP-6, NC-1 slice 11): the checks the service makes on a new source,
// the choices for a budget line (active sources, and a retired one the line already has), and each refusal's sentence.

import { ApiError } from "./api/client.js";
import type { StringKey } from "./i18n.svelte.js";
import type { FundingSource } from "./api/ppm-types.js";

/** A source's name is 2 to 120 characters once trimmed. */
export const SOURCE_NAME_MIN = 2;
export const SOURCE_NAME_MAX = 120;

/** The problem with a new source's name, or `null`; a name already on the list (in any case) is said first too. */
export function sourceNameProblem(
  name: string,
  existing: readonly FundingSource[],
): StringKey | null {
  const n = name.trim();
  const len = [...n].length;
  if (len < SOURCE_NAME_MIN || len > SOURCE_NAME_MAX)
    return "p.funding.problem_name";
  if (existing.some((s) => s.name.toLowerCase() === n.toLowerCase()))
    return "p.funding.problem_name_taken";
  return null;
}

/**
 * The sources a line may be given: every active one, and the line's own if it is retired (so the choice shows what
 * the line has; it cannot be chosen again once changed).
 */
export function lineChoices(
  sources: readonly FundingSource[],
  current: string | null | undefined,
): FundingSource[] {
  return sources.filter((s) => s.active || s.pid === current);
}

/** A financial year, written as the year it starts in, from 2000 to 2100, or empty for the current one. */
export function fyProblem(fy: string): StringKey | null {
  if (fy.trim() === "") return null;
  const n = Number(fy);
  return Number.isInteger(n) && n >= 2000 && n <= 2100
    ? null
    : "p.funding.problem_fy";
}

/** The sentence for a refusal: a name taken (409) or a source no longer active (422); else `null`. */
export function fundingRefusalKey(err: unknown): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  if (err.status === 409) return "p.funding.problem_name_taken";
  if (err.status === 422) return "p.funding.refused_inactive";
  return null;
}
