/**
 * Pure helpers for the earned-value view (SC-4). No DOM, no Svelte, no clock:
 * unit-tested. Ratios arrive as **basis points** (10 000 = 1.0) and money as minor
 * units; both are formatted with integer arithmetic only.
 */

import { dayNumber } from "./lily-grids";
import type {
  EvAmount,
  EvBlock,
  EvRag,
  EvRatio,
  EvSeries,
  EvTcpi,
} from "./api/ppm";
import { t } from "./i18n.svelte.js";
import { calendarDays } from "./units.js";

/** `8000` → `"0.80"`; `null` → `"—"`. Two decimals, half-up, integer arithmetic. */
export function ratioText(bp: number | null): string {
  if (bp === null) return "—";
  const hundredths = Math.floor((Math.abs(bp) + 50) / 100);
  const text = `${Math.floor(hundredths / 100)}.${String(hundredths % 100).padStart(2, "0")}`;
  return bp < 0 ? `-${text}` : text;
}

/** Why a ratio is missing, in words. */
export function absentText(r: EvRatio): string {
  switch (r.absent) {
    case "no_planned_value_yet":
      return t("l.ev_view.nothing_was_planned_to_be");
    case "no_actuals_recorded":
      return t("l.ev_view.no_cost_has_been_recorded");
    case "no_planned_value":
      return t("l.ev_view.the_baseline_plans_no_value");
    default:
      return t("l.ev_view.not_available");
  }
}

/** A ratio as text, or the reason it has none. */
export function ratioOrReason(r: EvRatio): string {
  return r.bp === null ? absentText(r) : ratioText(r.bp);
}

export type Health = "ahead" | "on_track" | "behind" | "unknown";

/** Within 5% either way counts as on track; the band is stated, not hidden. */
export const ON_TRACK_BAND_BP = 500;

/** Where a ratio stands: ahead / on track / behind, or unknown when it has no value. */
export function health(bp: number | null): Health {
  if (bp === null) return "unknown";
  if (bp > 10_000 + ON_TRACK_BAND_BP) return "ahead";
  if (bp < 10_000 - ON_TRACK_BAND_BP) return "behind";
  return "on_track";
}

const MARK: Record<Health, string> = {
  ahead: "▲",
  on_track: "●",
  behind: "▼",
  unknown: "?",
};

/** A marker for a state, so colour is never the only signal. */
export function marker(h: Health): string {
  return MARK[h];
}

/** The sentence for schedule performance. */
export function scheduleSentence(
  b: EvBlock,
  money: (minor: number) => string,
): string {
  const h = health(b.spi.bp);
  if (b.spi.bp === null) return t("l.ev_view.schedule_no_ratio", { absentText: absentText(b.spi) });
  const gap = t(b.sv_minor >= 0 ? "l.ev_view.gap_more" : "l.ev_view.gap_less", { money: money(Math.abs(b.sv_minor)) });
  const word =
    h === "ahead"
      ? t("l.ev_view.ahead_of_schedule")
      : h === "behind"
        ? t("l.ev_view.behind_schedule")
        : t("l.ev_view.on_schedule");
  return t("l.ev_view.spi", { word, ratioText: ratioText(b.spi.bp), gap });
}

/** The sentence for cost performance. */
export function costSentence(
  b: EvBlock,
  money: (minor: number) => string,
): string {
  if (b.cv_minor === null || b.cpi.bp === null)
    return t("l.ev_view.cost_no_ratio", { absentText: absentText(b.cpi) });
  const h = health(b.cpi.bp);
  const word =
    h === "ahead"
      ? t("l.ev_view.under_budget")
      : h === "behind"
        ? t("l.ev_view.over_budget")
        : t("l.ev_view.on_budget");
  return t(b.cv_minor >= 0 ? "l.ev_view.cost_line_less" : "l.ev_view.cost_line_more", { word, ratio: ratioText(b.cpi.bp), money: money(Math.abs(b.cv_minor)) });
}

