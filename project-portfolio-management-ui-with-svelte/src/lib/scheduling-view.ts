// The words, symbols and geometry for the scheduling views (SC-11): schedule
// health, leveling, the resource heatmap and the schedule-risk histogram. Pure
// and unit-tested.
//
// Rules every view here follows: colour is never the only signal (a word and a
// symbol always come with it), an undefined figure reads "not enough data" with
// its reason rather than 0, and every chart has a table twin.

import type {
  HealthCheck,
  LevelingMove,
  LevelingRun,
  ResourceRow,
  RiskTask,
  ScheduleRisk,
} from "#lib/api/ppm.js";
import { businessDays } from "./units.js";
import { t } from "./i18n.svelte.js";
import {
  formatBasisPoints,
  formatDate,
  formatDecimal,
  formatPercent,
  formatPercentValue,
} from "#lib/locale-format.js";

// ───────────────────────── schedule health (SC-9) ─────────────────────────

export interface HealthResult {
  kind: "pass" | "fail" | "na";
  symbol: string;
  text: string;
}

/** Pass, fail, or "not computable" — which is never read as a pass. */
export function healthResult(c: Pick<HealthCheck, "pass">): HealthResult {
  if (c.pass === true)
    return { kind: "pass", symbol: "✓", text: t("l.scheduling_view.pass") };
  if (c.pass === false)
    return { kind: "fail", symbol: "✗", text: t("l.scheduling_view.fail") };
  return {
    kind: "na",
    symbol: "–",
    text: t("l.scheduling_view.not_computable"),
  };
}

const REASONS: Record<string, string> = {
  get no_baseline() {
    return t("l.scheduling_view.no_schedule_baseline_has_been");
  },
  get nothing_baselined_due() {
    return t("l.scheduling_view.nothing_was_baselined_to_finish");
  },
  get cycle() {
    return t("l.scheduling_view.the_plan_s_dependencies_contain");
  },
  get no_incomplete_tasks() {
    return t("l.scheduling_view.there_are_no_incomplete_tasks");
  },
  get no_relationships() {
    return t("l.scheduling_view.there_are_no_relationships_between");
  },
  get no_scheduled_tasks() {
    return t("l.scheduling_view.no_incomplete_task_has_dates");
  },
  get no_durations() {
    return t("l.scheduling_view.no_incomplete_task_has_a");
  },
  get no_tasks() {
    return t("l.scheduling_view.the_plan_has_no_tasks");
  },
  get no_critical_path() {
    return t("l.scheduling_view.there_is_no_critical_task");
  },
  get no_remaining_path() {
    return t("l.scheduling_view.no_path_remains_to_measure");
  },
};

/** Why a check could not be computed, in a sentence ("not enough data" + reason). */
export function healthReason(reason: string | null | undefined): string {
  if (!reason) return t("l.scheduling_view.not_enough_data");
  return t("l.scheduling_view.not_enough_data_2", {
    reason: REASONS[reason] ?? reason,
  });
}

/** What the check was held to, in words. */
export function thresholdSentence(c: Pick<HealthCheck, "threshold">): string {
  const { op, limit, unit } = c.threshold;
  if (unit === "result")
    return t("l.scheduling_view.must_pass_a_business_day", { v: limit ?? 0 });
  if (unit === "days")
    return t("l.scheduling_view.at_least_business_days", { limit });
  const word =
    op === "<="
      ? t("l.scheduling_view.at_most")
      : op === ">="
        ? t("l.scheduling_view.at_least")
        : t("l.scheduling_view.exactly");
  if (unit === "milli")
    return `${word[0]?.toUpperCase()}${word.slice(1)} ${formatDecimal((limit ?? 0) / 1000, 2)}`;
  return `${word[0]?.toUpperCase()}${word.slice(1)} ${formatPercentValue(limit)}`;
}

/** What was found, in words. Null-value checks say there is nothing to show. */
export function healthValue(c: HealthCheck): string {
  if (c.value === null) return t("l.scheduling_view.no_figure");
  if (c.threshold.unit === "result")
    return t("l.scheduling_view.finish_moved_of_business_days", {
      count: c.count,
      of: c.of,
    });
  if (c.threshold.unit === "milli")
    return t("l.scheduling_view.of", {
      toFixed: formatDecimal(c.value, 2),
      count: c.count,
      of: c.of,
    });
  return t("l.scheduling_view.of_2", {
    value: c.value,
    count: c.count,
    of: c.of,
    of_what: c.of_what,
  });
}

