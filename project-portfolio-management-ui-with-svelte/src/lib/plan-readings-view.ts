// A plan's readings, read on asking (NC-1 slice 12 part i, RM-5, SC-4): on `/plans/[pid]/critical-path` the flow
// across the plans it contains (`GET /api/plans/{pid}/rollup`), the effort recorded (`…/effort`), the delivery
// forecast (`…/forecast`), schedule and cost performance (`…/performance`) and the task workflow (`…/workflow`); on
// `/plans/[pid]/flow` the mix of finished work (`…/flow-distribution`); on the task page a task's sub-tasks rolled up
// (`…/tasks/{t_pid}/rollup`). The queries are checked before anything is sent and a default is not sent; the
// service's notes and reasons are English and never shown, so each absence is said in the catalogue's words.

import { ApiError } from "./api/client.js";
import type {
  DeliveryForecastQuery,
  Derived,
  FlowDistributionQuery,
  FlowType,
  PerformanceAbsent,
} from "./api/ppm-types.js";
import type { StringKey } from "./i18n.svelte.js";

/** A number input's value: `null` or `undefined` when empty. */
type Typed = number | null | undefined;

const typed = (v: Typed): number | null =>
  v === null || v === undefined ? null : Number(v);
const whole = (v: number, min: number, max: number) =>
  Number.isSafeInteger(v) && v >= min && v <= max;

/** The deepest roll-up (and its default, not sent). */
export const ROLLUP_DEPTH_MAX = 32;

/** The depth to send for a plan's flow roll-up (`undefined` for the default), or the problem. */
export function rollupDepth(
  depth: Typed,
): { depth: number | undefined } | { problem: StringKey } {
  const d = typed(depth);
  if (d === null || d === ROLLUP_DEPTH_MAX) return { depth: undefined };
  return whole(d, 1, ROLLUP_DEPTH_MAX)
    ? { depth: d }
    : { problem: "p.readings.problem_depth" };
}

/** The forecast as asked; each empty box is the service's default. */
export interface ForecastForm {
  items: Typed;
  periods: Typed;
  periodDays: Typed;
  historyPeriods: Typed;
}

export const FORECAST_DEFAULTS = {
  periods: 4,
  periodDays: 7,
  historyPeriods: 12,
} as const;

/** The forecast's query (defaults not sent), or every problem at once. */
export function forecastQuery(
  form: ForecastForm,
): { query: DeliveryForecastQuery } | { problems: StringKey[] } {
  const problems: StringKey[] = [];
  const items = typed(form.items);
  const periods = typed(form.periods);
  const periodDays = typed(form.periodDays);
  const history = typed(form.historyPeriods);
  if (items !== null && !whole(items, 1, Number.MAX_SAFE_INTEGER))
    problems.push("p.readings.problem_items");
  if (periods !== null && !whole(periods, 1, 520))
    problems.push("p.readings.problem_periods");
  if (periodDays !== null && !whole(periodDays, 1, 90))
    problems.push("p.readings.problem_period_days");
  if (history !== null && !whole(history, 6, 260))
    problems.push("p.readings.problem_history");
  if (problems.length > 0) return { problems };
  const query: DeliveryForecastQuery = {};
  if (items !== null) query.items = items;
  if (periods !== null && periods !== FORECAST_DEFAULTS.periods)
    query.periods = periods;
  if (periodDays !== null && periodDays !== FORECAST_DEFAULTS.periodDays)
    query.period_days = periodDays;
  if (history !== null && history !== FORECAST_DEFAULTS.historyPeriods)
    query.history_periods = history;
  return { query };
}

/** The mix of finished work as asked. */
export interface DistributionForm {
  windowDays: Typed;
  rollup: boolean;
  depth: Typed;
}

/** The default window, in days. */
export const DISTRIBUTION_WINDOW = 90;

/**
 * The query (defaults not sent), or every problem at once. The service quietly reads a window outside 1 to 3650 as
 * 90 days, so the page refuses one first; the depth counts only when rolling up, 0 being the plan alone.
 */
