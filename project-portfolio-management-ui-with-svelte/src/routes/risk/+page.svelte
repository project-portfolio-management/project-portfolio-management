<!--
  Chief Risk Officer area (`/risk`): the probability×impact heatmap,
  top exposures, per-portfolio posture, concentration (server-disclosed
  threshold), review hygiene, and the declared risk appetite — or an
  honest "not configured" when no appetite is declared. All numbers
  server-derived. English-first, like the other PPM views.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient, type RiskHeatmap } from "#lib/api/ppm.js";

  const ppm = PpmClient.withFetch();
  let heatmap = $state<RiskHeatmap | null>(null);
  let error = $state<string | null>(null);

  onMount(async () => {
    try {
      heatmap = await ppm.riskHeatmap();
    } catch (err) {
      error = errorText(err, "ppm.common.loadFailed");
    }
  });

  const SCALE = [1, 2, 3, 4, 5] as const;
  const cell = (p: number, i: number) => heatmap?.cells[`p${p}i${i}`] ?? 0;

  type Top = RiskHeatmap["top_risks"][number];
  type Post = RiskHeatmap["posture"][number];
  const topColumns: DataGridColumn[] = $derived([
    { id: "title", header: t("p.risk.risk"), rowHeader: true },
    { id: "category", header: t("p.risk.category") },
    { id: "exposure", header: t("p.risk.exposure"), align: "end" },
    { id: "item", header: t("p.risk.item"), accessor: (r) => (r as unknown as Top).item?.name ?? "—" },
  ]);
  const postureColumns: DataGridColumn[] = $derived([
    { id: "portfolio", header: t("p.risk.portfolio"), rowHeader: true, accessor: (r) => (r as unknown as Post).portfolio?.name ?? t("p.risk.unassigned") },
    { id: "open_exposure", header: t("p.risk.open_exposure"), align: "end" },
    { id: "escalated", header: t("p.risk.escalated"), align: "end" },
    { id: "materialized", header: t("p.risk.materialized"), align: "end" },
  ]);
</script>

<svelte:head><title>{t("p.risk.ppm", { t: t("ppm.nav.risk") })}</title></svelte:head>

<h1>{t("ppm.nav.risk")}</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if heatmap}
  <p data-testid="risk-summary">
    <strong>{heatmap.open_risks}</strong> {t("p.risk.open_risks_estate_open_exposure")} <strong>{heatmap.estate_open_exposure}</strong>
  </p>
  <p class="muted" data-testid="risk-appetite">
    {heatmap.appetite_note}
    {#if heatmap.appetite}
      {t("p.risk.max_estate_max_single_risk", { v: heatmap.appetite.max_open_exposure ?? "—", v2: heatmap.appetite.max_item_exposure ?? "—" })}
    {/if}
  </p>
  {#if heatmap.breaches.length > 0}
    <ul class="banner" data-testid="risk-breaches">
      {#each heatmap.breaches as breach, index (index)}
        <li><code>{breach.rule}</code></li>
      {/each}
    </ul>
  {/if}

  <h2>{t("p.risk.heatmap")}</h2>
  <table class="matrix" data-testid="risk-matrix">
    <thead>
      <tr><th>{t("p.risk.probability_impact")}</th>{#each SCALE as impact (impact)}<th>{impact}</th>{/each}</tr>
    </thead>
    <tbody>
      {#each [...SCALE].reverse() as probability (probability)}
        <tr>
          <th>{probability}</th>
          {#each SCALE as impact (impact)}
            <td class:hot={probability * impact >= 15} class:warm={probability * impact >= 8 && probability * impact < 15}>
              {cell(probability, impact) || ""}
            </td>
          {/each}
        </tr>
      {/each}
    </tbody>
  </table>

  <h2>{t("p.risk.top_risks")}</h2>
  <DataTable label={t("p.risk.risk")} columns={topColumns} rows={heatmap.top_risks} rowId={(r) => String(r.pid)} emptyText={t("p.risk.no_open_risks")} testid="risk-top" />

  <h2>{t("p.risk.posture_by_portfolio")}</h2>
  <DataTable label={t("p.risk.portfolio")} columns={postureColumns} rows={heatmap.posture} rowId={(r) => String((r as unknown as Post).portfolio?.pid ?? "unassigned")} testid="risk-posture" />

  {#if heatmap.concentration.length > 0}
    <h2>{t("p.risk.concentration")}</h2>
    <ul data-testid="risk-concentration">
      {#each heatmap.concentration as row (row.pid)}
        <li><strong>{row.title}</strong> {t("p.risk.exposure_25_of_estate", { exposure: row.exposure })}</li>
      {/each}
    </ul>
  {/if}

  {#if heatmap.overdue_reviews.length > 0}
    <h2>{t("p.risk.overdue_reviews")}</h2>
    <ul data-testid="risk-overdue">
      {#each heatmap.overdue_reviews as row (row.pid)}
        <li>{row.title} <span class="muted">{t("p.risk.review_due", { review_date: row.review_date })}</span></li>
      {/each}
    </ul>
  {/if}
{/if}

<style>
  .matrix td { text-align: center; min-width: 2.5rem; }
  .matrix td.warm { background: color-mix(in srgb, #b45309 25%, transparent); }
  .matrix td.hot { background: color-mix(in srgb, #b91c1c 30%, transparent); }
</style>