export function healthSummary(r: {
  passed: number;
  failed: number;
  not_computable: number;
}): string {
  return t("l.scheduling_view.passed_failed_not_computable", {
    passed: r.passed,
    failed: r.failed,
    not_computable: r.not_computable,
  });
}

/** Where an offender task is shown: the critical-path page, anchored on the task. */
export function offenderHref(planPid: string, taskPid: string): string {
  return `/plans/${planPid}/critical-path#task-${taskPid}`;
}

// ───────────────────────── leveling (SC-6) ─────────────────────────

const LEVELING_REASON: Record<string, string> = {
  get fixed_tasks() {
    return t("l.scheduling_view.every_task_involved_has_already");
  },
  get critical_task() {
    return t("l.scheduling_view.the_tasks_involved_are_critical");
  },
  get no_float() {
    return t("l.scheduling_view.delaying_a_task_enough_would");
  },
  get hard_constraint() {
    return t("l.scheduling_view.a_task_involved_has_a");
  },
  get did_not_move() {
    return t("l.scheduling_view.the_schedule_would_not_let");
  },
  get limit() {
    return t("l.scheduling_view.the_search_stopped_at_its");
  },
};

export function levelingReason(reason: string | null | undefined): string {
  return reason
    ? (LEVELING_REASON[reason] ?? reason)
    : t("l.scheduling_view.not_resolved");
}

const days = businessDays;

/** One proposed move, as a sentence. */
export function moveText(m: LevelingMove): string {
  const who = m.masked ? m.person : (m.person_ref ?? m.person);
  const crit = m.critical ? t("l.scheduling_view.it_is_on_the_critical") : "";
  return t("l.scheduling_view.starts_later_finishes_instead_of", {
    pid: m.title ?? m.pid,
    who,
    from_start: m.from_start,
    scheduled_start: m.scheduled_start,
    days: days(m.delay_days),
    to_finish: m.to_finish,
    from_finish: m.from_finish,
    crit,
  });
}

/** An over-allocation run, as a sentence; masked callers get no numbers. */
export function runText(r: LevelingRun): string {
  const span =
    r.from === r.to
      ? r.from
      : t("l.scheduling_view.to", { from: r.from, to: r.to });
  const who = r.masked ? r.person : (r.person_ref ?? r.person);
  if (r.demand === null || r.capacity === null)
    return t("l.scheduling_view.is_over_capacity_on", { who, span });
  return t("l.scheduling_view.is_asked_for_on_and", {
    who,
    demand: r.demand,
    span,
    capacity: r.capacity,
  });
}

/** The finish effect of applying all moves, in a sentence. */
export function finishEffectText(
  before: string | null,
  after: string | null,
  delay: number,
): string {
  if (!before || !after) return t("l.scheduling_view.no_project_finish_can_be");
  if (delay === 0)
    return t("l.scheduling_view.the_project_finish_stays", { before });
  return t("l.scheduling_view.the_project_finish_moves_from", {
    before,
    after,
    days: days(delay),
  });
}

// ───────────────────────── resource heatmap (SC-7) ─────────────────────────

export type GapKind = "short" | "balanced" | "spare" | "unknown";

export interface GapCell {
  kind: GapKind;
  symbol: string;
  text: string;
  /** 0..1, how strong the colour should be; the text carries the meaning. */
  level: number;
}

/** A cell of the heatmap: gap in FTE as a word, a symbol and a strength. */
export function gapCell(
  row: Pick<ResourceRow, "gap_fte" | "supply_fte" | "demand_fte"> | undefined,
  maxAbs: number,
): GapCell {
  if (!row)
    return {
      kind: "unknown",
      symbol: "·",
      text: t("l.scheduling_view.no_demand_or_supply"),
      level: 0,
    };
  if (row.gap_fte === null || row.supply_fte === null) {
    return {
      kind: "unknown",
      symbol: "?",
      text: t("l.scheduling_view.demand_fte_supply_not_known", {
        demand_fte: row.demand_fte,
      }),
      level: 0,
    };
  }
  const level = maxAbs > 0 ? Math.min(1, Math.abs(row.gap_fte) / maxAbs) : 0;
  if (row.gap_fte < 0)
    return {
      kind: "short",
      symbol: "▼",
      text: t("l.scheduling_view.short_fte", { abs: Math.abs(row.gap_fte) }),
      level,
    };
  if (row.gap_fte > 0)
    return {
      kind: "spare",
      symbol: "▲",
      text: t("l.scheduling_view.spare_fte", { gap_fte: row.gap_fte }),
      level,
    };
  return {
    kind: "balanced",
    symbol: "●",
    text: t("l.scheduling_view.balanced"),
    level: 0,
  };
}

