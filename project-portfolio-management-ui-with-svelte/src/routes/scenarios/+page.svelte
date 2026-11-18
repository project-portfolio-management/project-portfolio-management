<!--
  Scenario planning (`/scenarios`, PPM-4): create what-if candidate
  portfolios (member pids + budget cap), evaluate them over live
  data, and commit the feasible winner.

  NC-7 (NC-1 slice 12 part g): the efficient frontier (`GET /api/scenarios/frontier`), drawn as a chart with a table
  twin, each point's choice shown plan by plan with why a plan was left out, and saved as a draft on asking; a draft
  generated within an optional cap (`POST /api/scenarios/generate`), its choice shown the same way; the plans every
  choice must include, chosen from the plans the reader can read (`GET /api/plans`, the list disabled until loaded); a
  committed scenario rolled back to a draft after a question (`POST /api/scenarios/{pid}/rollback`). Every rule said
  first; each refusal the catalogue's sentence; the service's text (a generated scenario's default name, the
  frontier's `value`, a comparison's `note`, a violation's words) never shown. Rules in `src/lib/frontier-view.ts`.
-->
<script lang="ts">
  import { rowOf } from "#lib/row-of.js";
  import { errorText } from "#lib/error-text.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import FrontierChart from "#lib/components/FrontierChart.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t, tn } from "#lib/i18n.svelte.js";
  import { currencyCode, toMinor } from "#lib/money.js";
  import { formatDecimal } from "#lib/locale-format.js";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import { onMount } from "svelte";
  import {
    PpmClient,
    money,
    type Frontier,
    type FrontierPoint,
    type Scenario,
    type ScenarioChoice,
    type ScenarioComparison,
    type ScenarioEvaluation,
  } from "#lib/api/ppm.js";
  import { PlanRepository } from "#lib/api/plans.js";
  import type { PlanRef } from "#lib/api/types.js";
  import {
    LIST_MAX,
    NAME_MAX_BYTES,
    STEPS_DEFAULT,
    STEPS_MAX,
    STEPS_MIN,
    choiceRows,
    frontierQuery,
    frontierRefusalKey,
    generateBody,
    pointBody,
    reasonKey,
    rollbackRefusalKey,
    type FrontierForm,
    type GenerateForm,
  } from "#lib/frontier-view.js";

  const ppm = PpmClient.withFetch();
  const plansRepo = PlanRepository.withFetch();
  let scenarios = $state<Scenario[]>([]);
  let evaluations = $state<Record<string, ScenarioEvaluation>>({});
  let error = $state<string | null>(null);
  let listNotice = $state("");

  let compareA = $state("");
  let compareB = $state("");
  let comparison = $state<ScenarioComparison | null>(null);

  // ---- The frontier, a generated draft and the plans every choice must include (NC-7) ----
  let plans = $state<PlanRef[] | null>(null);
  const names = $derived(new Map((plans ?? []).map((p) => [p.pid, p.name])));
  let mustInclude = $state<string[]>([]);
  let fForm = $state<FrontierForm>({ currency: "", how: "steps", steps: null, caps: "", mustInclude: [] });
  let frontier = $state<Frontier | null>(null);
  let frontierAsked = $state<{ currency: string; mustInclude: string[] } | null>(null);
  let fProblem = $state<string | null>(null);
  let fNotice = $state("");
  let gForm = $state<GenerateForm>({ name: "", cap: "", currency: "", mustInclude: [] });
  let gProblem = $state<string | null>(null);
  let gNotice = $state("");
  /** The choice on show: a frontier point's or a generated draft's. */
  let choice = $state<{ title: string; rows: ScenarioChoice[]; currency: string | null } | null>(null);
  let busy = $state(false);

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
  let currency = $state("");

  async function refresh() {
    try {
      scenarios = await ppm.listScenarios();
      error = null;
    } catch (err) {
      error = errorText(err, "ppm.common.loadFailed");
    }
  }
  onMount(() => {
    void refresh();
    plansRepo
      .list()
      .then((p) => (plans = p))
      .catch(() => (plans = []));
  });

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
    const code = currencyCode(currency);
    let capMinor: number | undefined;
    if (cap.trim()) {
      const m = code === null ? null : toMinor(cap, code);
      if (code === null || m === null || m < 0) {
        error = t("p.money.amount_or_currency");
        return;
      }
      capMinor = m;
    }
    await act(() =>
      ppm.createScenario({
        name,
        plan_pids: pids,
        ...(capMinor !== undefined && code !== null
          ? { budget_cap_minor: capMinor, currency: code }
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

  async function rollback(s: Scenario) {
    listNotice = "";
    error = null;
    if (s.status !== "committed") return;
    if (!(await askConfirm(t("p.frontier.confirm_rollback", { name: s.name })))) return;
    busy = true;
    try {
      await ppm.rollbackScenario(s.pid);
      listNotice = t("p.frontier.rolled_back", { name: s.name });
      await refresh();
    } catch (err) {
      const k = rollbackRefusalKey(err);
      // The list is read again first, as reading it clears the page's error.
      if (k) await refresh();
      error = k ? t(k, { name: s.name }) : errorText(err, "p.frontier.could_not_roll_back");
    } finally {
      busy = false;
    }
  }

  const listProblems = (keys: string[]) =>
    keys.map((k) => t(k as Parameters<typeof t>[0], { min: STEPS_MIN, max: STEPS_MAX, list: LIST_MAX, bytes: NAME_MAX_BYTES })).join(" ");

  async function runFrontier(e: SubmitEvent) {
    e.preventDefault();
    fNotice = "";
    const r = frontierQuery({ ...fForm, mustInclude });
    if ("problems" in r) {
      fProblem = listProblems(r.problems);
      return;
    }
    fProblem = null;
    busy = true;
    try {
      frontier = await ppm.scenarioFrontier(r.query);
      frontierAsked = { currency: r.query.currency, mustInclude: r.query.mustInclude ?? [] };
      choice = null;
    } catch (err) {
      const k = frontierRefusalKey("frontier", err);
      fProblem = k ? t(k) : errorText(err, "p.frontier.could_not_read");
    } finally {
      busy = false;
    }
  }

  function showPoint(p: FrontierPoint) {
    if (!frontier) return;
    choice = {
      title: t("p.frontier.choice_at", { cap: money(p.budget_cap_minor, frontier.currency) }),
      rows: p.rationale,
      currency: frontier.currency,
    };
  }

  async function savePoint(p: FrontierPoint) {
    if (!frontier || !frontierAsked) return;
    fNotice = "";
    fProblem = null;
    const label = t("p.frontier.point_name", { cap: money(p.budget_cap_minor, frontier.currency) });
    busy = true;
    try {
      const made = await ppm.generateScenario(pointBody(label, p, frontierAsked.currency, frontierAsked.mustInclude));
      fNotice = t("p.frontier.draft_saved", { name: label });
      choice = { title: t("p.frontier.choice_of", { name: label }), rows: made.rationale, currency: frontierAsked.currency };
      await refresh();
    } catch (err) {
      const k = frontierRefusalKey("generate", err);
      fProblem = k ? t(k) : errorText(err, "p.frontier.could_not_generate");
    } finally {
      busy = false;
    }
  }

  async function generate(e: SubmitEvent) {
    e.preventDefault();
    gNotice = "";
    const r = generateBody({ ...gForm, mustInclude });
    if ("problems" in r) {
      gProblem = listProblems(r.problems);
      return;
    }
    gProblem = null;
    busy = true;
    try {
      const made = await ppm.generateScenario(r.body);
      gNotice = t("p.frontier.draft_saved", { name: r.body.name ?? "" });
      choice = { title: t("p.frontier.choice_of", { name: r.body.name ?? "" }), rows: made.rationale, currency: r.body.currency ?? null };
      gForm = { name: "", cap: "", currency: gForm.currency, mustInclude: [] };
      await refresh();
    } catch (err) {
      const k = frontierRefusalKey("generate", err);
      gProblem = k ? t(k) : errorText(err, "p.frontier.could_not_generate");
    } finally {
      busy = false;
    }
  }

  const scenarioColumns: DataGridColumn[] = $derived([
    { id: "name", header: t("p.scenarios.scenario"), rowHeader: true },
    { id: "budget_cap_minor", header: t("p.scenarios.cap"), align: "end", format: (v, r) => (v !== null && v !== undefined ? money(Number(v), String(r.currency)) : "—") },
    { id: "status", header: t("p.scenarios.status"), format: (v) => (v === "committed" ? t("p.frontier.status_committed") : v === "draft" ? t("p.frontier.status_draft") : String(v)), cell: statusChip },
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

  const pointColumns: DataGridColumn[] = $derived([
    { id: "budget_cap_minor", header: t("p.frontier.col_cap"), rowHeader: true, align: "end", format: (v) => money(Number(v), frontier?.currency ?? "") },
    { id: "cost_minor", header: t("p.frontier.col_cost"), align: "end", format: (v) => money(Number(v), frontier?.currency ?? "") },
    { id: "value", header: t("p.frontier.col_value"), align: "end", format: (v) => formatDecimal(Number(v), 1) },
    { id: "included", header: t("p.frontier.col_included"), align: "end" },
    { id: "dominated", header: t("p.frontier.col_more"), format: (v) => (v ? t("p.frontier.more_no") : t("p.frontier.more_yes")) },
    { id: "rationale", header: "", sortable: false, cell: pointActions },
  ]);
  type ChoiceRow = ScenarioChoice & { name: string | null };
  const choiceColumns: DataGridColumn[] = $derived([
    { id: "name", header: t("p.frontier.col_plan"), rowHeader: true, format: (v) => (v ? String(v) : t("p.frontier.plan_unknown")) },
    { id: "included", header: t("p.frontier.col_chosen"), format: (v) => (v ? `✓ ${t("p.frontier.chosen")}` : `○ ${t("p.frontier.left_out")}`) },
    { id: "score", header: t("p.frontier.col_score"), align: "end", format: (v) => (v === null || v === undefined ? t("p.frontier.no_score") : formatDecimal(Number(v), 1)) },
    { id: "cost_minor", header: t("p.frontier.col_plan_cost"), align: "end", format: (v) => (v === null || v === undefined || !choice?.currency ? "—" : money(Number(v), choice.currency)) },
    { id: "reason", header: t("p.frontier.col_why"), sortable: false, accessor: (r) => (rowOf<ChoiceRow>(r).included ? "" : t(reasonKey(rowOf<ChoiceRow>(r).reason))) },
  ]);
</script>

{#snippet statusChip({ formatted }: DataGridCellContext)}<span class="chip">{formatted}</span>{/snippet}

{#snippet evalCell({ row }: DataGridCellContext)}
  {@const evaluated = evaluations[String(row.pid)]}
  {#if evaluated}
    {#each evaluated.evaluation.planned_by_currency as [code, total] (code)}<span class="chip">{money(total, code)}</span>{/each}
    <span class="chip">{t("ppm.scenarios.exposure")} {evaluated.evaluation.total_exposure}</span>
    <span class="chip">{t("ppm.scenarios.alignment")} {evaluated.evaluation.total_alignment}</span>
    {#if evaluated.evaluation.violations.length > 0}<span class="chip red">{tn("p.frontier.violations", evaluated.evaluation.violations.length, { n: evaluated.evaluation.violations.length })}</span>{/if}
    {#if evaluated.feasible}<span class="chip green">{t("ppm.scenarios.feasible")}</span>{/if}
  {/if}
{/snippet}

{#snippet scenarioActions({ row }: DataGridCellContext)}
  {@const s = rowOf<Scenario>(row)}
  <button class="button small" onclick={() => evaluate(s.pid)}>{t("ppm.scenarios.evaluate")}</button>
  {#if s.status === "draft"}
    <button class="button small" onclick={() => act(() => ppm.commitScenario(s.pid))}>{t("ppm.scenarios.commit")}</button>
  {:else if s.status === "committed"}
    <button class="button small" aria-label={t("p.frontier.rollback_named", { name: s.name })} disabled={busy} onclick={() => rollback(s)}>{t("p.frontier.rollback")}</button>
  {/if}
{/snippet}

{#snippet pointActions({ row }: DataGridCellContext)}
  {@const p = rowOf<FrontierPoint>(row)}
  {@const cap = money(p.budget_cap_minor, frontier?.currency ?? "")}
  <button class="button small" type="button" aria-label={t("p.frontier.show_choice_named", { cap })} onclick={() => showPoint(p)}>{t("p.frontier.show_choice")}</button>
  <button class="button small" type="button" aria-label={t("p.frontier.save_point_named", { cap })} disabled={busy} onclick={() => savePoint(p)}>{t("p.frontier.save_point")}</button>
{/snippet}


<svelte:head><title>{t("p.scenarios.scenarios_ppm")}</title></svelte:head>

<h1>{t("ppm.scenarios.title")}</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}
<p class="small" role="status">{listNotice}</p>

<form class="row" onsubmit={create}>
  <input placeholder={t("ppm.scenarios.name")} bind:value={name} required />
  <input placeholder={t("p.scenarios.work_item_pids_space_comma")} bind:value={members} size="50" />
  <input placeholder={t("ppm.scenarios.cap")} bind:value={cap} size="12" />
  <input bind:value={currency} size="4" maxlength="3" aria-label={t("p.scenarios.currency")} placeholder={t("p.money.currency_hint")} />
  <button class="button primary" type="submit">{t("ppm.scenarios.create")}</button>
</form>

<p class="small muted">{t("p.frontier.rollback_says")}</p>
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
  <p class="muted">{t("p.frontier.compare_note")}</p>
  <DataTable label={t("p.scenarios.delta_b_a")} columns={compareColumns(comparison)} rows={compareRows(comparison)} rowId={(_r, i) => String(i)} sortable={false} testid="scenario-compare" />
{/if}

<section class="choose" aria-labelledby="choose-h">
  <h2 id="choose-h">{t("p.frontier.heading")}</h2>
  <p class="small muted">{t("p.frontier.says")}</p>

  <fieldset class="must">
    <legend>{t("p.frontier.must_include", { max: LIST_MAX })}</legend>
    {#if plans === null}
      <p class="small">{t("p.frontier.plans_loading")}</p>
    {:else if plans.length === 0}
      <p class="small">{t("p.frontier.plans_none")}</p>
    {:else}
      {#each plans as p (p.pid)}
        <label class="check"><input type="checkbox" value={p.pid} bind:group={mustInclude} /> {p.name}</label>
      {/each}
    {/if}
  </fieldset>

  <section aria-labelledby="frontier-h">
    <h3 id="frontier-h">{t("p.frontier.frontier_heading")}</h3>
    <p class="small muted">{t("p.frontier.frontier_says", { min: STEPS_MIN, max: STEPS_MAX, fallback: STEPS_DEFAULT, list: LIST_MAX })}</p>
    {#if fProblem}<p class="banner error" role="alert">{fProblem}</p>{/if}
    <p class="small" role="status">{fNotice}</p>
    <form class="stack" novalidate onsubmit={runFrontier} aria-label={t("p.frontier.frontier_heading")}>
      <div class="row">
        <label>{t("p.frontier.f_currency")}<input bind:value={fForm.currency} size="4" maxlength="3" /></label>
      </div>
      <fieldset>
        <legend>{t("p.frontier.f_how")}</legend>
        <label class="check"><input type="radio" value="steps" bind:group={fForm.how} /> {t("p.frontier.how_steps")}</label>
        <label class="check"><input type="radio" value="caps" bind:group={fForm.how} /> {t("p.frontier.how_caps")}</label>
      </fieldset>
      {#if fForm.how === "steps"}
        <label>{t("p.frontier.f_steps", { fallback: STEPS_DEFAULT })}<input type="number" min={STEPS_MIN} max={STEPS_MAX} step="1" bind:value={fForm.steps} /></label>
      {:else}
        <label class="wide">{t("p.frontier.f_caps", { list: LIST_MAX })}<textarea bind:value={fForm.caps} rows="4"></textarea></label>
      {/if}
      <div class="row"><button class="button" type="submit" disabled={busy}>{t("p.frontier.run")}</button></div>
    </form>
    {#if frontier}
      <FrontierChart points={frontier.points} currency={frontier.currency} id="frontier" />
      <p class="small muted">{t("p.frontier.value_is")}</p>
      <DataTable label={t("p.frontier.table")} columns={pointColumns} rows={frontier.points} rowId={(r) => String(r.budget_cap_minor)} sortable={false} testid="frontier-table" />
    {/if}
  </section>

  <section aria-labelledby="generate-h">
    <h3 id="generate-h">{t("p.frontier.generate_heading")}</h3>
    <p class="small muted">{t("p.frontier.generate_says", { bytes: NAME_MAX_BYTES })}</p>
    {#if gProblem}<p class="banner error" role="alert">{gProblem}</p>{/if}
    <p class="small" role="status">{gNotice}</p>
    <form class="stack" novalidate onsubmit={generate} aria-label={t("p.frontier.generate_heading")}>
      <div class="row">
        <label>{t("p.frontier.g_name")}<input bind:value={gForm.name} /></label>
        <label>{t("p.frontier.g_cap")}<input bind:value={gForm.cap} size="12" /></label>
        <label>{t("p.frontier.g_currency")}<input bind:value={gForm.currency} size="4" maxlength="3" /></label>
      </div>
      <div class="row"><button class="button" type="submit" disabled={busy}>{t("p.frontier.generate")}</button></div>
    </form>
  </section>

  {#if choice}
    {@const rows = choiceRows(choice.rows, names)}
    <section aria-labelledby="choice-h">
      <h3 id="choice-h">{choice.title}</h3>
      <p class="small">{tn("p.frontier.chosen_count", rows.filter((r) => r.included).length, { n: rows.filter((r) => r.included).length, of: rows.length })}</p>
      <DataTable label={t("p.frontier.col_plan")} columns={choiceColumns} rows={rows} rowId={(r) => String(r.pid)} sortable={false} testid="choice-table" />
    </section>
  {/if}
</section>

<style>
  .row { display: flex; gap: 0.5rem; flex-wrap: wrap; align-items: center; margin: 0.8rem 0; }
  .choose { margin-block: 1.5rem; }
  .choose label { display: grid; gap: 0.2rem; }
  .choose label.check { display: flex; align-items: center; gap: 0.35rem; }
  .wide { max-width: 30rem; }
  fieldset { border: 1px solid var(--border, #ccc); padding: 0.5rem 0.75rem; margin: 0 0 0.75rem; }
  .must { display: flex; flex-wrap: wrap; gap: 0.25rem 1rem; max-block-size: 14rem; overflow-y: auto; }
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
