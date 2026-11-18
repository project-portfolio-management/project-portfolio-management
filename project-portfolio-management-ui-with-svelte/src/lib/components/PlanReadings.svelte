<!--
  A plan's readings beside its schedule (`/plans/[pid]/critical-path`, NC-1 slice 12 part i, RM-5, SC-4), each read on
  asking: the flow across the plans it contains (`GET /api/plans/{pid}/rollup`, a depth of 32 not sent); the effort
  recorded, as people asserted it (`…/effort`); the delivery forecast from its own throughput (`…/forecast`, the
  defaults not sent); schedule and cost performance and the stakeholder score (`…/performance`); and its task
  workflow (`…/workflow`). Each query is checked first; the service's notes and reasons are English and never shown.
  Rules in `src/lib/plan-readings-view.ts`.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { i18n, t, tn, localeHref, type StringKey } from "#lib/i18n.svelte.js";
  import { arrow, formatDecimal, formatInteger, formatBasisPoints, formatPercent } from "#lib/locale-format.js";
  import { durationText } from "#lib/timesheet-view.js";
  import { rowOf } from "#lib/row-of.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import {
    PpmClient,
    type DeliveryForecast,
    type PlanEffort,
    type PlanFlowRollup,
    type PlanPerformance,
    type PlanWorkflow,
  } from "#lib/api/ppm.js";
  import {
    CATEGORY_KEY,
    FORECAST_DEFAULTS,
    ROLLUP_DEPTH_MAX,
    absentKey,
    forecastQuery,
    readingRefusalKey,
    rollupDepth,
    stateName,
    type ForecastForm,
    type Reading,
  } from "#lib/plan-readings-view.js";

  let { pid, titles }: { pid: string; titles: ReadonlyMap<string, string> } = $props();

  const ppm = PpmClient.withFetch();
  let rollup = $state<PlanFlowRollup | null>(null);
  let effort = $state<PlanEffort | null>(null);
  let forecast = $state<DeliveryForecast | null>(null);
  let performance = $state<PlanPerformance | null>(null);
  let workflow = $state<PlanWorkflow | null>(null);
  let depth = $state<number | null | undefined>(null);
  let fform = $state<ForecastForm>({ items: null, periods: null, periodDays: null, historyPeriods: null });
  let problems = $state<Partial<Record<Reading, string>>>({});
  let busy = $state<Reading | null>(null);

  function refused(reading: Reading, err: unknown, fallback: StringKey) {
    const key = readingRefusalKey(reading, err);
    problems = { ...problems, [reading]: key ? t(key) : errorText(err, fallback) };
  }

  async function read<T>(reading: Reading, run: () => Promise<T>, set: (v: T) => void) {
    busy = reading;
    problems = { ...problems, [reading]: undefined };
    try {
      set(await run());
    } catch (err) {
      refused(reading, err, "p.readings.could_not_read");
    } finally {
      busy = null;
    }
  }

  function readRollup(e: SubmitEvent) {
    e.preventDefault();
    const r = rollupDepth(depth);
    if ("problem" in r) {
      problems = { ...problems, rollup: t(r.problem, { max: ROLLUP_DEPTH_MAX }) };
      return;
    }
    void read("rollup", () => ppm.planFlowRollup(pid, r.depth), (v) => (rollup = v));
  }
  function readForecast(e: SubmitEvent) {
    e.preventDefault();
    const r = forecastQuery(fform);
    if ("problems" in r) {
      problems = { ...problems, forecast: r.problems.map((k) => t(k)).join(" ") };
      return;
    }
    void read("forecast", () => ppm.deliveryForecast(pid, r.query), (v) => (forecast = v));
  }

  const ratio = (v: number | null) => (v === null ? t("p.readings.not_measured") : formatPercent(v));
  const days = (v: number | null | undefined) => (v === null || v === undefined ? t("p.readings.not_measured") : t("p.readings.days", { n: formatDecimal(v, 1) }));
  type ByPlan = PlanFlowRollup["by_plan"][number];
  const rollupColumns: DataGridColumn[] = $derived([
    { id: "plan", header: t("p.readings.col_plan"), rowHeader: true, accessor: (r) => rowOf<ByPlan>(r).plan.name },
    { id: "depth", header: t("p.readings.col_depth"), align: "end" },
    { id: "tasks", header: t("p.readings.col_tasks"), align: "end" },
    { id: "finished", header: t("p.readings.col_finished"), align: "end" },
    { id: "work_in_progress", header: t("p.readings.col_wip"), align: "end" },
    { id: "eff", header: t("p.readings.col_efficiency"), accessor: (r) => ratio(rowOf<ByPlan>(r).flow_efficiency.value) },
  ]);
  type Who = PlanEffort["by_assignee"][number];
  type What = PlanEffort["by_task"][number];
  const whoColumns: DataGridColumn[] = $derived([
    { id: "actor_ref", header: t("p.readings.col_person"), rowHeader: true },
    { id: "total", header: t("p.readings.col_time"), accessor: (r) => durationText(rowOf<Who>(r).totals.minutes) },
    { id: "billable", header: t("p.readings.col_billable"), accessor: (r) => durationText(rowOf<Who>(r).totals.billable_minutes) },
  ]);
  const taskColumns: DataGridColumn[] = $derived([
    { id: "task", header: t("p.readings.col_task"), rowHeader: true, accessor: (r) => titles.get(rowOf<What>(r).task_pid) ?? t("p.readings.task_unnamed", { ref: rowOf<What>(r).task_pid.slice(0, 8) }) },
    { id: "total", header: t("p.readings.col_time"), accessor: (r) => durationText(rowOf<What>(r).totals.minutes) },
    { id: "billable", header: t("p.readings.col_billable"), accessor: (r) => durationText(rowOf<What>(r).totals.billable_minutes) },
  ]);
  const stateOf = (key: string) => {
    const s = workflow?.workflow.states.find((x) => x.key === key);
    return s && workflow ? stateName(s, workflow.source, i18n.locale.split("-")[0] ?? "en", t) : key;
  };
