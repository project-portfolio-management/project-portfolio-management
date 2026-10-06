/**
 * Pure helpers behind the Lily headless Gantt and Kanban tables
 * (`GanttTable*`, `KanbanTable*` from `@lilydesignsystem/svelte-headless`).
 *
 * The SVAR Gantt and Kanban are canvas-like widgets; these tables are their
 * **keyboard- and screen-reader-first** counterpart: the same data as a real
 * `role="grid"`, where a task bar is the run of active cells and moving a card
 * is a labelled control, not only a drag. No DOM, no Svelte, no clock, so it is
 * unit-tested.
 *
 * Dates are zone-free calendar dates (`YYYY-MM-DD`), handled as UTC day
 * numbers so a daylight-saving change can never shift a bar by a cell.
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

/** A schedule item as the Gantt table takes it. */
export interface GanttItem {
  pid: string;
  name: string;
  start: string | null;
  end: string | null;
  critical?: boolean;
}

/** One time column. */
export interface GanttColumn {
  key: string;
  /** First day (inclusive) as `YYYY-MM-DD`. */
  from: string;
  /** Last day (inclusive). */
  to: string;
}

/** One cell of a task's row. */
export interface GanttCell {
  /** The task spans some day of this column. */
  active: boolean;
  /** The task's last day falls in this column. */
  ends: boolean;
}

/** One task row. */
export interface GanttRow {
  pid: string;
  name: string;
  critical: boolean;
  start: string;
  end: string;
  cells: GanttCell[];
}

/** The whole table. */
export interface GanttGrid {
  unit: "day" | "week" | "month";
  columns: GanttColumn[];
  rows: GanttRow[];
  /** Items left out because they have no usable start and end (never invented onto the timeline). */
  undated: GanttItem[];
  /** The timeline was longer than `maxColumns` and the unit was coarsened. */
  coarsened: boolean;
}

const MAX_COLUMNS_DEFAULT = 60;

function columnsFor(
  first: number,
  last: number,
  unit: GanttGrid["unit"],
): GanttColumn[] {
  const out: GanttColumn[] = [];
  if (unit === "day") {
    for (let d = first; d <= last; d++)
      out.push({ key: dateOf(d), from: dateOf(d), to: dateOf(d) });
    return out;
  }
  if (unit === "week") {
    // Weeks start on Monday: day 0 (1970-01-01) was a Thursday.
    const monday = first - ((first + 3) % 7);
    for (let d = monday; d <= last; d += 7) {
      out.push({ key: dateOf(d), from: dateOf(d), to: dateOf(d + 6) });
    }
    return out;
  }
  let cursor = new Date(dateOf(first).slice(0, 7) + "-01T00:00:00Z");
  while (Math.floor(cursor.getTime() / DAY_MS) <= last) {
    const next = new Date(
      Date.UTC(cursor.getUTCFullYear(), cursor.getUTCMonth() + 1, 1),
    );
    const from = Math.floor(cursor.getTime() / DAY_MS);
    out.push({
      key: dateOf(from).slice(0, 7),
      from: dateOf(from),
      to: dateOf(Math.floor(next.getTime() / DAY_MS) - 1),
    });
    cursor = next;
  }
  return out;
}

/**
 * Build the Gantt table. The unit is the finest of day / week / month whose
 * column count fits `maxColumns`; `coarsened` says it was not the finest asked
 * for. Items with a missing, malformed or reversed range go to `undated`.
 */
