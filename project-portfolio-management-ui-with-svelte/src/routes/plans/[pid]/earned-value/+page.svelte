<!--
  Earned value (`/plans/[pid]/earned-value`, SC-4): planned value, earned value and
  actual cost per currency, with schedule and cost performance (SPI, CPI) and
  earned-schedule SPI(t) in plain sentences. Percent complete is physical — work
  finished, not time elapsed — and how it is counted is said up front: the plan's
  default method, any task counted by a person-entered percent (always flagged,
  always with a note), and any task that carries no weight. Measured as of today.
  English-first, like the other recent views.
-->
<script lang="ts">
  import { rowOf } from "#lib/row-of.js";
  import { errorText } from "#lib/error-text.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import Term from "#lib/components/Term.svelte";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import {
    PpmClient,
    money,
    type EarnedValue,
    type EvForecast,
    type EvRollup,
    type EvSeries,
    type Task,
  } from "#lib/api/ppm.js";
  import EvBars from "#lib/components/EvBars.svelte";
  import SCurve from "#lib/components/SCurve.svelte";
  import {
    amountAbsentText,
    ragText,
    tcpiSentence,
    costSentence,
    earnedScheduleSentence,
    health,
    marker,
    methodSentence,
    ratioOrReason,
    scheduleSentence,
  } from "#lib/ev-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");

  let ev = $state<EarnedValue | null>(null);
  let forecast = $state<EvForecast | null>(null);
  let series = $state<EvSeries | null>(null);
  let rollup = $state<EvRollup | null>(null);
  let recorded = $state(false);
  let tasks = $state<Task[]>([]);
  /** Task names come from a capped list on a very large plan; the figures do not (the service adds up every task). */
  let namesTruncated = $state(false);
  let error = $state<string | null>(null);
  let editing = $state<string | null>(null);
  let method = $state("");
  let percent = $state(50);
  let note = $state("");

  async function load() {
    try {
      ev = await ppm.earnedValue(pid);
      const list = await ppm.listTasks(pid);
      tasks = list.tasks;
      namesTruncated = list.truncated === true;
      [forecast, series, rollup] = await Promise.all([
        ppm.earnedValueForecast(pid),
        ppm.earnedValueSeries(pid),
        ppm.earnedValueRollup(pid),
      ]);
      error = null;
    } catch (err) {
      error = errorText(err, "p.plans.pid.earned_value.could_not_load_earned_value");
    }
  }
  onMount(() => void load());

  const titleOf = $derived(new Map(tasks.map((t) => [t.pid, t.title])));

  async function setDefault(value: "zero_hundred" | "fifty_fifty" | "points") {
    error = null;
    try {
      await ppm.setEarnedValueMethod(pid, value);
      await load();
    } catch (err) {
      error = errorText(err, "p.plans.pid.earned_value.could_not_save");
    }
  }

  async function snapshotNow() {
    error = null;
    recorded = false;
    try {
      await ppm.snapshotEarnedValue(pid);
      recorded = true;
      await load();
    } catch (err) {
      error = errorText(err, "p.plans.pid.earned_value.could_not_record");
    }
  }

  function edit(taskPid: string) {
    editing = taskPid;
    const share = ev?.percent_complete?.tasks.find((t) => t.pid === taskPid);
    method = share?.method === "manual" ? "manual" : "";
    percent = share?.method === "manual" ? Math.round(share.percent_bp / 100) : 50;
    note = "";
  }

  async function save(event: SubmitEvent) {
    event.preventDefault();
    if (!editing) return;
    error = null;
    try {
      await ppm.setTaskPercent(
        pid,
        editing,
        method === "manual"
          ? { method: "manual", percent, note }
          : method
            ? { method: method as "zero_hundred" | "fifty_fifty" }
            : {},
      );
      editing = null;
      await load();
    } catch (err) {
      error = errorText(err, "p.plans.pid.earned_value.could_not_save");
    }
  }

  type Forecast = NonNullable<EvForecast["forecasts"]>[number];
  type Variant = Forecast["variants"][number];
  const forecastColumns = (currency: string, recommended: string): DataGridColumn[] => {
    const c = (n: number) => money(n, currency);
    return [
      { id: "formula", header: t("p.plans.pid.earned_value.forecast"), rowHeader: true, format: (v, r) => `${v}${rowOf<Variant>(r).variant === recommended ? ` — ${t("p.plans.pid.earned_value.the_one_to_lead_with")}` : ""}` },
      { id: "eac", header: t("p.plans.pid.earned_value.final_cost_eac"), align: "end", accessor: (r) => { const v = rowOf<Variant>(r); return v.eac.minor === null ? amountAbsentText(v.eac) : c(v.eac.minor); } },
      { id: "etc", header: t("p.plans.pid.earned_value.still_to_spend_etc"), align: "end", accessor: (r) => { const v = rowOf<Variant>(r); return v.etc.minor === null ? "—" : c(v.etc.minor); } },
      { id: "vac", header: t("p.plans.pid.earned_value.against_the_budget_vac"), align: "end", accessor: (r) => { const v = rowOf<Variant>(r); return v.vac.minor === null ? "—" : c(v.vac.minor) + (v.vac.minor < 0 ? t("p.plans.pid.earned_value.overrun") : ""); } },
    ];
  };
