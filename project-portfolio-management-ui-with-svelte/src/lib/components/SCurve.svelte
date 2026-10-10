<!--
  The S-curve (SC-5): planned value over the whole baseline, and earned value and
  actual cost as **dots only on the days they were recorded**. Between recorded days
  they are not known, so they are never joined or filled in. A dashed line marks the
  budget at completion. The table under the picture carries every number.
-->
<script lang="ts">
  import { t } from "#lib/i18n.svelte.js";
  import { money, type EvSeries } from "#lib/api/ppm.js";
  import { sCurve } from "#lib/ev-view.js";

  let { curve }: { curve: EvSeries["curves"][number] } = $props();
  const g = $derived(sCurve(curve));
</script>

{#if g}
  <figure data-testid="scurve">
    <svg viewBox="0 0 {g.width} {g.height}" role="img" aria-label={t("c.SCurve.s_curve_planned_value_over")} width="100%" style="max-width: 44rem">
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
      <summary>{t("c.SCurve.the_same_figures_as_a")}</summary>
      <table>
        <thead><tr><th scope="col">{t("c.SCurve.date")}</th><th scope="col">{t("c.SCurve.planned")}</th><th scope="col">{t("c.SCurve.earned")}</th><th scope="col">{t("c.SCurve.actual")}</th></tr></thead>
        <tbody>
          {#each curve.points as p (p.date)}
            <tr>
              <th scope="row">{p.date}</th>
              <td>{money(p.pv_minor, curve.currency)}</td>
              <td>{p.ev_minor === null ? t("c.SCurve.not_recorded") : money(p.ev_minor, curve.currency)}</td>
              <td>{p.ac_minor === null ? t("c.SCurve.not_recorded") : money(p.ac_minor, curve.currency)}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </details>
    <figcaption class="small">
      {t("c.SCurve.line_planned_value_to_earned", { from: g.from, to: g.to, snapshots: curve.snapshots, money: money(curve.bac_minor, curve.currency) })}
    </figcaption>
  </figure>
{:else}
  <p class="muted">{t("c.SCurve.no_curve_to_draw")}</p>
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
