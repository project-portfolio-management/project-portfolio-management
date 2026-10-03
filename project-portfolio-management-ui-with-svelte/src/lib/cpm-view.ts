/**
 * Pure helpers behind the critical-path and calendar views (SC-1, SC-2, PJ-12).
 * No DOM, no Svelte, no clock: unit-tested.
 *
 * Dates are zone-free `YYYY-MM-DD` strings, handled as UTC day numbers (see
 * `lily-grids.ts`), so a daylight-saving change can never move a bar or a
 * calendar cell. Float is in the **plan's working days**; bars are drawn in
 * calendar days, so a weekend inside a bar is simply drawn, not counted.
 */

import { dateOf, dayNumber } from "./lily-grids";

export const WEEKDAY_NAMES = [
  "Monday",
  "Tuesday",
  "Wednesday",
  "Thursday",
  "Friday",
  "Saturday",
  "Sunday",
] as const;

/** `1111100` → `[true, true, true, true, true, false, false]` (Monday first). Anything malformed is all-working. */
export function maskToWeekdays(mask: string): boolean[] {
  if (!/^[01]{7}$/.test(mask)) return Array<boolean>(7).fill(true);
  return [...mask].map((c) => c === "1");
}

/** The inverse of {@link maskToWeekdays}. */
export function weekdaysToMask(days: readonly boolean[]): string {
  return Array.from({ length: 7 }, (_, i) => (days[i] ? "1" : "0")).join("");
}

/** Whether a weekday selection can be saved: at least one day must work. */
export function weekdaysValid(days: readonly boolean[]): boolean {
  return days.slice(0, 7).some(Boolean);
}

/** Add `date` to `list` if absent, remove it if present; sorted, no duplicates. */
export function toggleHoliday(list: readonly string[], date: string): string[] {
  const set = new Set(list);
  if (set.has(date)) set.delete(date);
  else set.add(date);
  return [...set].sort();
}

/** One cell of a month grid. */
export interface MonthCell {
  date: string;
  inMonth: boolean;
  /** Works under the weekly pattern and is not a holiday or absence. */
  working: boolean;
  /** A listed holiday. */
  holiday: boolean;
  /** A recorded absence of the person. */
  absent: boolean;
  /** A weekend or other non-working weekday of the weekly pattern. */
  offDay: boolean;
}

/** Monday-first weekday index (0 = Monday) of a day number. */
function mondayIndex(day: number): number {
  return (((day + 3) % 7) + 7) % 7;
}

/**
 * The weeks of `month` (1–12) of `year`, Monday first, padded with the
 * neighbouring months' days so every row has seven cells.
 */
export function monthGrid(
  year: number,
  month: number,
  mask: string,
  holidays: readonly string[],
  absences: readonly string[] = [],
): MonthCell[][] {
  const days = maskToWeekdays(mask);
  const first = dayNumber(`${String(year).padStart(4, "0")}-${String(month).padStart(2, "0")}-01`);
  if (first === null) return [];
  const next = month === 12 ? dayNumber(`${year + 1}-01-01`) : dayNumber(`${year}-${String(month + 1).padStart(2, "0")}-01`);
  const length = (next ?? first + 30) - first;
  const lead = mondayIndex(first);
  const rows = Math.ceil((lead + length) / 7);
  const hol = new Set(holidays);
  const abs = new Set(absences);
  const grid: MonthCell[][] = [];
  for (let r = 0; r < rows; r++) {
    const row: MonthCell[] = [];
    for (let c = 0; c < 7; c++) {
      const day = first - lead + r * 7 + c;
      const date = dateOf(day);
      const off = !days[c];
      row.push({
        date,
        inMonth: day >= first && day < first + length,
        holiday: hol.has(date),
        absent: abs.has(date),
        offDay: off,
        working: !off && !hol.has(date) && !abs.has(date),
      });
    }
    grid.push(row);
  }
  return grid;
}

/** The (year, month) before / after `(year, month)`. */
export function shiftMonth(year: number, month: number, by: number): { year: number; month: number } {
  const index = year * 12 + (month - 1) + by;
  return { year: Math.floor(index / 12), month: (((index % 12) + 12) % 12) + 1 };
}

