// OKRs: the plain words and numbers the OKR page shows. Pure, so they are
// unit-testable without a component.
//
// A score is derived on the server and arrives in basis points, or `null`. A
// `null` is *not measured*, never "0%": an objective nobody can measure yet
// must not read as a failure, nor as a success.

import type { OkrKeyResult } from "#lib/api/ppm.js";

/** "40%" for 4000 basis points; "Not measured" for `null`. */
export function percentText(basisPoints: number | null | undefined): string {
  if (basisPoints === null || basisPoints === undefined) return "Not measured";
  const pct = basisPoints / 100;
  return `${Number.isInteger(pct) ? pct : pct.toFixed(1)}%`;
}

/** The value for a `<progress>` element (0-100), or `null` when unmeasured. */
export function progressValue(
  basisPoints: number | null | undefined,
): number | null {
  if (basisPoints === null || basisPoints === undefined) return null;
  return Math.max(0, Math.min(100, basisPoints / 100));
}

const REASONS: Record<string, string> = {
  NoRange:
    "Not measurable: the start and the target are the same, so there is no distance to travel.",
  NoTolerance: "Not measurable: a maintain key result needs a tolerance band.",
  Overflow: "Not measurable: the numbers are too large to score.",
};

/** Why a key result has no progress, in a sentence. */
export function unmeasurableText(reason: string | null | undefined): string {
  if (!reason) return "Not measured";
  return REASONS[reason] ?? `Not measurable (${reason}).`;
}

/** The progress text for a key result: a percentage, or the reason it has none. */
export function keyResultProgress(kr: OkrKeyResult): string {
  return kr.progress_basis_points === null
    ? unmeasurableText(kr.unmeasurable)
    : percentText(kr.progress_basis_points);
}

/** "40 of 200 users" style: current, target and unit. */
export function valueText(
  current: number,
  target: number,
  unit: string | null,
): string {
  const u = unit ? ` ${unit}` : "";
  return `${current} of ${target}${u}`;
}

/**
 * Confidence is recorded beside a score and never part of it. Said so in the
 * text, so nobody reads it as a weight.
 */
export function confidenceText(
  confidence: number | null | undefined,
): string | null {
  if (confidence === null || confidence === undefined) return null;
  return `Confidence ${confidence}% (not part of the score)`;
}