/** The largest absolute gap across all cells (for scaling colour). */
export function maxAbsGap(rows: Pick<ResourceRow, "gap_fte">[]): number {
  return rows.reduce((m, r) => Math.max(m, Math.abs(r.gap_fte ?? 0)), 0);
}

/** "Mon 5 Oct – Sun 11 Oct" style label is the API's dates; keep them plain. */
export function periodLabel(from: string, to: string): string {
  return from === to ? from : t("l.scheduling_view.to", { from, to });
}

// ───────────────────────── schedule-risk histogram (SC-8) ─────────────────────────

export interface Bar {
  date: string;
  count: number;
  x: number;
  y: number;
  w: number;
  h: number;
}

export interface Marker {
  p: number;
  finish: string;
  x: number;
}

export interface Histogram {
  width: number;
  height: number;
  baseline: number;
  bars: Bar[];
  markers: Marker[];
  cpmX: number | null;
}

/**
 * Bars over the days between the earliest and latest simulated finish (days with
 * no iterations are gaps, not zero-height lies: a bar is drawn only where there
 * are counts), with a vertical marker at each percentile and at the critical-path
 * finish. Positions are by calendar day.
 */
export function histogramGeometry(
  histogram: { date: string; count: number }[],
  percentiles: { p: number; finish: string }[],
  cpmFinish: string | null,
  width = 640,
  height = 220,
): Histogram | null {
  if (histogram.length === 0) return null;
  const day = (d: string): number =>
    Date.UTC(+d.slice(0, 4), +d.slice(5, 7) - 1, +d.slice(8, 10)) / 86_400_000;
  const pad = 24;
  const first = day(histogram[0]?.date ?? "1970-01-01");
  const last = day(histogram[histogram.length - 1]?.date ?? "1970-01-01");
  const span = Math.max(1, last - first + 1);
  const slot = (width - pad * 2) / span;
  const max = Math.max(...histogram.map((h) => h.count));
  const baseline = height - pad;
  const usable = baseline - pad;
  const xOf = (d: string): number => pad + (day(d) - first) * slot;
  return {
    width,
    height,
    baseline,
    bars: histogram.map((h) => {
      const hh = (h.count / max) * usable;
      return {
        date: h.date,
        count: h.count,
        x: xOf(h.date),
        y: baseline - hh,
        w: Math.max(1, slot - 1),
        h: hh,
      };
    }),
    markers: percentiles
      .filter((p) => [50, 80, 90].includes(p.p))
      .map((p) => ({ p: p.p, finish: p.finish, x: xOf(p.finish) + slot / 2 })),
    cpmX:
      cpmFinish && day(cpmFinish) >= first && day(cpmFinish) <= last
        ? xOf(cpmFinish) + slot / 2
        : null,
  };
}

/** How likely the plan's own critical-path finish is, in words. */
export function confidenceText(bp: number, cpmFinish: string | null): string {
  const pct = Math.round(bp / 100);
  const date = cpmFinish
    ? formatDate(cpmFinish)
    : t("l.scheduling_view.the_critical_path_finish");
  return t("l.scheduling_view.of_the_simulated_outcomes_finish", { pct, date });
}

/** "P80: 2026-11-05" style line. */
export function percentileText(p: { p: number; finish: string }): string {
  return `P${p.p}: ${formatDate(p.finish)}`;
}

/** Criticality as a percent, rounded. */
export function criticalityText(bp: number): string {
  return formatBasisPoints(bp, 0);
}

/** Sensitivity in words; null means the duration or the finish never varied. */
export function sensitivityText(r: number | null): string {
  if (r === null) return t("l.scheduling_view.no_variation");
  const strength =
    Math.abs(r) >= 0.7
      ? t("l.scheduling_view.strength_strong")
      : Math.abs(r) >= 0.4
        ? t("l.scheduling_view.strength_moderate")
        : t("l.scheduling_view.strength_weak");
  return `${formatDecimal(r, 2)} (${strength})`;
}