const plural = (n: number, one: string, many: string) => `${n} ${Math.abs(n) === 1 ? one : many}`;

/**
 * A plain sentence for a task's float, for the worker as much as the planner.
 * Negative float means a constraint or deadline cannot be met; it is said so,
 * not shown as a bare minus sign.
 */
export function floatLabel(n: { total_float: number; critical: boolean; near_critical: boolean }): string {
  if (n.total_float < 0) {
    return `Cannot meet its constraint or deadline: ${plural(-n.total_float, "working day", "working days")} short`;
  }
  if (n.critical) return "On the critical path: any slip moves the finish";
  const d = plural(n.total_float, "working day", "working days");
  return n.near_critical ? `${d} of float (near-critical)` : `${d} of float`;
}

/** The task's state for a key and a non-colour marker, so colour is never the only signal. */
export function floatState(n: {
  total_float: number;
  critical: boolean;
  near_critical: boolean;
}): { key: "late" | "critical" | "near" | "float"; marker: string } {
  if (n.total_float < 0) return { key: "late", marker: "!" };
  if (n.critical) return { key: "critical", marker: "●" };
  if (n.near_critical) return { key: "near", marker: "◐" };
  return { key: "float", marker: "○" };
}

/** One bar of the CPM Gantt, in percent of the whole span. */
export interface CpmBar {
  pid: string;
  title: string;
  startPct: number;
  widthPct: number;
  /** The slack drawn after the bar (early finish → late finish), 0 when none or negative. */
  floatPct: number;
  /** Where the latest allowed finish sits when it is **before** the early finish (negative float), else `null`. */
  limitPct: number | null;
  state: ReturnType<typeof floatState>["key"];
  marker: string;
  label: string;
}

/** A timeline tick. */
export interface Tick {
  pct: number;
  label: string;
}

/** What {@link cpmBars} returns. */
export interface CpmChart {
  from: string;
  to: string;
  days: number;
  ticks: Tick[];
  bars: CpmBar[];
}

interface BarInput {
  pid: string;
  title: string | null;
  early_start: string;
  early_finish: string;
  late_finish: string;
  total_float: number;
  critical: boolean;
  near_critical: boolean;
}

/**
 * Geometry for the CPM Gantt: each task's bar from early start to early finish
 * and, after it, its float up to the latest finish. A task whose latest finish is
 * *before* its early finish (negative float) gets a limit marker instead, so the
 * shortfall is visible. Weekly ticks for short spans, monthly for longer ones.
 */
export function cpmBars(nodes: readonly BarInput[]): CpmChart | null {
  const valid = nodes.filter(
    (n) => dayNumber(n.early_start) !== null && dayNumber(n.early_finish) !== null && dayNumber(n.late_finish) !== null,
  );
  if (valid.length === 0) return null;
  const starts = valid.map((n) => dayNumber(n.early_start) as number);
  const ends = valid.map((n) => Math.max(dayNumber(n.early_finish) as number, dayNumber(n.late_finish) as number));
  const first = Math.min(...starts);
  const last = Math.max(...ends);
  const days = last - first + 1;
  const pct = (d: number) => ((d - first) / days) * 100;
  const bars = valid.map((n): CpmBar => {
    const es = dayNumber(n.early_start) as number;
    const ef = dayNumber(n.early_finish) as number;
    const lf = dayNumber(n.late_finish) as number;
    const st = floatState(n);
    return {
      pid: n.pid,
      title: n.title ?? "(untitled)",
      startPct: pct(es),
      widthPct: ((ef - es + 1) / days) * 100,
      floatPct: lf > ef ? ((lf - ef) / days) * 100 : 0,
      limitPct: lf < ef ? pct(lf + 1) : null,
      state: st.key,
      marker: st.marker,
      label: `${n.title ?? "(untitled)"}: ${n.early_start} to ${n.early_finish}. ${floatLabel(n)}`,
    };
  });
  const ticks: Tick[] = [];
  if (days <= 56) {
    for (let d = first; d <= last; d++) {
      if (mondayIndex(d) === 0) ticks.push({ pct: pct(d), label: dateOf(d).slice(5) });
    }
  } else {
    for (let d = first; d <= last; d++) {
      if (dateOf(d).endsWith("-01")) ticks.push({ pct: pct(d), label: dateOf(d).slice(0, 7) });
    }
  }
  return { from: dateOf(first), to: dateOf(last), days, ticks, bars };
}

