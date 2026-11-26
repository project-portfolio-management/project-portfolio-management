<!--
  Changing one risk on the governance page (NC-1 slice 12 part c, `PUT /api/plans/{pid}/risks/{risk_pid}`): its
  status, likelihood, impact, mitigation and review date. Every rule is said first and checked before sending; only the
  fields that changed are sent, and nothing for no change. Rules in `src/lib/risk-edit-view.ts`. Refusals are the
  catalogue's sentences, never the service's text.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { t } from "#lib/i18n.svelte.js";
  import { enumLabel } from "#lib/enum-labels.js";
  import { formatDate } from "#lib/locale-format.js";
  import { PpmClient, type Risk, type RiskScoreRow } from "#lib/api/ppm.js";
  import {
    MITIGATION_MAX_BYTES,
    SCORE_MAX,
    SCORE_MIN,
    formFromRisk,
    riskBody,
    riskRefusalKey,
    statusChoices,
  } from "#lib/risk-edit-view.js";

  let {
    pid,
    risk,
    onsaved,
    onclose,
  }: { pid: string; risk: Risk; onsaved: () => void | Promise<void>; onclose: () => void } = $props();
  const ppm = PpmClient.withFetch();

  // The form starts from the risk it was opened on; the page opens a new form for another risk.
  // svelte-ignore state_referenced_locally
  let form = $state(formFromRisk(risk));
  let problems = $state<string[]>([]);
  let error = $state<string | null>(null);
  let notice = $state("");
  let busy = $state(false);
  const choices = $derived(statusChoices(risk.status));
  const score = { min: SCORE_MIN, max: SCORE_MAX };
  // The score history (OF-3); a failed read shows nothing rather than an error, the form still works.
  let history = $state<RiskScoreRow[]>([]);
  async function loadHistory() {
    try {
      history = (await ppm.riskScores(pid, risk.pid)).scores;
    } catch {
      history = [];
    }
  }
  $effect(() => {
    void loadHistory();
  });

  async function save(e: SubmitEvent) {
    e.preventDefault();
    problems = [];
    error = null;
    notice = "";
    const r = riskBody(form, risk);
    if (r === null) {
      notice = t("p.risk_edit.unchanged");
      return;
    }
    if ("problems" in r) {
      problems = r.problems.map((k) => t(k, { ...score, max_bytes: MITIGATION_MAX_BYTES }));
      return;
    }
    busy = true;
    try {
      await ppm.updateRisk(pid, risk.pid, r.body);
      notice = t("p.risk_edit.saved", { title: risk.title });
      form.reason = "";
      await onsaved();
      await loadHistory();
    } catch (err) {
      const k = riskRefusalKey(err);
      error = k ? t(k) : errorText(err, "p.risk_edit.could_not_save");
      if (k === "p.risk_edit.refused_gone") await onsaved();
    } finally {
      busy = false;
    }
  }
</script>

<section aria-labelledby="risk-edit-h" class="risk-edit">
  <h3 id="risk-edit-h">{t("p.risk_edit.heading", { title: risk.title })}</h3>
  <p class="small muted">{t("p.risk_edit.says", { ...score, max_bytes: MITIGATION_MAX_BYTES })}</p>
  <p class="small muted">{t("p.risk_edit.says_escalate")}</p>
  {#if problems.length > 0}
    <div class="banner error" role="alert">
      <ul>{#each problems as p, i (i)}<li>{p}</li>{/each}</ul>
    </div>
  {/if}
  {#if error}<p class="banner error" role="alert">{error}</p>{/if}
  <p class="small" role="status">{notice}</p>
  <form class="stack" novalidate onsubmit={save}>
    <label>{t("p.risk_edit.f_status")}
      <select bind:value={form.status}>
        {#each choices as s (s)}<option value={s}>{enumLabel("risk_status", s)}</option>{/each}
      </select>
    </label>
    <label>{t("p.risk_edit.f_probability", score)}
      <input type="number" step="1" min={SCORE_MIN} max={SCORE_MAX} inputmode="numeric" bind:value={form.probability} />
    </label>
    <label>{t("p.risk_edit.f_impact", score)}
      <input type="number" step="1" min={SCORE_MIN} max={SCORE_MAX} inputmode="numeric" bind:value={form.impact} />
    </label>
    <label>{t("p.risk_edit.f_mitigation")}
      <textarea rows="3" bind:value={form.mitigation}></textarea>
    </label>
    <label>{t("p.risk_edit.f_review_date")}
      <input type="date" bind:value={form.reviewDate} />
    </label>
    <label>{t("p.risk_edit.f_reason")}
      <input type="text" maxlength="500" bind:value={form.reason} />
    </label>
    <div class="row">
      <button class="button primary small" type="submit" disabled={busy}>{t("p.risk_edit.save")}</button>
      <button class="button small" type="button" onclick={onclose}>{t("p.risk_edit.close")}</button>
    </div>
  </form>
  {#if history.length > 0}
    <h4>{t("p.risk_edit.history_h")}</h4>
    <ul class="small">
      {#each history as h, i (i)}
        <li>
          {t("p.risk_edit.history_row", { date: formatDate(h.recorded_at), probability: h.probability, impact: h.impact, score: h.score })}
          {#if h.backfilled}{t("p.risk_edit.history_first")}{/if}
          {#if h.reason}{t("p.risk_edit.history_reason", { reason: h.reason })}{/if}
        </li>
      {/each}
    </ul>
  {/if}
</section>

<style>
  .risk-edit {
    max-inline-size: 32rem;
    margin-block: 1rem;
  }
  .stack {
    display: grid;
    gap: 0.5rem;
  }
  .stack label {
    display: grid;
    gap: 0.2rem;
  }
  .banner ul {
    margin: 0;
    padding-inline-start: 1.25rem;
  }
</style>
