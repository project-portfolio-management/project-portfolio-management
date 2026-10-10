<!--
  The Kanban board as an accessible grid (Lily headless `KanbanTable*`): the
  keyboard- and screen-reader-first counterpart of the drag board. Every card
  has a labelled "Move to" control — moving a card never requires a pointer —
  and the arrows / Home / End move between cells. A card with an unknown status
  is listed under the board, never dropped. The move is the caller's: this only
  reports `onmove(pid, status)`, so the service stays the one writer.
-->
<script lang="ts">
  import { t } from "#lib/i18n.svelte.js";
  import {
    KanbanTable,
    KanbanTableHead,
    KanbanTableBody,
    KanbanTableRow,
    KanbanTableTH,
    KanbanTableTD,
  } from "@lilydesignsystem/svelte-headless";
  import {
    gridKeyTarget,
    kanbanGrid,
    type KanbanCard,
    type KanbanColumn,
  } from "#lib/lily-grids.js";

  let {
    cards,
    columns,
    label,
    onmove,
  }: {
    cards: KanbanCard[];
    columns: KanbanColumn[];
    label?: string;
    onmove: (pid: string, status: string) => void;
  } = $props();

  const grid = $derived(kanbanGrid(cards, columns));
  let root: HTMLElement | undefined = $state();

  function onkeydown(event: KeyboardEvent) {
    const target = event.target as HTMLElement;
    // Only when the cell itself has focus: arrows inside the Move select belong to the select.
    if (target.dataset["r"] === undefined) return;
    const next = gridKeyTarget(
      { row: Number(target.dataset["r"]), col: Number(target.dataset["c"]) },
      event.key,
      grid.rows.length,
      grid.columns.length,
      event.ctrlKey,
    );
    if (!next) return;
    event.preventDefault();
    root?.querySelector<HTMLElement>(`[data-r="${next.row}"][data-c="${next.col}"]`)?.focus();
  }
</script>

<div class="lily-kanban" bind:this={root}>
  {#if cards.length === 0}
    <p class="muted">{t("c.LilyKanbanTable.no_tasks_yet")}</p>
  {:else}
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <div class="scroll" {onkeydown} role="presentation">
      <KanbanTable label={label ?? t("c.LilyKanbanTable.board")} class="kanban-table" data-testid="lily-kanban">
        <KanbanTableHead>
          <KanbanTableRow>
            {#each grid.columns as column (column.id)}
              <KanbanTableTH scope="col">{column.label} ({column.count})</KanbanTableTH>
            {/each}
          </KanbanTableRow>
        </KanbanTableHead>
        <KanbanTableBody>
          {#each grid.rows as row, r (r)}
            <KanbanTableRow>
              {#each row as card, c (c)}
                <KanbanTableTD
                  label={card ? `${card.title}, ${grid.columns[c]?.label}` : undefined}
                  data-r={r}
                  data-c={c}
                >
                  {#if card}
                    <span class="title">{card.title}</span>
                    <label class="move">
                      <span class="sr-only">{t("c.LilyKanbanTable.move_to", { title: card.title })}</span>
                      <select
                        value={card.status}
                        onchange={(event) => onmove(card.pid, event.currentTarget.value)}
                      >
                        {#each columns as column (column.id)}
                          <option value={column.id}>{column.label}</option>
                        {/each}
                      </select>
                    </label>
                  {/if}
                </KanbanTableTD>
              {/each}
            </KanbanTableRow>
          {/each}
        </KanbanTableBody>
      </KanbanTable>
    </div>
    {#if grid.orphans.length > 0}
      <p class="muted small">
        {t("c.LilyKanbanTable.unknown_status")}
        {#each grid.orphans as card (card.pid)}<span class="chip">{card.title} ({card.status})</span>{/each}
      </p>
    {/if}
  {/if}
</div>

<style>
  .scroll {
    overflow-x: auto;
  }
  .lily-kanban :global(.kanban-table) {
    border-collapse: collapse;
    width: 100%;
    table-layout: fixed;
  }
  .lily-kanban :global(th),
  .lily-kanban :global(td) {
    border: 1px solid var(--mxi-color-border, #ccc);
    padding: 0.4rem;
    vertical-align: top;
    text-align: start;
  }
  .title {
    display: block;
    font-weight: 600;
    margin-block-end: 0.25rem;
  }
  .sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    overflow: hidden;
    clip-path: inset(50%);
    white-space: nowrap;
  }
  .lily-kanban :global(td:focus-visible) {
    outline: 3px solid var(--mxi-color-focus, #1d4ed8);
    outline-offset: -3px;
  }
</style>
