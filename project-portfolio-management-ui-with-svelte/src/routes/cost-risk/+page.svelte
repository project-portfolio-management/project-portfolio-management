<!--
  Portfolio cost risk (`/cost-risk`): the same seeded simulation over every plan
  you can read, together, per currency (never added across), with each plan's mean
  simulated cost beside its budget. Plans you cannot read are not part of it.
-->
<script lang="ts">
  import { localeHref } from "#lib/i18n.svelte.js";
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
      error = err instanceof Error ? err.message : "Could not run the simulation";
    } finally {
      busy = false;
    }
  }
  onMount(() => void run());
</script>

<svelte:head><title>Portfolio cost risk — PPM</title></svelte:head>

<h1>Portfolio cost risk</h1>
<p class="small muted">
  What all your plans together could cost, given each budget line's range and the risks that add cost. Each currency
  is on its own; they are never added. <a href={localeHref("/resources")}>Resources</a> shows people instead.
</p>

{#if error}<p class="banner" role="alert">{error}</p>{/if}

<form class="filter" onsubmit={(e) => { e.preventDefault(); void run(); }}>
  <label>Iterations <input type="number" min="1" max="50000" step="1" bind:value={iterations} /></label>
  <label>Seed <input type="number" min="0" step="1" bind:value={seed} /></label>
  <label>Distribution
    <select bind:value={distribution}><option value="triangular">Triangular</option><option value="pert">PERT</option></select>
  </label>
  <label>Band for lines with no estimate (± %) <input type="number" min="0" max="100" step="1" bind:value={band} /></label>
  <button class="button small primary" type="submit" disabled={busy}>{busy ? "Running…" : "Run"}</button>
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
