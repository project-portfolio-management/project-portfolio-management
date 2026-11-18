<!--
  Costs (`/plans/[pid]/costs`): the planned cost by period (labour from the work
  profile at the rate in force each day, plus fixed and material expenses), the
  recorded labour cost, and the expense forms. Each currency stands alone and is
  never added to another. Work with no rate is listed, never counted as free.
  Rates are finance data: someone without access sees a plain note, not an error.
  Below (NC-1 slice 12 part f), read live: the estimate at completion, the phased budget baselines and a new version
  (frozen once approved, after a question), the stage-gated tranches and their release (after a question), and a
  promised business-case target (after a question). Rules in `forecast-view.ts` and `value-view.ts`, said first.
-->
<script lang="ts">
  import { rowOf } from "#lib/row-of.js";
  import { errorReason, errorText } from "#lib/error-text.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t, localeHref, tn } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { ApiError } from "#lib/api/client.js";
  import { PpmClient, money, type BudgetBaseline, type BudgetBoard, type CostPlan, type CpmNode, type LabourCosts, type PlanForecast } from "#lib/api/ppm.js";
  import { currencyCode } from "#lib/money.js";
  import { NO_RATE_TEXT, UNPLACED_TEXT, hoursText, minorFromMajor } from "#lib/work-view.js";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import { enumLabel } from "#lib/enum-labels.js";
  import { formatDate } from "#lib/locale-format.js";
  import { minorToInput } from "#lib/money.js";
  import {
    ETC_SOURCE_KEY,
    PERIODS_MAX,
    REASON_MAX_BYTES,
    baselineBody,
    baselineFormOf,
    baselineRefusalKey,
    releaseRefusalKey,
    releaseState,
    type BaselineForm,
  } from "#lib/forecast-view.js";
  import { SOURCES, SOURCE_KEY, businessCaseBody, businessCaseRefusalKey, type BusinessCaseForm } from "#lib/value-view.js";

  const ppm = PpmClient.withFetch();

  // ---- The plan's money read live (NC-1 slice 12 part f) ----
  let forecast = $state<PlanForecast | null>(null);
  let forecastError = $state<string | null>(null);
  let baselines = $state<BudgetBaseline[] | null>(null);
  let baselineError = $state<string | null>(null);
  let baseForm = $state<BaselineForm>({ currency: "", periods: [{ start: "", end: "", amount: "" }], reason: "" });
  let baseProblem = $state<string | null>(null);
  let baseNotice = $state("");
  let lines = $state<BudgetBoard["lines"] | null>(null);
  let stage = $state<string | null>(null);
  let linesError = $state<string | null>(null);
  let lineProblem = $state<string | null>(null);
  let lineNotice = $state("");
  let caseForm = $state<BusinessCaseForm>({ metric: "", baseline: null, target: null, unit: "", source: "charter" });
  let caseProblem = $state<string | null>(null);
  let caseNotice = $state("");
  let busy = $state(false);
  const latest = $derived(baselines?.[0] ?? null);
  const gated = $derived((lines ?? []).filter((l) => l.gate));

  async function loadForecast() {
    try {
      forecast = await ppm.planForecast(pid);
      forecastError = null;
    } catch (err) {
      forecastError = errorText(err, "p.forecast.could_not_read");
    }
  }
  /** Read the versions; `fresh` starts the form again from the latest (not after a refusal, which keeps what was typed). */
  async function loadBaselines(fresh = true) {
    try {
      baselines = await ppm.budgetBaselines(pid);
      baselineError = null;
      if (fresh) baseForm = baselineFormOf(baselines[0] ?? null, (m, c) => minorToInput(m, c));
    } catch (err) {
      baselineError = errorText(err, "p.forecast.could_not_read_baselines");
    }
  }
  async function loadLines() {
    try {
      const [board, gov] = await Promise.all([ppm.budget(pid), ppm.governance(pid)]);
      lines = board.lines;
      stage = gov.stage;
      linesError = null;
    } catch (err) {
      linesError = errorText(err, "p.forecast.could_not_read_lines");
    }
  }
  onMount(() => {
    void loadForecast();
    void loadBaselines();
    void loadLines();
  });

  async function approve(e: SubmitEvent) {
    e.preventDefault();
    baseNotice = "";
    const r = baselineBody(baseForm, latest);
    if ("problems" in r) {
      baseProblem = r.problems.map((k) => t(k, { max: PERIODS_MAX, bytes: REASON_MAX_BYTES })).join(" ");
      return;
    }
    baseProblem = null;
    const version = (latest?.version ?? 0) + 1;
    if (!(await askConfirm(t("p.forecast.confirm_baseline", { version })))) return;
    busy = true;
    try {
      const done = await ppm.approveBudgetBaseline(pid, r.body);
      baseNotice = t("p.forecast.baseline_approved", { version: done.version });
      await Promise.all([loadBaselines(), loadForecast()]);
    } catch (err) {
      const k = baselineRefusalKey(err);
      baseProblem = k ? t(k) : errorText(err, "p.forecast.could_not_approve");
      if (k === "p.forecast.refused_baseline") await loadBaselines(false);
    } finally {
      busy = false;
    }
  }
  function addPeriod() {
    baseForm.periods = [...baseForm.periods, { start: "", end: "", amount: "" }];
  }
  function removePeriod(i: number) {
    baseForm.periods = baseForm.periods.filter((_p, j) => j !== i);
  }

  async function release(line: BudgetBoard["lines"][number]) {
    lineNotice = "";
    lineProblem = null;
    if (releaseState(line, stage) !== "ready") return;
    if (!(await askConfirm(t("p.forecast.confirm_release", { description: line.description, gate: enumLabel("pk_gate", line.gate) })))) return;
    busy = true;
    try {
      await ppm.releaseBudgetLine(pid, line.pid);
      lineNotice = t("p.forecast.released", { description: line.description });
      await loadLines();
    } catch (err) {
      const k = releaseRefusalKey(err);
      lineProblem = k ? t(k) : errorText(err, "p.forecast.could_not_release");
      if (k === "p.forecast.refused_release") await loadLines();
    } finally {
      busy = false;
    }
  }

  async function recordTarget(e: SubmitEvent) {
    e.preventDefault();
    caseNotice = "";
    const r = businessCaseBody(caseForm);
    if ("problems" in r) {
      caseProblem = r.problems.map((k) => t(k)).join(" ");
      return;
    }
    caseProblem = null;
    if (!(await askConfirm(t("p.value.confirm_target", { metric: r.body.metric })))) return;
    busy = true;
    try {
      await ppm.recordBusinessCase(pid, r.body);
      caseNotice = t("p.value.target_recorded", { metric: r.body.metric });
      caseForm = { metric: "", baseline: null, target: null, unit: "", source: caseForm.source };
    } catch (err) {
      const k = businessCaseRefusalKey(err);
      caseProblem = k ? t(k) : errorText(err, "p.value.could_not_record_target");
    } finally {
      busy = false;
    }
  }
  const pid = $derived(page.params.pid ?? "");
  let plan = $state<CostPlan | null>(null);
  let actuals = $state<LabourCosts | null>(null);
  let nodes = $state<CpmNode[]>([]);
  let denied = $state(false);
  let error = $state<string | null>(null);
  let status = $state("");

  async function load() {
    try {
      nodes = (await ppm.planCpm(pid)).nodes;
      plan = await ppm.costPlan(pid);
      actuals = await ppm.labourCosts(pid);
      error = null;
    } catch (err) {
      const m = errorReason(err);
      // A 404 is how finance data hides from someone without access.
      if (err instanceof ApiError && err.status === 404) denied = true;
      else error = m || t("p.plans.pid.costs.could_not_load_the_costs");
    }
  }
  onMount(load);

  let taskPid = $state("");
  let kind = $state<"material" | "fixed">("material");
  let description = $state("");
  let amount = $state("");
  let currency = $state("");
  let accrual = $state<"start" | "prorated" | "end">("prorated");
  async function addExpense(e: SubmitEvent) {
    e.preventDefault();
    const code = currencyCode(currency);
    if (!taskPid) return void (status = t("p.plans.pid.costs.choose_a_task"));
    if (code === null) return void (status = t("p.plans.pid.costs.the_currency_must_be_a"));
    const minor = minorFromMajor(amount, code);
    if (minor === null) return void (status = t("p.plans.pid.costs.the_amount_must_be_above"));
    try {
      await ppm.addTaskCost(pid, taskPid, { kind, description, amount_minor: minor, currency: code, accrual });
      description = amount = "";
      status = t("p.plans.pid.costs.expense_added");
      await load();
    } catch (err) {
      status = errorText(err, "p.plans.pid.costs.could_not_add_the_expense");
    }
  }
  async function record() {
    try {
      const r = await ppm.recordLabourCosts(pid);
      status = tn("p.plans.pid.costs.recorded", r.recorded, { recorded: r.recorded, already_recorded: r.already_recorded, length: r.no_rate.length });
      await load();
    } catch (err) {
      status = errorText(err, "p.plans.pid.costs.could_not_record");
    }
  }
  const label = (b: { from: string; to: string }) => (b.from === b.to ? b.from : `${b.from.slice(5)}–${b.to.slice(5)}`);

  type Cur = CostPlan["currencies"][number];
  type Measure = Cur["labour"];
  const costRows = (c: Cur) => [
    { name: "Labour", m: c.labour },
    { name: "Expenses", m: c.fixed },
    { name: "Total", m: c.total },
  ];
  const costColumns = (c: Cur): DataGridColumn[] => [
    { id: "name", header: "", rowHeader: true },
    ...c.total.buckets.map((b, i): DataGridColumn => ({
      id: `b${i}`, header: label(b), align: "end",
      accessor: (r) => (rowOf<{ m: Measure }>(r).m.buckets[i]?.minor ?? 0), format: (v) => (v ? money(Number(v), c.currency) : "–"),
    })),
    { id: "total", header: t("p.plans.pid.costs.total"), align: "end", accessor: (r) => rowOf<{ m: Measure }>(r).m.total_minor, format: (v) => money(Number(v), c.currency) },
  ];
