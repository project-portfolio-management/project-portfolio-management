<!--
  The S-curve (SC-5): planned value over the whole baseline, and earned value and
  actual cost as **dots only on the days they were recorded**. Between recorded days
  they are not known, so they are never joined or filled in. A dashed line marks the
  budget at completion. The table under the picture carries every number.
-->
<script lang="ts">
  import { money, type EvSeries } from "#lib/api/ppm.js";
  import { sCurve } from "#lib/ev-view.js";

  let { curve }: { curve: EvSeries["curves"][number] } = $props();
  const g = $derived(sCurve(curve));
</script>

{#if g}
  <figure data-testid="scurve">
    <svg viewBox="0 0 {g.width} {g.height}" role="img" aria-label="S-curve: planned value over time, with recorded earned value and actual cost" width="100%" style="max-width: 44rem">
      <line x1="0" x2={g.width} y1={g.bacY} y2={g.bacY} class="bac" />
      <polyline points={g.pv} class="pv" fill="none" />
      {#each g.ev as d (d.label)}
        <circle cx={d.x} cy={d.y} r="5" class="ev"><title>{d.label}</title></circle>
      {/each}
      {#each g.ac as d (d.label)}
        <rect x={d.x - 4.5} y={d.y - 4.5} width="9" height="9" class="ac"><title>{d.label}</title></rect>
      {/each}
    </svg>
    <details>
      <summary>The same figures as a table</summary>
      <table>
        <thead><tr><th scope="col">Date</th><th scope="col">Planned</th><th scope="col">Earned</th><th scope="col">Actual</th></tr></thead>
        <tbody>
          {#each curve.points as p (p.date)}
            <tr>
              <th scope="row">{p.date}</th>
              <td>{money(p.pv_minor, curve.currency)}</td>
              <td>{p.ev_minor === null ? "not recorded" : money(p.ev_minor, curve.currency)}</td>
              <td>{p.ac_minor === null ? "not recorded" : money(p.ac_minor, curve.currency)}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </details>
    <figcaption class="small">
      Line: planned value ({g.from} to {g.to}). ● earned value and ■ actual cost, only on days recorded ({curve.snapshots} so far).
      Dashed: budget at completion, {money(curve.bac_minor, curve.currency)}.
    </figcaption>
  </figure>
{:else}
  <p class="muted">No curve to draw.</p>
{/if}

<style>
  svg {
    color: var(--mxi-color-fg, #333);
    background: var(--surface-2, #f7f7f7);
  }
  .pv {
    stroke: #35547c;
    stroke-width: 2.5;
  }
  .ev {
    fill: #1d8a4e;
  }
  .ac {
    fill: #b45309;
  }
  .bac {
    stroke: currentColor;
    stroke-dasharray: 6 4;
    stroke-width: 1;
  }
  table {
    border-collapse: collapse;
    font-size: 0.85em;
  }
  th,
  td {
    border: 1px solid var(--border, #ccc);
    padding: 0.2rem 0.5rem;
    text-align: start;
  }
</style>
