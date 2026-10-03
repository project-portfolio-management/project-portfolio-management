<!--
  The critical-path Gantt (SC-1): each task's bar from early start to early
  finish, then its **float** drawn as a hatched tail up to the latest finish. A
  task with negative float gets a limit marker where it would have had to end.
  State is never colour alone: every row carries a marker (● critical, ◐
  near-critical, ○ float, ! cannot meet a constraint or deadline) and a sentence.
  Read-only; selecting a row opens the schedule editor on the page.
-->
<script lang="ts">
  import type { CpmNode } from "#lib/api/ppm.js";
  import { cpmBars } from "#lib/cpm-view.js";

  let {
    nodes,
    selected = null,
    onselect,
  }: { nodes: CpmNode[]; selected?: string | null; onselect?: (pid: string) => void } = $props();

  const chart = $derived(cpmBars(nodes));
</script>

{#if chart}
  <div class="cpm-gantt" data-testid="cpm-gantt">
    <ul class="legend small" aria-label="Legend">
      <li><span class="mk">●</span> critical: any slip moves the finish</li>
      <li><span class="mk">◐</span> near-critical</li>
      <li><span class="mk">○</span> has float (the hatched tail)</li>
      <li><span class="mk">!</span> cannot meet a constraint or deadline (the line marks the limit)</li>
    </ul>
    <div class="axis" aria-hidden="true">
      <span class="name"></span>
      <span class="track">
        {#each chart.ticks as tick (tick.pct)}
          <span class="tick" style="left:{tick.pct}%">{tick.label}</span>
        {/each}
      </span>
    </div>
    <ol class="rows">
      {#each chart.bars as bar (bar.pid)}
        <li class="row" class:selected={selected === bar.pid}>
          <button type="button" class="name" aria-pressed={selected === bar.pid} onclick={() => onselect?.(bar.pid)}>
            <span class="mk" aria-hidden="true">{bar.marker}</span>
            <span>{bar.title}</span>
            <span class="sr-only">. {bar.label}</span>
          </button>
          <span class="track" aria-hidden="true">
            <span class="bar {bar.state}" style="left:{bar.startPct}%;width:{bar.widthPct}%"></span>
            {#if bar.floatPct > 0}
              <span class="float" style="left:{bar.startPct + bar.widthPct}%;width:{bar.floatPct}%"></span>
            {/if}
            {#if bar.limitPct !== null}
              <span class="limit" style="left:{bar.limitPct}%"></span>
            {/if}
          </span>
        </li>
      {/each}
    </ol>
    <p class="small muted">{chart.from} to {chart.to}. Float is counted in the plan's working days; bars are drawn in calendar days.</p>
  </div>
{:else}
  <p class="muted">No scheduled tasks to draw yet. Give a task a duration (and a start date or a predecessor).</p>
{/if}

<style>
  .cpm-gantt {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
  }
  .legend {
    display: flex;
    flex-wrap: wrap;
    gap: 0.25rem 1rem;
    list-style: none;
    margin: 0;
    padding: 0;
  }
  .mk {
    font-weight: 700;
    display: inline-block;
    min-width: 1.1em;
  }
  .rows {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }
  .row,
  .axis {
    display: grid;
    grid-template-columns: minmax(10rem, 1fr) 3fr;
    gap: 0.6rem;
    align-items: center;
  }
  .name {
    text-align: start;
    background: none;
    border: 1px solid transparent;
    padding: 0.1rem 0.3rem;
    font: inherit;
    cursor: pointer;
    display: flex;
    gap: 0.35rem;
    overflow-wrap: anywhere;
  }
  .row.selected .name {
    border-color: var(--mxi-color-focus, #1d4ed8);
  }
  .name:focus-visible {
    outline: 3px solid var(--mxi-color-focus, #1d4ed8);
  }
  .track {
    position: relative;
    height: 1.3rem;
    background: var(--surface-2, #f0f0f0);
    border-radius: 4px;
  }
  .axis .track {
    height: 1rem;
    background: none;
  }
  .tick {
    position: absolute;
    font-size: 0.7rem;
    border-inline-start: 1px solid var(--border, #ccc);
    padding-inline-start: 2px;
    white-space: nowrap;
  }
  .bar,
  .float {
    position: absolute;
    top: 0.2rem;
    height: 0.9rem;
    border-radius: 3px;
  }
  .bar {
    background: var(--accent, #35547c);
    min-width: 3px;
  }
  .bar.critical {
    background: #a4262c;
    outline: 2px solid #a4262c;
    outline-offset: 1px;
  }
  .bar.near {
    background: repeating-linear-gradient(90deg, #b45309 0 6px, #d97706 6px 12px);
  }
  .bar.late {
    background: #7f1d1d;
    outline: 2px dashed #7f1d1d;
    outline-offset: 1px;
  }
  .float {
    background: repeating-linear-gradient(135deg, var(--accent, #35547c) 0 2px, transparent 2px 6px);
    opacity: 0.6;
    border: 1px solid var(--accent, #35547c);
  }
  .limit {
    position: absolute;
    top: 0;
    bottom: 0;
    border-inline-start: 3px double #7f1d1d;
  }
  .sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    overflow: hidden;
    clip-path: inset(50%);
    white-space: nowrap;
  }
  @media (max-width: 600px) {
    .row,
    .axis {
      grid-template-columns: 1fr;
    }
  }
</style>
