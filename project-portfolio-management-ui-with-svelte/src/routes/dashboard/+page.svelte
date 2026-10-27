<!--
  PPM dashboard (`/dashboard`, PPM-7): site tiles + per-collection
  RAG / stage rollups from the ETag-conditional `/api/at-a-glance`.
  English-first (locale catalogues extend as a follow-up).
-->
<script lang="ts">
  import { rowOf } from "#lib/row-of.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { resource } from "#lib/resource.svelte.js";
  import { page } from "$app/state";
  import { PpmClient, type Dashboard } from "#lib/api/ppm.js";

  const ppm = PpmClient.withFetch();
  const loaded = resource(() => ppm.dashboard(), "ppm.common.loadFailed");
  const board = $derived(loaded.data);
  const error = $derived(loaded.error);

  onMount(() => void loaded.reload());

  type Collection = Dashboard["collections"][number];
  const columns: DataGridColumn[] = $derived([
    { id: "collection", header: t("ppm.dashboard.collection"), rowHeader: true },
    { id: "total", header: t("ppm.dashboard.total"), align: "end" },
    { id: "red", header: t("ppm.dashboard.red"), align: "end", accessor: (r) => rowOf<Collection>(r).rag.red, cellClass: "rag red" },
    { id: "amber", header: t("ppm.dashboard.amber"), align: "end", accessor: (r) => rowOf<Collection>(r).rag.amber, cellClass: "rag amber" },
    { id: "green", header: t("ppm.dashboard.green"), align: "end", accessor: (r) => rowOf<Collection>(r).rag.green, cellClass: "rag green" },
    { id: "stages", header: t("ppm.dashboard.stages"), sortable: false, accessor: (r) => Object.entries(rowOf<Collection>(r).stages).map(([k, v]) => `${k}: ${v}`).join(", "), cell: stagesCell },
  ]);
</script>

{#snippet stagesCell({ row }: DataGridCellContext)}
  {#each Object.entries(rowOf<Collection>(row).stages) as [stage, count] (stage)}<span class="chip">{stage}: {count}</span>{/each}
{/snippet}


<svelte:head><title>{t("p.dashboard.dashboard_ppm")}</title></svelte:head>

<!-- Signed in, the dashboard is the person's own home page; signed out it keeps its plain title. -->
<h1>{page.data.signedIn ? t("p.dashboard.my_home") : t("ppm.dashboard.title")}</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if board}
  <section class="tiles" data-testid="site-tiles">
    <div class="tile"><strong>{board.site_tiles.plans}</strong><span>{t("ppm.dashboard.workItems")}</span></div>
    <div class="tile"><strong>{board.site_tiles.proposals_open}</strong><span>{t("ppm.dashboard.openProposals")}</span></div>
    <div class="tile"><strong>{board.site_tiles.materialized_risks}</strong><span>{t("ppm.dashboard.materializedRisks")}</span></div>
    <div class="tile"><strong>{board.site_tiles.open_risk_exposure}</strong><span>{t("ppm.dashboard.exposure")}</span></div>
    <div class="tile"><strong>{board.site_tiles.schedule_violations}</strong><span>{t("ppm.dashboard.violations")}</span></div>
    <div class="tile"><strong>{board.site_tiles.over_allocated_people}</strong><span>{t("ppm.dashboard.overAllocated")}</span></div>
  </section>

  <DataTable label={t("ppm.dashboard.title")} {columns} rows={board.collections} rowId={(r) => String(r.collection)} />
  <p class="small muted">{t("ppm.common.asOf")} {board.as_of}</p>
{:else if !error}
  <p>{t("ppm.common.loading")}</p>
{/if}

<style>
  .tiles {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
    gap: 0.75rem;
    margin: 1rem 0;
  }
  .tile {
    border: 1px solid var(--border, #ccc);
    border-radius: 8px;
    padding: 0.7rem 0.9rem;
    display: flex;
    flex-direction: column;
  }
  .tile strong { font-size: 1.5rem; }
  .tile span { font-size: 0.8rem; opacity: 0.75; }
  :global(.rag) { font-weight: 700; }
  :global(.rag.red) { color: #a4262c; }
  :global(.rag.amber) { color: #a06a00; }
  :global(.rag.green) { color: #1d8a4e; }
  .chip {
    display: inline-block;
    border: 1px solid var(--border, #ccc);
    border-radius: 999px;
    padding: 0 0.5rem;
    margin: 0.1rem;
    font-size: 0.75rem;
  }
</style>
