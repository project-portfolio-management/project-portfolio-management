<!--
  Chief Risk Officer area (`/risk`): the probability×impact heatmap,
  top exposures, per-portfolio posture, concentration (server-disclosed
  threshold), review hygiene, and the declared risk appetite — or an
  honest "not configured" when no appetite is declared. All numbers
  server-derived. English-first, like the other PPM views.
-->
<script lang="ts">
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
      error = err instanceof Error ? err.message : t("ppm.common.loadFailed");
    }
  });

  const SCALE = [1, 2, 3, 4, 5] as const;
  const cell = (p: number, i: number) => heatmap?.cells[`p${p}i${i}`] ?? 0;
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
  <table data-testid="risk-top">
    <thead><tr><th>{t("p.risk.risk")}</th><th>{t("p.risk.category")}</th><th>{t("p.risk.exposure")}</th><th>{t("p.risk.item")}</th></tr></thead>
    <tbody>
      {#each heatmap.top_risks as row (row.pid)}
        <tr>
          <td>{row.title}</td>
          <td>{row.category}</td>
          <td>{row.exposure}</td>
          <td>{row.item?.name ?? "—"}</td>
        </tr>
      {:else}
        <tr><td colspan="4" class="muted">{t("p.risk.no_open_risks")}</td></tr>
      {/each}
    </tbody>
  </table>

  <h2>{t("p.risk.posture_by_portfolio")}</h2>
  <table data-testid="risk-posture">
    <thead><tr><th>{t("p.risk.portfolio")}</th><th>{t("p.risk.open_exposure")}</th><th>{t("p.risk.escalated")}</th><th>{t("p.risk.materialized")}</th></tr></thead>
    <tbody>
      {#each heatmap.posture as row (row.portfolio?.pid ?? "unassigned")}
        <tr>
          <td>{row.portfolio?.name ?? t("p.risk.unassigned")}</td>
          <td>{row.open_exposure}</td>
          <td>{row.escalated}</td>
          <td>{row.materialized}</td>
        </tr>
      {/each}
    </tbody>
  </table>

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
