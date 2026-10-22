<!--
  Portfolio cost risk (`/cost-risk`): the same seeded simulation over every plan
  you can read, together, per currency (never added across), with each plan's mean
  simulated cost beside its budget. Plans you cannot read are not part of it.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient, type CostRisk } from "#lib/api/ppm.js";
  import CostRiskView from "#lib/components/CostRiskView.svelte";

  const ppm = PpmClient.withFetch();
  let result = $state<CostRisk | null>(null);
  let error = $state<string | null>(null);
  let iterations = $state(5000);
  let seed = $state(1);
  let distribution = $state<"triangular" | "pert">("triangular");
  let band = $state(20);
  let busy = $state(false);

  async function run() {
    busy = true;
    try {
      result = await ppm.portfolioCostRisk({ iterations, seed, distribution, fallbackBandPercent: band });
      error = null;
    } catch (err) {
      error = errorText(err, "p.cost_risk.could_not_run_the_simulation");
    } finally {
      busy = false;
    }
  }
  onMount(() => void run());
</script>

<svelte:head><title>{t("p.cost_risk.portfolio_cost_risk_ppm")}</title></svelte:head>

<h1>{t("p.cost_risk.portfolio_cost_risk")}</h1>
<p class="small muted">
  {t("p.cost_risk.what_all_your_plans_together")} <a href={localeHref("/resources")}>{t("p.cost_risk.resources")}</a> {t("p.cost_risk.shows_people_instead")}
</p>

{#if error}<p class="banner" role="alert">{error}</p>{/if}

<form class="filter" onsubmit={(e) => { e.preventDefault(); void run(); }}>
  <label>{t("p.cost_risk.iterations")} <input type="number" min="1" max="50000" step="1" bind:value={iterations} /></label>
  <label>{t("p.cost_risk.seed")} <input type="number" min="0" step="1" bind:value={seed} /></label>
  <label>{t("p.cost_risk.distribution")}
    <select bind:value={distribution}><option value="triangular">{t("p.cost_risk.triangular")}</option><option value="pert">{t("p.cost_risk.pert")}</option></select>
  </label>
  <label>{t("p.cost_risk.band_for_lines_with_no")} <input type="number" min="0" max="100" step="1" bind:value={band} /></label>
  <button class="button small primary" type="submit" disabled={busy}>{busy ? t("p.cost_risk.running") : t("p.cost_risk.run")}</button>
</form>

{#if result}<CostRiskView {result} />{:else if !error}<p class="muted">{t("p.cost_risk.running")}</p>{/if}

<style>
  .filter {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: end;
    margin: 0.75rem 0;
  }
  .filter label {
    display: grid;
    gap: 0.2rem;
    font-size: 0.85rem;
  }
</style>
