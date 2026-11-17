<!--
  Scorecards (`/deescalation`, EX-10, NC-1 slice 11): a scorecard is a named set of KPIs read by finished quarter. List
  them; add one, perhaps as the one the de-escalation tracker reads (at most one is); choose one and see its KPIs by
  quarter (met, not met, or nothing recorded, as a word and a symbol); choose its KPIs from the KPI list (at most 60;
  nothing is sent for no change); and add measures from the starter template, which has no targets: the reader sets each
  target and red line, the direction rule said first. A refusal is the catalogue's sentence.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import type { DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import DataTable from "#lib/components/DataTable.svelte";
  import KpiQuarterTable from "#lib/components/KpiQuarterTable.svelte";
  import { errorText } from "#lib/error-text.js";
  import { t, tDynamic, tn } from "#lib/i18n.svelte.js";
  import { PpmClient, type ScorecardRow, type ScorecardView, type TemplateMeasure } from "#lib/api/ppm.js";
  import {
    SCORECARD_KPIS_MAX,
    SCORECARD_NAME_MAX,
    SCORECARD_NAME_MIN,
    STATE_MARK,
    TEMPLATE_KEYS,
    deescalationRefusalKey,
    kpisBody,
    quarterName,
    quarterReasons,
    scorecardProblem,
    templateBody,
    type TemplatePick,
  } from "#lib/deescalation-view.js";

  let { onChanged }: { onChanged: () => Promise<void> } = $props();

  const ppm = PpmClient.withFetch();
  let busy = $state(false);
  let scorecards = $state<ScorecardRow[] | null>(null);
  let loadError = $state<string | null>(null);

  let name = $state("");
  let forTracker = $state(false);
  let createError = $state<string | null>(null);
  let createNotice = $state("");

  let chosen = $state("");
  let view = $state<ScorecardView | null>(null);
  let viewError = $state<string | null>(null);
  let allKpis = $state<Array<{ pid: string; name: string }> | null>(null);
  let members = $state<string[]>([]);
  let kpisError = $state<string | null>(null);
  let kpisNotice = $state("");

  let measures = $state<TemplateMeasure[] | null>(null);
  let picks = $state<Record<string, TemplatePick>>({});
  let templateError = $state<string | null>(null);
  let templateNotice = $state("");

  const qn = (q: string) => quarterName(q, t);
  const measureName = (key: string) =>
    (TEMPLATE_KEYS as readonly string[]).includes(key) ? tDynamic(`p.deescalation.measure_${key}`) : key;
  const unitWord = (unit: string) =>
    unit === "percent" ? t("p.deescalation.unit_percent") : unit === "count" ? t("p.deescalation.unit_count") : unit;

  async function load() {
    try {
      scorecards = (await ppm.scorecards()).scorecards;
      loadError = null;
    } catch (err) {
      loadError = errorText(err, "p.deescalation.could_not_load_scorecards");
    }
  }
  onMount(() => void load());

  // The KPI checkboxes are filled from the scorecard and the KPI list, and the template's inputs from the template;
  // each stays disabled until its answer has come, so a late answer cannot overwrite the reader's choice.
  const membersReady = $derived(view !== null && allKpis !== null && view.scorecard.pid === chosen);

  async function loadView(pid: string, fill: boolean) {
    try {
      const v = await ppm.scorecard(pid);
      if (pid !== chosen) return;
      view = v;
      if (fill) members = v.kpis.map((k) => k.kpi_pid);
      viewError = null;
    } catch (err) {
      viewError = errorText(err, "p.deescalation.could_not_load_scorecard");
    }
  }
  async function loadKpis() {
    try {
      allKpis = (await ppm.kpis()).kpis.map((k) => ({ pid: k.kpi.pid, name: k.kpi.name }));
    } catch (err) {
      viewError = errorText(err, "p.deescalation.could_not_load_scorecard");
    }
  }
  async function loadTemplate() {
    if (measures) return;
    try {
      const m = (await ppm.scorecardTemplate()).measures;
      picks = Object.fromEntries(m.map((x) => [x.key, { key: x.key, chosen: false, target: "", red: "" }]));
      measures = m;
    } catch (err) {
      templateError = errorText(err, "p.deescalation.could_not_load_template");
    }
  }

  function choose() {
    view = null;
    members = [];
    viewError = null;
    kpisError = null;
    kpisNotice = "";
    templateError = null;
    templateNotice = "";
    if (!chosen) return;
    void loadView(chosen, true);
    void loadKpis();
    void loadTemplate();
  }

  /** Every KPI that can be put on the card: the KPI list, and any on the card it does not hold. */
  const kpiOptions = $derived.by(() => {
    const out = [...(allKpis ?? [])];
    for (const k of view?.kpis ?? []) if (!out.some((x) => x.pid === k.kpi_pid)) out.push({ pid: k.kpi_pid, name: k.name });
    return out;
  });

  async function create(e: SubmitEvent) {
    e.preventDefault();
    createError = null;
    createNotice = "";
    const problem = scorecardProblem(name, forTracker, scorecards ?? []);
    if (problem) {
      createError = t(problem, {
        min: SCORECARD_NAME_MIN,
        max: SCORECARD_NAME_MAX,
        name: (scorecards ?? []).find((s) => s.for_deescalation)?.name ?? "",
      });
      return;
    }
    busy = true;
    try {
      await ppm.createScorecard(forTracker ? { name: name.trim(), for_deescalation: true } : { name: name.trim() });
      createNotice = t("p.deescalation.scorecard_added", { name: name.trim() });
      name = "";
      forTracker = false;
      await load();
      await onChanged();
    } catch (err) {
      const k = deescalationRefusalKey("create", err, forTracker);
      createError = k ? t(k, { min: SCORECARD_NAME_MIN, max: SCORECARD_NAME_MAX, name: "" }) : errorText(err, "p.deescalation.could_not_save");
    } finally {
      busy = false;
    }
  }

  async function saveKpis(e: SubmitEvent) {
    e.preventDefault();
    if (!view) return;
    kpisError = null;
    kpisNotice = "";
    const r = kpisBody(
      view.kpis.map((k) => k.kpi_pid),
      members,
    );
    if (r === null) {
      kpisNotice = t("p.deescalation.unchanged");
      return;
    }
    if ("problem" in r) {
      kpisError = t(r.problem, { max: SCORECARD_KPIS_MAX });
      return;
    }
    busy = true;
    try {
      await ppm.setScorecardKpis(view.scorecard.pid, r.body.kpi_pids);
      kpisNotice = tn("p.deescalation.kpis_saved", r.body.kpi_pids.length, { n: r.body.kpi_pids.length });
      await loadView(view.scorecard.pid, true);
      await onChanged();
    } catch (err) {
      const k = deescalationRefusalKey("kpis", err);
      kpisError = k ? t(k) : errorText(err, "p.deescalation.could_not_save");
    } finally {
      busy = false;
    }
  }

  async function addMeasures(e: SubmitEvent) {
    e.preventDefault();
    if (!view || !measures) return;
    templateError = null;
    templateNotice = "";
    const r = templateBody(Object.values(picks), measures);
    if ("problem" in r) {
      templateError = t(r.problem, { measure: r.key ? measureName(r.key) : "" });
      return;
    }
    busy = true;
    try {
      await ppm.scorecardFromTemplate(view.scorecard.pid, r.body.measures);
      templateNotice = tn("p.deescalation.measures_added", r.body.measures.length, { n: r.body.measures.length });
      for (const p of Object.values(picks)) Object.assign(p, { chosen: false, target: "", red: "" });
      await Promise.all([loadView(view.scorecard.pid, true), loadKpis()]);
      await onChanged();
    } catch (err) {
      const k = deescalationRefusalKey("template", err);
      templateError = k ? t(k) : errorText(err, "p.deescalation.could_not_save");
    } finally {
      busy = false;
    }
  }

  const listColumns: DataGridColumn[] = $derived([
    { id: "name", header: t("p.deescalation.col_scorecard"), rowHeader: true },
    {
      id: "for_deescalation",
      header: t("p.deescalation.col_tracker"),
      format: (v) => (v ? t("p.deescalation.is_tracker") : t("p.deescalation.not_tracker")),
    },
  ]);

  function standing(quarter: string, state: "met" | "not_met" | "missing"): string {
    if (!view) return "";
    const r = quarterReasons(quarter, view.kpis, []);
    const parts: string[] = [];
    if (r.nothing) parts.push(t("p.deescalation.reason_no_kpis"));
    if (r.notMet.length > 0) parts.push(t("p.deescalation.reason_not_met", { list: r.notMet.join("; ") }));
    if (r.missing.length > 0) parts.push(t("p.deescalation.reason_missing", { list: r.missing.join("; ") }));
    return t("p.deescalation.quarter_standing", {
      quarter: qn(quarter),
      state: t(STATE_MARK[state].key),
      reasons: parts.join(" "),
    }).trim();
  }
