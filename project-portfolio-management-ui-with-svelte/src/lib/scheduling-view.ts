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

// ───────────────────────── schedule health (SC-9) ─────────────────────────

export interface HealthResult {
  kind: "pass" | "fail" | "na";
  symbol: string;
  text: string;
}

/** Pass, fail, or "not computable" — which is never read as a pass. */
export function healthResult(c: Pick<HealthCheck, "pass">): HealthResult {
  if (c.pass === true) return { kind: "pass", symbol: "✓", text: "Pass" };
  if (c.pass === false) return { kind: "fail", symbol: "✗", text: "Fail" };
  return { kind: "na", symbol: "–", text: "Not computable" };
}

const REASONS: Record<string, string> = {
  no_baseline: "No schedule baseline has been taken for this plan.",
  nothing_baselined_due: "Nothing was baselined to finish by the status date.",
  cycle: "The plan's dependencies contain a cycle.",
  no_incomplete_tasks: "There are no incomplete tasks to measure.",
  no_relationships: "There are no relationships between tasks to measure.",
  no_scheduled_tasks: "No incomplete task has dates the schedule can place.",
  no_durations: "No incomplete task has a duration.",
  no_tasks: "The plan has no tasks.",
  no_critical_path: "There is no critical task to test.",
  no_remaining_path: "No path remains to measure.",
};

/** Why a check could not be computed, in a sentence ("not enough data" + reason). */
export function healthReason(reason: string | null | undefined): string {
  if (!reason) return "Not enough data.";
  return `Not enough data: ${REASONS[reason] ?? reason}`;
}

/** What the check was held to, in words. */
export function thresholdSentence(c: Pick<HealthCheck, "threshold">): string {
  const { op, limit, unit } = c.threshold;
  if (unit === "result")
    return `Must pass: a ${limit ?? 0}-business-day delay has to move the finish`;
  if (unit === "days") return `At least ${limit} business days`;
  const word = op === "<=" ? "at most" : op === ">=" ? "at least" : "exactly";
  if (unit === "milli")
    return `${word[0]?.toUpperCase()}${word.slice(1)} ${((limit ?? 0) / 1000).toFixed(2)}`;
  return `${word[0]?.toUpperCase()}${word.slice(1)} ${limit}%`;
}

/** What was found, in words. Null-value checks say there is nothing to show. */
export function healthValue(c: HealthCheck): string {
  if (c.value === null) return "No figure";
  if (c.threshold.unit === "result")
    return `Finish moved ${c.count} of ${c.of} business days`;
  if (c.threshold.unit === "milli")
    return `${c.value.toFixed(2)} (${c.count} of ${c.of})`;
  return `${c.value}% (${c.count} of ${c.of} ${c.of_what})`;
}

export function healthSummary(r: {
  passed: number;
  failed: number;
  not_computable: number;
}): string {
  return `${r.passed} passed, ${r.failed} failed, ${r.not_computable} not computable`;
}

/** Where an offender task is shown: the critical-path page, anchored on the task. */
export function offenderHref(planPid: string, taskPid: string): string {
  return `/plans/${planPid}/critical-path#task-${taskPid}`;
}

// ───────────────────────── leveling (SC-6) ─────────────────────────

const LEVELING_REASON: Record<string, string> = {
  fixed_tasks: "Every task involved has already started, so none may move.",
  critical_task:
    "The tasks involved are critical; moving one would move the finish. Allow critical tasks to see that cost.",
  no_float:
    "Delaying a task enough would break the schedule (negative float or a later finish).",
  hard_constraint: "A task involved has a hard constraint and cannot be moved.",
  did_not_move: "The schedule would not let the task start any later.",
  limit: "The search stopped at its limit before resolving this.",
};

export function levelingReason(reason: string | null | undefined): string {
  return reason ? (LEVELING_REASON[reason] ?? reason) : "Not resolved.";
}

const days = (n: number): string =>
  `${n} business day${Math.abs(n) === 1 ? "" : "s"}`;

/** One proposed move, as a sentence. */
export function moveText(m: LevelingMove): string {
  const who = m.masked ? m.person : (m.person_ref ?? m.person);
  const crit = m.critical
    ? " It is on the critical path, so the project finish moves."
    : "";
  return `${m.title ?? m.pid} (${who}) starts ${m.from_start} → ${m.scheduled_start}, ${days(m.delay_days)} later; finishes ${m.to_finish} instead of ${m.from_finish}.${crit}`;
}

/** An over-allocation run, as a sentence; masked callers get no numbers. */
export function runText(r: LevelingRun): string {
  const span = r.from === r.to ? r.from : `${r.from} to ${r.to}`;
  const who = r.masked ? r.person : (r.person_ref ?? r.person);
  if (r.demand === null || r.capacity === null)
    return `${who} is over capacity on ${span}.`;
  return `${who} is asked for ${r.demand}% on ${span} and can give ${r.capacity}%.`;
}

