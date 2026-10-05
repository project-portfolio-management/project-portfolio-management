<!--
  KPIs (`/kpis`): standing measures with a target and a red line. Status comes
  from the latest reading and is worked out each time — never stored. "No data
  yet" is its own state, not red and not green; status is a word and a shape,
  never colour alone.

  English-first, like the other recent views.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { PpmClient, type KpiList, type KpiStatus, type KpiDirection, type KpiCadence } from "#lib/api/ppm.js";
  import KpiCard from "#lib/components/KpiCard.svelte";
  import { STATUS_SYMBOL, summaryParts } from "#lib/kpi-view.js";

  const ppm = PpmClient.withFetch();
  let list = $state<KpiList | null>(null);
  let error = $state<string | null>(null);
  let status = $state<KpiStatus | "">("");

  async function refresh() {
    try {
      list = await ppm.kpis(status ? { status } : {});
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : "Could not load the KPIs";
    }
  }
  onMount(refresh);

  let name = $state("");
  let unit = $state("");
  let direction = $state<KpiDirection>("higher_better");
  let target = $state("");
  let red = $state("");
  let cadence = $state<KpiCadence | "">("");
  let planPid = $state("");
  async function addKpi(e: SubmitEvent) {
    e.preventDefault();
    try {
      await ppm.createKpi({
        name,
        unit: unit.trim() || null,
        direction,
        target_value: Number(target),
        red_value: Number(red),
        cadence: cadence || null,
        plan_pid: planPid.trim() || null,
      });
      name = "";
      target = "";
      red = "";
      await refresh();
    } catch (err) {
      error = err instanceof Error ? err.message : "Could not define the KPI";
    }
  }
</script>

<svelte:head><title>KPIs — PPM</title></svelte:head>

<h1>KPIs</h1>
<p class="small muted">
  Standing measures. Green is at or better than the target; red is at or worse than the red
  line; amber is between. A KPI nobody has read yet shows “No data yet”.
</p>

{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if list}
  <ul class="summary" aria-label="KPIs by status">
    {#each summaryParts(list.summary) as part (part.status)}
      <li><span aria-hidden="true">{STATUS_SYMBOL[part.status]}</span> {part.label}: <strong>{part.count}</strong></li>
    {/each}
  </ul>
{/if}

<form class="filter" onsubmit={(e) => { e.preventDefault(); void refresh(); }}>
  <label>Show
    <select bind:value={status} onchange={refresh}>
      <option value="">All</option>
      <option value="red">Off target</option>
      <option value="amber">Watch</option>
      <option value="no_data">No data yet</option>
      <option value="green">On target</option>
    </select>
  </label>
</form>

<details class="surface">
  <summary>Define a KPI</summary>
  <form class="filter" onsubmit={addKpi}>
    <label>Name <input type="text" required bind:value={name} /></label>
    <label>Unit <input type="text" bind:value={unit} placeholder="percent, defects, GBP…" /></label>
    <label>Which way is good
      <select bind:value={direction}>
        <option value="higher_better">Higher is better</option>
        <option value="lower_better">Lower is better</option>
      </select>
    </label>
    <label>Target <input type="number" step="1" required bind:value={target} /></label>
    <label>Red line <input type="number" step="1" required bind:value={red} /></label>
    <label>Read how often
      <select bind:value={cadence}>
        <option value="">Not set</option>
        <option value="daily">Daily</option>
        <option value="weekly">Weekly</option>
        <option value="monthly">Monthly</option>
        <option value="quarterly">Quarterly</option>
      </select>
    </label>
    <label>Plan (optional pid) <input type="text" bind:value={planPid} placeholder="blank for the whole portfolio" /></label>
    <button class="button small" type="submit">Define</button>
  </form>
  <p class="small muted">The red line must be on the worse side of the target. Scale values to whole numbers (basis points, pence).</p>
</details>

{#if list}
  {#if list.kpis.length === 0}
    <p>No KPIs{status ? " with that status" : ""} yet.</p>
  {:else}
    {#each list.kpis as item (item.kpi.pid)}
      <KpiCard {item} {ppm} onchange={refresh} onerror={(m) => (error = m)} />
    {/each}
  {/if}
{:else if !error}
  <p class="muted">Loading…</p>
{/if}

<style>
  .summary {
    display: flex;
    flex-wrap: wrap;
    gap: 1.25rem;
    list-style: none;
    padding: 0;
  }
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
