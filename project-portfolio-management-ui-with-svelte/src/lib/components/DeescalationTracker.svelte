<!--
  The de-escalation tracker (`/deescalation`, EX-10, NC-1 slice 11): the escalation level as the sponsor set it (never
  computed) and its history; the run of consecutive finished quarters met against the number the rule asks for, which
  is offered to the person who decides and moves no level; each quarter's state with the reasons, which this page
  names from the detail (the KPIs and criteria not met or with nothing recorded), never from the service's sentences;
  the criteria with each quarter's evidence. Criteria and evidence are added, never edited: the latest record speaks.
  Who set the level (and each earlier one) and who recorded each piece of evidence, and when, are shown as the service
  returns them: references to people, not names (NC-1 slice 12 part a).
  Every rule the service checks is said first; a refusal is the catalogue's sentence.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import DataTable from "#lib/components/DataTable.svelte";
  import KpiQuarterTable from "#lib/components/KpiQuarterTable.svelte";
  import { errorText } from "#lib/error-text.js";
  import { t, tDynamic, tn } from "#lib/i18n.svelte.js";
  import { formatDate } from "#lib/locale-format.js";
  import { rowOf } from "#lib/row-of.js";
  import { PpmClient, type DeescalationTracker, type QuarterState } from "#lib/api/ppm.js";
  import {
    CRITERION_MAX,
    CRITERION_MIN,
    LEVEL_MAX,
    LEVEL_MIN,
    LINK_MAX,
    NOTE_MIN,
    STATE_MARK,
    criterionProblem,
    deescalationRefusalKey,
    evidenceBody,
    kpiName,
    levelBody,
    levelProblem,
    quarterName,
    quarterReasons,
    sameEvidence,
    stateOf,
  } from "#lib/deescalation-view.js";

  const ppm = PpmClient.withFetch();
  const today = () => new Date().toISOString().slice(0, 10);

  let tracker = $state<DeescalationTracker | null>(null);
  let loadError = $state<string | null>(null);
  // The evidence form's choices are filled from the service; until then it is disabled, so a late answer cannot
  // overwrite what the reader has chosen.
  let loaded = $state(false);
  let busy = $state(false);

  let level = $state("");
  let levelNote = $state("");
  let levelError = $state<string | null>(null);
  let levelNotice = $state("");

  let criterion = $state("");
  let criterionError = $state<string | null>(null);
  let criterionNotice = $state("");

  let evCriterion = $state("");
  let evQuarter = $state("");
  let evMet = $state<"" | "met" | "not_met">("");
  let evNote = $state("");
  let evLink = $state("");
  let evError = $state<string | null>(null);
  let evNotice = $state("");

  export async function reload() {
    try {
      const tr = await ppm.deescalation();
      tracker = tr;
      if (!loaded) {
        evCriterion = tr.criteria[0]?.pid ?? "";
        evQuarter = tr.by_quarter.at(-1)?.quarter ?? "";
      } else if (!evCriterion) {
        evCriterion = tr.criteria[0]?.pid ?? "";
      }
      loadError = null;
      loaded = true;
    } catch (err) {
      loadError = errorText(err, "p.deescalation.could_not_load");
    }
  }
  onMount(() => void reload());

  const qn = (q: string) => quarterName(q, t);
  const list = (xs: string[]) => xs.join("; ");

  function reasons(quarter: string, state: QuarterState): string {
    if (!tracker) return "";
    const r = quarterReasons(quarter, tracker.kpis, tracker.criteria, (k) =>
      kpiName(k, (key) => tDynamic(`p.deescalation.measure_${key}`)),
    );
    if (r.nothing) return t("p.deescalation.reason_nothing");
    if (state.state === "met") return t("p.deescalation.reason_all_met");
    const parts: string[] = [];
    if (r.notMet.length > 0) parts.push(t("p.deescalation.reason_not_met", { list: list(r.notMet) }));
    if (r.missing.length > 0) parts.push(t("p.deescalation.reason_missing", { list: list(r.missing) }));
    return parts.join(" ");
  }

  interface QuarterRow {
    quarter: string;
    state: QuarterState["state"];
    reasons: string;
  }
  const quarterRows = $derived<QuarterRow[]>(
    (tracker?.by_quarter ?? []).map((b) => ({ quarter: b.quarter, state: b.state.state, reasons: reasons(b.quarter, b.state) })),
  );
  const quarterColumns: DataGridColumn[] = $derived([
    { id: "quarter", header: t("p.deescalation.col_quarter"), rowHeader: true, format: (v) => qn(String(v)) },
    {
      id: "state",
      header: t("p.deescalation.col_state"),
      format: (v) => t(STATE_MARK[v as QuarterState["state"]].key),
      cell: stateCell,
    },
    { id: "reasons", header: t("p.deescalation.col_reasons"), accessor: (r) => rowOf<QuarterRow>(r).reasons, sortable: false },
  ]);

  const failure = (action: Parameters<typeof deescalationRefusalKey>[0], err: unknown, fallback: Parameters<typeof errorText>[1]) => {
    const k = deescalationRefusalKey(action, err);
    return k ? t(k, { min: LEVEL_MIN, max: LEVEL_MAX }) : errorText(err, fallback);
  };

  async function saveLevel(e: SubmitEvent) {
    e.preventDefault();
    levelError = null;
    levelNotice = "";
    const problem = levelProblem(level);
    if (problem) {
      levelError = t(problem, { min: LEVEL_MIN, max: LEVEL_MAX });
      return;
    }
    const body = levelBody(tracker?.level ?? null, level, levelNote);
    if (!body) {
      levelNotice = t("p.deescalation.unchanged");
      return;
    }
    busy = true;
    try {
      await ppm.setEscalationLevel(body);
      levelNotice = t("p.deescalation.level_saved", { level: body.level });
      level = "";
      levelNote = "";
      await reload();
    } catch (err) {
      levelError = failure("level", err, "p.deescalation.could_not_save");
    } finally {
      busy = false;
    }
  }

  async function addCriterion(e: SubmitEvent) {
    e.preventDefault();
    criterionError = null;
    criterionNotice = "";
    const problem = criterionProblem(criterion);
    if (problem) {
      criterionError = t(problem, { min: CRITERION_MIN, max: CRITERION_MAX });
      return;
    }
    busy = true;
    try {
      await ppm.addDeescalationCriterion(criterion.trim());
      criterionNotice = t("p.deescalation.criterion_added");
      criterion = "";
      await reload();
    } catch (err) {
      const k = deescalationRefusalKey("criterion", err);
      criterionError = k ? t(k, { min: CRITERION_MIN, max: CRITERION_MAX }) : errorText(err, "p.deescalation.could_not_save");
    } finally {
      busy = false;
    }
  }

  async function saveEvidence(e: SubmitEvent) {
    e.preventDefault();
    evError = null;
    evNotice = "";
    if (!evCriterion) {
      evError = t("p.deescalation.problem_criterion_choose");
      return;
    }
    const r = evidenceBody(
      { quarter: evQuarter, met: evMet === "" ? null : evMet === "met", note: evNote, link: evLink },
      today(),
    );
    if ("problem" in r) {
      evError = t(r.problem, { min: NOTE_MIN, max: LINK_MAX });
      return;
    }
    const c = tracker?.criteria.find((x) => x.pid === evCriterion);
    if (sameEvidence(c, r.body)) {
      evNotice = t("p.deescalation.unchanged");
      return;
    }
    busy = true;
    try {
      await ppm.setCriterionEvidence(evCriterion, r.body);
      evNotice = t("p.deescalation.evidence_saved", { quarter: qn(r.body.quarter) });
      evNote = "";
      evLink = "";
      evMet = "";
      await reload();
    } catch (err) {
      const k = deescalationRefusalKey("evidence", err);
      evError = k ? t(k) : errorText(err, "p.deescalation.could_not_save");
    } finally {
      busy = false;
    }
  }