</script>

<section aria-labelledby="readings-h" data-testid="plan-readings">
  <h2 id="readings-h">{t("p.readings.heading")}</h2>
  <p class="small muted">{t("p.readings.says")}</p>

  <section aria-labelledby="rd-rollup">
    <h3 id="rd-rollup">{t("p.readings.rollup_heading")}</h3>
    <p class="small muted">{t("p.readings.rollup_says", { max: ROLLUP_DEPTH_MAX })}</p>
    <form class="row" novalidate onsubmit={readRollup} aria-label={t("p.readings.rollup_heading")}>
      <label>{t("p.readings.f_depth")}<input type="number" min="1" max={ROLLUP_DEPTH_MAX} step="1" bind:value={depth} /></label>
      <button class="button" type="submit" disabled={busy !== null}>{t("p.readings.read")}</button>
    </form>
    {#if problems.rollup}<p class="banner error" role="alert">{problems.rollup}</p>{/if}
    {#if rollup}
      {@const c = rollup.combined}
      <p>{tn("p.readings.rollup_plans", rollup.tree.plans, { n: formatInteger(rollup.tree.plans) })}
        {tn("p.readings.rollup_tasks", c.tasks, { n: formatInteger(c.tasks), finished: formatInteger(c.finished), wip: formatInteger(c.work_in_progress), not_started: formatInteger(c.not_started) })}</p>
      <p>{t("p.readings.rollup_times", { cycle: days(c.cycle_time?.p85_days), lead: days(c.lead_time?.p85_days), efficiency: ratio(c.aggregate_flow_efficiency.value) })}</p>
      {#if rollup.tree.truncated}<p class="banner" role="note">{t("p.readings.rollup_truncated")}</p>{/if}
      {#if rollup.tree.revisits > 0}<p class="small">{tn("p.readings.rollup_revisits", rollup.tree.revisits, { n: rollup.tree.revisits })}</p>{/if}
      <DataTable label={t("p.readings.rollup_table")} columns={rollupColumns} rows={rollup.by_plan} rowId={(r) => String(rowOf<ByPlan>(r).plan.pid)} />
    {/if}
  </section>

  <section aria-labelledby="rd-effort">
    <h3 id="rd-effort">{t("p.readings.effort_heading")}</h3>
    <p class="small muted">{t("p.readings.effort_says")}</p>
    <button class="button" type="button" disabled={busy !== null} onclick={() => void read("effort", () => ppm.planEffort(pid), (v) => (effort = v))}>{t("p.readings.read_effort")}</button>
    {#if problems.effort}<p class="banner error" role="alert">{problems.effort}</p>{/if}
    {#if effort}
      {@const p = effort.plan}
      {#if p.minutes === 0}
        <p>{t("p.readings.effort_none")}</p>
      {:else}
        <p>{t("p.readings.effort_total", { total: durationText(p.minutes), billable: durationText(p.billable_minutes) })}</p>
        <p class="small">{t("p.readings.effort_split", { capex: durationText(p.capex_minutes), opex: durationText(p.opex_minutes), unclassified: durationText(p.unclassified_minutes) })}</p>
        <DataTable label={t("p.readings.effort_by_person")} columns={whoColumns} rows={effort.by_assignee} rowId={(r) => String(rowOf<Who>(r).actor_ref)} />
        <DataTable label={t("p.readings.effort_by_task")} columns={taskColumns} rows={effort.by_task} rowId={(r) => String(rowOf<What>(r).task_pid)} />
      {/if}
    {/if}
  </section>

  <section aria-labelledby="rd-forecast">
    <h3 id="rd-forecast">{t("p.readings.forecast_heading")}</h3>
    <p class="small muted">{t("p.readings.forecast_says")}</p>
    <form class="row" novalidate onsubmit={readForecast} aria-label={t("p.readings.forecast_heading")}>
      <label>{t("p.readings.f_items")}<input type="number" min="1" step="1" bind:value={fform.items} /></label>
      <label>{t("p.readings.f_periods", { n: FORECAST_DEFAULTS.periods })}<input type="number" min="1" max="520" step="1" bind:value={fform.periods} /></label>
      <label>{t("p.readings.f_period_days", { n: FORECAST_DEFAULTS.periodDays })}<input type="number" min="1" max="90" step="1" bind:value={fform.periodDays} /></label>
      <label>{t("p.readings.f_history", { n: FORECAST_DEFAULTS.historyPeriods })}<input type="number" min="6" max="260" step="1" bind:value={fform.historyPeriods} /></label>
      <button class="button" type="submit" disabled={busy !== null}>{t("p.readings.read_forecast")}</button>
    </form>
    {#if problems.forecast}<p class="banner error" role="alert">{problems.forecast}</p>{/if}
    {#if forecast}
      {@const b = forecast.batch}
      {@const h = forecast.horizon}
      <p class="small">{tn("p.readings.forecast_history", forecast.throughput_history.length, { n: forecast.throughput_history.length, days: b.period_days })}</p>
      {#if b.p50_days !== null && b.p85_days !== null && b.p95_days !== null}
        <p>{tn("p.readings.forecast_batch", b.items, { n: formatInteger(b.items), p50: formatInteger(b.p50_days), p85: formatInteger(b.p85_days), p95: formatInteger(b.p95_days) })}</p>
      {:else}
        <p>{t("p.readings.forecast_no_batch")}</p>
      {/if}
      {#if h.median_items !== null && h.at_least_items !== null && h.at_most_items !== null}
        <p>{t("p.readings.forecast_horizon", { days: formatInteger(h.days), median: formatInteger(h.median_items), least: formatInteger(h.at_least_items), most: formatInteger(h.at_most_items) })}</p>
      {:else}
        <p>{t("p.readings.forecast_no_horizon")}</p>
      {/if}
    {/if}
  </section>

  <section aria-labelledby="rd-perf">
    <h3 id="rd-perf">{t("p.readings.performance_heading")}</h3>
    <p class="small muted">{t("p.readings.performance_says")}</p>
    <button class="button" type="button" disabled={busy !== null} onclick={() => void read("performance", () => ppm.planPerformance(pid), (v) => (performance = v))}>{t("p.readings.read_performance")}</button>
    {#if problems.performance}<p class="banner error" role="alert">{problems.performance}</p>{/if}
    {#if performance}
      {@const nps = performance.stakeholder.nps}
      <dl>
        <dt>{t("p.readings.schedule")}</dt>
        <dd>{performance.schedule.spi !== null ? formatBasisPoints(performance.schedule.spi) : t(absentKey(performance.schedule.absent))}</dd>
        <dt>{t("p.readings.cost")}</dt>
        <dd>{performance.financial.cpi !== null ? formatBasisPoints(performance.financial.cpi) : t(absentKey(performance.financial.absent))}</dd>
        <dt>{t("p.readings.stakeholders")}</dt>
        <dd>{nps.score !== null
            ? tn("p.readings.nps", nps.responses, { score: formatInteger(nps.score), n: formatInteger(nps.responses), promoters: formatInteger(nps.promoters), detractors: formatInteger(nps.detractors) })
            : t("p.readings.nps_none")}</dd>
      </dl>
      <p class="small"><a href={localeHref(`/plans/${pid}/earned-value`)}>{t("p.readings.earned_value_link")}</a></p>
    {/if}
  </section>

  <section aria-labelledby="rd-wf">
    <h3 id="rd-wf">{t("p.readings.workflow_heading")}</h3>
    <button class="button" type="button" disabled={busy !== null} onclick={() => void read("workflow", () => ppm.planWorkflow(pid), (v) => (workflow = v))}>{t("p.readings.read_workflow")}</button>
    {#if problems.workflow}<p class="banner error" role="alert">{problems.workflow}</p>{/if}
    {#if workflow}
      <p>{workflow.source === "plan" ? t("p.readings.workflow_plan") : t("p.readings.workflow_built_in")}
        {workflow.constrained ? t("p.readings.workflow_constrained") : t("p.readings.workflow_free")}</p>
      <ul>
        {#each workflow.workflow.states as s (s.key)}
          <li>
            <strong>{stateOf(s.key)}</strong>
            <span class="small muted">· {t(CATEGORY_KEY[s.category] ?? "p.readings.cat_other")}{s.is_initial ? ` · ${t("p.readings.initial")}` : ""}{s.is_terminal ? ` · ${t("p.readings.terminal")}` : ""}{s.wip_limit !== null ? ` · ${t("p.readings.wip_limit", { n: s.wip_limit })}` : ""}</span>
          </li>
        {/each}
      </ul>
      {#if workflow.workflow.transitions.length > 0}
        <ul class="small">
          {#each workflow.workflow.transitions as m, i (i)}
            <li>{stateOf(m.from)} {arrow()} {stateOf(m.to)} · {m.roles.length === 0 ? t("p.readings.anyone") : t("p.readings.roles", { roles: m.roles.join(", ") })}</li>
          {/each}
        </ul>
      {/if}
    {/if}
  </section>
</section>

<style>
  section section { margin-block: 1rem; }
  .row { display: flex; flex-wrap: wrap; gap: 0.5rem 1rem; align-items: end; }
  .row label { display: grid; gap: 0.2rem; max-inline-size: 14rem; }
  dl { display: grid; grid-template-columns: max-content 1fr; gap: 0.2rem 0.75rem; }
  dd { margin: 0; }
</style>
