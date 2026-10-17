/**
 * Pure helpers behind the Gantt (`/gantt`): the schedule view as Lily GanttChart tasks.
 * Dates are zone-free `YYYY-MM-DD` strings. Lily computes nothing about the schedule, so the
 * critical path, the dependencies and the dates are the service's own, passed through.
 */

import type { ScheduleView } from "./api/ppm.js";
import { dateOf, dayNumber } from "./lily-grids";

/** The tasks Lily's GanttChart takes (a subset of its `GanttTask`). */
export interface ChartTask {
  id: string;
  label: string;
  start?: string;
  end?: string;
  critical?: boolean;
  dependsOn?: string[];
}

/**
 * Schedule items as chart tasks. An item with a missing, malformed or reversed range is still listed, as an
 * unscheduled row with no bar, never invented onto the timeline. A dependency whose predecessor is not in the
 * schedule is left out. (The service's lag is in business days, which the chart's day-based lag cannot say, so
 * it is not passed.)
 */
export function chartTasks(schedule: Pick<ScheduleView, "items" | "edges">): ChartTask[] {
  const known = new Set(schedule.items.map((i) => i.pid));
  const preds = new Map<string, string[]>();
  for (const e of schedule.edges) {
    if (!known.has(e.predecessor_pid) || e.predecessor_pid === e.successor_pid) continue;
    preds.set(e.successor_pid, [...(preds.get(e.successor_pid) ?? []), e.predecessor_pid]);
  }
  return schedule.items.map((i) => {
    const s = dayNumber(i.start);
    const e = dayNumber(i.end);
    const dated = s !== null && e !== null && e >= s;
    const task: ChartTask = { id: i.pid, label: i.name, critical: i.on_critical_path };
    if (dated) {
      task.start = dateOf(s);
      task.end = dateOf(e);
    }
    const dependsOn = preds.get(i.pid);
    if (dependsOn) task.dependsOn = dependsOn;
    return task;
  });
}

/**
 * The chart's time range: the dated tasks' span, widened by `pad` days each side so a bar never touches the
 * edge, and widened to include `today` when it is given and close. With nothing dated, the month around `today`
 * (or null when there is no `today` either).
 */
export function chartRange(
  tasks: readonly ChartTask[],
  today?: string,
  pad = 3,
): { start: string; end: string } | null {
  const days = tasks.flatMap((t) => [dayNumber(t.start), dayNumber(t.end)]).filter((d): d is number => d !== null);
  const t = dayNumber(today);
  if (days.length === 0) return t === null ? null : { start: dateOf(t - 14), end: dateOf(t + 14) };
  let first = Math.min(...days);
  let last = Math.max(...days);
  if (t !== null && t >= first - 60 && t <= last + 60) {
    first = Math.min(first, t);
    last = Math.max(last, t);
  }
  return { start: dateOf(first - pad), end: dateOf(last + pad) };
}
