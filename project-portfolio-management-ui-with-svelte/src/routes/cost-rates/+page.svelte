<!--
  Cost rates (`/cost-rates`): named rate tables (standard, overtime…) for a person
  or a role, each rate effective from a date. Rates are append-only: a new date
  adds a rate; nothing is edited, so what a day cost is what it was said to cost.
  Finance data: someone without access sees a plain note.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { ApiError } from "#lib/api/client.js";
  import { PpmClient, money, type CostRateTable } from "#lib/api/ppm.js";
  import { minorFromMajor } from "#lib/work-view.js";

  const ppm = PpmClient.withFetch();
  let tables = $state<CostRateTable[] | null>(null);
  let denied = $state(false);
  let error = $state<string | null>(null);
  let status = $state("");

  async function load() {
    try {
      tables = (await ppm.costRateTables()).tables;
      error = null;
    } catch (err) {
      const m = err instanceof Error ? err.message : "";
      if (err instanceof ApiError && err.status === 404) denied = true;
      else error = m || "Could not load the rate tables";
    }
  }
  onMount(load);

  let name = $state("standard");
  let scope = $state<"person" | "role">("role");
  let who = $state("");
  let currency = $state("GBP");
  async function addTable(e: SubmitEvent) {
    e.preventDefault();
    try {
      await ppm.createCostRateTable({ name, currency, ...(scope === "person" ? { person_ref: who } : { role: who }) });
      who = "";
      status = "Rate table added.";
      await load();
    } catch (err) {
      status = err instanceof Error ? err.message : "Could not add the table";
    }
  }
  let from = $state<Record<string, string>>({});
  let rate = $state<Record<string, string>>({});
  async function addRate(t: CostRateTable, e: SubmitEvent) {
    e.preventDefault();
    const minor = minorFromMajor(rate[t.pid], true);
    const date = from[t.pid];
    if (minor === null || !date) return void (status = "Give a date and an hourly rate of zero or more.");
    try {
      await ppm.addCostRate(t.pid, { effective_from: date, per_hour_minor: minor });
      rate[t.pid] = "";
      status = "Rate added.";
      await load();
    } catch (err) {
      status = err instanceof Error ? err.message : "Could not add the rate";
    }
  }
</script>

<svelte:head><title>Cost rates — PPM</title></svelte:head>

<h1>Cost rates</h1>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}
{#if denied}
  <p class="muted" data-testid="denied">Cost rates are finance data. You do not have access to them.</p>
{:else}
  {#if tables && tables.length === 0}<p class="muted">No rate tables yet.</p>{/if}
  <ul class="stack tables">
    {#each tables ?? [] as t (t.pid)}
      <li class="surface">
        <h2>{t.name} — {t.role ?? t.person_ref} <span class="muted small">({t.currency})</span></h2>
        {#if t.rates.length === 0}
          <p class="small muted">No rate yet: work by this {t.role ? "role" : "person"} is not costed.</p>
        {:else}
          <table>
            <thead><tr><th scope="col">From</th><th scope="col">Per hour</th></tr></thead>
            <tbody>{#each t.rates as r}<tr><td>{r.effective_from}</td><td>{money(r.per_hour_minor, t.currency)}</td></tr>{/each}</tbody>
          </table>
        {/if}
        <form class="row" onsubmit={(e) => addRate(t, e)}>
          <label>Effective from <input type="date" value={from[t.pid] ?? ""} oninput={(e) => (from[t.pid] = e.currentTarget.value)} /></label>
          <label>Per hour <input type="number" min="0" step="0.01" value={rate[t.pid] ?? ""} oninput={(e) => (rate[t.pid] = e.currentTarget.value)} /></label>
          <button class="button" type="submit">Add rate</button>
        </form>
      </li>
    {/each}
  </ul>

  <h2>Add a rate table</h2>
  <form class="surface stack" onsubmit={addTable}>
    <label>Name <input bind:value={name} required maxlength="40" /></label>
    <label>For
      <select bind:value={scope}><option value="role">A role</option><option value="person">A person</option></select>
    </label>
    <label>{scope === "role" ? "Role" : "Person (person:… reference)"} <input bind:value={who} required /></label>
    <label>Currency <input bind:value={currency} maxlength="3" size="4" /></label>
    <button class="button primary" type="submit">Add table</button>
  </form>
  {#if status}<p role="status" class="small">{status}</p>{/if}
{/if}

<style>
  .tables {
    list-style: none;
    padding: 0;
  }
  h2 {
    font-size: 1.1rem;
  }
  label {
    display: block;
  }
</style>
