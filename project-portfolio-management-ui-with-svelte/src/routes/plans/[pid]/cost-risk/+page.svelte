<!--
  Cost risk (`/plans/[pid]/cost-risk`): a seeded simulation of the plan's final
  cost from its budget lines and register risks, per currency. See the shared
  `CostRiskView` for what it shows and what it does not claim.
-->
<script lang="ts">
  import { localeHref } from "#lib/i18n.svelte.js";
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
      error = err instanceof Error ? err.message : "Could not run the simulation";
    } finally {
      busy = false;
    }
  }
  onMount(() => void run());
</script>

<svelte:head><title>Cost risk — PPM</title></svelte:head>

<h1>Cost risk</h1>
<p class="small"><a href={localeHref(`/plans/${pid}`)}>← Plan</a> · <a href={localeHref(`/plans/${pid}/joint-risk`)}>Cost and schedule together</a> · <a href={localeHref("/cost-risk")}>Whole portfolio</a></p>

{#if error}<p class="banner" role="alert">{error}</p>{/if}

<form class="filter" onsubmit={(e) => { e.preventDefault(); void run(); }}>
  <label>Iterations <input type="number" min="1" max="50000" step="1" bind:value={iterations} /></label>
  <label>Seed <input type="number" min="0" step="1" bind:value={seed} /></label>
  <label>Distribution
    <select bind:value={distribution}><option value="triangular">Triangular</option><option value="pert">PERT</option></select>
  </label>
  <label>Band for lines with no estimate (± %) <input type="number" min="0" max="100" step="1" bind:value={band} /></label>
  <button class="button small" type="submit" disabled={busy}>{busy ? "Running…" : "Run"}</button>
</form>

{#if result}<CostRiskView {result} />{:else if !error}<p class="muted">Running…</p>{/if}

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
