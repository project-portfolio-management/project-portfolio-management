<!--
  Roadmap (`/roadmap`, NC-10): ideas and proposals on an impact / effort matrix, and in Now / Next / Later.

  The service holds no impact or effort for either, so a person rates each item here and the rating
  stays in this browser (localStorage); nothing is guessed. The table of items is the keyboard way
  to the same thing the matrix shows: every item has an impact and an effort choice.
-->
<script lang="ts">
  import { rowOf } from "#lib/row-of.js";
  import { resource } from "#lib/resource.svelte.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t, type StringKey } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient, type Idea, type Proposal } from "#lib/api/ppm.js";
  import {
    HORIZONS,
    LEVELS,
    horizonOf,
    inCell,
    onHorizon,
    parseRatings,
    roadmapItems,
    serializeRatings,
    withRating,
    type Horizon,
    type Level,
    type Ratings,
    type RoadmapItem,
  } from "#lib/roadmap-view.js";

  const STORAGE_KEY = "ppm.roadmap.ratings";
  const ppm = PpmClient.withFetch();
  const ideasLoad = resource<Idea[]>(() => ppm.listIdeas(), "ppm.common.loadFailed");
  const proposalsLoad = resource<Proposal[]>(() => ppm.listProposals(), "ppm.common.loadFailed");
  const error = $derived(ideasLoad.error ?? proposalsLoad.error);
  const items = $derived(roadmapItems(ideasLoad.data ?? [], proposalsLoad.data ?? []));

  let ratings = $state<Ratings>({});

  onMount(() => {
    try {
      ratings = parseRatings(localStorage.getItem(STORAGE_KEY));
    } catch {
      // Storage can be blocked; the page works without remembering.
    }
    void ideasLoad.reload();
    void proposalsLoad.reload();
  });

  function rate(id: string, field: "impact" | "effort", value: string) {
    ratings = withRating(ratings, id, field, (LEVELS as readonly string[]).includes(value) ? (value as Level) : null);
    try {
      localStorage.setItem(STORAGE_KEY, serializeRatings(ratings));
    } catch {
      // Not remembered; still shown for this visit.
    }
  }

  const levelLabel = (l: Level) => t(`p.roadmap.level_${l}` as StringKey);
  const kindLabel = (i: RoadmapItem) => t(i.kind === "idea" ? "p.roadmap.kind_idea" : "p.roadmap.kind_proposal");
  const horizonLabel = (h: Horizon) => t(`p.roadmap.h_${h}` as StringKey);
  const itemName = (i: RoadmapItem) => `${kindLabel(i)}: ${i.title}`;

  type Row = RoadmapItem & { impact: string; effort: string; horizon: string };
  const rows = $derived(
    items.map(
      (i): Row => ({
        ...i,
        impact: ratings[i.id]?.impact ?? "",
        effort: ratings[i.id]?.effort ?? "",
        horizon: horizonLabel(horizonOf(ratings[i.id])),
      }),
    ),
  );
  const columns: DataGridColumn[] = $derived([
    { id: "title", header: t("p.roadmap.col_item"), rowHeader: true },
    { id: "kind", header: t("p.roadmap.col_type"), accessor: (r) => kindLabel(rowOf<RoadmapItem>(r)) },
    { id: "impact", header: t("p.roadmap.impact"), cell: impactCell },
    { id: "effort", header: t("p.roadmap.effort"), cell: effortCell },
    { id: "horizon", header: t("p.roadmap.col_horizon") },
  ]);
</script>

{#snippet levelSelect(id: string, title: string, field: "impact" | "effort", value: string)}
  <select
    aria-label={t(field === "impact" ? "p.roadmap.rate_impact" : "p.roadmap.rate_effort", { title })}
    {value}
    onchange={(e) => rate(id, field, e.currentTarget.value)}
  >
    <option value="" selected={value === ""}>{t("p.roadmap.not_rated")}</option>
    {#each LEVELS as level (level)}
      <option value={level} selected={value === level}>{levelLabel(level)}</option>
    {/each}
  </select>
{/snippet}

{#snippet impactCell({ row }: DataGridCellContext)}
  {@render levelSelect(String(row.id), String(row.title), "impact", String(row.impact))}
{/snippet}

{#snippet effortCell({ row }: DataGridCellContext)}
  {@render levelSelect(String(row.id), String(row.title), "effort", String(row.effort))}
{/snippet}

<svelte:head><title>{t("p.roadmap.title")}</title></svelte:head>

<h1>{t("p.roadmap.h1")}</h1>
<p class="small muted">{t("p.roadmap.lede")}</p>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if ideasLoad.data && proposalsLoad.data}
  <h2>{t("p.roadmap.matrix_h2")}</h2>
    <table class="matrix" data-testid="roadmap-matrix">
      <caption class="small muted">{t("p.roadmap.matrix_caption")}</caption>
      <thead>
        <tr>
          <td></td>
          {#each LEVELS as effort (effort)}
            <th scope="col">{t("p.roadmap.effort_level", { level: levelLabel(effort) })}</th>
          {/each}
        </tr>
      </thead>
      <tbody>
        {#each [...LEVELS].reverse() as impact (impact)}
          <tr>
            <th scope="row">{t("p.roadmap.impact_level", { level: levelLabel(impact) })}</th>
            {#each LEVELS as effort (effort)}
              {@const cell = inCell(items, ratings, impact, effort)}
              <td>
                {#if cell.length === 0}
                  <span class="muted">—</span>
                {:else}
                  <ul>{#each cell as item (item.id)}<li>{itemName(item)}</li>{/each}</ul>
                {/if}
              </td>
            {/each}
          </tr>
        {/each}
      </tbody>
    </table>

  <h2>{t("p.roadmap.rate_h2")}</h2>
  <p class="small muted">{t("p.roadmap.rate_hint")}</p>
  <DataTable
    label={t("p.roadmap.rate_h2")}
    {columns}
    {rows}
    rowId={(r) => String(r.id)}
    emptyText={t("p.roadmap.no_items")}
    searchable
    testid="roadmap-rate"
  />

  <h2>{t("p.roadmap.horizons_h2")}</h2>
  <p class="small muted">{t("p.roadmap.horizon_rule")}</p>
  <div class="horizons">
    {#each HORIZONS as horizon (horizon)}
      {@const list = onHorizon(items, ratings, horizon)}
      <section aria-labelledby={`h-${horizon}`} data-testid={`horizon-${horizon}`}>
        <h3 id={`h-${horizon}`}>{horizonLabel(horizon)}</h3>
        {#if list.length === 0}
          <p class="small muted">{t("p.roadmap.nothing_here")}</p>
        {:else}
          <ul>{#each list as item (item.id)}<li>{itemName(item)}</li>{/each}</ul>
        {/if}
      </section>
    {/each}
  </div>
{:else if !error}
  <p class="small muted">{t("p.roadmap.loading")}</p>
{/if}

<style>
  .matrix { border-collapse: collapse; width: 100%; }
  .matrix th, .matrix td {
    border: 1px solid var(--border, #ccc);
    padding: 0.4rem 0.6rem;
    vertical-align: top;
    text-align: start;
  }
  .matrix ul, .horizons ul { margin: 0; padding-inline-start: 1.1rem; }
  .horizons { display: grid; grid-template-columns: repeat(auto-fit, minmax(14rem, 1fr)); gap: 1rem; }
</style>