</script>

{#snippet stateCell({ value, formatted }: DataGridCellContext)}
  <span aria-hidden="true">{STATE_MARK[value as QuarterState["state"]].symbol}</span>
  {formatted}
{/snippet}

<section aria-labelledby="tracker-h">
  <h2 id="tracker-h">{t("p.deescalation.tracker_heading")}</h2>
  <p class="small muted">{t("p.deescalation.tracker_says")}</p>
  {#if loadError}<p class="banner error" role="alert">{loadError}</p>{/if}
  {#if !tracker && !loadError}<p class="muted">{t("p.deescalation.loading")}</p>{/if}

  {#if tracker}
    <h3>{t("p.deescalation.level_heading")}</h3>
    {#if tracker.level}
      <p data-testid="level-now">
        {tracker.level.set_by
          ? t("p.deescalation.level_now_by", { level: tracker.level.level, date: formatDate(tracker.level.set_on), who: tracker.level.set_by })
          : t("p.deescalation.level_now", { level: tracker.level.level, date: formatDate(tracker.level.set_on) })}
      </p>
      {#if tracker.level.note}<p class="small">{t("p.deescalation.level_note", { note: tracker.level.note })}</p>{/if}
    {:else}
      <p class="muted">{t("p.deescalation.level_none")}</p>
    {/if}
    {#if tracker.level_history.length > 1}
      <ul class="small" aria-label={t("p.deescalation.level_history")}>
        {#each tracker.level_history as h, i (i)}
          <li>
            {h.set_by
              ? t("p.deescalation.level_history_item_by", { level: h.level, date: formatDate(h.set_on), who: h.set_by })
              : t("p.deescalation.level_history_item", { level: h.level, date: formatDate(h.set_on) })}
          </li>
        {/each}
      </ul>
    {/if}

    <h3>{t("p.deescalation.run_heading")}</h3>
    <p data-testid="run">
      {tn("p.deescalation.run", tracker.consecutive_quarters_met, { n: tracker.consecutive_quarters_met, need: tracker.required_quarters })}
      {tracker.rule_satisfied ? t("p.deescalation.rule_satisfied") : t("p.deescalation.rule_not_satisfied")}
    </p>
    {#if tracker.required_quarters_setting_unusable}
      <p class="small">{t("p.deescalation.setting_unusable", { n: tracker.required_quarters })}</p>
    {/if}
    {#if tracker.scorecard}
      <p class="small">{t("p.deescalation.reads_scorecard", { name: tracker.scorecard.name })}</p>
    {:else}
      <p class="small">{t("p.deescalation.reads_no_scorecard")}</p>
    {/if}
    {#if tracker.kpis_you_may_not_read > 0}
      <p class="small">{tn("p.deescalation.hidden_kpis", tracker.kpis_you_may_not_read, { n: tracker.kpis_you_may_not_read })}</p>
    {/if}
    <DataTable label={t("p.deescalation.quarters_label")} columns={quarterColumns} rows={quarterRows} rowId={(r) => String(r.quarter)} exportable={false} />
    {#if tracker.scorecard}
      <KpiQuarterTable label={t("p.deescalation.tracker_kpis_label")} kpis={tracker.kpis} quarters={tracker.by_quarter.map((b) => b.quarter)} />
    {/if}
  {/if}
</section>

<section aria-labelledby="level-h">
  <h2 id="level-h">{t("p.deescalation.set_level_heading")}</h2>
  <p class="small muted">{t("p.deescalation.set_level_says", { min: LEVEL_MIN, max: LEVEL_MAX })}</p>
  {#if levelError}<p class="banner error" role="alert">{levelError}</p>{/if}
  <p class="small" role="status">{levelNotice}</p>
  <form class="stack" novalidate onsubmit={saveLevel}>
    <label>{t("p.deescalation.f_level")}<input type="text" bind:value={level} maxlength={LEVEL_MAX} /></label>
    <label>{t("p.deescalation.f_level_note")}<textarea bind:value={levelNote} rows="2"></textarea></label>
    <div class="row"><button class="button" type="submit" disabled={busy || !loaded}>{t("p.deescalation.save_level")}</button></div>
  </form>
</section>

<section aria-labelledby="criteria-h">
  <h2 id="criteria-h">{t("p.deescalation.criteria_heading")}</h2>
  <p class="small muted">{t("p.deescalation.criteria_says")}</p>
  {#if tracker}
    {#if tracker.criteria.length === 0}
      <p class="muted">{t("p.deescalation.criteria_none")}</p>
    {:else}
      <ul class="criteria">
        {#each tracker.criteria as c (c.pid)}
          <li>
            <p>{c.criterion}</p>
            <ul class="small" aria-label={t("p.deescalation.evidence_of", { criterion: c.criterion })}>
              {#each c.quarters as q (q.quarter)}
                {@const s = stateOf(q.met)}
                <li>
                  {qn(q.quarter)}: <span aria-hidden="true">{STATE_MARK[s].symbol}</span>
                  {t(STATE_MARK[s].key)}
                  {#if q.note}— {q.note}{/if}
                  {#if q.link}<a href={q.link} rel="noopener noreferrer" target="_blank">{t("p.deescalation.evidence_link")}</a>{/if}
                  {#if q.recorded_on}
                    <span class="muted">
                      ({q.recorded_by
                        ? t("p.deescalation.evidence_recorded_by", { who: q.recorded_by, date: formatDate(q.recorded_on) })
                        : t("p.deescalation.evidence_recorded_on", { date: formatDate(q.recorded_on) })})
                    </span>
                  {/if}
                </li>
              {/each}
            </ul>
          </li>
        {/each}
      </ul>
    {/if}
  {/if}

  <h3>{t("p.deescalation.add_criterion_heading")}</h3>
  {#if criterionError}<p class="banner error" role="alert">{criterionError}</p>{/if}
  <p class="small" role="status">{criterionNotice}</p>
  <form class="stack" novalidate onsubmit={addCriterion}>
    <label>{t("p.deescalation.f_criterion", { min: CRITERION_MIN, max: CRITERION_MAX })}<textarea bind:value={criterion} rows="2"></textarea></label>
    <div class="row"><button class="button" type="submit" disabled={busy}>{t("p.deescalation.add_criterion")}</button></div>
  </form>

  <h3>{t("p.deescalation.evidence_heading")}</h3>
  <p class="small muted">{t("p.deescalation.evidence_says", { min: NOTE_MIN, max: LINK_MAX })}</p>
  {#if evError}<p class="banner error" role="alert">{evError}</p>{/if}
  <p class="small" role="status">{evNotice}</p>
  <form class="stack" novalidate onsubmit={saveEvidence}>
    <label>
      {t("p.deescalation.f_ev_criterion")}
      <select bind:value={evCriterion} disabled={!loaded}>
        {#each tracker?.criteria ?? [] as c (c.pid)}<option value={c.pid}>{c.criterion}</option>{/each}
      </select>
    </label>
    <label>
      {t("p.deescalation.f_ev_quarter")}
      <select bind:value={evQuarter} disabled={!loaded}>
        {#each tracker?.by_quarter ?? [] as b (b.quarter)}<option value={b.quarter}>{qn(b.quarter)}</option>{/each}
      </select>
    </label>
    <fieldset disabled={!loaded}>
      <legend>{t("p.deescalation.f_ev_met")}</legend>
      <label class="check"><input type="radio" name="ev-met" value="met" bind:group={evMet} />{t("p.deescalation.state_met")}</label>
      <label class="check"><input type="radio" name="ev-met" value="not_met" bind:group={evMet} />{t("p.deescalation.state_not_met")}</label>
    </fieldset>
    <label>{t("p.deescalation.f_ev_note", { min: NOTE_MIN })}<textarea bind:value={evNote} rows="2"></textarea></label>
    <label>{t("p.deescalation.f_ev_link")}<input type="url" bind:value={evLink} /></label>
    <div class="row"><button class="button" type="submit" disabled={busy || !loaded}>{t("p.deescalation.save_evidence")}</button></div>
  </form>
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
  .criteria {
    padding-inline-start: 1.25rem;
  }
  .criteria p {
    margin-block: 0.5rem 0.25rem;
  }
  fieldset {
    border: 0;
    /* Room around the radios, so each is a large enough target (axe target-size). */
    padding: 0.4rem 0;
    margin: 0;
    display: flex;
    gap: 1rem;
  }
  .check {
    display: inline-flex;
    gap: 0.35rem;
    align-items: center;
  }
</style>
