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
  import { t } from "#lib/i18n.svelte.js";

  let {
    label,
    caption,
    columns,
    rows,
    rowId,
    emptyText,
    searchable = false,
    pageSize,
    grid = false,
    selectionMode = "none",
    selected = $bindable([]),
    rowClass,
    onRowActivate,
    footer,
    storageKey,
    class: className = "",
  }: {
    /** The table's accessible name (also the scroll region's). */
    label: string;
    caption?: string;
    columns: DataGridColumn[];
    /** Any objects: interfaces are fine (they are read, never changed). */
    rows: readonly object[];
    rowId?: (row: DataGridRow, index: number) => string;
    /** What to show with no rows; the default says "Nothing to show." */
    emptyText?: string;
    /** Show a filter box. */
    searchable?: boolean;
    pageSize?: number;
    /** Use the grid keyboard model instead of a plain table. */
    grid?: boolean;
    selectionMode?: "none" | "single" | "multiple";
    selected?: string[];
    rowClass?: (row: DataGridRow) => string;
    onRowActivate?: (row: DataGridRow, id: string) => void;
    footer?: Snippet<[{ columns: DataGridColumn[]; rows: DataGridRow[] }]>;
    storageKey?: string;
    class?: string;
  } = $props();

  // Sort on every column unless it says no. Done here so a table's column list stays short.
  const cols = $derived(columns.map((c) => ({ sortable: true, ...c })));
  const direction = (d: string) => t(`p.table.${d === "ascending" ? "asc" : d === "descending" ? "desc" : "none"}` as never);
</script>

<DataGrid
  {label}
  {caption}
  columns={cols}
  rows={rows as DataGridRow[]}
  {rowId}
  mode={grid ? "grid" : "table"}
  {selectionMode}
  bind:selected
  {pageSize}
  {rowClass}
  {onRowActivate}
  {footer}
  {storageKey}
  class={`data-table ${className}`.trim()}
  labels={{
    ...(searchable ? { search: t("p.table.search") } : {}),
    columnVisibility: undefined,
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

<style>
  /* Cell tones. A tone is never the only signal: the cell text carries a word or a symbol as well. */
  :global(.data-table .cell-red) { background: #fde8e8; }
  :global(.data-table .cell-amber) { background: #fff4d6; }
  :global(.data-table .cell-green) { background: #e3f4e3; }
  :global(.data-table .cell-grey) { background: #eeeeee; }
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
