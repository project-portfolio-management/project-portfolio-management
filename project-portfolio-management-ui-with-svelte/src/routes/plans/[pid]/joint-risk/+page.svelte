<!--
  Cost and schedule together (`/plans/[pid]/joint-risk`): one simulation in which
  the finish and the cost move together. A risk with both a schedule and a cost
  impact strikes once and drives both. A late finish adds a delay cost you set
  (zero by default). The headline is the joint confidence — both within budget
  and on time — which is never above either on its own. The quadrants are a table
  and the scatter has the same numbers beneath it.
-->
<script lang="ts">
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { PpmClient, money, type JointRisk } from "#lib/api/ppm.js";
  import {
    budgetConfidenceText,
    confidenceText,
    costSkippedText,
    delayCostText,
    jointConfidenceText,
    lineSourceText,
    percentileText,
    quadrantRows,
    scatterGeometry,
    skippedReasonText,
  } from "#lib/scheduling-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");
  let result = $state<JointRisk | null>(null);
  let error = $state<string | null>(null);
  let iterations = $state(2000);
  let seed = $state(1);
  let distribution = $state<"triangular" | "pert">("triangular");
  let band = $state(20);
  let currency = $state("");
  let delayCost = $state(0);
  let busy = $state(false);

  async function run() {
    busy = true;
    try {
      result = await ppm.jointRisk(pid, {
        iterations,
        seed,
        distribution,
        fallbackBandPercent: band,
        currency: currency.trim().toUpperCase() || undefined,
        delayCostPerDayMinor: Math.round(delayCost * 100),
      });
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.plans.pid.joint_risk.could_not_run_the_simulation");
    } finally {
      busy = false;
    }
  }
  onMount(() => void run());

  const scatter = $derived(result ? scatterGeometry(result.points, result.cost.budget_minor) : null);

  const quadrantColumns: DataGridColumn[] = $derived([
    { id: "label", header: t("p.plans.pid.joint_risk.outcome"), rowHeader: true, cell: quadrantLabel },
    { id: "count", header: t("p.plans.pid.joint_risk.simulations"), align: "end" },
    { id: "share", header: t("p.plans.pid.joint_risk.share"), align: "end" },
  ]);
  const costColumns: DataGridColumn[] = $derived([
    { id: "label", header: t("p.plans.pid.joint_risk.outcome"), rowHeader: true },
    { id: "value", header: t("p.plans.pid.joint_risk.cost"), align: "end" },
  ]);
  const finishColumns: DataGridColumn[] = $derived([
    { id: "label", header: t("p.plans.pid.joint_risk.outcome"), rowHeader: true },
    { id: "value", header: t("p.plans.pid.joint_risk.finish"), align: "end" },
  ]);
  type Line = JointRisk["cost"]["lines"][number];
  const lineColumns = (currencyOf: string): DataGridColumn[] => [
    { id: "description", header: t("p.plans.pid.joint_risk.line"), rowHeader: true, accessor: (r) => (r as unknown as Line).description ?? (r as unknown as Line).pid },
    { id: "range", header: t("p.plans.pid.joint_risk.low_planned_high"), sortable: false, accessor: (r) => { const l = r as unknown as Line; return `${money(l.low, currencyOf)} / ${money(l.likely, currencyOf)} / ${money(l.high, currencyOf)}`; } },
    { id: "estimate", header: t("p.plans.pid.joint_risk.estimate"), accessor: (r) => lineSourceText(r as unknown as Line), cell: lineSourceCell },
  ];
  const costRows = (result: JointRisk) => [
    { label: t("p.plans.pid.joint_risk.budget_2"), value: money(result.cost.budget_minor, result.currency) },
    { label: t("p.plans.pid.joint_risk.mean_delay_cost_included"), value: money(result.cost.mean_minor, result.currency) },
    { label: t("p.plans.pid.joint_risk.of_which_delay_cost_on"), value: money(result.cost.mean_delay_cost_minor, result.currency) },
    ...result.cost.percentiles.map((p) => ({ label: `P${p.p}`, value: money(p.minor, result.currency) })),
  ];
  const finishRows = (result: JointRisk) => [
    ...result.schedule.percentiles.map((p) => ({ label: `P${p.p}`, value: p.finish })),
    { label: t("p.plans.pid.joint_risk.critical_path_finish_the_plan"), value: result.schedule.cpm_finish ?? t("p.plans.pid.joint_risk.not_enough_data") },
  ];