/** The sentence for earned schedule, which still says so when a project is late and finished. */
export function earnedScheduleSentence(b: EvBlock): string {
  if (b.spi_t.bp === null || b.earned_schedule_days === null)
    return t("l.ev_view.earned_schedule_no_ratio", { absentText: absentText(b.spi_t) });
  const days = calendarDays;
  const h = health(b.spi_t.bp);
  const word = h === "ahead" ? t("l.ev_view.word_ahead") : h === "behind" ? t("l.ev_view.word_behind") : t("p.plans.pid.joint_risk.on_time");
  return t("l.ev_view.by_earned_schedule_the_work", { word, days: days(b.actual_days), days2: days(b.earned_schedule_days), ratioText: ratioText(b.spi_t.bp) });
}

/** One bar of the PV / EV / AC chart, as a percent of the budget at completion. */
export interface EvBar {
  key: "pv" | "ev" | "ac";
  label: string;
  minor: number | null;
  /** Percent of the chart's full width (the budget, or the largest figure if it is bigger). */
  widthPct: number;
  /** Where the budget-at-completion line sits, in percent of the chart's width. */
  bacPct: number;
}

/** Three bars (planned, earned, actual) against the budget line; the chart widens if cost overran. */
export function evBars(b: EvBlock): EvBar[] {
  const top = Math.max(b.bac_minor, b.pv_minor, b.ev_minor, b.ac_minor ?? 0, 1);
  const pct = (n: number) => Math.max(0, Math.min(100, (n / top) * 100));
  const bacPct = pct(b.bac_minor);
  return [
    {
      key: "pv",
      label: t("p.plans.pid.earned_value.planned_value"),
      minor: b.pv_minor,
      widthPct: pct(b.pv_minor),
      bacPct,
    },
    {
      key: "ev",
      label: t("p.plans.pid.earned_value.earned_value"),
      minor: b.ev_minor,
      widthPct: pct(b.ev_minor),
      bacPct,
    },
    {
      key: "ac",
      label: t("p.plans.pid.earned_value.actual_cost"),
      minor: b.ac_minor,
      widthPct: b.ac_minor === null ? 0 : pct(b.ac_minor),
      bacPct,
    },
  ];
}

/** The honest one-liner for how the percent complete was counted. */
export function methodSentence(pc: {
  default_method: string;
  weighting: string;
  tasks_counted: number;
  manual_tasks: string[];
  unweighted_tasks: string[];
}): string {
  const how =
    pc.default_method === "fifty_fifty"
      ? t("p.plans.pid.earned_value.started_work_counts_half")
      : pc.default_method === "points"
        ? t("l.ev_view.finished_work_counts_for_its")
        : t("l.ev_view.only_finished_work_counts");
  const parts = [
    t(pc.tasks_counted === 1 ? "l.ev_view.tasks_counted_one" : "l.ev_view.tasks_counted_other", { count: pc.tasks_counted, weighting: pc.weighting === "points" ? t("l.ev_view.by_story_points") : t("l.ev_view.equally"), how }),
  ];
  if (pc.manual_tasks.length > 0)
    parts.push(
      t("l.ev_view.counted_by_a_person_entered", { length: pc.manual_tasks.length }),
    );
  if (pc.unweighted_tasks.length > 0)
    parts.push(t("l.ev_view.with_no_points_carry_no", { length: pc.unweighted_tasks.length }));
  return parts.join("; ");
}

const RAG: Record<EvRag, { marker: string; word: string }> = {
  green: { marker: "●", get word() { return t("l.ev_view.on_track"); } },
  amber: { marker: "◐", get word() { return t("l.ev_view.slipping"); } },
  red: { marker: "✖", get word() { return t("l.ev_view.off_plan"); } },
  unknown: { marker: "?", get word() { return t("l.ev_view.not_yet_known"); } },
};

/** A marker and a word for a traffic light, so the colour is never the only signal. */
export function ragText(r: EvRag): { marker: string; word: string } {
  return RAG[r];
}

