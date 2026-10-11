<!--
  One buffer's fever chart (CC-6): chain completed (x) against buffer used (y), with the green, amber and
  red zones from the configured lines. The zone is a word, a shape and a colour: the point is a circle
  (green), a diamond (amber) or a triangle (red), the zones are labelled in words, and the caption says it
  all in a sentence. A figure beyond the chart is drawn on its edge and the caption says so. The same
  figures are in the page's table.
-->
<script lang="ts">
  import { t } from "#lib/i18n.svelte.js";
  import type { BufferReading, FeverLines } from "#lib/api/ppm.js";
  import { bands, FEVER_BOX, markerPath, toPixel, zoneLook } from "#lib/chain-view.js";

  let { reading, lines, label, id }: { reading: BufferReading; lines: FeverLines; label: string; id: string } = $props();

  const box = FEVER_BOX;
  const zones = $derived(bands(box, lines));
  const p = $derived(reading.point);
  const look = $derived(zoneLook(p?.zone));
  const at = $derived(p ? toPixel(box, p.plotted_x_pct, p.plotted_y_pct) : null);
  const ticks = [0, 50, 100];
  const caption = $derived(
    p
      ? t("p.critical_chain.chart_caption", {
          buffer: label,
          zone: `${look.symbol} ${t(look.word as never)}`,
          consumed: Math.round(p.buffer_consumed_pct),
          done: Math.round(p.chain_completed_pct),
        })
      : t("p.critical_chain.no_point", { reason: reading.no_point_reason ?? "" }),
  );
</script>

<figure class="fever" data-testid="fever-chart">
  <figcaption id={`${id}-cap`}>{caption}</figcaption>
  {#if p && at}
    <svg viewBox={`0 0 ${box.width} ${box.height}`} role="img" aria-labelledby={`${id}-cap`}>
      <polygon points={zones.green} class="zone green" />
      <polygon points={zones.amber} class="zone amber" />
      <polygon points={zones.red} class="zone red" />
      <text x={toPixel(box, 50, 5)[0]} y={toPixel(box, 50, 5)[1]} class="zword" text-anchor="middle">{t("p.critical_chain.zone_green")}</text>
      <text x={toPixel(box, 62, 62)[0]} y={toPixel(box, 62, 62)[1]} class="zword" text-anchor="middle">{t("p.critical_chain.zone_amber")}</text>
      <text x={toPixel(box, 28, 92)[0]} y={toPixel(box, 28, 92)[1]} class="zword" text-anchor="middle">{t("p.critical_chain.zone_red")}</text>
      {#each ticks as v (v)}
        <text x={toPixel(box, v, 0)[0]} y={box.height - box.bottom + 13} class="tick" text-anchor="middle">{v}</text>
        <text x={box.left - 5} y={toPixel(box, 0, v)[1] + 3} class="tick" text-anchor="end">{v}</text>
      {/each}
      <text x={(box.left + box.width - box.right) / 2} y={box.height - 4} class="axis" text-anchor="middle">{t("p.critical_chain.axis_x")}</text>
      <text transform={`translate(10 ${(box.top + box.height - box.bottom) / 2}) rotate(-90)`} class="axis" text-anchor="middle">{t("p.critical_chain.axis_y")}</text>
      <path d={markerPath(p.zone, at[0], at[1])} class={`mark ${p.zone}`} />
    </svg>
    {#if p.edge}<p class="small muted">{t("p.critical_chain.beyond_edge")}</p>{/if}
  {/if}
</figure>

<style>
  .fever { margin: 0 0 1rem; max-inline-size: 22rem; }
  figcaption { font-weight: 600; margin-block-end: 0.25rem; }
  svg { inline-size: 100%; block-size: auto; }
  /* Pale zone tints keep the words readable; the shape and the word carry the meaning. */
  .zone.green { fill: #e3f4e3; }
  .zone.amber { fill: #fff4d6; }
  .zone.red { fill: #fde8e8; }
  .zword { font-size: 10px; fill: #222; }
  .tick, .axis { font-size: 9px; fill: #333; }
  .mark { stroke: #111; stroke-width: 1.5; }
  .mark.green { fill: #2e7d32; }
  .mark.amber { fill: #f9a825; }
  .mark.red { fill: #c62828; }
</style>
