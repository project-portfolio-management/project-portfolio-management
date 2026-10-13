<!--
  Resource capacity (`/capacity`, PPM-8): the per-person rollup over
  a window; summed percent over 100 flags over-allocation.
-->
<script lang="ts">
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient, type CapacityView } from "#lib/api/ppm.js";
  import CapacityMeter from "#lib/components/CapacityMeter.svelte";

  const ppm = PpmClient.withFetch();
  let view = $state<CapacityView | null>(null);
  let error = $state<string | null>(null);
  let from = $state("");
  let to = $state("");

  async function refresh() {
    try {
      view = await ppm.capacity(from || undefined, to || undefined);
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : t("ppm.common.loadFailed");
    }
  }
  onMount(refresh);

  type Person = CapacityView["people"][number];
  const columns: DataGridColumn[] = $derived([
    { id: "person_ref", header: t("ppm.capacity.person"), rowHeader: true },
    { id: "allocated_percent", header: t("ppm.capacity.allocated"), align: "end", format: (v) => `${v}%`, cell: allocCell, cellClass: (c) => ((c.row as unknown as Person).over_allocated ? "cell-red" : "") },
    { id: "proposed", header: t("p.capacity.proposed"), align: "end", accessor: (r) => (r as unknown as Person).proposed.percent, format: (_v, r) => ((r as unknown as Person).proposed.allocations > 0 ? `${(r as unknown as Person).proposed.percent}%` : "—") },
    { id: "load", header: t("ppm.capacity.load"), sortable: false, accessor: (r) => (r as unknown as Person).allocated_percent, cell: loadCell },
    { id: "allocations", header: t("ppm.nav.capacity"), align: "end" },
  ]);
</script>

{#snippet allocCell({ formatted, row }: DataGridCellContext)}
  {formatted}
  {#if (row as unknown as Person).over_allocated}<span class="chip red">{t("p.capacity.over")}</span>{/if}
{/snippet}

{#snippet loadCell({ row }: DataGridCellContext)}
  {#if view}<CapacityMeter person={row as unknown as Person} from={view.from} to={view.to} />{/if}
{/snippet}


<svelte:head><title>{t("p.capacity.capacity_ppm")}</title></svelte:head>

<h1>{t("ppm.capacity.title")}</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

<form
  class="row"
  onsubmit={(e) => {
    e.preventDefault();
    refresh();
  }}
>
  <label class="small">{t("p.capacity.from")} <input type="date" bind:value={from} /></label>
  <label class="small">{t("p.capacity.to")} <input type="date" bind:value={to} /></label>
  <button class="button small primary" type="submit">{t("ppm.capacity.apply")}</button>
</form>

{#if view}
  <p class="small muted">{t("p.capacity.window", { from: view.from, to: view.to })}</p>
  <ul class="legend small" aria-label={t("p.capacity.legend")}>
    <li><span class="mk">○</span> {t("p.capacity.within_capacity")}</li>
    <li><span class="mk">◐</span> {t("p.capacity.within_capacity_but_committing_the")}</li>
    <li><span class="mk">!</span> {t("p.capacity.over_capacity_solid_bar_firm")}</li>
    <li>{t("p.capacity.hatched_extension_proposed_not_count")}</li>
  </ul>
  <DataTable label={t("ppm.nav.capacity")} {columns} rows={view.people} rowId={(r) => String(r.person_ref)} searchable />
  {#if view.people.length === 0}
    <p class="small muted">{t("ppm.capacity.empty")}</p>
  {/if}
{/if}

<style>
  .row { display: flex; gap: 0.5rem; flex-wrap: wrap; align-items: center; margin: 0.8rem 0; }
  .chip.red {
    display: inline-block;
    border: 1px solid #a4262c;
    color: #a4262c;
    border-radius: 999px;
    padding: 0 0.5rem;
    font-size: 0.75rem;
  }
  .legend { display: flex; flex-wrap: wrap; gap: 0.25rem 1rem; list-style: none; margin: 0.5rem 0; padding: 0; }
  .mk { font-weight: 700; display: inline-block; min-width: 1.1em; }
</style>
