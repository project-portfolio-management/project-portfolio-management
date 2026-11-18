<!--
  Data-driven prioritization (`/prioritization`): the Smart Score queue.
  Highest score first, with each plan's evidence coverage on show, and
  an expandable per-component breakdown — a score that ranks somebody's
  work has to be able to explain itself.

  Unscored plans keep the `unscored` band and sort last: no evidence is
  reported as no evidence, never as a low score.

  NC-6 (NC-1 slice 12 part g): the queue ranks by a chosen model, the Smart Score (the default, not sent), WSJF or
  RICE (`GET /api/prioritization?model=`); the band filter is the Smart Score's and is offered only with it. A WSJF or
  RICE row says the inputs its score waits for in the catalogue's words. Each row opens the plan's estimates
  (`ScoringEstimates.svelte`: `GET`/`PUT /api/plans/{pid}/scoring-inputs`, `GET /api/plans/{pid}/score?model=`).
  Rules in `src/lib/scoring-view.ts`.
-->
<script lang="ts">
  import { rowOf } from "#lib/row-of.js";
  import { errorText } from "#lib/error-text.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t, localeHref, tn } from "#lib/i18n.svelte.js";
  import { formatDecimal, formatPercent } from "#lib/locale-format.js";
  import { enumLabel } from "#lib/enum-labels.js";
  import { onMount } from "svelte";
  import {
    CapabilityClient,
    type Prioritization,
    type PlanSmartScore,
  } from "#lib/api/capabilities.js";
  import { PpmClient, type ModelRanking, type RankModel } from "#lib/api/ppm.js";
  import ScoringEstimates from "#lib/components/ScoringEstimates.svelte";
  import { MODELS, MODEL_KEY, inputKey } from "#lib/scoring-view.js";

  const api = CapabilityClient.withFetch();
  const ppm = PpmClient.withFetch();
  let view = $state<Prioritization | null>(null);
  let ranking = $state<ModelRanking | null>(null);
  let error = $state<string | null>(null);
  let band = $state("");
  let model = $state<RankModel>("smart");
  /** The model the rows on show were ranked by. */
  let shown = $state<RankModel>("smart");
  let expanded = $state<string | null>(null);
  let detail = $state<PlanSmartScore | null>(null);
  let estimating = $state<{ pid: string; name: string } | null>(null);

  async function refresh() {
    try {
      if (model === "smart") {
        view = await api.prioritization({ band: band || undefined });
        ranking = null;
      } else {
        ranking = await ppm.rankByModel(model);
        view = null;
        expanded = null;
        detail = null;
      }
      shown = model;
      error = null;
    } catch (err) {
      error = errorText(err, "p.prioritization.could_not_load_the_queue");
    }
  }

  async function toggle(pid: string) {
    if (expanded === pid) {
      expanded = null;
      detail = null;
      return;
    }
    expanded = pid;
    detail = null;
    try {
      detail = await api.smartScore(pid);
    } catch (err) {
      error = errorText(err, "p.prioritization.could_not_load_the_score");
    }
  }

  onMount(refresh);

  type PRow = Prioritization["plans"][number];
  const columns: DataGridColumn[] = $derived([
    { id: "name", header: t("p.prioritization.plan"), rowHeader: true, cell: planCell },
    { id: "score", header: t("p.prioritization.score"), align: "end", format: (v) => (v === null || v === undefined ? "—" : formatDecimal(Number(v), 1)) },
    { id: "band", header: t("p.prioritization.band_2"), format: (v) => enumLabel("band", String(v)), cell: bandCell },
    { id: "coverage", header: t("p.prioritization.coverage_2"), align: "end", format: (v) => formatPercent(Number(v)) },
    { id: "missing_evidence", header: t("p.prioritization.missing_evidence"), accessor: (r) => (rowOf<PRow>(r).missing_evidence.length === 0 ? t("p.prioritization.none") : rowOf<PRow>(r).missing_evidence.join(", ")) },
    { id: "pid", header: "", sortable: false, cell: explainCell },
    { id: "estimates", header: t("p.scoring.open"), sortable: false, accessor: () => "", cell: estimateCell },
  ]);
  type MRow = ModelRanking["plans"][number];
  const modelColumns: DataGridColumn[] = $derived([
    { id: "name", header: t("p.prioritization.plan"), rowHeader: true, cell: planCell },
    { id: "score", header: t("p.prioritization.score"), align: "end", format: (v) => (v === null || v === undefined ? "—" : formatDecimal(Number(v), 2)) },
    {
      id: "missing_inputs",
      header: t("p.scoring.waiting_for"),
      sortable: false,
      accessor: (r) => {
        const m = rowOf<MRow>(r).missing_inputs;
        return m.length === 0 ? t("p.prioritization.none") : m.map((x) => t(inputKey(x.input))).join(", ");
      },
    },
    { id: "pid", header: "", sortable: false, cell: estimateCell },
  ]);
  const componentColumns: DataGridColumn[] = $derived([
    { id: "name", header: t("p.prioritization.component"), rowHeader: true },
    { id: "weight", header: t("p.prioritization.weight"), align: "end", format: (v) => formatPercent(Number(v)) },
    { id: "raw", header: t("p.prioritization.value"), align: "end", format: (v) => formatDecimal(Number(v), 2) },
    { id: "contribution", header: t("p.prioritization.points"), align: "end", format: (v) => formatDecimal(Number(v), 1) },
  ]);
