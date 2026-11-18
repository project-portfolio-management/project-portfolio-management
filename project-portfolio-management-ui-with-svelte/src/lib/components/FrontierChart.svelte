<!--
  The efficient frontier as a picture (`/scenarios`, NC-1 slice 12 part g, NC-7): the cost of what the generator
  chooses (across) against its value, the summed Smart Score (up), one mark per budget cap. A point that buys more
  value is a filled circle on the line; one that buys no more than a cheaper cap is a hollow square, said in words in
  the caption and the legend. The same figures are in the page's table, its twin.
-->
<script lang="ts">
  import { t, tn } from "#lib/i18n.svelte.js";
  import { formatDecimal } from "#lib/locale-format.js";
  import { money, type FrontierPoint } from "#lib/api/ppm.js";
  import { FRONTIER_BOX, bestPoint, frontierLine, frontierPlot } from "#lib/frontier-view.js";

  let { points, currency, id }: { points: FrontierPoint[]; currency: string; id: string } = $props();

  const box = FRONTIER_BOX;
  const placed = $derived(frontierPlot(points, box));
  const line = $derived(frontierLine(placed));
  const best = $derived(bestPoint(points));
  const maxCost = $derived(Math.max(0, ...points.map((p) => p.cost_minor)));
  const maxValue = $derived(Math.max(0, ...points.map((p) => p.value)));
  const dominated = $derived(points.filter((p) => p.dominated).length);
  const caption = $derived(
    best
      ? [
          tn("p.frontier.chart_points", points.length, { n: points.length }),
          t("p.frontier.chart_best", { value: formatDecimal(best.value, 1), cost: money(best.cost_minor, currency) }),
          tn("p.frontier.chart_dominated", dominated, { n: dominated }),
        ].join(" ")
      : t("p.frontier.no_points"),
  );
</script>

<figure class="frontier" data-testid="frontier-chart">
  <figcaption id={`${id}-cap`}>{caption}</figcaption>
  {#if placed.length > 0}
    <svg viewBox={`0 0 ${box.width} ${box.height}`} role="img" aria-labelledby={`${id}-cap`}>
      <line x1={box.left} y1={box.height - box.bottom} x2={box.width - box.right} y2={box.height - box.bottom} class="axisline" />
      <line x1={box.left} y1={box.top} x2={box.left} y2={box.height - box.bottom} class="axisline" />
      <text x={box.left} y={box.height - box.bottom + 13} class="tick" text-anchor="start">0</text>
      <text x={box.width - box.right} y={box.height - box.bottom + 13} class="tick" text-anchor="end">{money(maxCost, currency)}</text>
      <text x={box.left - 5} y={box.height - box.bottom} class="tick" text-anchor="end">0</text>
      <text x={box.left - 5} y={box.top + 8} class="tick" text-anchor="end">{formatDecimal(maxValue, 0)}</text>
      <text x={(box.left + box.width - box.right) / 2} y={box.height - 4} class="axis" text-anchor="middle">{t("p.frontier.axis_cost")}</text>
      <text transform={`translate(10 ${(box.top + box.height - box.bottom) / 2}) rotate(-90)`} class="axis" text-anchor="middle">{t("p.frontier.axis_value")}</text>
      {#if line}<polyline points={line} class="line" />{/if}
      {#each placed as p (p.point.budget_cap_minor)}
        {#if p.point.dominated}
          <rect x={p.x - 4} y={p.y - 4} width="8" height="8" class="mark dominated" />
        {:else}
          <circle cx={p.x} cy={p.y} r="4.5" class="mark efficient" />
        {/if}
      {/each}
    </svg>
    <p class="small legend"><span aria-hidden="true">●</span> {t("p.frontier.legend_efficient")} · <span aria-hidden="true">□</span> {t("p.frontier.legend_dominated")}</p>
  {/if}
</figure>

<style>
  .frontier { margin: 0 0 1rem; max-inline-size: 30rem; }
  figcaption { font-weight: 600; margin-block-end: 0.25rem; }
  svg { inline-size: 100%; block-size: auto; }
  .axisline { stroke: currentColor; stroke-width: 1; }
  .tick, .axis { font-size: 10px; fill: currentColor; }
  .line { fill: none; stroke: var(--mxi-color-primary); stroke-width: 2; }
  .mark.efficient { fill: var(--mxi-color-primary); }
  .mark.dominated { fill: var(--mxi-color-surface); stroke: var(--mxi-color-danger); stroke-width: 1.5; }
  .legend { margin: 0.25rem 0 0; }
</style>
