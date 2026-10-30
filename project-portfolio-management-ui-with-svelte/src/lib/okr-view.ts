// OKRs: the plain words and numbers the OKR page shows. Pure, so they are
// unit-testable without a component.
//
// A score is derived on the server and arrives in basis points, or `null`. A
// `null` is *not measured*, never "0%": an objective nobody can measure yet
// must not read as a failure, nor as a success.

import type { OkrKeyResult } from "#lib/api/ppm.js";
import { t } from "./i18n.svelte.js";
import { formatBasisPoints } from "#lib/locale-format.js";

/** "40%" for 4000 basis points; "Not measured" for `null`. */
export function percentText(basisPoints: number | null | undefined): string {
  if (basisPoints === null || basisPoints === undefined)
    return t("l.okr_view.not_measured");
  return formatBasisPoints(basisPoints, basisPoints % 100 === 0 ? 0 : 1);
}

/** The value for a `<progress>` element (0-100), or `null` when unmeasured. */
export function progressValue(
  basisPoints: number | null | undefined,
): number | null {
  if (basisPoints === null || basisPoints === undefined) return null;
  return Math.max(0, Math.min(100, basisPoints / 100));
}

const REASONS: Record<string, string> = {
  get NoRange() {
    return t("l.okr_view.not_measurable_the_start_and");
  },
  get NoTolerance() {
    return t("l.okr_view.not_measurable_a_maintain_key");
  },
  get Overflow() {
    return t("l.okr_view.not_measurable_the_numbers_are");
  },
};

/** Why a key result has no progress, in a sentence. */
export function unmeasurableText(reason: string | null | undefined): string {
  if (!reason) return t("l.okr_view.not_measured");
  return REASONS[reason] ?? t("l.okr_view.not_measurable", { reason });
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
  return t("l.okr_view.of", { current, target, u });
}

/**
 * Confidence is recorded beside a score and never part of it. Said so in the
 * text, so nobody reads it as a weight.
 */
export function confidenceText(
  confidence: number | null | undefined,
): string | null {
  if (confidence === null || confidence === undefined) return null;
  return t("l.okr_view.confidence_not_part_of_the", { confidence });
}
