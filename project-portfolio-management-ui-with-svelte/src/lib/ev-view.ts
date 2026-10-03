/**
 * Pure helpers for the earned-value view (SC-4). No DOM, no Svelte, no clock:
 * unit-tested. Ratios arrive as **basis points** (10 000 = 1.0) and money as minor
 * units; both are formatted with integer arithmetic only.
 */

import type { EvBlock, EvRatio } from "./api/ppm";

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

const MARK: Record<Health, string> = { ahead: "▲", on_track: "●", behind: "▼", unknown: "?" };

/** A marker for a state, so colour is never the only signal. */
export function marker(h: Health): string {
  return MARK[h];
}

/** The sentence for schedule performance. */
export function scheduleSentence(b: EvBlock, money: (minor: number) => string): string {
  const h = health(b.spi.bp);
  if (b.spi.bp === null) return `Schedule: no ratio, ${absentText(b.spi)}.`;
  const gap = `${money(Math.abs(b.sv_minor))} ${b.sv_minor >= 0 ? "more" : "less"} value earned than planned by now`;
  const word = h === "ahead" ? "Ahead of schedule" : h === "behind" ? "Behind schedule" : "On schedule";
  return `${word}: SPI ${ratioText(b.spi.bp)}, ${gap}.`;
}

/** The sentence for cost performance. */
export function costSentence(b: EvBlock, money: (minor: number) => string): string {
  if (b.cv_minor === null || b.cpi.bp === null) return `Cost: no ratio, ${absentText(b.cpi)}.`;
  const h = health(b.cpi.bp);
  const word = h === "ahead" ? "Under budget" : h === "behind" ? "Over budget" : "On budget";
  return `${word}: CPI ${ratioText(b.cpi.bp)}, ${money(Math.abs(b.cv_minor))} ${b.cv_minor >= 0 ? "less" : "more"} spent than the work earned.`;
}

/** The sentence for earned schedule, which still says so when a project is late and finished. */
export function earnedScheduleSentence(b: EvBlock): string {
  if (b.spi_t.bp === null || b.earned_schedule_days === null) return `Earned schedule: no ratio, ${absentText(b.spi_t)}.`;
  const days = (n: number) => `${n} day${n === 1 ? "" : "s"}`;
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
    { key: "pv", label: "Planned value", minor: b.pv_minor, widthPct: pct(b.pv_minor), bacPct },
    { key: "ev", label: "Earned value", minor: b.ev_minor, widthPct: pct(b.ev_minor), bacPct },
    { key: "ac", label: "Actual cost", minor: b.ac_minor, widthPct: b.ac_minor === null ? 0 : pct(b.ac_minor), bacPct },
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
  const parts = [`${pc.tasks_counted} task${pc.tasks_counted === 1 ? "" : "s"} counted, weighted ${pc.weighting === "points" ? "by story points" : "equally"}; ${how}`];
  if (pc.manual_tasks.length > 0) parts.push(`${pc.manual_tasks.length} counted by a person-entered percent (manual)`);
  if (pc.unweighted_tasks.length > 0) parts.push(`${pc.unweighted_tasks.length} with no points carry no weight`);
  return parts.join("; ");
}
