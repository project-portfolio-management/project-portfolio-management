<!--
  A month of a working calendar (SC-2): which days work under the weekly
  pattern, which are holidays, and which are the person's recorded absences.
  Every state has a symbol as well as a colour (– off day, ✕ holiday, A absent),
  and when `editable`, a day is a button that toggles a holiday.
-->
<script lang="ts">
  import { WEEKDAY_NAMES, monthGrid, shiftMonth } from "#lib/cpm-view.js";

  let {
    year = $bindable(),
    month = $bindable(),
    mask,
    holidays,
    absences = [],
    editable = false,
    ontoggle,
  }: {
    year: number;
    month: number;
    mask: string;
    holidays: string[];
    absences?: string[];
    editable?: boolean;
    ontoggle?: (date: string) => void;
  } = $props();

  const grid = $derived(monthGrid(year, month, mask, holidays, absences));
  const MONTHS = [
    "January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December",
  ];

  function go(by: number) {
    const next = shiftMonth(year, month, by);
    year = next.year;
    month = next.month;
  }

  function describe(c: (typeof grid)[number][number], dayName: string): string {
    const state = c.holiday ? "holiday" : c.absent ? "absent" : c.offDay ? "not a working day" : "working day";
    return `${c.date}, ${dayName}: ${state}`;
  }
</script>

<div class="cal" data-testid="calendar-month">
  <div class="head">
    <button type="button" onclick={() => go(-1)} aria-label="Previous month">‹</button>
    <strong aria-live="polite">{MONTHS[month - 1]} {year}</strong>
    <button type="button" onclick={() => go(1)} aria-label="Next month">›</button>
  </div>
  <table>
    <thead>
      <tr>
        {#each WEEKDAY_NAMES as d (d)}<th scope="col" abbr={d}>{d.slice(0, 3)}</th>{/each}
      </tr>
    </thead>
    <tbody>
      {#each grid as week, w (w)}
        <tr>
          {#each week as c, i (c.date)}
            <td class:out={!c.inMonth} class:off={c.offDay} class:holiday={c.holiday} class:absent={c.absent}>
              {#if editable && c.inMonth && !c.offDay}
                <button
                  type="button"
                  aria-pressed={c.holiday}
                  aria-label={describe(c, WEEKDAY_NAMES[i] ?? "")}
                  onclick={() => ontoggle?.(c.date)}
                >
                  <span class="d">{Number(c.date.slice(8))}</span>
                  <span class="m" aria-hidden="true">{c.holiday ? "✕" : c.absent ? "A" : ""}</span>
                </button>
              {:else}
                <span class="cell" role="img" aria-label={describe(c, WEEKDAY_NAMES[i] ?? "")}>
                  <span class="d">{Number(c.date.slice(8))}</span>
                  <span class="m" aria-hidden="true">{c.holiday ? "✕" : c.absent ? "A" : c.offDay ? "–" : ""}</span>
                </span>
              {/if}
            </td>
          {/each}
        </tr>
      {/each}
    </tbody>
  </table>
  <p class="small muted">– not a working day &nbsp; ✕ holiday &nbsp; A recorded absence{editable ? " · select a working day to mark or clear a holiday" : ""}</p>
</div>

<style>
  .cal {
    display: inline-block;
    max-width: 100%;
  }
  .head {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 0.5rem;
    margin-block-end: 0.35rem;
  }
  table {
    border-collapse: collapse;
  }
  th,
  td {
    border: 1px solid var(--border, #ccc);
    width: 3.1rem;
    height: 2.6rem;
    padding: 0;
    text-align: center;
    vertical-align: middle;
    font-size: 0.85em;
  }
  td.out {
    opacity: 0.4;
  }
  td.off {
    background: var(--surface-2, #eee);
  }
  td.holiday {
    background: color-mix(in srgb, #a4262c 18%, transparent);
  }
  td.absent {
    background: color-mix(in srgb, #b45309 18%, transparent);
  }
  .cell,
  button {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
  }
  button {
    background: none;
    border: 0;
    font: inherit;
    cursor: pointer;
  }
  button[aria-pressed="true"] {
    font-weight: 700;
  }
  button:focus-visible {
    outline: 3px solid var(--mxi-color-focus, #1d4ed8);
    outline-offset: -3px;
  }
  .m {
    font-size: 0.75em;
    line-height: 1;
    min-height: 0.8em;
  }
</style>