/** The estimate source in words, flagging a fallback loudly. */
export function sourceText(task: Pick<RiskTask, "source" | "widened">): string {
  if (task.source === "fallback")
    return t("l.scheduling_view.no_estimate_using_the_fallback");
  if (task.source === "milestone")
    return t("l.scheduling_view.milestone_no_spread");
  return task.widened
    ? t("l.scheduling_view.estimated_widened_to_include_the")
    : t("l.scheduling_view.estimated");
}

const EVENT_REASON: Record<string, string> = {
  get no_probability() {
    return t("l.scheduling_view.no_probability");
  },
  get no_task() {
    return t("l.scheduling_view.no_task_named");
  },
  get no_impact() {
    return t("l.scheduling_view.no_impact_in_business_days");
  },
  get closed() {
    return t("l.scheduling_view.the_risk_is_not_open");
  },
  get task_not_scheduled() {
    return t("l.scheduling_view.its_task_is_not_on");
  },
  incomplete: "incomplete",
};

export function skippedReasonText(reason: string): string {
  return EVENT_REASON[reason] ?? reason;
}

/** The distribution in words, for the assumptions block. */
export function distributionText(
  a: Pick<ScheduleRisk["assumptions"], "distribution">,
): string {
  return a.distribution === "pert"
    ? t("l.scheduling_view.pert_favours_the_likely_value")
    : t("p.plans.pid.joint_risk.dist_triangular");
}

// ───────────────────────── cost risk and joint risk ─────────────────────────

export interface CostBar {
  from: number;
  to: number;
  count: number;
  x: number;
  y: number;
  w: number;
  h: number;
}

export interface CostHistogram {
  width: number;
  height: number;
  baseline: number;
  bars: CostBar[];
  /** P50, P80 and P90 lines. */
  markers: { p: number; minor: number; x: number }[];
  /** The budget line, if the budget falls inside the plotted range. */
  budgetX: number | null;
}

/** Bars for the cost histogram, with P50 / P80 / P90 and the budget marked. */
export function costHistogramGeometry(
  bins: { from_minor: number; to_minor: number; count: number }[],
  percentiles: { p: number; minor: number }[],
  budgetMinor: number,
  width = 640,
  height = 220,
): CostHistogram | null {
  const first = bins[0];
  const last = bins[bins.length - 1];
  if (!first || !last) return null;
  const pad = 24;
  const lo = first.from_minor;
  const hi = Math.max(last.to_minor, lo + 1);
  const baseline = height - pad;
  const usable = baseline - pad;
  const max = Math.max(1, ...bins.map((b) => b.count));
  const xOf = (v: number): number =>
    pad + ((v - lo) / (hi - lo)) * (width - pad * 2);
  return {
    width,
    height,
    baseline,
    bars: bins.map((b) => {
      const h = (b.count / max) * usable;
      return {
        from: b.from_minor,
        to: b.to_minor,
        count: b.count,
        x: xOf(b.from_minor),
        y: baseline - h,
        w: Math.max(1, xOf(b.to_minor) - xOf(b.from_minor) - 1),
        h,
      };
    }),
    markers: percentiles
      .filter((p) => [50, 80, 90].includes(p.p))
      .map((p) => ({
        p: p.p,
        minor: p.minor,
        x: xOf(Math.min(Math.max(p.minor, lo), hi)),
      })),
    budgetX: budgetMinor >= lo && budgetMinor <= hi ? xOf(budgetMinor) : null,
  };
}

/** How likely the budget is, in words. `budgetText` is the formatted budget. */
export function budgetConfidenceText(bp: number, budgetText: string): string {
  return t("l.scheduling_view.of_the_simulated_outcomes_cost", {
    round: Math.round(bp / 100),
    budgetText,
  });
}

const COST_SKIPPED: Record<string, string> = {
  get no_probability() {
    return t("l.scheduling_view.no_probability");
  },
  get closed() {
    return t("l.scheduling_view.the_risk_is_not_open");
  },
  get no_budget_in_currency() {
    return t("l.scheduling_view.the_plan_has_no_budget");
  },
  incomplete: "incomplete",
};

