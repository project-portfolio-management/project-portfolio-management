<!--
  Cost risk (`/plans/[pid]/cost-risk`): a seeded simulation of the plan's final
  cost from its budget lines and register risks, per currency. See the shared
  `CostRiskView` for what it shows and what it does not claim.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { PpmClient, type CostRisk } from "#lib/api/ppm.js";
  import CostRiskView from "#lib/components/CostRiskView.svelte";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");
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
      result = await ppm.costRisk(pid, { iterations, seed, distribution, fallbackBandPercent: band });
      error = null;
    } catch (err) {
      error = errorText(err, "p.plans.pid.cost_risk.could_not_run_the_simulation");
    } finally {
      busy = false;
    }
  }
  onMount(() => void run());
</script>

<svelte:head><title>{t("p.plans.pid.cost_risk.cost_risk_ppm")}</title></svelte:head>

<h1>{t("p.plans.pid.cost_risk.cost_risk")}</h1>
<p class="lede">{t("p.plans.pid.cost_risk.many_what_if_runs_of")}</p>
<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plans.pid.cost_risk.plan")}</a> · <a href={localeHref(`/plans/${pid}/joint-risk`)}>{t("p.plans.pid.cost_risk.cost_and_schedule_together")}</a> · <a href={localeHref("/cost-risk")}>{t("p.plans.pid.cost_risk.whole_portfolio")}</a></p>

{#if error}<p class="banner" role="alert">{error}</p>{/if}

<form class="filter" onsubmit={(e) => { e.preventDefault(); void run(); }}>
  <label>{t("p.plans.pid.cost_risk.iterations")} <input type="number" min="1" max="50000" step="1" bind:value={iterations} /></label>
  <label>{t("p.plans.pid.cost_risk.seed")} <input type="number" min="0" step="1" bind:value={seed} /></label>
  <label>{t("p.plans.pid.cost_risk.distribution")}
    <select bind:value={distribution}><option value="triangular">{t("p.plans.pid.cost_risk.triangular")}</option><option value="pert">{t("p.plans.pid.cost_risk.pert")}</option></select>
  </label>
  <label>{t("p.plans.pid.cost_risk.band_for_lines_with_no")} <input type="number" min="0" max="100" step="1" bind:value={band} /></label>
  <button class="button small primary" type="submit" disabled={busy}>{busy ? t("p.plans.pid.cost_risk.running") : t("p.plans.pid.cost_risk.run")}</button>
</form>

{#if result}<CostRiskView {result} />{:else if !error}<p class="muted">{t("p.plans.pid.cost_risk.running")}</p>{/if}

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
