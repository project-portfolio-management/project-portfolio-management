<!--
  Time-based analysis (`/plans/{pid}/flow`) — the delivery board read
  through the lens of time rather than of activity.

  The page is ordered by what an operator can act on, not by what is
  easiest to compute: the service level expectation and today's aging
  work in progress come first, the cumulative flow diagram and the
  constraint ranking next, and the distributions last. Cycle time and
  lead time are always shown together — quoting the first as "our
  delivery time" is the commonest misreport in flow measurement, and it
  always flatters.

  Nothing here is per-person. Assignees appear on an aging item so the
  reader knows who to ask, never as a ranked comparison; see the entity
  spec's `time-based-analysis.md` §12.4 for why that is a decision
  rather than a missing feature.

  English-first, like the other PPM views.
-->
<script lang="ts">
  import { rowOf } from "#lib/row-of.js";
  import { errorText } from "#lib/error-text.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { page } from "$app/state";
  import CumulativeFlow from "#lib/components/CumulativeFlow.svelte";
  import {
    TbaClient,
    days,
    flowEfficiencyBand,
    interpretationLabel,
    msAsDays,
    percent,
    type AgingWip,
    type Constraints,
    type CumulativeFlow as CumulativeFlowData,
    type PlanFlow,
    type PlanTimeAnalysis,
  } from "#lib/api/tba.js";

  // Reactive, so the page follows the URL to another plan (CQ-18); the load effect reads it.
  const pid = $derived(page.params.pid ?? "");
  const tba = TbaClient.withFetch();

  let analysis = $state<PlanTimeAnalysis | null>(null);
  let aging = $state<AgingWip | null>(null);
  let constraints = $state<Constraints | null>(null);
  let flow = $state<PlanFlow | null>(null);
  let cfd = $state<CumulativeFlowData | null>(null);
  let windowDays = $state(60);
  let error = $state<string | null>(null);

  // The effect re-runs when `windowDays` changes, so the window picker needs no load of its own
  // (CQ-17); a late answer for an older window is dropped rather than shown over a newer one.
  let latest = 0;
  async function load(days: number) {
    const mine = ++latest;
    try {
      const answers = await Promise.all([
        tba.planTimeAnalysis(pid),
        tba.agingWip(pid),
        tba.constraints(pid),
        tba.flow(pid, days),
        tba.cumulativeFlow(pid, days),
      ]);
      if (mine !== latest) return;
      [analysis, aging, constraints, flow, cfd] = answers;
      error = null;
    } catch (err) {
      if (mine === latest) error = errorText(err, "p.plans.pid.flow.could_not_load_flow_data");
    }
  }
  $effect(() => {
    void load(windowDays);
  });

  const sle = $derived(analysis?.service_level_expectation ?? null);
  const summary = $derived(analysis?.plan_analysis ?? null);
  const efficiency = $derived(summary?.aggregate_flow_efficiency.value ?? null);
  const band = $derived(flowEfficiencyBand(efficiency));

  /** Items already past the expectation — the ones worth a conversation today. */
  const overdue = $derived((aging?.aging ?? []).filter((row) => row.aging.past_sle));

  type Aging = AgingWip["aging"][number];
  type FlowColumn = PlanFlow["columns"][number];
  const agingColumns: DataGridColumn[] = $derived([
    { id: "title", header: t("p.plans.pid.flow.item"), rowHeader: true, accessor: (r) => rowOf<Aging>(r).task.title, cell: agingTitle },
    { id: "status", header: t("p.plans.pid.flow.status") },
    { id: "age", header: t("p.plans.pid.flow.age"), align: "end", accessor: (r) => rowOf<Aging>(r).aging.age_days, format: (v) => days(Number(v)) },
    { id: "sle", header: t("p.plans.pid.flow.vs_expectation"), align: "end", accessor: (r) => rowOf<Aging>(r).aging.sle_ratio ?? -1, cell: sleCell },
    { id: "blocked", header: t("p.plans.pid.flow.blocked"), align: "end", accessor: (r) => rowOf<Aging>(r).blocked_time_ms, format: (v) => msAsDays(Number(v)) },
    { id: "rework_count", header: t("p.plans.pid.flow.rework"), align: "end" },
    { id: "assignee", header: t("p.plans.pid.flow.assignee"), accessor: (r) => rowOf<Aging>(r).task.assignee_ref ?? t("p.plans.pid.flow.unassigned") },
  ]);
  const constraintColumns: DataGridColumn[] = $derived([
    { id: "rule", header: t("p.plans.pid.flow.rule"), rowHeader: true, cell: ruleCell },
    { id: "subject", header: t("p.plans.pid.flow.subject") },
    { id: "recoverable_days", header: t("p.plans.pid.flow.recoverable"), align: "end", format: (v) => days(v as number | null) },
    { id: "detail", header: t("p.plans.pid.flow.detail") },
  ]);
  const columnColumns: DataGridColumn[] = $derived([
    { id: "status", header: t("p.plans.pid.flow.column"), rowHeader: true },
    { id: "count", header: t("p.plans.pid.flow.count"), align: "end" },
    { id: "limit", header: t("p.plans.pid.flow.limit"), align: "end", accessor: (r) => rowOf<FlowColumn>(r).limit ?? "", cell: limitCell },
  ]);
  const distributionRows = (s: NonNullable<typeof summary>) => [{ label: "Cycle time", d: s.cycle_time }, { label: "Lead time", d: s.lead_time }];
  type Dist = ReturnType<typeof distributionRows>[number];
  const distributionColumns: DataGridColumn[] = $derived([
    { id: "label", header: t("p.plans.pid.flow.measure"), rowHeader: true },
    { id: "n", header: "n", align: "end", accessor: (r) => rowOf<Dist>(r).d?.n ?? "—" },
    { id: "min", header: t("p.plans.pid.flow.min"), align: "end", accessor: (r) => msAsDays(rowOf<Dist>(r).d?.min_ms ?? null) },
    { id: "p50", header: "p50", align: "end", accessor: (r) => msAsDays(rowOf<Dist>(r).d?.p50_ms ?? null) },
    { id: "p85", header: "p85", align: "end", accessor: (r) => msAsDays(rowOf<Dist>(r).d?.p85_ms ?? null) },
    { id: "p95", header: "p95", align: "end", accessor: (r) => msAsDays(rowOf<Dist>(r).d?.p95_ms ?? null) },
    { id: "max", header: t("p.plans.pid.flow.max"), align: "end", accessor: (r) => msAsDays(rowOf<Dist>(r).d?.max_ms ?? null) },
    { id: "mean", header: t("p.plans.pid.flow.mean"), align: "end", accessor: (r) => msAsDays(rowOf<Dist>(r).d?.mean_ms ?? null) },
  ]);
