<!--
  OKRs (`/okrs`): objectives and their key results across the portfolio, with
  scores derived on the server. Least progressed first; an objective nobody can
  measure yet says "Not measured" and sorts last — never 0%. Confidence sits
  beside a key result and is not part of any score.

  English-first, like the other recent views.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { PpmClient, type OkrList } from "#lib/api/ppm.js";
  import ObjectiveCard from "#lib/components/ObjectiveCard.svelte";

  const ppm = PpmClient.withFetch();
  let list = $state<OkrList | null>(null);
  let error = $state<string | null>(null);
  let period = $state("");

  async function refresh() {
    try {
      list = await ppm.okrs(period.trim() || undefined);
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : "Could not load the OKRs";
    }
  }
  onMount(refresh);

  let title = $state("");
  let newPeriod = $state("");
  async function addObjective(e: SubmitEvent) {
    e.preventDefault();
    try {
      await ppm.createObjective({ title, period: newPeriod.trim() || null });
      title = "";
      await refresh();
    } catch (err) {
      error = err instanceof Error ? err.message : "Could not create the objective";
    }
  }
</script>

<svelte:head><title>OKRs — PPM</title></svelte:head>

<h1>Goals and key results (OKRs)</h1>
<p class="lede">A goal, with a few measurable results that show whether you reached it.</p>
<p class="small muted">
  Objectives and key results. Scores are worked out from the check-ins every time you look;
  nothing is stored that could go stale.
</p>

{#if error}<p class="banner" role="alert">{error}</p>{/if}

<form class="filter" onsubmit={(e) => { e.preventDefault(); void refresh(); }}>
  <label>Period <input type="text" bind:value={period} placeholder="2026-Q4 (blank for all)" /></label>
  <button class="button small" type="submit">Show</button>
</form>

<details class="surface">
  <summary>New objective</summary>
  <form class="filter" onsubmit={addObjective}>
    <label>Title <input type="text" required bind:value={title} /></label>
    <label>Period <input type="text" bind:value={newPeriod} placeholder="2026-Q4" /></label>
    <button class="button small primary" type="submit">Create</button>
  </form>
</details>

{#if list}
  {#if list.objectives.length === 0}
    <p>No objectives{list.period ? ` for ${list.period}` : ""} yet.</p>
  {:else}
    <p class="small muted" aria-live="polite">{list.objectives.length} objective{list.objectives.length === 1 ? "" : "s"}, least progressed first</p>
    {#each list.objectives as item (item.objective.pid)}
      <ObjectiveCard {item} {ppm} onchange={refresh} onerror={(m) => (error = m)} />
    {/each}
  {/if}
{:else if !error}
  <p class="muted">Loading…</p>
{/if}

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
