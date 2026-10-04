<!--
  The Gantt as an accessible grid (Lily headless `GanttTable*`): the keyboard-
  and screen-reader-first counterpart of the SVAR chart. A task bar is the run
  of marked cells in its row; each marked cell is named ("Design: 5 Oct – 11
  Oct"), the arrows / Home / End move between cells, and a task with no dates
  is listed, never invented onto the timeline. Read-only, like the chart.
-->
<script lang="ts">
  import { localeHref } from "#lib/i18n.svelte.js";
  import {
    GanttTable,
    GanttTableThead,
    GanttTableTbody,
    GanttTableTr,
    GanttTableTH,
    GanttTableTD,
  } from "@lilydesignsystem/svelte-headless";
  import { ganttGrid, gridKeyTarget, type GanttGrid, type GanttItem } from "#lib/lily-grids.js";

  let {
    items,
    label = "Schedule",
  }: { items: GanttItem[]; label?: string } = $props();

  let unit = $state<GanttGrid["unit"]>("week");
  const grid = $derived(ganttGrid(items, { unit }));
  let table: HTMLElement | undefined = $state();

  function onkeydown(event: KeyboardEvent) {
    const target = event.target as HTMLElement;
    const row = Number(target.dataset["r"]);
    const col = Number(target.dataset["c"]);
    if (Number.isNaN(row) || Number.isNaN(col)) return;
    const next = gridKeyTarget({ row, col }, event.key, grid.rows.length, grid.columns.length, event.ctrlKey);
    if (!next) return;
    event.preventDefault();
    table?.querySelector<HTMLElement>(`[data-r="${next.row}"][data-c="${next.col}"]`)?.focus();
  }

  function heading(from: string): string {
    return grid.unit === "month" ? from.slice(0, 7) : from.slice(5);
  }
</script>

<div class="lily-gantt" bind:this={table}>
  <label class="unit">
    Time scale
    <select bind:value={unit}>
      <option value="day">Day</option>
      <option value="week">Week</option>
      <option value="month">Month</option>
    </select>
  </label>
  {#if grid.coarsened}
    <p class="muted small" role="status">
      That range is too long for one column per {unit}; showing one per {grid.unit}.
    </p>
  {/if}
  {#if grid.rows.length === 0}
    <p class="muted">No dated tasks to show.</p>
  {:else}
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <div class="scroll" {onkeydown} role="presentation">
      <GanttTable {label} class="gantt-table" data-testid="lily-gantt">
        <GanttTableThead>
          <GanttTableTr>
            <GanttTableTH scope="col">Task</GanttTableTH>
            {#each grid.columns as column (column.key)}
              <GanttTableTH scope="col" title="{column.from} – {column.to}">{heading(column.from)}</GanttTableTH>
            {/each}
          </GanttTableTr>
        </GanttTableThead>
        <GanttTableTbody>
          {#each grid.rows as row, r (row.pid)}
            <GanttTableTr>
              <GanttTableTH scope="row">
                <a href={localeHref(`/plans/${row.pid}`)}>{row.name}</a>{#if row.critical}
                  <span class="chip">critical path</span>{/if}
              </GanttTableTH>
              {#each row.cells as cell, c (c)}
                <GanttTableTD
                  active={cell.active}
                  class={cell.active ? (row.critical ? "bar critical" : "bar") : ""}
                  aria-label={cell.active
                    ? `${row.name}: ${row.start} to ${row.end}${cell.ends ? " (ends here)" : ""}`
                    : `${row.name}: not scheduled`}
                  data-r={r}
                  data-c={c}
                >
                  {#if cell.active}<span aria-hidden="true">{cell.ends ? "◆" : "■"}</span>{/if}
                </GanttTableTD>
              {/each}
            </GanttTableTr>
          {/each}
        </GanttTableTbody>
      </GanttTable>
    </div>
  {/if}
  {#if grid.undated.length > 0}
    <p class="muted small">
      No usable dates:
      {#each grid.undated as item (item.pid)}<span class="chip">{item.name}</span>{/each}
    </p>
  {/if}
</div>

<style>
  .scroll {
    overflow-x: auto;
  }
  .lily-gantt :global(.gantt-table) {
    border-collapse: collapse;
    min-width: 100%;
  }
  .lily-gantt :global(th),
  .lily-gantt :global(td) {
    border: 1px solid var(--mxi-color-border, #ccc);
    padding: 0.25rem 0.4rem;
    text-align: center;
    white-space: nowrap;
    font-size: 0.85em;
  }
  .lily-gantt :global(th[scope="row"]) {
    text-align: start;
    position: sticky;
    inset-inline-start: 0;
    background: var(--mxi-color-bg, Canvas);
  }
  .lily-gantt :global(td.bar) {
    background: color-mix(in srgb, var(--mxi-color-accent, #2563eb) 35%, transparent);
  }
  .lily-gantt :global(td.bar.critical) {
    background: color-mix(in srgb, #c0392b 40%, transparent);
  }
  .lily-gantt :global(td:focus-visible) {
    outline: 3px solid var(--mxi-color-focus, #1d4ed8);
    outline-offset: -3px;
  }
</style>
