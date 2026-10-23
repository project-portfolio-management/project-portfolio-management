// Tolerances: the words, shapes and unit conversions the tolerance pages show. Pure and tested.
//
// A colour is never the whole message. Every status has a word and a shape, `unknown` (a limit is set but could not
// be measured) and `not_set` (no limit) are their own states, and neither is green.

import type { ToleranceDimension, ToleranceStatus } from "#lib/api/ppm.js";
import { t } from "./i18n.svelte.js";

export type DimensionKey = ToleranceDimension["dimension"];

export const DIMENSIONS: readonly DimensionKey[] = [
  "time",
  "schedule",
  "cost",
  "risk",
  "resources",
];

/** Worst first, the order the board sorts in. */
export const STATUS_ORDER: readonly ToleranceStatus[] = [
  "red",
  "amber",
  "unknown",
  "green",
  "not_set",
];

/** A decorative shape per status (hidden from screen readers; the word is there). */
export const STATUS_SYMBOL: Record<ToleranceStatus, string> = {
  green: "●",
  amber: "▲",
  red: "■",
  unknown: "?",
  not_set: "–",
};

export const STATUS_TEXT: Record<ToleranceStatus, string> = {
  get green() {
    return t("l.tolerance.green");
  },
  get amber() {
    return t("l.tolerance.amber");
  },
  get red() {
    return t("l.tolerance.red");
  },
  get unknown() {
    return t("l.tolerance.unknown");
  },
  get not_set() {
    return t("l.tolerance.not_set");
  },
};

/** The name of a dimension. */
export function dimensionLabel(d: string): string {
  return d === "time"
    ? t("l.tolerance.time")
    : d === "schedule"
      ? t("l.tolerance.schedule")
      : d === "cost"
        ? t("l.tolerance.cost")
        : d === "risk"
          ? t("l.tolerance.risk")
          : d === "resources"
            ? t("l.tolerance.resources")
            : d;
}

/** A figure in the dimension's own unit: "5 days", "12.5%", "score 20", "30 points". */
export function amountText(
  d: string,
  value: number | null | undefined,
): string {
  if (value === null || value === undefined) return "";
  if (d === "time" || d === "schedule")
    return value === 1
      ? t("l.tolerance.day_one", { n: value })
      : t("l.tolerance.day_other", { n: value });
  if (d === "cost") return `${value / 100}%`;
  if (d === "risk") return t("l.tolerance.score", { n: value });
  return t("l.tolerance.points", { n: value });
}

/** What the person types for a limit, as the API stores it (cost is typed in percent, stored in basis points). */
export function toApiLimit(d: string, typed: number): number {
  return d === "cost" ? Math.round(typed * 100) : Math.round(typed);
}

/** The inverse of {@link toApiLimit}, for filling a form. */
export function fromApiLimit(d: string, stored: number): number {
  return d === "cost" ? stored / 100 : stored;
}

/** The label for the limit field, saying what unit it is in. */
export function limitFieldLabel(d: string): string {
  return d === "time"
    ? t("l.tolerance.field_time")
    : d === "schedule"
      ? t("l.tolerance.field_schedule")
      : d === "cost"
        ? t("l.tolerance.field_cost")
        : d === "risk"
          ? t("l.tolerance.field_risk")
          : t("l.tolerance.field_resources");
}

/** "Used 80% of the limit" or the reason there is nothing to compare. */
export function usageText(dim: ToleranceDimension): string {
  if (dim.status === "not_set") return t("l.tolerance.no_limit");
  if (dim.status === "unknown")
    return dim.reason ?? t("l.tolerance.could_not_measure");
  if (dim.usage_percent === null || dim.usage_percent === undefined)
    return t("l.tolerance.zero_tolerance");
  return t("l.tolerance.used", { percent: dim.usage_percent });
}

/** Where the limit comes from, in words. */
export function sourceText(dim: ToleranceDimension): string {
  return dim.source === "own"
    ? t("l.tolerance.own")
    : dim.source === "inherited"
      ? t("l.tolerance.inherited")
      : "";
}
