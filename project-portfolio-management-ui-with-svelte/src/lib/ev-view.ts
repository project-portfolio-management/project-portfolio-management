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
      return "nothing was planned to be done yet";
    case "no_actuals_recorded":
      return "no cost has been recorded yet";
    case "no_planned_value":
      return "the baseline plans no value";
    default:
      return "not available";
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
  if (b.spi.bp === null) return `Schedule: no ratio, ${absentText(b.spi)}.`;
  const gap = `${money(Math.abs(b.sv_minor))} ${b.sv_minor >= 0 ? "more" : "less"} value earned than planned by now`;
  const word =
    h === "ahead"
      ? "Ahead of schedule"
      : h === "behind"
        ? "Behind schedule"
        : "On schedule";
  return `${word}: SPI ${ratioText(b.spi.bp)}, ${gap}.`;
}

/** The sentence for cost performance. */
export function costSentence(
  b: EvBlock,
  money: (minor: number) => string,
): string {
  if (b.cv_minor === null || b.cpi.bp === null)
    return `Cost: no ratio, ${absentText(b.cpi)}.`;
  const h = health(b.cpi.bp);
  const word =
    h === "ahead"
      ? "Under budget"
      : h === "behind"
        ? "Over budget"
        : "On budget";
  return `${word}: CPI ${ratioText(b.cpi.bp)}, ${money(Math.abs(b.cv_minor))} ${b.cv_minor >= 0 ? "less" : "more"} spent than the work earned.`;
}

/** The sentence for earned schedule, which still says so when a project is late and finished. */
export function earnedScheduleSentence(b: EvBlock): string {
  if (b.spi_t.bp === null || b.earned_schedule_days === null)
    return `Earned schedule: no ratio, ${absentText(b.spi_t)}.`;
  const days = (n: number) => `${n} calendar day${n === 1 ? "" : "s"}`;
  const h = health(b.spi_t.bp);
  const word = h === "ahead" ? "ahead" : h === "behind" ? "behind" : "on time";
  return `By earned schedule the work is ${word}: it has earned in ${days(b.actual_days)} what the plan expected after ${days(b.earned_schedule_days)} (SPI(t) ${ratioText(b.spi_t.bp)}).`;
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
      label: "Planned value",
      minor: b.pv_minor,
      widthPct: pct(b.pv_minor),
      bacPct,
    },
    {
      key: "ev",
      label: "Earned value",
      minor: b.ev_minor,
      widthPct: pct(b.ev_minor),
      bacPct,
    },
    {
      key: "ac",
      label: "Actual cost",
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
      ? "started work counts half"
      : pc.default_method === "points"
        ? "finished work counts for its story points"
        : "only finished work counts";
  const parts = [
    `${pc.tasks_counted} task${pc.tasks_counted === 1 ? "" : "s"} counted, weighted ${pc.weighting === "points" ? "by story points" : "equally"}; ${how}`,
  ];
  if (pc.manual_tasks.length > 0)
    parts.push(
      `${pc.manual_tasks.length} counted by a person-entered percent (manual)`,
    );
  if (pc.unweighted_tasks.length > 0)
    parts.push(`${pc.unweighted_tasks.length} with no points carry no weight`);
  return parts.join("; ");
}

const RAG: Record<EvRag, { marker: string; word: string }> = {
  green: { marker: "●", word: "on track" },
  amber: { marker: "◐", word: "slipping" },
  red: { marker: "✖", word: "off plan" },
  unknown: { marker: "?", word: "not yet known" },
};

/** A marker and a word for a traffic light, so the colour is never the only signal. */
export function ragText(r: EvRag): { marker: string; word: string } {
  return RAG[r];
}

/** Why a forecast amount is missing, in words. */
export function amountAbsentText(a: EvAmount): string {
  switch (a.absent) {
    case "no_actual_cost":
      return "no actual cost is recorded";
    case "undefined_denominator":
      return "the ratio it divides by is not defined yet";
    case "no_bottom_up_estimate":
      return "no estimate to complete has been entered";
    case "budget_exhausted":
      return "the budget is already spent";
    case "no_remaining_cost":
      return "no cost remains";
    default:
      return "not available";
  }
}

/** The sentence for what the remaining work needs, which says so when it cannot be met. */
export function tcpiSentence(t: EvTcpi, target: string): string {
  if (t.bp === null) {
    return t.absent === "budget_exhausted"
      ? `Against ${target}: the money is already spent, so no efficiency can deliver the remaining work within it.`
      : `Against ${target}: ${amountAbsentText({ minor: null, absent: t.absent })}.`;
  }
  if (t.needs_improved_efficiency) {
    return `Against ${target}: the remaining work must be done ${ratioText(t.bp)} times as efficiently as the work so far (TCPI ${ratioText(t.bp)}), which needs improved efficiency.`;
  }
  return `Against ${target}: the remaining work can be done at ${ratioText(t.bp)} of the cost efficiency of the work so far (TCPI ${ratioText(t.bp)}).`;
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
        label: `${p.date}: earned ${p.ev_minor}`,
      })),
    ac: curve.points
      .filter((p) => p.ac_minor !== null)
      .map((p) => ({
        x: x(at(p.date)),
        y: y(p.ac_minor ?? 0),
        label: `${p.date}: spent ${p.ac_minor}`,
      })),
    bacY: y(curve.bac_minor),
    from: curve.points[0]?.date ?? "",
    to: curve.points[curve.points.length - 1]?.date ?? "",
  };
}