</script>

<svelte:head><title>{t("p.plans.pid.costs.costs_ppm")}</title></svelte:head>

<h1>{t("p.plans.pid.costs.costs")}</h1>
<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plans.pid.costs.plan")}</a> · <a href={localeHref(`/plans/${pid}/work`)}>{t("p.plans.pid.costs.work")}</a> · <a href={localeHref("/cost-rates")}>{t("p.plans.pid.costs.cost_rates")}</a></p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}
{#if denied}
  <p class="muted" data-testid="denied">{t("p.plans.pid.costs.costs_and_rates_are_finance")}</p>
{:else}
  <h2>{t("p.plans.pid.costs.planned_cost")}</h2>
  {#if plan?.reason === "no_costs"}
    <p class="muted">{t("p.plans.pid.costs.nothing_could_be_costed_yet")}</p>
  {/if}
  {#each plan?.currencies ?? [] as c (c.currency)}
    <DataTable label={t("p.plans.pid.costs.per_period", { currency: c.currency })} columns={costColumns(c)} rows={costRows(c)} rowId={(r) => String(r.name)} sortable={false} testid={`cost-${c.currency}`} />
  {/each}
  {#each [...(plan?.unrated ?? []), ...(plan?.unassigned ?? [])] as u, i (i)}
    <p class="small">{t("p.plans.pid.costs.so_its_labour_is_not", { title: u.title, hoursText: hoursText(u.work_minutes), reason: NO_RATE_TEXT[u.reason] ?? u.reason })}</p>
  {/each}
  {#each plan?.unplaced ?? [] as u, i (i)}
    <p class="small">{t("p.plans.pid.costs.an_expense_of_so_it", { money: money(u.amount_minor, u.currency), reason: UNPLACED_TEXT[u.reason] ?? u.reason })}</p>
  {/each}

  <h2>{t("p.plans.pid.costs.recorded_labour_cost")}</h2>
  {#if actuals}
    {#each actuals.currencies as c (c.currency)}
      <p>{tn("p.plans.pid.costs.for_across", c.entries, { money: money(c.cost_minor, c.currency), hoursText: hoursText(c.minutes), entries: c.entries })}</p>
    {/each}
    {#if actuals.currencies.length === 0}<p class="muted">{t("p.plans.pid.costs.no_time_has_been_costed")}</p>{/if}
    {#if actuals.unpriced.entries}
      <p class="small">{t("p.plans.pid.costs.time_entries_are_not_costed", { entries: actuals.unpriced.entries, hoursText: hoursText(actuals.unpriced.minutes) })}</p>
    {/if}
  {/if}
  <button class="button" type="button" onclick={record}>{t("p.plans.pid.costs.record_labour_costs")}</button>
  <p class="small muted">{t("p.plans.pid.costs.a_recorded_cost_keeps_the")}</p>

  <h2>{t("p.plans.pid.costs.add_an_expense")}</h2>
  <form class="surface stack" onsubmit={addExpense}>
    <label>{t("p.plans.pid.costs.task")}
      <select bind:value={taskPid}>
        <option value="">{t("p.plans.pid.costs.choose")}</option>
        {#each nodes as n (n.pid)}<option value={n.pid}>{n.title ?? n.pid}</option>{/each}
      </select>
    </label>
    <label>{t("p.plans.pid.costs.kind")}
      <select bind:value={kind}><option value="material">{t("p.plans.pid.costs.material")}</option><option value="fixed">{t("p.plans.pid.costs.fixed_cost")}</option></select>
    </label>
    <label>{t("p.plans.pid.costs.description")} <input bind:value={description} required maxlength="200" /></label>
    <label>{t("p.plans.pid.costs.amount")} <input type="text" inputmode="decimal" bind:value={amount} /></label>
    <label>{t("p.plans.pid.costs.currency")} <input bind:value={currency} required maxlength="3" size="4" placeholder={t("p.money.currency_hint")} /></label>
    <label>{t("p.plans.pid.costs.falls")}
      <select bind:value={accrual}>
        <option value="start">{t("p.plans.pid.costs.all_at_the_start")}</option>
        <option value="prorated">{t("p.plans.pid.costs.evenly_across_the_task")}</option>
        <option value="end">{t("p.plans.pid.costs.all_at_the_end")}</option>
      </select>
    </label>
    <button class="button primary" type="submit">{t("p.plans.pid.costs.add_expense")}</button>
  </form>
  {#if status}<p role="status" class="small">{status}</p>{/if}

  <section aria-labelledby="forecast-h" data-testid="forecast">
    <h2 id="forecast-h">{t("p.forecast.heading")}</h2>
    <p class="small muted">{t("p.forecast.says")}</p>
    {#if forecastError}<p class="banner error" role="alert">{forecastError}</p>{/if}
    {#if forecast}
      {@const f = forecast.forecast}
      {#if f.currency === null}
        <p class="muted">{t("p.forecast.absent_no_currency")}</p>
      {:else}
        <dl class="figures">
          <dt>{t("p.forecast.c_actual")}</dt><dd>{money(f.actual_minor, f.currency)}</dd>
          <dt>{t("p.forecast.c_etc")}</dt>
          <dd>{f.estimate_to_complete_minor === null ? t("p.forecast.not_known") : money(f.estimate_to_complete_minor, f.currency)}{#if f.etc_source}{` · ${t(ETC_SOURCE_KEY[f.etc_source])}`}{/if}</dd>
          <dt>{t("p.forecast.c_eac")}</dt><dd>{f.estimate_at_completion_minor === null ? t("p.forecast.not_known") : money(f.estimate_at_completion_minor, f.currency)}</dd>
        </dl>
        <p class="small">{f.baseline_version === null ? t("p.forecast.no_baseline_used") : t("p.forecast.baseline_used", { version: f.baseline_version })} · {t("p.forecast.as_of", { date: formatDate(forecast.as_of) })}</p>
        {#if f.excluded_other_currency_minor !== 0}
          <p class="small">{t("p.forecast.other_currency", { currency: f.currency })}</p>
        {/if}
      {/if}
    {/if}
  </section>

  <section aria-labelledby="baselines-h" data-testid="baselines">
    <h2 id="baselines-h">{t("p.forecast.baselines_heading")}</h2>
    <p class="small muted">{t("p.forecast.baselines_says", { max: PERIODS_MAX })}</p>
    {#if baselineError}<p class="banner error" role="alert">{baselineError}</p>{/if}
    {#if baselines}
      {#if baselines.length === 0}
        <p class="muted">{t("p.forecast.no_baselines")}</p>
      {:else}
        <ol class="versions" reversed>
          {#each baselines as b (b.pid)}
            <li>
              <p><strong>{t("p.forecast.version_line", { version: b.version, currency: b.currency, date: formatDate(b.approved_at) })}</strong>{#if b.reason}{` · ${b.reason}`}{/if}</p>
              <ul class="small">
                {#each b.periods as p, i (i)}
                  <li>{t("p.forecast.period_line", { start: formatDate(p.period_start), end: formatDate(p.period_end), amount: money(p.planned_minor, b.currency) })}</li>
                {/each}
              </ul>
            </li>
          {/each}
        </ol>
      {/if}
    {/if}
    <h3>{latest ? t("p.forecast.rebaseline_heading", { version: latest.version + 1 }) : t("p.forecast.first_baseline_heading")}</h3>
    <form class="surface stack" novalidate onsubmit={approve} data-testid="baseline-form">
      <label>{t("p.forecast.f_currency")} <input bind:value={baseForm.currency} maxlength="3" size="4" placeholder={t("p.money.currency_hint")} disabled={baselines === null} /></label>
      <fieldset>
        <legend>{t("p.forecast.periods_legend")}</legend>
        {#each baseForm.periods as p, i (i)}
          <div class="period-row">
            <label>{t("p.forecast.f_start", { n: i + 1 })} <input type="date" bind:value={p.start} disabled={baselines === null} /></label>
            <label>{t("p.forecast.f_end", { n: i + 1 })} <input type="date" bind:value={p.end} disabled={baselines === null} /></label>
            <label>{t("p.forecast.f_amount", { n: i + 1 })} <input type="text" inputmode="decimal" bind:value={p.amount} disabled={baselines === null} /></label>
            <button class="button small" type="button" onclick={() => removePeriod(i)} disabled={baselines === null}>{t("p.forecast.remove_period", { n: i + 1 })}</button>
          </div>
        {/each}
        <button class="button small" type="button" onclick={addPeriod} disabled={baselines === null || baseForm.periods.length >= PERIODS_MAX}>{t("p.forecast.add_period")}</button>
      </fieldset>
      <label>{latest ? t("p.forecast.f_reason_needed") : t("p.forecast.f_reason")} <textarea bind:value={baseForm.reason} rows="2" disabled={baselines === null}></textarea></label>
      <div class="row"><button class="button" type="submit" disabled={busy || baselines === null}>{t("p.forecast.approve")}</button></div>
    </form>
    {#if baseProblem}<p class="banner error" role="alert">{baseProblem}</p>{/if}
    {#if baseNotice}<p class="small" role="status">{baseNotice}</p>{/if}
  </section>

  <section aria-labelledby="tranches-h" data-testid="tranches">
    <h2 id="tranches-h">{t("p.forecast.tranches_heading")}</h2>
    <p class="small muted">{t("p.forecast.tranches_says")}</p>
    {#if linesError}<p class="banner error" role="alert">{linesError}</p>{/if}
    {#if lines}
      <p class="small">{stage ? t("p.forecast.stage_now", { stage: enumLabel("pk_gate", stage) }) : t("p.forecast.stage_none")}</p>
      {#if gated.length === 0}
        <p class="muted">{t("p.forecast.no_tranches")}</p>
      {:else}
        <ul class="tranches">
          {#each gated as l (l.pid)}
            {@const st = releaseState(l, stage)}
            <li>
              <span>{l.description} · {money(l.planned_minor, l.currency)} · {t("p.forecast.gate_line", { gate: enumLabel("pk_gate", l.gate) })}</span>
              {#if st === "released"}
                <span><span aria-hidden="true">✓ </span>{t("p.forecast.state_released", { date: formatDate(l.released_at ?? "") })}</span>
              {:else if st === "held"}
                <span><span aria-hidden="true">○ </span>{t("p.forecast.state_held")}</span>
              {:else}
                <button class="button small" type="button" disabled={busy} onclick={() => release(l)}>{t("p.forecast.release", { description: l.description })}</button>
              {/if}
            </li>
          {/each}
        </ul>
      {/if}
    {/if}
    {#if lineProblem}<p class="banner error" role="alert">{lineProblem}</p>{/if}
    {#if lineNotice}<p class="small" role="status">{lineNotice}</p>{/if}
  </section>

  <section aria-labelledby="case-h" data-testid="business-case">
    <h2 id="case-h">{t("p.value.case_heading")}</h2>
    <p class="small muted">{t("p.value.case_says")} <a href={localeHref(`/plans/${pid}/benefits`)}>{t("p.value.to_benefits")}</a></p>
    <form class="surface stack" novalidate onsubmit={recordTarget}>
      <label>{t("p.value.f_metric")} <input bind:value={caseForm.metric} /></label>
      <label>{t("p.value.f_baseline")} <input type="number" step="1" bind:value={caseForm.baseline} /></label>
      <label>{t("p.value.f_target")} <input type="number" step="1" bind:value={caseForm.target} /></label>
      <label>{t("p.value.f_unit")} <input bind:value={caseForm.unit} /></label>
      <label>{t("p.value.f_source")}
        <select bind:value={caseForm.source}>
          {#each SOURCES as s (s)}<option value={s}>{t(SOURCE_KEY[s])}</option>{/each}
        </select>
      </label>
      <div class="row"><button class="button" type="submit" disabled={busy}>{t("p.value.record_target")}</button></div>
    </form>
    {#if caseProblem}<p class="banner error" role="alert">{caseProblem}</p>{/if}
    {#if caseNotice}<p class="small" role="status">{caseNotice}</p>{/if}
  </section>
{/if}

<style>
  label {
    display: block;
  }
  section {
    margin-block-start: 1.5rem;
  }
  .figures {
    display: grid;
    grid-template-columns: max-content 1fr;
    gap: 0.25rem 1rem;
  }
  .figures dd {
    margin: 0;
  }
  .period-row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    align-items: end;
    margin-block-end: 0.5rem;
  }
  .versions,
  .tranches {
    padding-inline-start: 1.5rem;
  }
  .tranches li {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    align-items: center;
    margin-block-end: 0.35rem;
  }
</style>
