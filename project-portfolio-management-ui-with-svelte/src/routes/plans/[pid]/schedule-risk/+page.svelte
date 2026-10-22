<!--
  Schedule risk (`/plans/[pid]/schedule-risk`): a seeded simulation of the plan's
  task network. The histogram shows how often each finish date came up, with P50,
  P80 and P90 marked by labelled lines — never colour alone — and the same numbers
  in a table beneath. Every assumption is stated, including that the inputs are
  treated as independent. A task with no estimate is marked as using the
  fallback band; a risk that could not be used is named with the reason.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { PpmClient, type ScheduleRisk } from "#lib/api/ppm.js";
  import {
    confidenceText,
    criticalityText,
    distributionText,
    histogramGeometry,
    percentileText,
    sensitivityText,
    skippedReasonText,
    sourceText,
  } from "#lib/scheduling-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");
  let run = $state<ScheduleRisk | null>(null);
  let ranOn = $state("");
  let error = $state<string | null>(null);
  let iterations = $state(2000);
  let seed = $state(1);
  let distribution = $state<"triangular" | "pert">("triangular");
  let band = $state(20);
  let busy = $state(false);

  async function simulate() {
    busy = true;
    try {
      run = await ppm.scheduleRisk(pid, { iterations, seed, distribution, fallbackBandPercent: band });
      ranOn = new Date().toISOString().slice(0, 10);
      error = null;
    } catch (err) {
      error = errorText(err, "p.plans.pid.schedule_risk.could_not_run_the_simulation");
    } finally {
      busy = false;
    }
  }
  onMount(() => void simulate());

  const g = $derived(run ? histogramGeometry(run.histogram, run.percentiles, run.cpm_finish) : null);
  const pName = (p: number) => `P${p}`;

  type RTask = ScheduleRisk["tasks"][number];
  type Run = ScheduleRisk;
  const finishColumns: DataGridColumn[] = $derived([
    { id: "label", header: t("p.plans.pid.schedule_risk.outcome"), rowHeader: true },
    { id: "value", header: t("p.plans.pid.schedule_risk.finish"), align: "end" },
  ]);
  const finishRows = (run: Run) => [
    ...run.percentiles.map((p) => ({ label: pName(p.p), value: p.finish })),
    { label: t("p.plans.pid.schedule_risk.critical_path_finish_the_plan"), value: run.cpm_finish ?? t("p.plans.pid.schedule_risk.not_enough_data") },
  ];
  const histogramColumns = (iterations: number): DataGridColumn[] => [
    { id: "date", header: t("p.plans.pid.schedule_risk.finish_date"), rowHeader: true },
    { id: "count", header: t("p.plans.pid.schedule_risk.simulations"), align: "end" },
    { id: "share", header: t("p.plans.pid.schedule_risk.share"), align: "end", accessor: (r) => ((r as { count: number }).count / iterations) * 100, format: (v) => `${Number(v).toFixed(1)}%` },
  ];
  const taskColumns: DataGridColumn[] = $derived([
    { id: "title", header: t("p.plans.pid.schedule_risk.task"), rowHeader: true, accessor: (r) => (r as unknown as RTask).title ?? (r as unknown as RTask).pid },
    { id: "range", header: t("p.plans.pid.schedule_risk.optimistic_likely_pessimistic_busine"), accessor: (r) => { const x = r as unknown as RTask; return `${x.optimistic} / ${x.likely} / ${x.pessimistic}`; } },
    { id: "estimate", header: t("p.plans.pid.schedule_risk.estimate"), accessor: (r) => sourceText(r as unknown as RTask), cell: sourceCell },
    { id: "critical", header: t("p.plans.pid.schedule_risk.critical_in"), accessor: (r) => t("p.plans.pid.schedule_risk.of_runs", { criticalityText: criticalityText((r as unknown as RTask).criticality_bp) }) },
    { id: "sensitivity", header: t("p.plans.pid.schedule_risk.sensitivity"), accessor: (r) => sensitivityText((r as unknown as RTask).sensitivity) },
  ]);
