<!--
  DataTable: this app's one way to show a table, on Lily's DataGrid (`@lilydesignsystem/svelte-data-grid`).

  What a table gets from it, for free: a named, scrollable region; sorting on every column unless a column
  opts out; an optional filter box; paging when `pageSize` is set; the empty message; and announcements for
  sort, filter and paging in the page's language. Cells may hold links, buttons and inputs (a column's `cell`
  snippet), a column's `cellClass` colours a cell, and `rowHeader` makes the first column a row header.

  It is a plain `table` by default (`mode="table"`): data that is read, with ordinary tab stops. Pass
  `grid` for the full grid keyboard model (arrow keys between cells) on a table people move around in.
-->
<script lang="ts">
  import DataGrid from "@lilydesignsystem/svelte-data-grid";
  import type { DataGridColumn, DataGridRow } from "@lilydesignsystem/svelte-data-grid";
  import type { Snippet } from "svelte";
  import { t, i18n } from "#lib/i18n.svelte.js";
  import { exportLabels } from "#lib/table-labels.js";

  let {
    label,
    caption,
    captionVisible = false,
    columns,
    rows,
    rowId,
    emptyText,
    searchable = false,
    pageSize,
    sortable = true,
    grid = false,
    selectionMode = "none",
    selected = $bindable([]),
    rowClass,
    rowAttrs,
    onRowActivate,
    footer,
    storageKey,
    exportable = true,
    movable = grid,
    hideable,
    exportFilename,
    testid,
    class: className = "",
  }: {
    /** The table's accessible name (also the scroll region's). */
    label: string;
    /** Text for the table's `<caption>`. Without it the label is the caption, hidden unless `captionVisible`. */
    caption?: string;
    captionVisible?: boolean;
    columns: DataGridColumn[];
    /** Any objects: interfaces are fine (they are read, never changed). */
    rows: readonly object[];
    rowId?: (row: DataGridRow, index: number) => string;
    /** What to show with no rows; the default says "Nothing to show." */
    emptyText?: string;
    /** Show a filter box. */
    searchable?: boolean;
    pageSize?: number;
    /** Sort by clicking a column header (default). Turn off for a table whose rows have a meaningful order. */
    sortable?: boolean;
    /** Use the grid keyboard model instead of a plain table. */
    grid?: boolean;
    selectionMode?: "none" | "single" | "multiple";
    selected?: string[];
    rowClass?: (row: DataGridRow) => string;
    /** Extra attributes for a row (for example an `id` that a link can point to). */
    rowAttrs?: (row: DataGridRow) => Record<string, unknown>;
    onRowActivate?: (row: DataGridRow, id: string) => void;
    footer?: Snippet<[{ columns: DataGridColumn[]; rows: DataGridRow[] }]>;
    storageKey?: string;
    /** Offer CSV, TSV and Excel export of the rows the filter leaves (default). Turn off for a table that is not data. */
    exportable?: boolean;
    /** Columns move with Alt+Arrow on the header (only in the grid keyboard model) or by drag. Default: on exactly when `grid` is, so a move always has a keyboard way. Shift+click on a second header sorts by it as a tie-break. A column may set `movable: false`. */
    movable?: boolean;
    /** A "Columns" control to hide and show columns (not the row header or an unnamed column). Default: on for a table of five or more columns. `storageKey` remembers the choice. */
    hideable?: boolean;
    /** The export's file name without its extension. Default: the label, lower-cased and hyphenated. */
    exportFilename?: string;
    /** A `data-testid` for tests. */
    testid?: string;
    class?: string;
  } = $props();

  // Sort on every column unless it says no. Done here so a table's column list stays short.
  const canHide = $derived(hideable ?? columns.length >= 5);
  const cols = $derived(columns.map((c) => ({ sortable, hidable: canHide && !c.rowHeader && c.header !== "", ...c })));
  // The filter box's name is its aria-label; give sighted readers the same words as a placeholder
  // (the grid sets no placeholder of its own).
  let host = $state<HTMLElement | undefined>();
  $effect(() => {
    const text = t("p.table.search");
    host?.querySelectorAll<HTMLInputElement>(".data-grid-search").forEach((i) => i.setAttribute("placeholder", text));
  });
  const slug = (s: string) =>
    s.toLowerCase().normalize("NFKD").replace(/[^\p{L}\p{N}]+/gu, "-").replace(/^-+|-+$/g, "") || "table";
  const direction = (d: string) => t(`p.table.${d === "ascending" ? "asc" : d === "descending" ? "desc" : "none"}` as never);