</script>

<section aria-labelledby="scorecards-h">
  <h2 id="scorecards-h">{t("p.deescalation.scorecards_heading")}</h2>
  <p class="small muted">{t("p.deescalation.scorecards_says")}</p>
  {#if loadError}<p class="banner error" role="alert">{loadError}</p>{/if}
  {#if scorecards}
    {#if scorecards.length === 0}
      <p class="muted">{t("p.deescalation.scorecards_none")}</p>
    {:else}
      <DataTable label={t("p.deescalation.scorecards_heading")} columns={listColumns} rows={scorecards} rowId={(r) => String(r.pid)} exportable={false} />
    {/if}
  {/if}

  <h3>{t("p.deescalation.add_scorecard_heading")}</h3>
  <p class="small muted">{t("p.deescalation.add_scorecard_says", { min: SCORECARD_NAME_MIN, max: SCORECARD_NAME_MAX })}</p>
  {#if createError}<p class="banner error" role="alert">{createError}</p>{/if}
  <p class="small" role="status">{createNotice}</p>
  <form class="stack" novalidate onsubmit={create}>
    <label>{t("p.deescalation.f_name")}<input type="text" bind:value={name} maxlength={SCORECARD_NAME_MAX} /></label>
    <label class="check"><input type="checkbox" bind:checked={forTracker} disabled={!scorecards} />{t("p.deescalation.f_for_tracker")}</label>
    <div class="row"><button class="button" type="submit" disabled={busy || !scorecards}>{t("p.deescalation.add_scorecard")}</button></div>
  </form>
</section>

<section aria-labelledby="scorecard-h">
  <h2 id="scorecard-h">{t("p.deescalation.scorecard_heading")}</h2>
  <label>
    {t("p.deescalation.f_scorecard")}
    <select bind:value={chosen} onchange={choose} disabled={!scorecards}>
      <option value="">{t("p.deescalation.choose_scorecard")}</option>
      {#each scorecards ?? [] as s (s.pid)}<option value={s.pid}>{s.name}</option>{/each}
    </select>
  </label>
  {#if viewError}<p class="banner error" role="alert">{viewError}</p>{/if}
  {#if chosen && view}
    <p class="small muted">{t("p.deescalation.scorecard_says")}</p>
    {#if view.kpis_you_may_not_read > 0}
      <p class="small">{tn("p.deescalation.hidden_kpis", view.kpis_you_may_not_read, { n: view.kpis_you_may_not_read })}</p>
    {/if}
    <KpiQuarterTable label={t("p.deescalation.scorecard_kpis_label", { name: view.scorecard.name })} kpis={view.kpis} quarters={view.quarters} />
    <ul class="small" aria-label={t("p.deescalation.scorecard_quarters_label")}>
      {#each view.by_quarter as b (b.quarter)}
        <li><span aria-hidden="true">{STATE_MARK[b.state.state].symbol}</span> {standing(b.quarter, b.state.state)}</li>
      {/each}
    </ul>

    <h3>{t("p.deescalation.kpis_heading")}</h3>
    <p class="small muted">{t("p.deescalation.kpis_says", { max: SCORECARD_KPIS_MAX })}</p>
    {#if kpisError}<p class="banner error" role="alert">{kpisError}</p>{/if}
    <p class="small" role="status">{kpisNotice}</p>
    <form class="stack" novalidate onsubmit={saveKpis}>
      <fieldset disabled={!membersReady}>
        <legend>{t("p.deescalation.f_kpis")}</legend>
        {#if kpiOptions.length === 0}<p class="muted">{t("p.deescalation.kpis_none")}</p>{/if}
        {#each kpiOptions as k (k.pid)}
          <label class="check"><input type="checkbox" value={k.pid} bind:group={members} />{k.name}</label>
        {/each}
      </fieldset>
      <div class="row"><button class="button" type="submit" disabled={busy || !membersReady}>{t("p.deescalation.save_kpis")}</button></div>
    </form>

    <h3>{t("p.deescalation.template_heading")}</h3>
    <p class="small muted">{t("p.deescalation.template_says")}</p>
    <p class="small">{t("p.deescalation.template_rule")}</p>
    {#if templateError}<p class="banner error" role="alert">{templateError}</p>{/if}
    <p class="small" role="status">{templateNotice}</p>
    {#if measures}
      <form class="stack" novalidate onsubmit={addMeasures}>
        {#each measures as m (m.key)}
          {@const p = picks[m.key]}
          {#if p}
            <fieldset class="measure">
              <legend>{measureName(m.key)}</legend>
              <p class="small">
                {m.direction === "lower_better" ? t("p.deescalation.lower_better") : t("p.deescalation.higher_better")} ·
                {t("p.deescalation.unit", { unit: unitWord(m.unit) })}
              </p>
              <label class="check"><input type="checkbox" bind:checked={p.chosen} />{t("p.deescalation.f_pick", { measure: measureName(m.key) })}</label>
              <label>{t("p.deescalation.f_target")}<input type="number" step="1" inputmode="numeric" bind:value={p.target} /></label>
              <label>{t("p.deescalation.f_red")}<input type="number" step="1" inputmode="numeric" bind:value={p.red} /></label>
            </fieldset>
          {/if}
        {/each}
        <div class="row"><button class="button" type="submit" disabled={busy}>{t("p.deescalation.add_measures")}</button></div>
      </form>
    {:else if !templateError}
      <p class="muted">{t("p.deescalation.loading")}</p>
    {/if}
  {/if}
</section>

<style>
  section {
    margin-block-start: 2rem;
  }
  form {
    max-width: 40rem;
  }
  h3 {
    font-size: 1rem;
    margin-block: 1rem 0.25rem;
  }
  fieldset {
    border: 0;
    padding: 0;
    margin: 0;
    display: grid;
    gap: 0.35rem;
  }
  .measure {
    margin-block-start: 0.75rem;
  }
  .measure legend {
    font-weight: 600;
  }
  .check {
    display: inline-flex;
    gap: 0.35rem;
    align-items: center;
  }
</style>