</script>

{#snippet agingTitle({ formatted }: DataGridCellContext)}<a href={localeHref(`/plans/${pid}/board`)}>{formatted}</a>{/snippet}

{#snippet sleCell({ row }: DataGridCellContext)}
  {@const a = rowOf<Aging>(row).aging}
  {#if a.sle_ratio === null}<span class="muted">{t("p.plans.pid.flow.no_expectation_yet")}</span>{:else}{a.past_sle ? "⚠ " : ""}{percent(a.sle_ratio)}{/if}
{/snippet}

{#snippet ruleCell({ formatted }: DataGridCellContext)}<code>{formatted}</code>{/snippet}

{#snippet limitCell({ row }: DataGridCellContext)}
  {@const c = rowOf<FlowColumn>(row)}
  {#if c.limit === null}<span class="muted">{t("p.plans.pid.flow.no_cap")}</span>{:else}{c.over_limit ? "⚠ " : ""}{c.limit}{/if}
{/snippet}


<svelte:head><title>{t("p.plans.pid.flow.flow_ppm")}</title></svelte:head>

<h1>{t("p.plans.pid.flow.how_long_work_takes")}</h1>
<p class="lede">{t("p.plans.pid.flow.how_long_work_takes_to")}</p>
<p class="muted">
  {t("p.plans.pid.flow.of_the_calendar_time_these")}
</p>

{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if analysis && summary}
  <!-- The headline row: an expectation you can quote, and the ratio
       behind it. Stat tiles rather than charts — each is one number. -->
  <div class="tiles">
    <div class="tile" data-testid="sle-badge">
      <span class="tile-label">{t("p.plans.pid.flow.service_level_expectation")}</span>
      {#if sle?.within_days !== null && sle?.within_days !== undefined}
        <span class="tile-value">{days(sle.within_days)}</span>
        <span class="tile-note">
          {(sle.sample === 1 ? t("p.plans.pid.flow.of_items_finish_within_this_one", { percent: percent(sle.percentile), sample: sle.sample }) : t("p.plans.pid.flow.of_items_finish_within_this_other", { percent: percent(sle.percentile), sample: sle.sample }))}
        </span>
      {:else}
        <span class="tile-value muted">—</span>
        <span class="tile-note">{sle?.reason ?? t("p.plans.pid.flow.not_enough_finished_items_yet")}</span>
      {/if}
    </div>

    <div class="tile" data-testid="flow-efficiency">
      <span class="tile-label">{t("p.plans.pid.flow.flow_efficiency")}</span>
      <span class="tile-value">{percent(efficiency, 1)}</span>
      <span class="tile-note">
        {#if band === "suspicious"}
          {t("p.plans.pid.flow.unusually_high_this_usually_means")}
        {:else if band === "strong"}
          {t("p.plans.pid.flow.strong_against_the_5_15")}
        {:else if band === "typical"}
          {t("p.plans.pid.flow.typical_median_item", { percent: percent(summary.median_flow_efficiency, 1) })}
        {:else}
          {t("p.plans.pid.flow.no_finished_work_to_measure")}
        {/if}
      </span>
    </div>

    <div class="tile">
      <span class="tile-label">{t("p.plans.pid.flow.first_pass_yield")}</span>
      <span class="tile-value">{percent(summary.rolled_first_pass_yield)}</span>
      <span class="tile-note">
        {(summary.rework_count === 1 ? t("p.plans.pid.flow.finished_without_ever_moving_backwar_one", { rework_count: summary.rework_count }) : t("p.plans.pid.flow.finished_without_ever_moving_backwar_other", { rework_count: summary.rework_count }))}
      </span>
    </div>

    <div class="tile">
      <span class="tile-label">{t("p.plans.pid.flow.work_in_progress")}</span>
      <span class="tile-value">{summary.work_in_progress}</span>
      <span class="tile-note">
        {t("p.plans.pid.flow.not_started_finished_of", { not_started: summary.not_started, finished: summary.finished, tasks: summary.tasks })}
      </span>
    </div>
  </div>

  {#if summary.waste_shape === "concentrated"}
    <p class="finding">
      {t("p.plans.pid.flow.the_waste_is")} <strong>{t("p.plans.pid.flow.concentrated")}</strong>{t("p.plans.pid.flow.the_plan_s_overall_ratio", { percent: percent(efficiency, 1), percent2: percent(summary.median_flow_efficiency, 1) })}
    </p>
  {/if}

  {#if summary.backfilled_ratio !== null && summary.backfilled_ratio > 0}
    <p class="finding muted">
      {t("p.plans.pid.flow.of_the_transitions_behind_these", { percent: percent(summary.backfilled_ratio) })}
    </p>
  {/if}
{/if}

<!-- Aging WIP: the only view here about work that can still be helped. -->
<h2>{t("p.plans.pid.flow.aging_work_in_progress")}</h2>
{#if aging}
  <p class="muted">{aging.note}</p>
  {#if aging.aging.length === 0}
    <p class="muted">{t("p.plans.pid.flow.nothing_in_progress")}</p>
  {:else}
    <DataTable label={t("p.plans.pid.flow.item")} columns={agingColumns} rows={aging.aging} rowId={(r) => String(rowOf<Aging>(r).task.pid)} testid="aging-wip" />
    {#if overdue.length > 0}
      <p class="finding">
        {overdue.length === 1 ? t("p.plans.pid.flow.overdue_one", { count: overdue.length }) : t("p.plans.pid.flow.overdue_other", { count: overdue.length })}
      </p>
    {/if}
  {/if}
{/if}

<!-- The picture. -->
<h2>{t("p.plans.pid.flow.cumulative_flow")}</h2>
<label class="window">
  {t("p.plans.pid.flow.window")}
  <select
    value={String(windowDays)}
    onchange={(event) => {
      windowDays = Number.parseInt(event.currentTarget.value, 10);
    }}
  >
    <option value="30">{t("p.plans.pid.flow.30_calendar_days")}</option>
    <option value="60">{t("p.plans.pid.flow.60_calendar_days")}</option>
    <option value="90">{t("p.plans.pid.flow.90_calendar_days")}</option>
    <option value="180">{t("p.plans.pid.flow.180_calendar_days")}</option>
  </select>
</label>
{#if cfd}
  <CumulativeFlow samples={cfd.samples} note={cfd.note} />
{/if}

<!-- Where the time goes. -->
<h2>{t("p.plans.pid.flow.constraints")}</h2>
{#if constraints}
  <p class="muted">{constraints.note}</p>
    <DataTable label={t("p.plans.pid.flow.rule")} columns={constraintColumns} rows={constraints.findings} rowId={(r) => String(r.rule) + String(r.subject)} emptyText={t("p.plans.pid.flow.no_findings_yet")} testid="constraints" />
{/if}

<!-- Little's Law, and the columns it points at. -->
<h2>{t("p.plans.pid.flow.flow")}</h2>
{#if flow}
  <p class="finding" data-testid="littles-law">
    <strong>{interpretationLabel(flow.flow.interpretation)}.</strong>
    {flow.flow.detail}
  </p>
  <table data-testid="flow-rates">
    <tbody>
      <tr>
        <th scope="row">{t("p.plans.pid.flow.arrival_rate")}</th>
        <td>{t("p.plans.pid.flow.calendar_day", { v: flow.flow.arrival_rate_per_day?.toFixed(2) ?? "—" })}</td>
        <th scope="row">{t("p.plans.pid.flow.throughput")}</th>
        <td>{t("p.plans.pid.flow.calendar_day", { v: flow.flow.throughput_per_day?.toFixed(2) ?? "—" })}</td>
      </tr>
      <tr>
        <th scope="row">{t("p.plans.pid.flow.utilization")}</th>
        <td>
          {flow.flow.utilization?.toFixed(2) ?? "—"}
          {#if flow.flow.utilization_reason}
            <span class="muted">— {flow.flow.utilization_reason}</span>
          {/if}
        </td>
        <th scope="row">{t("p.plans.pid.flow.work_in_progress_2")}</th>
        <td>{flow.flow.work_in_progress}</td>
      </tr>
      <tr>
        <th scope="row">{t("p.plans.pid.flow.implied_cycle_time")}</th>
        <td>{days(flow.flow.implied_cycle_time_days)}</td>
        <th scope="row">{t("p.plans.pid.flow.observed_median")}</th>
        <td>{days(flow.flow.observed_p50_cycle_time_days)}</td>
      </tr>
    </tbody>
  </table>

  <h3>{t("p.plans.pid.flow.columns")}</h3>
  <p class="muted">
    {t("p.plans.pid.flow.cycle_time_is_work_in")}
  </p>
    <DataTable label={t("p.plans.pid.flow.column")} columns={columnColumns} rows={flow.columns} rowId={(r) => String(r.status)} sortable={false} testid="columns" />
{/if}

<!-- The distributions last: history, not something to act on today. -->
{#if summary?.cycle_time && summary?.lead_time}
  <h2>{t("p.plans.pid.flow.distributions")}</h2>
  <p class="muted">
    {t("p.plans.pid.flow.percentiles_are_nearest_rank_so")}
  </p>
    <DataTable label={t("p.plans.pid.flow.measure")} columns={distributionColumns} rows={distributionRows(summary)} rowId={(r) => String(r.label)} sortable={false} testid="distributions" />
  <p class="muted">
    {t("p.plans.pid.flow.the_mean_is_shown_for")}
  </p>
{/if}

{#if analysis}
  <p class="muted classes">
    {t("p.plans.pid.flow.classified_with", { map: analysis.classification.overridden ? t("p.plans.pid.flow.map_override") : t("p.plans.pid.flow.map_default") })}
    {#each Object.entries(analysis.classification.classes) as [status, category], i (status)}{i >
      0
        ? " · "
        : ""}{status} → {category}{/each}
  </p>
{/if}

<style>
  .tiles {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(15rem, 1fr));
    gap: 1rem;
    margin: 1rem 0;
  }
  .tile {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    padding: 0.75rem 1rem;
    border-radius: 6px;
    border: 1px solid rgb(128 128 128 / 0.25);
  }
  .tile-label {
    font-size: 0.8rem;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    opacity: 0.75;
  }
  .tile-value {
    font-size: 2rem;
    line-height: 1.1;
  }
  .tile-note {
    font-size: 0.8rem;
    opacity: 0.8;
  }
  .finding {
    margin: 0.75rem 0;
    padding: 0.6rem 0.8rem;
    border-left: 3px solid rgb(128 128 128 / 0.4);
    font-size: 0.9rem;
  }
  .window {
    display: inline-flex;
    gap: 0.5rem;
    align-items: center;
    margin-bottom: 0.5rem;
  }
  table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.9rem;
    margin-bottom: 1rem;
  }
  th,
  td {
    text-align: left;
    padding: 0.3rem 0.5rem;
    vertical-align: top;
  }
  td {
    font-variant-numeric: tabular-nums;
  }
  .muted {
    opacity: 0.75;
    font-size: 0.9rem;
  }
  .classes {
    font-size: 0.8rem;
  }
</style>