/** Why a forecast amount is missing, in words. */
export function amountAbsentText(a: EvAmount): string {
  switch (a.absent) {
    case "no_actual_cost":
      return t("l.ev_view.no_actual_cost_is_recorded");
    case "undefined_denominator":
      return t("l.ev_view.the_ratio_it_divides_by");
    case "no_bottom_up_estimate":
      return t("l.ev_view.no_estimate_to_complete_has");
    case "budget_exhausted":
      return t("l.ev_view.the_budget_is_already_spent");
    case "no_remaining_cost":
      return t("l.ev_view.no_cost_remains");
    default:
      return t("l.ev_view.not_available");
  }
}

/** The sentence for what the remaining work needs, which says so when it cannot be met. */
export function tcpiSentence(tcpi: EvTcpi, target: string): string {
  if (tcpi.bp === null) {
    return tcpi.absent === "budget_exhausted"
      ? t("l.ev_view.against_the_money_is_already", { target })
      : t("l.ev_view.against", { target, amountAbsentText: amountAbsentText({ minor: null, absent: tcpi.absent }) });
  }
  if (tcpi.needs_improved_efficiency) {
    return t("l.ev_view.against_the_remaining_work_must", { target, ratioText: ratioText(tcpi.bp), ratioText2: ratioText(tcpi.bp) });
  }
  return t("l.ev_view.against_the_remaining_work_can", { target, ratioText: ratioText(tcpi.bp), ratioText2: ratioText(tcpi.bp) });
}

/** The S-curve's drawing data for one currency, in SVG user units. */
export interface SCurveGeometry {
  width: number;
  height: number;
  /** Planned value as a polyline `x,y x,y …`. */
  pv: string;
  /** Earned value on the days it was recorded. */
  ev: { x: number; y: number; label: string }[];
  /** Actual cost on the days it was recorded. */
  ac: { x: number; y: number; label: string }[];
  /** The y of the budget-at-completion line. */
  bacY: number;
  from: string;
  to: string;
}

/**
 * Geometry for the S-curve. The planned curve is the whole baseline; earned value
 * and actual cost are **dots only where they were recorded** (never joined or
 * filled in between, because between snapshots they are not known).
 */
export function sCurve(
  curve: EvSeries["curves"][number],
  width = 640,
  height = 260,
): SCurveGeometry | null {
  const days = curve.points
    .map((p) => dayNumber(p.date))
    .filter((d): d is number => d !== null);
  if (days.length === 0) return null;
  const [lo, hi] = [Math.min(...days), Math.max(...days)];
  const span = Math.max(hi - lo, 1);
  const top = Math.max(
    curve.bac_minor,
    ...curve.points.map((p) =>
      Math.max(p.pv_minor, p.ev_minor ?? 0, p.ac_minor ?? 0),
    ),
    1,
  );
  const x = (d: number) => ((d - lo) / span) * width;
  const y = (v: number) => height - (v / top) * height;
  const at = (date: string) => dayNumber(date) ?? lo;
  return {
    width,
    height,
    pv: curve.points
      .map((p) => `${x(at(p.date)).toFixed(1)},${y(p.pv_minor).toFixed(1)}`)
      .join(" "),
    ev: curve.points
      .filter((p) => p.ev_minor !== null)
      .map((p) => ({
        x: x(at(p.date)),
        y: y(p.ev_minor ?? 0),
        label: t("l.ev_view.earned", { date: p.date, ev_minor: p.ev_minor }),
      })),
    ac: curve.points
      .filter((p) => p.ac_minor !== null)
      .map((p) => ({
        x: x(at(p.date)),
        y: y(p.ac_minor ?? 0),
        label: t("l.ev_view.spent", { date: p.date, ac_minor: p.ac_minor }),
      })),
    bacY: y(curve.bac_minor),
    from: curve.points[0]?.date ?? "",
    to: curve.points[curve.points.length - 1]?.date ?? "",
  };
}
