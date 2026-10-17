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
