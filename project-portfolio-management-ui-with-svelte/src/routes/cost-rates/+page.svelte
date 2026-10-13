<!--
  Cost rates (`/cost-rates`): named rate tables (standard, overtime…) for a person
  or a role, each rate effective from a date. Rates are append-only: a new date
  adds a rate; nothing is edited, so what a day cost is what it was said to cost.
  Finance data: someone without access sees a plain note.
-->
<script lang="ts">
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t } from "#lib/i18n.svelte.js";
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
      else error = m || t("p.cost_rates.could_not_load_the_rate");
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
      status = t("p.cost_rates.rate_table_added");
      await load();
    } catch (err) {
      status = err instanceof Error ? err.message : t("p.cost_rates.could_not_add_the_table");
    }
  }
  let from = $state<Record<string, string>>({});
  let rate = $state<Record<string, string>>({});
  async function addRate(table: CostRateTable, e: SubmitEvent) {
    e.preventDefault();
    const minor = minorFromMajor(rate[table.pid], true);
    const date = from[table.pid];
    if (minor === null || !date) return void (status = t("p.cost_rates.give_a_date_and_an"));
    try {
      await ppm.addCostRate(table.pid, { effective_from: date, per_hour_minor: minor });
      rate[table.pid] = "";
      status = t("p.cost_rates.rate_added");
      await load();
    } catch (err) {
      status = err instanceof Error ? err.message : t("p.cost_rates.could_not_add_the_rate");
    }
  }

  const rateColumns = (currency: string): DataGridColumn[] => [
    { id: "effective_from", header: t("p.cost_rates.from"), rowHeader: true },
    { id: "per_hour_minor", header: t("p.cost_rates.per_hour"), align: "end", format: (v) => money(Number(v), currency) },
  ];
</script>

<svelte:head><title>{t("p.cost_rates.cost_rates_ppm")}</title></svelte:head>

<h1>{t("p.cost_rates.cost_rates")}</h1>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}
{#if denied}
  <p class="muted" data-testid="denied">{t("p.cost_rates.cost_rates_are_finance_data")}</p>
{:else}
  {#if tables && tables.length === 0}<p class="muted">{t("p.cost_rates.no_rate_tables_yet")}</p>{/if}
  <ul class="stack tables">
    {#each tables ?? [] as row (row.pid)}
      <li class="surface">
        <h2>{row.name} — {row.role ?? row.person_ref} <span class="muted small">({row.currency})</span></h2>
        {#if row.rates.length === 0}
          <p class="small muted">{t("p.cost_rates.no_rate_yet_work_by", { v: row.role ? t("p.common.role_word") : t("p.common.person_word") })}</p>
        {:else}
          <DataTable label={`${row.name} — ${row.role ?? row.person_ref}`} columns={rateColumns(row.currency)} rows={row.rates} rowId={(r) => String(r.effective_from)} />
        {/if}
        <form class="row" onsubmit={(e) => addRate(row, e)}>
          <label>{t("p.cost_rates.effective_from")} <input type="date" value={from[row.pid] ?? ""} oninput={(e) => (from[row.pid] = e.currentTarget.value)} /></label>
          <label>{t("p.cost_rates.per_hour")} <input type="number" min="0" step="0.01" value={rate[row.pid] ?? ""} oninput={(e) => (rate[row.pid] = e.currentTarget.value)} /></label>
          <button class="button" type="submit">{t("p.cost_rates.add_rate")}</button>
        </form>
      </li>
    {/each}
  </ul>

  <h2>{t("p.cost_rates.add_a_rate_table")}</h2>
  <form class="surface stack" onsubmit={addTable}>
    <label>{t("p.cost_rates.name")} <input bind:value={name} required maxlength="40" /></label>
    <label>{t("p.cost_rates.for")}
      <select bind:value={scope}><option value="role">{t("p.cost_rates.a_role")}</option><option value="person">{t("p.cost_rates.a_person")}</option></select>
    </label>
    <label>{scope === "role" ? t("p.cost_rates.role") : t("p.cost_rates.person_person_reference")} <input bind:value={who} required /></label>
    <label>{t("p.cost_rates.currency")} <input bind:value={currency} maxlength="3" size="4" /></label>
    <button class="button primary" type="submit">{t("p.cost_rates.add_table")}</button>
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