</script>

<div bind:this={host} class="data-table-host">
<DataGrid
  {label}
  caption={caption ?? label}
  columns={cols}
  rows={rows as DataGridRow[]}
  {rowId}
  mode={grid ? "grid" : "table"}
  {selectionMode}
  bind:selected
  {pageSize}
  {rowClass}
  {rowAttrs}
  {onRowActivate}
  {footer}
  {storageKey}
  locale={i18n.locale}
  movableColumns={movable}
  exportFilename={exportFilename ?? slug(label)}
  data-testid={testid}
  class={`data-table ${caption && captionVisible ? "caption-visible" : "caption-hidden"} ${className}`.trim()}
  labels={{
    ...(searchable ? { search: t("p.table.search") } : {}),
    ...(exportable ? exportLabels() : {}),
    pagination: t("p.table.pages"),
    columnMoveAnnouncement: (header: string, position: number, count: number) => t("p.table.moved", { header, position, count }),
    ...(searchable ? { searchField: t("p.table.search_in"), searchAllFields: t("p.table.all_columns") } : {}),
    columnVisibility: canHide ? t("p.table.columns") : undefined,
    columnVisibilityOption: (header: string) => t("p.table.show_column", { header }),
    resizeHandle: (header: string) => t("p.table.resize", { header }),
    selectAll: t("p.table.select_all"),
    selectionColumn: t("p.table.select_col"),
    selectRow: (row: string) => t("p.table.select_row", { row }),
    previousPage: t("p.table.prev"),
    nextPage: t("p.table.next"),
    pageStatus: (page: number, pages: number, count: number) => t("p.table.page_status", { page, pages, rows: count }),
    pageAnnouncement: (page: number, pages: number) => t("p.table.page_announce", { page, pages }),
    sortAnnouncement: (header: string, d: string) => t("p.table.sorted", { header, direction: direction(d) }),
    filterAnnouncement: (matches: number, total: number) => t("p.table.filtered", { matches, total }),
    selectionAnnouncement: (n: number) => t("p.table.selected", { n }),
  }}
>
  {#snippet empty()}{emptyText ?? t("p.table.empty")}{/snippet}
</DataGrid>
</div>

<style>
  /* Cell tones. A tone is never the only signal: the cell text carries a word or a symbol as well. */
  :global(.data-table .cell-red) { background: #fde8e8; }
  :global(.data-table .cell-amber) { background: #fff4d6; }
  :global(.data-table .cell-green) { background: #e3f4e3; }
  :global(.data-table .cell-grey) { background: #eeeeee; }
  /* A caption names the table for assistive technology; it shows only when asked for. */
  :global(.data-table.caption-hidden caption) { position: absolute; width: 1px; height: 1px; margin: -1px; padding: 0; overflow: hidden; clip-path: inset(50%); white-space: nowrap; border: 0; }
  :global(.data-table.caption-visible caption) { text-align: start; font-weight: 600; padding: 0.25rem 0; }
  :global(.data-grid-scroll) { margin-block: 0.5rem 1rem; overflow-x: auto; }
  :global(.data-grid-toolbar) { margin-block-end: 0.25rem; }
  :global(.data-grid-search) { box-sizing: border-box; inline-size: min(100%, 20rem); padding: 0.4rem 0.6rem; }
  /* Column headers sort: look like header text, with an arrow for the state (the state is also in aria-sort). */
  :global(.data-table .data-grid-sort-button) {
    all: unset;
    box-sizing: border-box;
    cursor: pointer;
    font-weight: 600;
    padding-block: 0.15rem;
  }
  :global(.data-table .data-grid-sort-button:hover) { text-decoration: underline; }
  :global(.data-table .data-grid-sort-button:focus-visible) { outline: 2px solid currentColor; outline-offset: 2px; }
  :global(.data-table th[aria-sort="ascending"] .data-grid-sort-button::after) { content: " \25B2"; font-size: 0.7em; }
  :global(.data-table th[aria-sort="descending"] .data-grid-sort-button::after) { content: " \25BC"; font-size: 0.7em; }
  :global(.data-table th[aria-sort="none"] .data-grid-sort-button::after) { content: " \21C5"; font-size: 0.7em; opacity: 0.45; }
</style>