/** The finish effect of applying all moves, in a sentence. */
export function finishEffectText(
  before: string | null,
  after: string | null,
  delay: number,
): string {
  if (!before || !after) return "No project finish can be computed.";
  if (delay === 0) return `The project finish stays ${before}.`;
  return `The project finish moves from ${before} to ${after} (${days(delay)} later).`;
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
      text: "No demand or supply",
      level: 0,
    };
  if (row.gap_fte === null || row.supply_fte === null) {
    return {
      kind: "unknown",
      symbol: "?",
      text: `Demand ${row.demand_fte} FTE; supply not known`,
      level: 0,
    };
  }
  const level = maxAbs > 0 ? Math.min(1, Math.abs(row.gap_fte) / maxAbs) : 0;
  if (row.gap_fte < 0)
    return {
      kind: "short",
      symbol: "▼",
      text: `Short ${Math.abs(row.gap_fte)} FTE`,
      level,
    };
  if (row.gap_fte > 0)
    return {
      kind: "spare",
      symbol: "▲",
      text: `Spare ${row.gap_fte} FTE`,
      level,
    };
  return { kind: "balanced", symbol: "●", text: "Balanced", level: 0 };
}

/** The largest absolute gap across all cells (for scaling colour). */
export function maxAbsGap(rows: Pick<ResourceRow, "gap_fte">[]): number {
  return rows.reduce((m, r) => Math.max(m, Math.abs(r.gap_fte ?? 0)), 0);
}

/** "Mon 5 Oct – Sun 11 Oct" style label is the API's dates; keep them plain. */
export function periodLabel(from: string, to: string): string {
  return from === to ? from : `${from} to ${to}`;
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
  const date = cpmFinish ?? "the critical-path finish";
  return `${pct}% of the simulated outcomes finish on or before ${date}.`;
}

/** "P80: 2026-11-05" style line. */
export function percentileText(p: { p: number; finish: string }): string {
  return `P${p.p}: ${p.finish}`;
}

/** Criticality as a percent, rounded. */
export function criticalityText(bp: number): string {
  return `${Math.round(bp / 100)}%`;
}

/** Sensitivity in words; null means the duration or the finish never varied. */
export function sensitivityText(r: number | null): string {
  if (r === null) return "No variation";
  const strength =
    Math.abs(r) >= 0.7 ? "strong" : Math.abs(r) >= 0.4 ? "moderate" : "weak";
  return `${r.toFixed(2)} (${strength})`;
}

/** The estimate source in words, flagging a fallback loudly. */
export function sourceText(t: Pick<RiskTask, "source" | "widened">): string {
  if (t.source === "fallback") return "No estimate: using the fallback band";
  if (t.source === "milestone") return "Milestone: no spread";
  return t.widened
    ? "Estimated (widened to include the likely value)"
    : "Estimated";
}

const EVENT_REASON: Record<string, string> = {
  no_probability: "no probability",
  no_task: "no task named",
  no_impact: "no impact in business days",
  closed: "the risk is not open",
  task_not_scheduled: "its task is not on the schedule",
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
    ? "PERT (favours the likely value)"
    : "Triangular";
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
  return `${Math.round(bp / 100)}% of the simulated outcomes cost no more than the budget (${budgetText}).`;
}

const COST_SKIPPED: Record<string, string> = {
  no_probability: "no probability",
  closed: "the risk is not open",
  no_budget_in_currency: "the plan has no budget line in its currency",
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
  if (l.source === "fallback") return "No estimate: using the fallback band";
  return l.widened
    ? "Estimated (widened to include the planned amount)"
    : "Estimated";
}

/** The stated probability against how often the event actually struck. */
export function eventHitText(probabilityBp: number, hitBp: number): string {
  return `stated ${Math.round(probabilityBp / 100)}%, struck in ${(hitBp / 100).toFixed(1)}% of runs`;
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
    total === 0 ? "—" : `${((n / total) * 100).toFixed(1)}%`;
  return [
    {
      key: "both",
      label: "Within budget and on time",
      symbol: "✓",
      count: q.both,
      share: share(q.both),
    },
    {
      key: "budget_only",
      label: "Within budget but late",
      symbol: "▲",
      count: q.budget_only,
      share: share(q.budget_only),
    },
    {
      key: "time_only",
      label: "On time but over budget",
      symbol: "▲",
      count: q.time_only,
      share: share(q.time_only),
    },
    {
      key: "neither",
      label: "Over budget and late",
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
  const pct = (bp: number): string => `${Math.round(bp / 100)}%`;
  return `Both within budget and on time: ${pct(c.joint_bp)} (within budget alone ${pct(c.budget_bp)}; on time alone ${pct(c.finish_bp)}).`;
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
    ? "A late finish costs nothing extra in this run (delay cost per business day is 0)."
    : `Each business day late adds ${format(perDayMinor, currency)}; finishing early earns nothing back.`;
}