/** A laid-out node of the network diagram. */
export interface NetNode {
  pid: string;
  layer: number;
  row: number;
}

/** A laid-out edge. */
export interface NetEdge {
  from: string;
  to: string;
  /** On a critical chain (consecutive tasks of one). */
  critical: boolean;
}

/**
 * Layered layout for the precedence diagram: a node's **layer** is the length of
 * its longest chain of predecessors, rows within a layer are ordered by two
 * barycenter sweeps to cut crossings (stable: ties keep input order). A cycle
 * cannot be laid out by depth, so its members fall back to layer 0 instead of
 * looping; the service refuses cycles, this is only a guard.
 */
export function layoutNetwork(
  ids: readonly string[],
  edges: readonly { from: string; to: string }[],
  chains: readonly (readonly string[])[] = [],
): { nodes: NetNode[]; edges: NetEdge[]; layers: number } {
  const known = new Set(ids);
  const usable = edges.filter((e) => known.has(e.from) && known.has(e.to) && e.from !== e.to);
  const preds = new Map<string, string[]>();
  for (const e of usable) preds.set(e.to, [...(preds.get(e.to) ?? []), e.from]);
  const depth = new Map<string, number>();
  const visiting = new Set<string>();
  const layerOf = (id: string): number => {
    const cached = depth.get(id);
    if (cached !== undefined) return cached;
    if (visiting.has(id)) return 0;
    visiting.add(id);
    const d = Math.max(-1, ...(preds.get(id) ?? []).map(layerOf)) + 1;
    visiting.delete(id);
    depth.set(id, d);
    return d;
  };
  const layerCount = Math.max(0, ...ids.map(layerOf)) + 1;
  const columns: string[][] = Array.from({ length: layerCount }, () => []);
  for (const id of ids) columns[layerOf(id)]?.push(id);
  const succs = new Map<string, string[]>();
  for (const e of usable) succs.set(e.from, [...(succs.get(e.from) ?? []), e.to]);
  const rowOf = new Map<string, number>();
  const reindex = () => columns.forEach((col) => col.forEach((id, i) => rowOf.set(id, i)));
  reindex();
  const sweep = (neighbours: Map<string, string[]>, order: number[]) => {
    for (const c of order) {
      const col = columns[c] ?? [];
      const score = (id: string) => {
        const ns = (neighbours.get(id) ?? []).map((n) => rowOf.get(n) ?? 0);
        return ns.length === 0 ? (rowOf.get(id) ?? 0) : ns.reduce((a, b) => a + b, 0) / ns.length;
      };
      const original = new Map(col.map((id, i) => [id, i]));
      col.sort((a, b) => score(a) - score(b) || (original.get(a) ?? 0) - (original.get(b) ?? 0));
      col.forEach((id, i) => rowOf.set(id, i));
    }
  };
  const forward = Array.from({ length: layerCount }, (_, i) => i);
  sweep(preds, forward.slice(1));
  sweep(succs, [...forward].reverse().slice(1));
  const critical = new Set<string>();
  for (const chain of chains) {
    for (let i = 0; i + 1 < chain.length; i++) critical.add(`${chain[i]}>${chain[i + 1]}`);
  }
  return {
    nodes: ids.map((pid) => ({ pid, layer: layerOf(pid), row: rowOf.get(pid) ?? 0 })),
    edges: usable.map((e) => ({ ...e, critical: critical.has(`${e.from}>${e.to}`) })),
    layers: layerCount,
  };
}

/** Human text for why a task has no float. */
export function unscheduledReason(reason: string): string {
  switch (reason) {
    case "no_duration":
      return "has no duration yet";
    case "no_anchor":
      return "has no start date and nothing before it is scheduled";
    case "date_out_of_range":
      return "has a date outside the supported range";
    default:
      return reason;
  }
}