export function costSkippedText(reason: string): string {
  return COST_SKIPPED[reason] ?? reason;
}

/** A line's estimate source, flagging a fallback loudly. */
export function lineSourceText(l: {
  source: "estimated" | "fallback";
  widened: boolean;
}): string {
  if (l.source === "fallback")
    return t("l.scheduling_view.no_estimate_using_the_fallback");
  return l.widened
    ? t("l.scheduling_view.estimated_widened_to_include_the_2")
    : t("l.scheduling_view.estimated");
}

/** The stated probability against how often the event actually struck. */
export function eventHitText(probabilityBp: number, hitBp: number): string {
  return t("l.scheduling_view.stated_struck_in_of_runs", {
    round: Math.round(probabilityBp / 100),
    toFixed: formatDecimal(hitBp / 100, 1),
  });
}

export interface QuadrantRow {
  key: "both" | "budget_only" | "time_only" | "neither";
  label: string;
  symbol: string;
  count: number;
  share: string;
}

/** The four ways an iteration can fall, as table rows with shares. */
export function quadrantRows(q: {
  both: number;
  budget_only: number;
  time_only: number;
  neither: number;
}): QuadrantRow[] {
  const total = q.both + q.budget_only + q.time_only + q.neither;
  const share = (n: number): string =>
    total === 0 ? "—" : formatPercent(n / total, 1);
  return [
    {
      key: "both",
      label: t("l.scheduling_view.within_budget_and_on_time"),
      symbol: "✓",
      count: q.both,
      share: share(q.both),
    },
    {
      key: "budget_only",
      label: t("l.scheduling_view.within_budget_but_late"),
      symbol: "▲",
      count: q.budget_only,
      share: share(q.budget_only),
    },
    {
      key: "time_only",
      label: t("l.scheduling_view.on_time_but_over_budget"),
      symbol: "▲",
      count: q.time_only,
      share: share(q.time_only),
    },
    {
      key: "neither",
      label: t("l.scheduling_view.over_budget_and_late"),
      symbol: "■",
      count: q.neither,
      share: share(q.neither),
    },
  ];
}

/** The three confidences in one sentence; the joint one is never the largest. */
export function jointConfidenceText(c: {
  budget_bp: number;
  finish_bp: number;
  joint_bp: number;
}): string {
  const pct = (bp: number): string => formatBasisPoints(bp, 0);
  return t("l.scheduling_view.both_within_budget_and_on", {
    pct: pct(c.joint_bp),
    pct2: pct(c.budget_bp),
    pct3: pct(c.finish_bp),
  });
}

export interface Scatter {
  width: number;
  height: number;
  dots: { x: number; y: number; days: number; cost: number }[];
  /** The "on time" line (zero business days late). */
  onTimeX: number;
  /** The budget line. */
  budgetY: number;
}

/** Cost against business days late, with the on-time and budget lines. */
export function scatterGeometry(
  points: { days_late: number; cost_minor: number }[],
  budgetMinor: number,
  width = 480,
  height = 320,
): Scatter | null {
  if (points.length === 0) return null;
  const pad = 32;
  const days = points.map((p) => p.days_late).concat(0);
  const costs = points.map((p) => p.cost_minor).concat(budgetMinor);
  const [dMin, dMax] = [Math.min(...days), Math.max(...days)];
  const [cMin, cMax] = [Math.min(...costs), Math.max(...costs)];
  const x = (d: number): number =>
    pad +
    (dMax === dMin ? 0.5 : (d - dMin) / (dMax - dMin)) * (width - pad * 2);
  const y = (c: number): number =>
    height -
    pad -
    (cMax === cMin ? 0.5 : (c - cMin) / (cMax - cMin)) * (height - pad * 2);
  return {
    width,
    height,
    dots: points.map((p) => ({
      x: x(p.days_late),
      y: y(p.cost_minor),
      days: p.days_late,
      cost: p.cost_minor,
    })),
    onTimeX: x(0),
    budgetY: y(budgetMinor),
  };
}

/** Delay cost in words. */
export function delayCostText(
  perDayMinor: number,
  currency: string,
  format: (minor: number, currency: string) => string,
): string {
  return perDayMinor === 0
    ? t("l.scheduling_view.a_late_finish_costs_nothing")
    : t("l.scheduling_view.each_business_day_late_adds", {
        format: format(perDayMinor, currency),
      });
}