</script>

{#snippet sourceCell({ row, formatted }: DataGridCellContext)}{#if row.source === "fallback"}<span aria-hidden="true">▲</span> {/if}{formatted}{/snippet}


<svelte:head><title>{t("p.plans.pid.schedule_risk.schedule_risk_ppm")}</title></svelte:head>

<h1>{t("p.plans.pid.schedule_risk.schedule_risk")}</h1>
<p class="lede">{t("p.plans.pid.schedule_risk.many_what_if_runs_of")}</p>
<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plans.pid.schedule_risk.plan")}</a> · <a href={localeHref(`/plans/${pid}/critical-path`)}>{t("p.plans.pid.schedule_risk.critical_path")}</a></p>

{#if error}<p class="banner" role="alert">{error}</p>{/if}

<form class="filter" onsubmit={(e) => { e.preventDefault(); void simulate(); }}>
  <label>{t("p.plans.pid.schedule_risk.iterations")} <input type="number" min="1" max="20000" step="1" bind:value={iterations} /></label>
  <label>{t("p.plans.pid.schedule_risk.seed")} <input type="number" min="0" step="1" bind:value={seed} /></label>
  <label>{t("p.plans.pid.schedule_risk.distribution")}
    <select bind:value={distribution}>
      <option value="triangular">{t("p.plans.pid.schedule_risk.triangular")}</option>
      <option value="pert">{t("p.plans.pid.schedule_risk.weighted_towards_the_likely_value")}</option>
    </select>
  </label>
  <label>{t("p.plans.pid.schedule_risk.band_for_tasks_with_no")} <input type="number" min="0" max="100" step="1" bind:value={band} /></label>
  <button class="button small primary" type="submit" disabled={busy}>{busy ? t("p.plans.pid.schedule_risk.running") : t("p.plans.pid.schedule_risk.run")}</button>
</form>

{#if run}
  <p class="small muted">
    {t("p.plans.pid.schedule_risk.run_on_against_the_plan", { ranOn, seed: run.assumptions.seed, iterations: run.assumptions.iterations, distributionText: distributionText(run.assumptions), calendar: run.calendar })}
  </p>

  <h2>{t("p.plans.pid.schedule_risk.when_it_finishes")}</h2>
    <DataTable label={t("p.plans.pid.schedule_risk.finish_date_at_each_percentile")} columns={finishColumns} rows={finishRows(run)} rowId={(r) => String(r.label)} sortable={false} />
  <p>{confidenceText(run.cpm_finish_confidence_bp, run.cpm_finish)}</p>
  <p class="small muted">{t("p.plans.pid.schedule_risk.earliest_latest", { join: run.percentiles.map(percentileText).join(" · "), earliest: run.earliest, latest: run.latest })}</p>

  {#if g}
    <figure>
      <svg viewBox="0 0 {g.width} {g.height}" role="img" aria-label={t("p.plans.pid.schedule_risk.histogram_of_simulated_finish_dates")} width="100%" style:max-width="44rem">
        {#each g.bars as b (b.date)}
          <rect x={b.x} y={b.y} width={b.w} height={b.h} class="bar"><title>{t("p.plans.pid.schedule_risk.of", { date: b.date, count: b.count, iterations: run.assumptions.iterations })}</title></rect>
        {/each}
        <line x1="24" x2={g.width - 24} y1={g.baseline} y2={g.baseline} class="axis" />
        {#if g.cpmX !== null}
          <line x1={g.cpmX} x2={g.cpmX} y1="14" y2={g.baseline} class="cpm" />
          <text x={g.cpmX} y="10" text-anchor="middle" class="label">{t("p.plans.pid.schedule_risk.plan_2")}</text>
        {/if}
        {#each g.markers as m (m.p)}
          <line x1={m.x} x2={m.x} y1="14" y2={g.baseline} class={`pct p${m.p}`} />
          <text x={m.x} y={g.baseline + 14} text-anchor="middle" class="label">P{m.p}</text>
        {/each}
      </svg>
      <details>
        <summary>{t("p.plans.pid.schedule_risk.the_same_figures_as_a")}</summary>
        <DataTable label={t("p.plans.pid.schedule_risk.finish_date")} columns={histogramColumns(run.assumptions.iterations)} rows={run.histogram} rowId={(r) => String(r.date)} sortable={false} />
      </details>
      <figcaption class="small muted">{t("p.plans.pid.schedule_risk.each_bar_is_a_finish")}</figcaption>
    </figure>
  {/if}

  <h2>{t("p.plans.pid.schedule_risk.what_drives_it")}</h2>
    <DataTable label={t("p.plans.pid.schedule_risk.per_task_estimate_criticality_and")} columns={taskColumns} rows={[...run.tasks].sort((a, b) => b.criticality_bp - a.criticality_bp)} rowId={(r) => String(r.pid)} sortable={false} />

  <h2>{t("p.plans.pid.schedule_risk.risks_used")}</h2>
  {#if run.events.used.length === 0}
    <p>{t("p.plans.pid.schedule_risk.no_register_risk_was_used")}</p>
  {:else}
    <ul>
      {#each run.events.used as e (e.risk_pid)}
        <li>{(e.impact_days === 1 ? t("p.plans.pid.schedule_risk.chance_of_adding_business_day_one", { title: e.title, probability_percent: e.probability_percent, impact_days: e.impact_days }) : t("p.plans.pid.schedule_risk.chance_of_adding_business_day_other", { title: e.title, probability_percent: e.probability_percent, impact_days: e.impact_days }))}</li>
      {/each}
    </ul>
  {/if}
  {#if run.events.skipped.length > 0}
    <p class="small">{t("p.plans.pid.schedule_risk.left_out")}</p>
    <ul class="small">
      {#each run.events.skipped as e (e.risk_pid)}
        <li>{e.title}: {skippedReasonText(e.reason)}</li>
      {/each}
    </ul>
  {/if}
  {#if run.events.risks_without_schedule_fields > 0}
    <p class="small muted">{run.events.risks_without_schedule_fields === 1 ? t("p.plans.pid.schedule_risk.risks_without_schedule_one", { count: run.events.risks_without_schedule_fields }) : t("p.plans.pid.schedule_risk.risks_without_schedule_other", { count: run.events.risks_without_schedule_fields })}</p>
  {/if}
  {#if run.unscheduled.length > 0}
    <p class="small">{t("p.plans.pid.schedule_risk.not_simulated_because_the_schedule", { join: run.unscheduled.map((u) => `${u.title ?? u.pid} (${u.reason})`).join(", ") })}</p>
  {/if}

  <h2>{t("p.plans.pid.schedule_risk.assumptions")}</h2>
  <ul class="small">
    <li><strong>{t("p.plans.pid.schedule_risk.independence")}</strong> {run.assumptions.independence}</li>
    <li>{run.assumptions.durations}</li>
    <li>{run.assumptions.fallback}</li>
    <li>{t("p.plans.pid.schedule_risk.seed_the_same_seed_gives", { seed: run.assumptions.seed, max_iterations: run.assumptions.max_iterations })}</li>
  </ul>
  <p class="small muted">{run.note}</p>
{:else if !error}
  <p class="muted">{t("p.plans.pid.schedule_risk.running")}</p>
{/if}

<style>
  .bar {
    fill: var(--mxi-color-accent, #4a6fa5);
  }
  .axis {
    stroke: currentColor;
    stroke-width: 1;
  }
  .pct {
    stroke: currentColor;
    stroke-width: 2;
  }
  .pct.p80 {
    stroke-dasharray: 8 3;
  }
  .pct.p90 {
    stroke-dasharray: 2 3;
  }
  .cpm {
    stroke: currentColor;
    stroke-width: 1;
    stroke-dasharray: 4 4;
  }
  .label {
    font-size: 11px;
    fill: currentColor;
  }
  .filter {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: end;
    margin: 0.75rem 0;
  }
  .filter label {
    display: grid;
    gap: 0.2rem;
    font-size: 0.85rem;
  }
</style>
