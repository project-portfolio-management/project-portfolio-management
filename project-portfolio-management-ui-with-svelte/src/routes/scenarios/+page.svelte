<!--
  Scenario planning (`/scenarios`, PPM-4): create what-if candidate
  portfolios (member pids + budget cap), evaluate them over live
  data, and commit the feasible winner.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import {
    PpmClient,
    money,
    type Scenario,
    type ScenarioComparison,
    type ScenarioEvaluation,
  } from "#lib/api/ppm.js";

  const ppm = PpmClient.withFetch();
  let scenarios = $state<Scenario[]>([]);
  let evaluations = $state<Record<string, ScenarioEvaluation>>({});
  let error = $state<string | null>(null);

  let compareA = $state("");
  let compareB = $state("");
  let comparison = $state<ScenarioComparison | null>(null);

  async function runCompare() {
    error = null;
    comparison = null;
    try {
      comparison = await ppm.compareScenarios(compareA, compareB);
    } catch (err) {
      error = errorText(err, "ppm.common.actionFailed");
    }
  }

  let name = $state("");
  let members = $state("");
  let cap = $state("");
  let currency = $state("GBP");

  async function refresh() {
    try {
      scenarios = await ppm.listScenarios();
      error = null;
    } catch (err) {
      error = errorText(err, "ppm.common.loadFailed");
    }
  }
  onMount(refresh);

  async function act(action: () => Promise<unknown>) {
    error = null;
    try {
      await action();
      await refresh();
    } catch (err) {
      error = errorText(err, "ppm.common.actionFailed");
    }
  }

  async function create(event: SubmitEvent) {
    event.preventDefault();
    const pids = members
      .split(/[\s,]+/)
      .map((value) => value.trim())
      .filter(Boolean);
    const capMinor = cap.trim() ? Math.round(Number(cap) * 100) : undefined;
    await act(() =>
      ppm.createScenario({
        name,
        work_item_pids: pids,
        ...(capMinor !== undefined && Number.isFinite(capMinor)
          ? { budget_cap_minor: capMinor, currency }
          : {}),
      }),
    );
    name = "";
    members = "";
    cap = "";
  }

  async function evaluate(pid: string) {
    try {
      evaluations = { ...evaluations, [pid]: await ppm.evaluateScenario(pid) };
    } catch (err) {
      error = errorText(err, "p.scenarios.evaluate_failed");
    }
  }

  const scenarioColumns: DataGridColumn[] = $derived([
    { id: "name", header: t("p.scenarios.scenario"), rowHeader: true },
    { id: "budget_cap_minor", header: t("p.scenarios.cap"), align: "end", format: (v, r) => (v !== null && v !== undefined ? money(Number(v), String(r.currency)) : "—") },
    { id: "status", header: t("p.scenarios.status"), cell: statusChip },
    { id: "evaluation", header: t("p.scenarios.evaluation"), sortable: false, accessor: (r) => (evaluations[String(r.pid)] ? "evaluated" : ""), cell: evalCell },
    { id: "pid", header: "", sortable: false, cell: scenarioActions },
  ]);
  type CRow = { metric: string; a: string; b: string; delta: string };
  const compareRows = (c: ScenarioComparison): CRow[] => [
    { metric: t("p.scenarios.feasible"), a: c.a.feasible ? t("p.scenarios.yes") : t("p.scenarios.no"), b: c.b.feasible ? t("p.scenarios.yes") : t("p.scenarios.no"), delta: "—" },
    ...c.deltas.planned_by_currency.map((r) => ({ metric: t("p.scenarios.planned", { currency: r.currency }), a: money(r.a_minor, r.currency), b: money(r.b_minor, r.currency), delta: money(r.delta_minor, r.currency) })),
    { metric: t("p.scenarios.open_exposure"), a: String(c.a.evaluation.total_exposure), b: String(c.b.evaluation.total_exposure), delta: String(c.deltas.exposure) },
    { metric: t("p.scenarios.alignment"), a: String(c.a.evaluation.total_alignment), b: String(c.b.evaluation.total_alignment), delta: String(c.deltas.alignment) },
  ];
  const compareColumns = (c: ScenarioComparison): DataGridColumn[] => [
    { id: "metric", header: "", rowHeader: true },
    { id: "a", header: c.a.name, align: "end" },
    { id: "b", header: c.b.name, align: "end" },
    { id: "delta", header: t("p.scenarios.delta_b_a"), align: "end" },
  ];
