/**
 * Zone-free calendar dates (`YYYY-MM-DD`) as UTC day numbers, so a daylight-saving change can never shift a bar,
 * a cell or a date by a day. No DOM, no Svelte, no clock, so it is unit-tested. (The file keeps its name from when
 * it also held the Gantt and Kanban grid helpers, which Lily's GanttChart and KanbanBoard replaced.)
 */

const DAY_MS = 86_400_000;

/** A calendar date as a whole day number since the epoch, or `null` if malformed. */
export function dayNumber(date: string | null | undefined): number | null {
  if (!date || !/^\d{4}-\d{2}-\d{2}/.test(date)) return null;
  const ms = Date.parse(`${date.slice(0, 10)}T00:00:00Z`);
  return Number.isNaN(ms) ? null : Math.floor(ms / DAY_MS);
}

/** The `YYYY-MM-DD` of a day number. */
export function dateOf(day: number): string {
  return new Date(day * DAY_MS).toISOString().slice(0, 10);
}