</script>

{#snippet quadrantLabel({ row, formatted }: DataGridCellContext)}<span aria-hidden="true">{row.symbol}</span> {formatted}{/snippet}

{#snippet lineSourceCell({ row, formatted }: DataGridCellContext)}{#if row.source === "fallback"}<span aria-hidden="true">▲</span> {/if}{formatted}{/snippet}

<svelte:head><title>{t("p.plans.pid.joint_risk.cost_and_schedule_risk_ppm")}</title></svelte:head>

<h1>{t("p.plans.pid.joint_risk.cost_and_schedule_together")}</h1>
<p class="lede">{t("p.plans.pid.joint_risk.a_late_project_usually_costs")}</p>
<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plans.pid.joint_risk.plan")}</a> · <a href={localeHref(`/plans/${pid}/schedule-risk`)}>{t("p.plans.pid.joint_risk.schedule_risk")}</a> · <a href={localeHref(`/plans/${pid}/cost-risk`)}>{t("p.plans.pid.joint_risk.cost_risk")}</a></p>

{#if error}<p class="banner" role="alert">{error}</p>{/if}

<form class="filter" onsubmit={(e) => { e.preventDefault(); void run(); }}>
  <label>{t("p.plans.pid.joint_risk.currency_if_the_plan_has")} <input type="text" maxlength="3" bind:value={currency} placeholder={t("p.plans.pid.joint_risk.gbp")} /></label>
  <label>{t("p.plans.pid.joint_risk.cost_of_each_business_day")} <input type="number" min="0" step="0.01" bind:value={delayCost} /></label>
  <label>{t("p.plans.pid.joint_risk.iterations")} <input type="number" min="1" max="20000" step="1" bind:value={iterations} /></label>
  <label>{t("p.plans.pid.joint_risk.seed")} <input type="number" min="0" step="1" bind:value={seed} /></label>
  <label>{t("p.plans.pid.joint_risk.distribution")}
    <select bind:value={distribution}><option value="triangular">{t("p.plans.pid.joint_risk.triangular")}</option><option value="pert">{t("p.plans.pid.joint_risk.pert")}</option></select>
  </label>
  <label>{t("p.plans.pid.joint_risk.band_when_there_is_no")} <input type="number" min="0" max="100" step="1" bind:value={band} /></label>
  <button class="button small primary" type="submit" disabled={busy}>{busy ? t("p.plans.pid.joint_risk.running") : t("p.plans.pid.joint_risk.run")}</button>
</form>

{#if result}
  {@const a = result.assumptions}
  <p class="small muted">
    {t("p.plans.pid.joint_risk.run_summary", { currency: result.currency, seed: a.seed, iterations: a.iterations, distribution: a.distribution === "pert" ? t("p.plans.pid.joint_risk.dist_pert") : t("p.plans.pid.joint_risk.dist_triangular"), calendar: result.calendar })}
  </p>
  {#if result.other_currencies_not_simulated.length > 0}
    <p class="banner" role="note">{t("p.plans.pid.joint_risk.budget_lines_in_are_not", { join: result.other_currencies_not_simulated.join(", ") })}</p>
  {/if}

  <h2>{t("p.plans.pid.joint_risk.both_at_once")}</h2>
  <p><strong>{jointConfidenceText(result.confidence)}</strong></p>
    <DataTable label={t("p.plans.pid.joint_risk.how_the_simulated_outcomes_fall")} columns={quadrantColumns} rows={quadrantRows(result.quadrants)} rowId={(r) => String(r.key)} sortable={false} />

  {#if scatter}
    <figure>
      <svg viewBox="0 0 {scatter.width} {scatter.height}" role="img" aria-label={t("p.plans.pid.joint_risk.scatter_of_simulated_total_cost")} width="100%" style="max-width: 32rem">
        {#each scatter.dots as d, i (i)}
          <circle cx={d.x} cy={d.y} r="2.5" class="dot"><title>{t("p.plans.pid.joint_risk.business_days_late", { days: d.days, money: money(d.cost, result.currency) })}</title></circle>
        {/each}
        <line x1={scatter.onTimeX} x2={scatter.onTimeX} y1="16" y2={scatter.height - 32} class="ref" />
        <text x={scatter.onTimeX} y="12" text-anchor="middle" class="label">{t("p.plans.pid.joint_risk.on_time")}</text>
        <line x1="32" x2={scatter.width - 32} y1={scatter.budgetY} y2={scatter.budgetY} class="ref budget" />
        <text x={scatter.width - 32} y={scatter.budgetY - 4} text-anchor="end" class="label">{t("p.plans.pid.joint_risk.budget")}</text>
        <text x={scatter.width / 2} y={scatter.height - 6} text-anchor="middle" class="label">{t("p.plans.pid.joint_risk.business_days_late_2")}</text>
      </svg>
      <figcaption class="small muted">{t("p.plans.pid.joint_risk.each_dot_is_one_simulation")}</figcaption>
    </figure>
  {/if}

  <h2>{t("p.plans.pid.joint_risk.cost")}</h2>
    <DataTable label={t("p.plans.pid.joint_risk.total_cost_delay_cost_included")} columns={costColumns} rows={costRows(result)} rowId={(r) => String(r.label)} sortable={false} />
  <p>{budgetConfidenceText(result.confidence.budget_bp, money(result.cost.budget_minor, result.currency))}</p>
  <p class="small">{delayCostText(a.delay_cost.per_day_minor, a.delay_cost.currency, money)}</p>

  <h2>{t("p.plans.pid.joint_risk.finish")}</h2>
    <DataTable label={t("p.plans.pid.joint_risk.finish_date_at_each_percentile")} columns={finishColumns} rows={finishRows(result)} rowId={(r) => String(r.label)} sortable={false} />
  <p>{confidenceText(result.confidence.finish_bp, result.schedule.cpm_finish)}</p>
  <p class="small muted">{result.schedule.percentiles.map(percentileText).join(" · ")}</p>

  <h2>{t("p.plans.pid.joint_risk.cost_lines")}</h2>
    <DataTable label={t("p.plans.pid.joint_risk.per_line_estimate_in", { currency: result.currency })} columns={lineColumns(result.currency)} rows={result.cost.lines} rowId={(r) => String(r.pid)} sortable={false} />

  <h2>{t("p.plans.pid.joint_risk.risks")}</h2>
  {#if result.assumptions.shared_events.length > 0}
    <p class="small">{(result.assumptions.shared_events.length === 1 ? t("p.plans.pid.joint_risk.risk_affect_both_the_schedule_one", { length: result.assumptions.shared_events.length }) : t("p.plans.pid.joint_risk.risk_affect_both_the_schedule_other", { length: result.assumptions.shared_events.length }))}</p>
  {/if}
  {#if result.events.schedule.skipped.length + result.events.cost.skipped.length > 0}
    <ul class="small">
      {#each result.events.schedule.skipped as e (`s-${e.risk_pid}`)}<li>{t("p.plans.pid.joint_risk.schedule", { title: e.title, skippedReasonText: skippedReasonText(e.reason) })}</li>{/each}
      {#each result.events.cost.skipped as e (`c-${e.risk_pid}`)}<li>{t("p.plans.pid.joint_risk.cost_2", { title: e.title, costSkippedText: costSkippedText(e.reason) })}</li>{/each}
    </ul>
  {:else if result.events.schedule.used.length + result.events.cost.used.length === 0}
    <p class="small muted">{t("p.plans.pid.joint_risk.no_register_risk_was_used")}</p>
  {/if}

  <h2>{t("p.plans.pid.joint_risk.assumptions")}</h2>
  <ul class="small">
    <li><strong>{t("p.plans.pid.joint_risk.independence")}</strong> {a.independence}</li>
    <li>{a.shared_events_rule}</li>
    <li>{a.delay_cost.rule}</li>
    <li>{a.floor}</li>
    <li>{a.fallback}</li>
    <li>{t("p.plans.pid.joint_risk.seed_the_same_seed_gives", { seed: a.seed })}</li>
  </ul>
  <p class="small muted">{result.note}</p>
{:else if !error}
  <p class="muted">{t("p.plans.pid.joint_risk.running")}</p>
{/if}

<style>
  .dot {
    fill: var(--mxi-color-accent, #4a6fa5);
    fill-opacity: 0.55;
  }
  .ref {
    stroke: currentColor;
    stroke-width: 1;
    stroke-dasharray: 5 4;
  }
  .ref.budget {
    stroke-dasharray: 2 3;
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