</script>

<svelte:head><title>{t("p.plans.pid.earned_value.earned_value_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref(`/plans/${pid}/governance`)}>{t("p.plans.pid.earned_value.governance")}</a> · <a href={localeHref(`/plans/${pid}/critical-path`)}>{t("p.plans.pid.earned_value.critical_path")}</a></p>
<h1>{t("p.plans.pid.earned_value.progress_against_budget_earned_value")}</h1>
<p class="lede">{t("p.plans.pid.earned_value.is_the_work_finished_worth")}</p>
{#if error}<p class="banner" role="alert">{error}</p>{/if}
{#if namesTruncated}<p class="banner" role="note" data-testid="ev-names-truncated">{t("p.plans.pid.earned_value.names_truncated")}</p>{/if}

{#if ev}
  <p class="small muted">{t("p.plans.pid.earned_value.measured_as_of_percent_complete", { as_of: ev.as_of })}</p>

  {#if ev.blocks === null}
    <p class="banner" role="status" data-testid="ev-absent">{ev.reason}</p>
  {:else}
    {#if ev.percent_complete}
      {@const pc = ev.percent_complete}
      <section aria-labelledby="pc-h" class="stack">
        <h2 id="pc-h">{t("p.plans.pid.earned_value.complete", { round: Math.round(pc.percent_bp / 100) })}</h2>
        <p class="small">{methodSentence(pc)}.</p>
        {#if pc.manual}
          <p class="banner" role="note">{t("p.plans.pid.earned_value.manual_percent_banner", { tasks: pc.manual_tasks.map((p) => titleOf.get(p) ?? t("p.plans.pid.earned_value.a_task")).join(", ") })}</p>
        {/if}
        <label>
          {t("p.plans.pid.earned_value.count_tasks")}
          <select value={pc.default_method} onchange={(e) => void setDefault(e.currentTarget.value as "zero_hundred" | "fifty_fifty" | "points")}>
            <option value="zero_hundred">{t("p.plans.pid.earned_value.only_when_finished")}</option>
            <option value="fifty_fifty">{t("p.plans.pid.earned_value.started_work_counts_half")}</option>
            <option value="points">{t("p.plans.pid.earned_value.by_story_points_when_finished")}</option>
          </select>
        </label>
      </section>
    {/if}

    {#each ev.blocks as b (b.currency)}
      {@const h = marker(health(b.spi.bp))}
      <section class="surface stack" aria-labelledby="blk-{b.currency}">
        <h2 id="blk-{b.currency}">{b.currency} <span class="small muted">{t("p.plans.pid.earned_value.baseline_version", { baseline_version: b.baseline_version })}</span></h2>
        <EvBars block={b} />
        <p><span aria-hidden="true">{h}</span> {scheduleSentence(b, (n) => money(n, b.currency))}</p>
        <p><span aria-hidden="true">{marker(health(b.cpi.bp))}</span> {costSentence(b, (n) => money(n, b.currency))}</p>
        <p><span aria-hidden="true">{marker(health(b.spi_t.bp))}</span> {earnedScheduleSentence(b)}</p>
        <table>
          <caption class="sr-only">{t("p.plans.pid.earned_value.earned_value_figures", { currency: b.currency })}</caption>
          <tbody>
            <tr><th scope="row">{t("p.plans.pid.earned_value.budget_at_completion")}</th><td>{money(b.bac_minor, b.currency)}</td></tr>
            <tr><th scope="row">{t("p.plans.pid.earned_value.planned_value")}</th><td>{money(b.pv_minor, b.currency)}</td></tr>
            <tr><th scope="row">{t("p.plans.pid.earned_value.earned_value")}</th><td>{money(b.ev_minor, b.currency)}</td></tr>
            <tr><th scope="row">{t("p.plans.pid.earned_value.actual_cost")}</th><td>{b.ac_minor === null ? t("p.plans.pid.earned_value.unknown") : money(b.ac_minor, b.currency)}</td></tr>
            <tr><th scope="row">{t("p.plans.pid.earned_value.schedule_variance_ev_pv")}</th><td>{money(b.sv_minor, b.currency)}</td></tr>
            <tr><th scope="row">{t("p.plans.pid.earned_value.cost_variance_ev_ac")}</th><td>{b.cv_minor === null ? t("p.plans.pid.earned_value.unknown") : money(b.cv_minor, b.currency)}</td></tr>
            <tr><th scope="row">{t("p.plans.pid.earned_value.schedule_index")}<Term t="SPI" />)</th><td>{ratioOrReason(b.spi)}</td></tr>
            <tr><th scope="row">{t("p.plans.pid.earned_value.cost_index")}<Term t="CPI" />)</th><td>{ratioOrReason(b.cpi)}</td></tr>
            <tr><th scope="row">{t("p.plans.pid.earned_value.schedule_index_by_time")}<Term t="SPI(t)" />)</th><td>{ratioOrReason(b.spi_t)}</td></tr>
          </tbody>
        </table>
      </section>
    {/each}

    {#if ev.unbaselined_currencies && ev.unbaselined_currencies.length > 0}
      <p class="banner" role="note">
        {t("p.plans.pid.earned_value.budget_lines_in_have_no", { join: ev.unbaselined_currencies.join(", ") })}
      </p>
    {/if}

    {#if forecast?.forecasts}
      <h2>{t("p.plans.pid.earned_value.where_it_is_heading")}</h2>
      {#each forecast.forecasts as f (f.currency)}
        {@const c = (n: number) => money(n, f.currency)}
        <section class="surface stack" aria-labelledby="fc-{f.currency}">
          <h3 id="fc-{f.currency}">{f.currency}</h3>
          <p>
            <span aria-hidden="true">{ragText(f.spi.rag).marker}</span> {t("p.plans.pid.earned_value.schedule_is", { word: ragText(f.spi.rag).word })}
            <span aria-hidden="true">{ragText(f.cpi.rag).marker}</span> {t("p.plans.pid.earned_value.cost_is", { word: ragText(f.cpi.rag).word })}
          </p>
          <DataTable label={t("p.plans.pid.earned_value.forecasts_of_the_final_cost", { currency: f.currency })} columns={forecastColumns(f.currency, f.recommended.variant)} rows={f.variants} rowId={(r) => String(r.variant)} sortable={false} rowClass={(r) => (rowOf<Variant>(r).variant === f.recommended.variant ? "chosen" : "")} />
          <p class="small">{t("p.plans.pid.earned_value.leading_with_that_one_because", { why: f.recommended.why })}</p>
          <p><span aria-hidden="true">{ragText(f.tcpi_to_budget.rag).marker}</span> {tcpiSentence(f.tcpi_to_budget.tcpi, t("p.plans.pid.earned_value.target_budget"))}</p>
          {#if f.tcpi_to_recommended_eac}
            <p><span aria-hidden="true">{ragText(f.tcpi_to_recommended_eac.rag).marker}</span> {tcpiSentence(f.tcpi_to_recommended_eac.tcpi, t("p.plans.pid.earned_value.target_forecast"))}</p>
          {/if}
        </section>
      {/each}
      {#if forecast.thresholds}
        <p class="small muted">{t("p.plans.pid.earned_value.lights_use", { which: forecast.thresholds.source === "default" ? t("p.plans.pid.earned_value.thresholds_default") : t("p.plans.pid.earned_value.thresholds_deployment"), values: JSON.stringify(forecast.thresholds.values) })}</p>
      {/if}
    {/if}

    <h2>{t("p.plans.pid.earned_value.over_time")}</h2>
    {#if series && series.curves.length > 0}
      {#each series.curves as curve (curve.currency)}
        <h3>{curve.currency}</h3>
        <SCurve {curve} />
      {/each}
      <p class="small muted">{series.note}</p>
    {/if}
    <p>
      <button type="button" onclick={() => void snapshotNow()}>{t("p.plans.pid.earned_value.record_today_s_figures_now")}</button>
      {#if recorded}<span role="status"> {t("p.plans.pid.earned_value.recorded")}</span>{/if}
      <span class="small muted"> {t("p.plans.pid.earned_value.the_scheduler_also_records_them")}</span>
    </p>

    {#if rollup && rollup.currencies.length > 0 && rollup.currencies.some((c) => c.totals.plans > 1)}
      <h2>{t("p.plans.pid.earned_value.this_plan_and_everything_beneath")}</h2>
      {#each rollup.currencies as r (r.currency)}
        <section class="surface stack" aria-labelledby="ru-{r.currency}">
          <h3 id="ru-{r.currency}">{r.currency} <span class="small muted">{t("p.plans.pid.earned_value.plans", { plans: r.totals.plans })}</span></h3>
          <p>
            {t("p.plans.pid.earned_value.rollup_money", { bac: money(r.totals.bac_minor, r.currency), pv: money(r.totals.pv_minor, r.currency), ev: money(r.totals.ev_minor, r.currency), ac: money(r.totals.ac_minor, r.currency), unknown: r.totals.plans_without_ac > 0 ? t("p.plans.pid.earned_value.cost_unknown_for", { plans: r.totals.plans_without_ac }) : "" })}
          </p>
          <p>
            <span aria-hidden="true">{ragText(r.spi.rag).marker}</span> {t("p.plans.pid.earned_value.spi", { ratioOrReason: ratioOrReason({ bp: r.spi.bp }) })} <span aria-hidden="true">{ragText(r.cpi.rag).marker}</span> {t("p.plans.pid.earned_value.cpi_value", { value: r.cpi.bp === null ? t("p.plans.pid.earned_value.cpi_not_stated") : ratioOrReason({ bp: r.cpi.bp }) })}
            {#if r.eac.minor !== null}{t("p.plans.pid.earned_value.forecast_final_cost", { money: money(r.eac.minor, r.currency) })}{/if}
          </p>
        </section>
      {/each}
      {#if rollup.plans_without_figures > 0}
        <p class="small muted">{(rollup.plans_without_figures === 1 ? t("p.plans.pid.earned_value.plan_below_have_no_figures_one", { plans_without_figures: rollup.plans_without_figures }) : t("p.plans.pid.earned_value.plan_below_have_no_figures_other", { plans_without_figures: rollup.plans_without_figures }))}</p>
      {/if}
    {/if}

    <h2>{t("p.plans.pid.earned_value.count_a_task_another_way")}</h2>
    <ul class="small">
      {#each ev.percent_complete?.tasks ?? [] as row (row.pid)}
        <li>
          {titleOf.get(row.pid) ?? t("p.plans.pid.earned_value.task")}: {Math.round(row.percent_bp / 100)}%
          ({row.method === "manual" ? t("p.plans.pid.earned_value.manual") : row.method === "fifty_fifty" ? t("p.plans.pid.earned_value.half_for_started") : t("p.plans.pid.earned_value.finished_only")}{ev.percent_complete?.weighting === "points" ? t("p.plans.pid.earned_value.weight_points", { weight: row.weight }) : ""})
          <button type="button" onclick={() => edit(row.pid)}>{t("p.plans.pid.earned_value.change")}</button>
        </li>
      {/each}
    </ul>
    {#if editing}
      <form onsubmit={save} class="stack surface">
        <h3>{titleOf.get(editing) ?? t("p.plans.pid.earned_value.task_2")}</h3>
        <label>
          {t("p.plans.pid.earned_value.method")}
          <select bind:value={method}>
            <option value="">{t("p.plans.pid.earned_value.use_the_plan_s_method")}</option>
            <option value="zero_hundred">{t("p.plans.pid.earned_value.only_when_finished_2")}</option>
            <option value="fifty_fifty">{t("p.plans.pid.earned_value.started_work_counts_half_2")}</option>
            <option value="manual">{t("p.plans.pid.earned_value.a_percent_i_enter_flagged")}</option>
          </select>
        </label>
        {#if method === "manual"}
          <label>{t("p.plans.pid.earned_value.percent_complete")} <input type="number" min="0" max="100" bind:value={percent} required /></label>
          <label>{t("p.plans.pid.earned_value.why_required")} <input type="text" bind:value={note} required /></label>
        {/if}
        <button class="button primary" type="submit">{t("p.plans.pid.earned_value.save")}</button>
        <button class="button" type="button" onclick={() => (editing = null)}>{t("p.plans.pid.earned_value.cancel")}</button>
      </form>
    {/if}
    <p class="small muted">{ev.note}</p>
  {/if}
{:else if !error}
  <p>{t("p.plans.pid.earned_value.loading")}</p>
{/if}

<style>
  :global(.data-table tr.chosen th[scope="row"]) {
    border-inline-start: 4px solid var(--mxi-color-focus, #1d4ed8);
  }
  table {
    border-collapse: collapse;
  }
  th,
  td {
    border: 1px solid var(--border, #ccc);
    padding: 0.25rem 0.6rem;
    text-align: start;
  }
  td {
    font-variant-numeric: tabular-nums;
  }
  .sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    overflow: hidden;
    clip-path: inset(50%);
    white-space: nowrap;
  }
  label {
    display: flex;
    flex-direction: column;
    gap: 0.2rem;
    max-width: 24rem;
  }
</style>