</script>

{#snippet planCell({ row, formatted }: DataGridCellContext)}<a href={localeHref(`/plans/${row.pid}`)}>{formatted}</a>{/snippet}

{#snippet bandCell({ row, formatted }: DataGridCellContext)}<span class={`chip ${rowOf<PRow>(row).band}`}>{formatted}</span>{/snippet}

{#snippet explainCell({ row }: DataGridCellContext)}
  <button class="button small" type="button" onclick={() => toggle(String(row.pid))}>
    {expanded === row.pid ? t("p.prioritization.hide") : t("p.prioritization.explain")}
  </button>
{/snippet}

{#snippet estimateCell({ row }: DataGridCellContext)}
  <button class="button small" type="button" aria-label={t("p.scoring.open_named", { name: String(row.name) })} onclick={() => (estimating = { pid: String(row.pid), name: String(row.name) })}>
    {t("p.scoring.open")}
  </button>
{/snippet}


<svelte:head><title>{t("p.prioritization.prioritization_ppm")}</title></svelte:head>

<h1>{t("p.prioritization.prioritization")}</h1>
<p class="small muted">
  {t("p.prioritization.smart_score_over_roi_strategic")} <em>{t("p.prioritization.coverage")}</em> {t("p.prioritization.shows_how_much_of_the")}
</p>

{#if error}<p class="banner" role="alert">{error}</p>{/if}

<form
  class="row"
  onsubmit={(e) => {
    e.preventDefault();
    refresh();
  }}
>
  <label class="small">
    {t("p.scoring.rank_by")}
    <select bind:value={model}>
      {#each MODELS as m (m)}<option value={m}>{t(MODEL_KEY[m])}</option>{/each}
    </select>
  </label>
  {#if model === "smart"}
  <label class="small">
    {t("p.prioritization.band")}
    <select bind:value={band}>
      <option value="">{t("p.prioritization.all")}</option>
      <option value="high">{t("p.prioritization.high")}</option>
      <option value="medium">{t("p.prioritization.medium")}</option>
      <option value="low">{t("p.prioritization.low")}</option>
      <option value="unscored">{t("p.prioritization.unscored")}</option>
    </select>
  </label>
  {/if}
  <button class="button small primary" type="submit">{t("p.prioritization.apply")}</button>
</form>
<p class="small muted">{t("p.scoring.models_say")}</p>

{#if ranking && shown !== "smart"}
  <p class="small muted">{tn("p.scoring.of_plans", ranking.scored_of, { length: ranking.plans.length, scored_of: ranking.scored_of, model: t(MODEL_KEY[shown]) })}</p>
  <DataTable label={t("p.scoring.ranked_by", { model: t(MODEL_KEY[shown]) })} columns={modelColumns} rows={ranking.plans} rowId={(r) => String(r.pid)} searchable grid storageKey="prioritization-model" />
  {#if ranking.plans.length === 0}
    <p class="small muted">{t("p.prioritization.no_plans_match_that_filter")}</p>
  {/if}
{/if}

{#if estimating}
  {#key estimating.pid}
    <ScoringEstimates pid={estimating.pid} name={estimating.name} onsaved={() => void refresh()} onclose={() => (estimating = null)} />
  {/key}
{/if}

{#if view}
  <p class="small muted">{tn("p.prioritization.of_plans", view.scored_of, { length: view.plans.length, scored_of: view.scored_of })}</p>
  <DataTable label={t("p.prioritization.plan")} columns={columns} rows={view.plans} rowId={(r) => String(r.pid)} searchable grid storageKey="prioritization-queue" />
  {#if expanded}
    {@const open = view.plans.find((p) => p.pid === expanded)}
    <section aria-labelledby="pr-detail">
      <h2 id="pr-detail">{open?.name ?? ""}</h2>
      {#if detail}
        <DataTable label={t("p.prioritization.component")} columns={componentColumns} rows={detail.smart_score.components} rowId={(r) => String(r.name)} sortable={false} />
        {#if detail.smart_score.components.length === 0}
          <p class="small muted">{t("p.prioritization.no_evidence_for_any_component")}</p>
        {/if}
      {:else}
        <p class="small muted">{t("p.prioritization.loading_the_breakdown")}</p>
      {/if}
    </section>
  {/if}
  {#if view.plans.length === 0}
    <p class="small muted">{t("p.prioritization.no_plans_match_that_filter")}</p>
  {/if}
{/if}

<style>
  .row { display: flex; gap: 0.5rem; flex-wrap: wrap; align-items: center; margin: 0.8rem 0; }
  .chip {
    display: inline-block;
    border-radius: 999px;
    padding: 0 0.5rem;
    font-size: 0.75rem;
    border: 1px solid currentColor;
  }
  .chip.high { color: var(--mxi-color-success); }
  .chip.medium { color: #8a6d1d; }
  .chip.low { color: #a4262c; }
  .chip.unscored { color: var(--text-2, #666); }
</style>
