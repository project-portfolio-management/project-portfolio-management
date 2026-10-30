<!--
  Capacity heatmap (`/capacity-heatmap`, DP-10, FB-6): `GET /api/capacity-heatmap`, skill pool by month. Every
  square says its reading in a word and a symbol (the service's `shape`) as well as a tone, so colour is never the
  only signal. For a pool linked to the workforce service the square says the supply is from there, how it was
  read (live, last received with its age, hidden under the floor, unknown with the reason), and what was entered
  here beside it, never added; a month with no figure is unknown, never zero. The words come from
  `src/lib/capacity-heatmap-view.ts`; this page only draws them. Every figure is also in a table.
-->
<script lang="ts">
  import { resource } from "#lib/resource.svelte.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient } from "#lib/api/ppm.js";
  import { cellView, heatmapRows, monthLabel, poolName, type CapacityHeatmap } from "#lib/capacity-heatmap-view.js";

  const ppm = PpmClient.withFetch();
  let from = $state("");
  let months = $state(6);
  const loaded = resource<CapacityHeatmap>(() => ppm.capacityHeatmap(from || undefined, months), "p.capacity_heatmap.could_not_load");
  const heat = $derived(loaded.data);
  const error = $derived(loaded.error);
  const load = () => loaded.reload();
  onMount(load);

  const rows = $derived(heat ? heatmapRows(heat) : []);
  const columns: DataGridColumn[] = $derived([
    { id: "pool", header: t("p.capacity_heatmap.c_pool"), rowHeader: true },
    { id: "month", header: t("p.capacity_heatmap.c_month") },
    { id: "reading", header: t("p.capacity_heatmap.c_reading") },
    { id: "demand", header: t("p.capacity_heatmap.c_demand"), align: "end" },
    { id: "supply", header: t("p.capacity_heatmap.c_supply"), align: "end" },
    { id: "source", header: t("p.capacity_heatmap.c_source") },
    { id: "status", header: t("p.capacity_heatmap.c_status") },
    { id: "entered_here", header: t("p.capacity_heatmap.c_entered_here") },
  ]);
</script>

<svelte:head><title>{t("p.capacity_heatmap.title")}</title></svelte:head>

<h1>{t("p.capacity_heatmap.h1")}</h1>
<p class="lede">{t("p.capacity_heatmap.lede")}</p>
<p class="banner" role="note">{t("p.capacity_heatmap.bridge_note")}</p>
<p><a href={localeHref("/skill-pools")}>{t("p.capacity_heatmap.see_pools")}</a></p>

<form class="filter" onsubmit={(e) => { e.preventDefault(); void load(); }}>
  <label>{t("p.capacity_heatmap.from")} <input type="month" bind:value={from} /></label>
  <label>{t("p.capacity_heatmap.months")}
    <select bind:value={months}>
      {#each [3, 6, 12, 24] as n (n)}<option value={n}>{n}</option>{/each}
    </select>
  </label>
  <button class="button small primary" type="submit">{t("p.capacity_heatmap.show")}</button>
</form>

{#if error}<p class="banner error" role="alert">{error}</p>{/if}

{#if heat}
  {#if heat.pools.length === 0}
    <p>{t("p.capacity_heatmap.none")}</p>
  {:else}
    <!-- A region that scrolls sideways must be reachable by keyboard (axe scrollable-region-focusable); the table
         holds nothing focusable, so the region itself takes focus. -->
    <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
    <div class="scroll" role="region" aria-labelledby="heat-caption" tabindex="0">
      <table class="heat" data-testid="heatmap">
        <caption id="heat-caption">{t("p.capacity_heatmap.caption")}</caption>
        <thead>
          <tr>
            <th scope="col">{t("p.capacity_heatmap.col_pool")}</th>
            {#each heat.months as m (m)}<th scope="col">{monthLabel(m)}</th>{/each}
          </tr>
        </thead>
        <tbody>
          {#each heat.pools as p (p.pool_pid ?? "none")}
            <tr>
              <th scope="row">
                {poolName(p)}
                {#if p.peer_pool_ref}<span class="small linked">{t("p.capacity_heatmap.linked_to")}</span>{/if}
              </th>
              {#each p.cells as c (c.month)}
                {@const v = cellView(c)}
                <td class={`cell tone-${v.tone}`} data-testid={`cell-${p.pool_pid ?? "none"}-${c.month}`} data-state={c.state}>
                  <strong><span aria-hidden="true" class="symbol">{v.symbol}</span> {v.word}</strong>
                  <span class="line">{t("p.capacity_heatmap.demand", { fte: v.demand })}</span>
                  <span class="line">{t("p.capacity_heatmap.supply", { fte: v.supply })}</span>
                  {#if v.gap !== null}<span class="line">{t("p.capacity_heatmap.gap", { fte: v.gap })}</span>{/if}
                  {#if v.linked}
                    <span class="line source">{v.source}</span>
                    <span class="line small" data-peer={v.peerStatus}>{v.peerText}</span>
                    <span class="line small">{v.enteredHere}</span>
                  {/if}
                  {#if v.partnersShort}<span class="line small">{t("p.capacity_heatmap.partners_short")}</span>{/if}
                </td>
              {/each}
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
    <p class="small muted">{t("p.capacity_heatmap.key")}</p>

    <details>
      <summary>{t("p.capacity_heatmap.every_figure")}</summary>
      <DataTable label={t("p.capacity_heatmap.table_label")} {columns} {rows} rowId={(r) => String(r.key)} sortable={false} pageSize={50} searchable />
    </details>
  {/if}
  <p class="small muted">{t("p.capacity_heatmap.no_commitment_form")}</p>
{:else if !error}
  <p class="muted">{t("p.capacity_heatmap.loading")}</p>
{/if}

<style>
  .scroll { overflow-x: auto; }
  table { border-collapse: collapse; width: 100%; }
  th, td { text-align: left; vertical-align: top; padding: 0.4rem 0.5rem; border-bottom: 1px solid var(--mxi-color-border, #ddd); }
  .cell { min-width: 11rem; border-left: 6px solid var(--mxi-color-border, #ddd); }
  .cell.tone-red { border-left-color: #b91c1c; }
  .cell.tone-amber { border-left-color: #b45309; }
  .cell.tone-green { border-left-color: #15803d; }
  .cell.tone-grey { border-left-color: #6b7280; }
  .cell.tone-none { border-left-style: dashed; }
  .line { display: block; }
  .source { font-weight: 600; }
  .linked { display: block; font-weight: normal; }
  .symbol { display: inline-block; min-width: 1em; }
  .filter { display: flex; flex-wrap: wrap; gap: 0.75rem; align-items: end; margin: 0.75rem 0; }
  .filter label { display: grid; gap: 0.2rem; font-size: 0.85rem; }
</style>
