<!--
  Data-driven prioritization (`/prioritization`): the Smart Score queue.
  Highest score first, with each plan's evidence coverage on show, and
  an expandable per-component breakdown — a score that ranks somebody's
  work has to be able to explain itself.

  Unscored plans keep the `unscored` band and sort last: no evidence is
  reported as no evidence, never as a low score.
-->
<script lang="ts">
  import { rowOf } from "#lib/row-of.js";
  import { errorText } from "#lib/error-text.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { enumLabel } from "#lib/enum-labels.js";
  import { onMount } from "svelte";
  import {
    CapabilityClient,
    type Prioritization,
    type PlanSmartScore,
  } from "#lib/api/capabilities.js";

  const api = CapabilityClient.withFetch();
  let view = $state<Prioritization | null>(null);
  let error = $state<string | null>(null);
  let band = $state("");
  let expanded = $state<string | null>(null);
  let detail = $state<PlanSmartScore | null>(null);

  async function refresh() {
    try {
      view = await api.prioritization({ band: band || undefined });
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
  type Component = PlanSmartScore["smart_score"]["components"][number];
  const columns: DataGridColumn[] = $derived([
    { id: "name", header: t("p.prioritization.plan"), rowHeader: true, cell: planCell },
    { id: "score", header: t("p.prioritization.score"), align: "end", format: (v) => (v === null || v === undefined ? "—" : Number(v).toFixed(1)) },
    { id: "band", header: t("p.prioritization.band_2"), format: (v) => enumLabel("band", String(v)), cell: bandCell },
    { id: "coverage", header: t("p.prioritization.coverage_2"), align: "end", format: (v) => `${Math.round(Number(v) * 100)}%` },
    { id: "missing_evidence", header: t("p.prioritization.missing_evidence"), accessor: (r) => (rowOf<PRow>(r).missing_evidence.length === 0 ? t("p.prioritization.none") : rowOf<PRow>(r).missing_evidence.join(", ")) },
    { id: "pid", header: "", sortable: false, cell: explainCell },
  ]);
  const componentColumns: DataGridColumn[] = $derived([
    { id: "name", header: t("p.prioritization.component"), rowHeader: true },
    { id: "weight", header: t("p.prioritization.weight"), align: "end", format: (v) => `${Math.round(Number(v) * 100)}%` },
    { id: "raw", header: t("p.prioritization.value"), align: "end", format: (v) => Number(v).toFixed(2) },
    { id: "contribution", header: t("p.prioritization.points"), align: "end", format: (v) => Number(v).toFixed(1) },
  ]);
</script>

{#snippet planCell({ row, formatted }: DataGridCellContext)}<a href={localeHref(`/plans/${row.pid}`)}>{formatted}</a>{/snippet}

{#snippet bandCell({ row, formatted }: DataGridCellContext)}<span class={`chip ${rowOf<PRow>(row).band}`}>{formatted}</span>{/snippet}

{#snippet explainCell({ row }: DataGridCellContext)}
  <button class="button small" type="button" onclick={() => toggle(String(row.pid))}>
    {expanded === row.pid ? t("p.prioritization.hide") : t("p.prioritization.explain")}
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
    {t("p.prioritization.band")}
    <select bind:value={band}>
      <option value="">{t("p.prioritization.all")}</option>
      <option value="high">{t("p.prioritization.high")}</option>
      <option value="medium">{t("p.prioritization.medium")}</option>
      <option value="low">{t("p.prioritization.low")}</option>
      <option value="unscored">{t("p.prioritization.unscored")}</option>
    </select>
  </label>
  <button class="button small primary" type="submit">{t("p.prioritization.apply")}</button>
</form>

{#if view}
  <p class="small muted">{t("p.prioritization.of_plan_s", { length: view.plans.length, scored_of: view.scored_of })}</p>
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
  .chip.high { color: #1d8a4e; }
  .chip.medium { color: #8a6d1d; }
  .chip.low { color: #a4262c; }
  .chip.unscored { color: var(--text-2, #666); }
</style>
