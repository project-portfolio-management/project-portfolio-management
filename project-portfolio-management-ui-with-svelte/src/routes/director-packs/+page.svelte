<!--
  Director packs (`/director-packs`, DP-11): the packs you may read, newest first. Each opens as an
  accessible page; the figures are read from the issued pack, never recalculated.
-->
<script lang="ts">
  import { rowOf } from "#lib/row-of.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { formatDate } from "#lib/locale-format.js";
  import { onMount } from "svelte";
  import { resource } from "#lib/resource.svelte.js";
  import { PpmClient, type DirectorPackMeta } from "#lib/api/ppm.js";

  const ppm = PpmClient.withFetch();
  const loaded = resource(async () => (await ppm.directorPacks()).packs, "p.director_pack.could_not_load");
  const packs = $derived(loaded.data);
  const error = $derived(loaded.error);

  onMount(() => void loaded.reload());

  const columns: DataGridColumn[] = $derived([
    { id: "period", header: t("p.director_pack.col_period"), rowHeader: true, cell: periodCell },
    { id: "audience", header: t("p.director_pack.col_audience") },
    { id: "version", header: t("p.director_pack.col_version"), align: "end" },
    { id: "status", header: t("p.director_pack.col_status"), format: (v) => (v === "issued" ? t("p.director_pack.status_issued") : t("p.director_pack.status_draft")) },
    { id: "issued_at", header: t("p.director_pack.col_issued"), format: (v) => (v ? formatDate(String(v)) : "–") },
  ]);
</script>

{#snippet periodCell({ row, formatted }: DataGridCellContext)}
  {@const p = rowOf<DirectorPackMeta>(row)}
  <a href={localeHref(`/director-packs/${p.pid}`)} aria-label={t("p.director_pack.open_pack", { period: p.period, audience: p.audience, version: p.version })}>{formatted}</a>
{/snippet}


<svelte:head><title>{t("p.director_pack.director_packs_ppm")}</title></svelte:head>

<h1>{t("p.director_pack.director_packs")}</h1>
<p class="small muted">{t("p.director_pack.intro")}</p>

{#if error}<p class="banner error" role="alert">{error}</p>{/if}

{#if packs}
  {#if packs.length === 0}
    <p>{t("p.director_pack.none")}</p>
  {:else}
    <DataTable label={t("p.director_pack.col_period")} columns={columns} rows={packs} rowId={(r) => String(r.pid)} />
  {/if}
{:else if !error}
  <p class="muted">{t("p.director_pack.loading")}</p>
{/if}
