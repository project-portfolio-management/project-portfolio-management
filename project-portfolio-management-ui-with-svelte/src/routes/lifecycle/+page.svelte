<!--
  Bird's-eye visibility (`/lifecycle`): the whole challenge lifecycle in
  one view — every phase from idea to closed, how much is live in each,
  and how much has stalled there past the configured threshold.

  Every phase is shown even at zero, so an empty stage is visible rather
  than absent, and items whose phase could not be resolved are counted
  separately instead of being quietly folded in somewhere.
-->
<script lang="ts">
  import { rowOf } from "#lib/row-of.js";
  import { errorText } from "#lib/error-text.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { CapabilityClient, type LifecycleFunnel } from "#lib/api/capabilities.js";

  const api = CapabilityClient.withFetch();
  let view = $state<LifecycleFunnel | null>(null);
  let error = $state<string | null>(null);

  const total = $derived(
    view ? view.phases.reduce((sum, phase) => sum + phase.live, 0) : 0,
  );

  async function refresh() {
    try {
      view = await api.lifecycle();
      error = null;
    } catch (err) {
      error = errorText(err, "p.lifecycle.could_not_load_the_funnel");
    }
  }
  onMount(refresh);

  type Phase = LifecycleFunnel["phases"][number];
  const columns: DataGridColumn[] = $derived([
    { id: "phase", header: t("p.lifecycle.phase"), rowHeader: true, format: (v) => String(v).replaceAll("_", " ") },
    { id: "live", header: t("p.lifecycle.live"), align: "end" },
    { id: "stalled", header: t("p.lifecycle.stalled"), align: "end", cellClass: (c) => (Number(c.value) > 0 ? "cell-red" : "") },
    { id: "share", header: t("p.lifecycle.share"), sortable: false, accessor: (r) => rowOf<Phase>(r).live, cell: shareCell },
  ]);
</script>

{#snippet shareCell({ row }: DataGridCellContext)}
  <span class="meter"><span class="fill" style:width="{total === 0 ? 0 : (rowOf<Phase>(row).live / total) * 100}%"></span></span>
{/snippet}


<svelte:head><title>{t("p.lifecycle.lifecycle_ppm")}</title></svelte:head>

<h1>{t("p.lifecycle.lifecycle")}</h1>
<p class="small muted">
  {t("p.lifecycle.where_everything_sits_across_the")}
</p>

{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if view}
  <p class="small muted">
    {t("p.lifecycle.live_item_s_stall_threshold", { total, stall_days: view.stall_days })}
    {#if view.unknown_phase > 0}
      · <strong>{view.unknown_phase}</strong> {t("p.lifecycle.in_an_unrecognized_phase")}
    {/if}
  </p>
  <DataTable label={t("p.lifecycle.phase")} {columns} rows={view.phases} rowId={(r) => String(r.phase)} />
  <p class="small muted">{t("p.lifecycle.as_of", { as_of: view.as_of })}</p>
{/if}

<style>

  .meter {
    display: inline-block;
    width: 200px;
    height: 0.7rem;
    background: var(--surface-2, #f0f0f0);
    border-radius: 4px;
    overflow: hidden;
  }
  .fill { display: block; height: 100%; background: #1d8a4e; }
</style>