export function distributionQuery(
  form: DistributionForm,
): { query: FlowDistributionQuery } | { problems: StringKey[] } {
  const problems: StringKey[] = [];
  const w = typed(form.windowDays);
  const d = typed(form.depth);
  if (w !== null && !whole(w, 1, 3650))
    problems.push("p.readings.problem_window");
  if (form.rollup && d !== null && !whole(d, 0, ROLLUP_DEPTH_MAX))
    problems.push("p.readings.problem_depth_zero");
  if (problems.length > 0) return { problems };
  const query: FlowDistributionQuery = {};
  if (w !== null && w !== DISTRIBUTION_WINDOW) query.window_days = w;
  if (form.rollup) {
    query.rollup = true;
    if (d !== null && d !== ROLLUP_DEPTH_MAX) query.depth = d;
  }
  return { query };
}

/** Each kind of work. */
export const FLOW_TYPE_KEY: Record<FlowType, StringKey> = {
  feature: "p.readings.flow_feature",
  defect: "p.readings.flow_defect",
  risk: "p.readings.flow_risk",
  debt: "p.readings.flow_debt",
  unclassified: "p.readings.flow_unclassified",
};

export function flowTypeKey(type: string): StringKey {
  return (
    (FLOW_TYPE_KEY as Record<string, StringKey>)[type] ??
    "p.readings.flow_other"
  );
}

/** Why schedule or cost performance is absent. */
const ABSENT_KEY: Record<PerformanceAbsent, StringKey> = {
  no_baseline: "p.readings.absent_no_baseline",
  no_weighted_tasks: "p.readings.absent_no_weighted_tasks",
  not_yet_defined: "p.readings.absent_not_yet_defined",
  several_currencies: "p.readings.absent_several_currencies",
};

export function absentKey(absent: string | null): StringKey {
  return (
    (ABSENT_KEY as Record<string, StringKey>)[absent ?? ""] ??
    "p.readings.absent_other"
  );
}

/** A workflow state's category. */
export const CATEGORY_KEY: Record<string, StringKey> = {
  todo: "p.readings.cat_todo",
  active: "p.readings.cat_active",
  waiting: "p.readings.cat_waiting",
  done: "p.readings.cat_done",
};

/** The built-in task states, in words; a state the plan named keeps the name it was given. */
const BUILT_IN_STATE: Record<string, StringKey> = {
  todo: "p.task.s_todo",
  in_progress: "p.task.s_in_progress",
  in_review: "p.task.s_in_review",
  done: "p.task.s_done",
  blocked: "p.task.s_blocked",
};

/** A state's name: the catalogue's for a built-in one, else the plan's own label in the reader's language when it
 * has one, else its label. */
export function stateName(
  state: { key: string; label: string; labels?: Record<string, string> },
  source: string,
  lang: string,
  t: (k: StringKey) => string,
): string {
  const builtIn = BUILT_IN_STATE[state.key];
  if (source !== "plan" && builtIn) return t(builtIn);
  return state.labels?.[lang] ?? state.label;
}

/** Why a rolled-up figure is absent. */
export function derivedKey(d: Derived<unknown>): StringKey | null {
  if (d.value !== null) return null;
  if (d.reason === "no_children") return "p.taskrollup.no_children";
  if (d.reason === "no_dated_children") return "p.taskrollup.no_dated_children";
  if (d.reason === "no_estimates") return "p.taskrollup.no_estimates";
  return "p.taskrollup.absent";
}

/** A reading that was refused. */
export type Reading =
  | "rollup"
  | "effort"
  | "forecast"
  | "performance"
  | "workflow"
  | "distribution"
  | "task_rollup";

/** The catalogue sentence for a refused reading, or `null` for the general rule. Every query is checked first. */
export function readingRefusalKey(
  reading: Reading,
  err: unknown,
): StringKey | null {
  if (!(err instanceof ApiError)) return null;
  if (err.status === 422) return "p.readings.refused_query";
  if (err.status === 404) {
    const code = (err.body as { code?: unknown } | undefined)?.code;
    if (code === "module_disabled") return "p.readings.refused_module";
    return reading === "task_rollup"
      ? "p.taskrollup.refused_gone"
      : "p.readings.refused_gone";
  }
  return null;
}