</script>

{#snippet statusChip({ formatted }: DataGridCellContext)}<span class="chip">{formatted}</span>{/snippet}

{#snippet evalCell({ row }: DataGridCellContext)}
  {@const evaluated = evaluations[String(row.pid)]}
  {#if evaluated}
    {#each evaluated.evaluation.planned_by_currency as [code, total] (code)}<span class="chip">{code} {money(total)}</span>{/each}
    <span class="chip">{t("ppm.scenarios.exposure")} {evaluated.evaluation.total_exposure}</span>
    <span class="chip">{t("ppm.scenarios.alignment")} {evaluated.evaluation.total_alignment}</span>
    {#each evaluated.evaluation.violations as violation (violation)}<span class="chip red">{violation}</span>{/each}
    {#if evaluated.feasible}<span class="chip green">{t("ppm.scenarios.feasible")}</span>{/if}
  {/if}
{/snippet}

{#snippet scenarioActions({ row }: DataGridCellContext)}
  <button class="button small" onclick={() => evaluate(String(row.pid))}>{t("ppm.scenarios.evaluate")}</button>
  {#if row.status === "draft"}
    <button class="button small" onclick={() => act(() => ppm.commitScenario(String(row.pid)))}>{t("ppm.scenarios.commit")}</button>
  {/if}
{/snippet}


<svelte:head><title>{t("p.scenarios.scenarios_ppm")}</title></svelte:head>

<h1>{t("ppm.scenarios.title")}</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

<form class="row" onsubmit={create}>
  <input placeholder={t("ppm.scenarios.name")} bind:value={name} required />
  <input placeholder={t("p.scenarios.work_item_pids_space_comma")} bind:value={members} size="50" />
  <input placeholder={t("ppm.scenarios.cap")} bind:value={cap} size="12" />
  <input bind:value={currency} size="4" aria-label={t("p.scenarios.currency")} />
  <button class="button primary" type="submit">{t("ppm.scenarios.create")}</button>
</form>

<DataTable label={t("p.scenarios.scenario")} columns={scenarioColumns} rows={scenarios} rowId={(r) => String(r.pid)} />


<h2>{t("p.scenarios.compare")}</h2>
<form
  class="compare"
  onsubmit={(event) => {
    event.preventDefault();
    void runCompare();
  }}
>
  <label>
    A
    <select bind:value={compareA}>
      <option value="" disabled>{t("p.scenarios.pick")}</option>
      {#each scenarios as s (s.pid)}<option value={s.pid}>{s.name}</option>{/each}
    </select>
  </label>
  <label>
    B
    <select bind:value={compareB}>
      <option value="" disabled>{t("p.scenarios.pick")}</option>
      {#each scenarios as s (s.pid)}<option value={s.pid}>{s.name}</option>{/each}
    </select>
  </label>
  <button type="submit" disabled={!compareA || !compareB || compareA === compareB}>
    {t("p.scenarios.compare")}
  </button>
</form>

{#if comparison}
  <p class="muted">{comparison.note}</p>
  <DataTable label={t("p.scenarios.delta_b_a")} columns={compareColumns(comparison)} rows={compareRows(comparison)} rowId={(_r, i) => String(i)} sortable={false} testid="scenario-compare" />
{/if}

<style>
  .row { display: flex; gap: 0.5rem; flex-wrap: wrap; align-items: center; margin: 0.8rem 0; }
  .chip {
    display: inline-block;
    border: 1px solid var(--border, #ccc);
    border-radius: 999px;
    padding: 0 0.55rem;
    margin: 0.1rem;
    font-size: 0.78rem;
  }
  .chip.red { color: #a4262c; border-color: #a4262c; }
  .chip.green { color: #1d8a4e; border-color: #1d8a4e; }
</style>