export function ganttGrid(
  items: GanttItem[],
  options: { unit?: GanttGrid["unit"]; maxColumns?: number } = {},
): GanttGrid {
  const maxColumns = options.maxColumns ?? MAX_COLUMNS_DEFAULT;
  const dated: Array<{ item: GanttItem; s: number; e: number }> = [];
  const undated: GanttItem[] = [];
  for (const item of items) {
    const s = dayNumber(item.start);
    const e = dayNumber(item.end);
    if (s === null || e === null || e < s) undated.push(item);
    else dated.push({ item, s, e });
  }
  const wanted = options.unit ?? "week";
  if (dated.length === 0)
    return { unit: wanted, columns: [], rows: [], undated, coarsened: false };
  const first = Math.min(...dated.map((d) => d.s));
  const last = Math.max(...dated.map((d) => d.e));
  const order: GanttGrid["unit"][] = ["day", "week", "month"];
  let unit = wanted;
  let columns = columnsFor(first, last, unit);
  while (columns.length > maxColumns && unit !== "month") {
    unit = order[order.indexOf(unit) + 1] ?? "month";
    columns = columnsFor(first, last, unit);
  }
  const bounds = columns.map(
    (c) => [dayNumber(c.from) as number, dayNumber(c.to) as number] as const,
  );
  const rows = dated.map(({ item, s, e }) => ({
    pid: item.pid,
    name: item.name,
    critical: item.critical === true,
    start: dateOf(s),
    end: dateOf(e),
    cells: bounds.map(([from, to]) => ({
      active: s <= to && e >= from,
      ends: e >= from && e <= to,
    })),
  }));
  return { unit, columns, rows, undated, coarsened: unit !== wanted };
}

/** A card, as the Kanban table takes it. */
export interface KanbanCard {
  pid: string;
  title: string;
  status: string;
}

/** A Kanban column definition. */
export interface KanbanColumn {
  id: string;
  label: string;
}

/** The Kanban table: a grid of rows, one card per column per row. */
export interface KanbanGrid {
  columns: Array<KanbanColumn & { count: number }>;
  /** `rows[r][c]` is the r-th card of column c, or `null` for an empty cell. */
  rows: Array<Array<KanbanCard | null>>;
  /** Cards whose status matches no column (shown, never dropped). */
  orphans: KanbanCard[];
}

/** Lay cards out in columns by status, keeping each column's order. */
export function kanbanGrid(
  cards: KanbanCard[],
  columns: KanbanColumn[],
): KanbanGrid {
  const byColumn = columns.map((c) =>
    cards.filter((card) => card.status === c.id),
  );
  const known = new Set(columns.map((c) => c.id));
  const depth = Math.max(0, ...byColumn.map((c) => c.length));
  const rows = Array.from({ length: depth }, (_, r) =>
    byColumn.map((col) => col[r] ?? null),
  );
  return {
    columns: columns.map((c, i) => ({ ...c, count: byColumn[i]?.length ?? 0 })),
    rows,
    orphans: cards.filter((card) => !known.has(card.status)),
  };
}

/** A grid position. */
export interface Pos {
  row: number;
  col: number;
}

/**
 * Where a key press moves the focus in a `rows × cols` grid (the WAI-ARIA grid
 * pattern the Lily tables leave to the consumer): arrows step and stop at the
 * edges, Home / End go to the row's ends, Ctrl+Home / Ctrl+End to the grid's
 * corners. `null` when the key is not a navigation key.
 */
export function gridKeyTarget(
  pos: Pos,
  key: string,
  rows: number,
  cols: number,
  ctrl = false,
): Pos | null {
  if (rows <= 0 || cols <= 0) return null;
  const clamp = (n: number, max: number) => Math.min(Math.max(n, 0), max - 1);
  switch (key) {
    case "ArrowRight":
      return { row: pos.row, col: clamp(pos.col + 1, cols) };
    case "ArrowLeft":
      return { row: pos.row, col: clamp(pos.col - 1, cols) };
    case "ArrowDown":
      return { row: clamp(pos.row + 1, rows), col: pos.col };
    case "ArrowUp":
      return { row: clamp(pos.row - 1, rows), col: pos.col };
    case "Home":
      return ctrl ? { row: 0, col: 0 } : { row: pos.row, col: 0 };
    case "End":
      return ctrl
        ? { row: rows - 1, col: cols - 1 }
        : { row: pos.row, col: cols - 1 };
    default:
      return null;
  }
}
