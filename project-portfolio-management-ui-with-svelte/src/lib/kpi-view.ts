// KPIs: the words, symbols and sentences the KPI page shows. Pure and tested.
//
// Status is never conveyed by colour alone: every status has a word and a
// symbol, and "no data" is its own state — not red, not green.

import type { Kpi, KpiStatus, KpiTrend, KpiView } from "#lib/api/ppm.js";
import { t } from "./i18n.svelte.js";

export const STATUS_ORDER: readonly KpiStatus[] = [
  "red",
  "amber",
  "no_data",
  "green",
];

export const STATUS_TEXT: Record<KpiStatus, string> = {
  get green() { return t("p.kpis.on_target"); },
  get amber() { return t("p.kpis.watch"); },
  get red() { return t("p.kpis.off_target"); },
  get no_data() { return t("p.kpis.no_data_yet"); },
};

/** A decorative shape per status (hide it from screen readers; the word is there). */
export const STATUS_SYMBOL: Record<KpiStatus, string> = {
  green: "●",
  amber: "▲",
  red: "■",
  no_data: "○",
};

/** A reading with its unit. */
export function valueText(
  value: number,
  unit: string | null | undefined,
): string {
  return unit ? `${value} ${unit}` : String(value);
}

/** "Target 95 or more; red at 80 or less (percent)". */
export function thresholdText(
  kpi: Pick<Kpi, "direction" | "target_value" | "red_value" | "unit">,
): string {
  const u = kpi.unit ? ` ${kpi.unit}` : "";
  return kpi.direction === "higher_better"
    ? t("l.kpi_view.target_or_more_red_at", { target_value: kpi.target_value, red_value: kpi.red_value, u })
    : t("l.kpi_view.target_or_less_red_at", { target_value: kpi.target_value, red_value: kpi.red_value, u });
}

const TREND_TEXT: Record<KpiTrend, string> = {
  get improving() { return t("l.kpi_view.improving"); },
  get worsening() { return t("l.kpi_view.getting_worse"); },
  get flat() { return t("l.kpi_view.no_change"); },
};

export function trendText(trend: KpiTrend | null | undefined): string | null {
  return trend ? TREND_TEXT[trend] : null;
}

/** Says a cadence KPI is overdue for a reading; nothing when it is not. */
export function staleText(view: Pick<KpiView, "stale" | "kpi">): string | null {
  return view.stale
    ? t("l.kpi_view.no_reading_within_twice_its", { cadence: view.kpi.cadence })
    : null;
}

/** The one-line summary for a KPI row, readable aloud. */
export function summaryText(view: KpiView): string {
  const latest = view.latest
    ? t("c.KpiCard.on", { valueText: valueText(view.latest.value, view.kpi.unit), observed_on: view.latest.observed_on })
    : t("l.kpi_view.no_reading_yet");
  return `${view.kpi.name}: ${STATUS_TEXT[view.status]}, ${latest}`;
}

/** Counts in the order a reader should look: red first. */
export function summaryParts(
  summary: Record<KpiStatus, number>,
): { status: KpiStatus; label: string; count: number }[] {
  return STATUS_ORDER.map((status) => ({
    status,
    label: STATUS_TEXT[status],
    count: summary[status],
  }));
}
