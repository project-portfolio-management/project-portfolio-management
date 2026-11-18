<!--
  A plan's WSJF and RICE estimates and scores (`/prioritization`, NC-1 slice 12 part g, NC-6): the inputs as stored
  (`GET /api/plans/{pid}/scoring-inputs`; the inputs disabled until read), replaced on saving (`PUT`, every input
  sent, an empty box cleared; nothing sent for no change; every problem said at once, first), and each model's score
  (`GET /api/plans/{pid}/score?model=wsjf|rice`) with every component named and explained in the catalogue's words and
  what a score still waits for. The service's formula, explanations and reasons are English and never shown. Rules in
  `src/lib/scoring-view.ts`.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { errorText } from "#lib/error-text.js";
  import { t } from "#lib/i18n.svelte.js";
  import { formatDecimal, formatNumber, formatPercent, formatDateTime } from "#lib/locale-format.js";
  import { PpmClient, type EstimateModel, type PlanModelScore, type ScoringInputsView } from "#lib/api/ppm.js";
  import {
    FORMULA_KEY,
    IMPACTS,
    IMPACT_KEY,
    INPUT_KEY,
    MODEL_KEY,
    REACH_MAX,
    WSJF_INPUTS,
    WSJF_MAX,
    WSJF_MIN,
    componentKeys,
    componentShape,
    inputKey,
    missingKey,
    scoringBody,
    scoringFormOf,
    scoringRefusalKey,
    type ScoringForm,
  } from "#lib/scoring-view.js";

  let { pid, name, onsaved, onclose }: { pid: string; name: string; onsaved: () => void; onclose: () => void } = $props();

  const ppm = PpmClient.withFetch();
  const MODELS: EstimateModel[] = ["wsjf", "rice"];
  let stored = $state<ScoringInputsView | null>(null);
  let form = $state<ScoringForm>(scoringFormOf(null));
  let scores = $state<Partial<Record<EstimateModel, PlanModelScore>>>({});
  let loadError = $state<string | null>(null);
  let problem = $state<string | null>(null);
  let notice = $state("");
  let busy = $state(false);

  async function loadScores() {
    const [wsjf, rice] = await Promise.all(MODELS.map((m) => ppm.modelScore(pid, m)));
    scores = { wsjf, rice };
  }
  async function load() {
    try {
      stored = await ppm.scoringInputs(pid);
      form = scoringFormOf(stored.inputs);
      await loadScores();
      loadError = null;
    } catch (err) {
      loadError = errorText(err, "p.scoring.could_not_read");
    }
  }
  onMount(load);

  async function save(e: SubmitEvent) {
    e.preventDefault();
    if (!stored) return;
    notice = "";
    const r = scoringBody(form, stored.set ? stored.inputs : null);
    if (r === null) {
      problem = null;
      notice = t("p.scoring.unchanged");
      return;
    }
    if ("problems" in r) {
      problem = r.problems.map((p) => t(p.key, { input: t(INPUT_KEY[p.input]), min: WSJF_MIN, max: WSJF_MAX })).join(" ");
      return;
    }
    problem = null;
    busy = true;
    try {
      stored = await ppm.setScoringInputs(pid, r.body);
      form = scoringFormOf(stored.inputs);
      notice = t("p.scoring.saved", { name });
      await loadScores();
      onsaved();
    } catch (err) {
      const k = scoringRefusalKey(err);
      problem = k ? t(k) : errorText(err, "p.scoring.could_not_save");
    } finally {
      busy = false;
    }
  }

  function valueText(component: string, value: number | null): string {
    const s = componentShape(component, value);
    if (s.kind === "absent") return t("p.scoring.not_estimated");
    if (s.kind === "percent") return formatPercent(s.fraction);
    if (s.kind === "impact") return s.word ? t("p.scoring.impact_value", { value: formatNumber(s.value), word: t(s.word) }) : formatNumber(s.value);
    return formatNumber(s.value);
  }
</script>

<section class="estimates" aria-labelledby="est-h">
  <h2 id="est-h">{t("p.scoring.heading", { name })}</h2>
  <p class="small muted">{t("p.scoring.says")}</p>
  {#if loadError}<p class="banner" role="alert">{loadError}</p>{/if}
  {#if stored?.set && stored.updated_at}
    <p class="small">{t("p.scoring.last_set", { when: formatDateTime(stored.updated_at) })}</p>
  {:else if stored}
    <p class="small">{t("p.scoring.none_set")}</p>
  {/if}

  {#if problem}<p class="banner error" role="alert">{problem}</p>{/if}
  <p class="small" role="status">{notice}</p>
  <form class="stack" novalidate onsubmit={save} aria-label={t("p.scoring.form", { name })}>
    <fieldset>
      <legend>{t("p.scoring.wsjf_inputs", { min: WSJF_MIN, max: WSJF_MAX })}</legend>
      {#each WSJF_INPUTS as input (input)}
        <label>{t(INPUT_KEY[input])}<input type="number" min={WSJF_MIN} max={WSJF_MAX} step="1" bind:value={form[input]} disabled={!stored} /></label>
      {/each}
    </fieldset>
    <fieldset>
      <legend>{t("p.scoring.rice_inputs")}</legend>
      <label>{t(INPUT_KEY.reach)}<input type="number" min="0" max={REACH_MAX} step="1" bind:value={form.reach} disabled={!stored} /></label>
      <label>{t(INPUT_KEY.impact)}
        <select bind:value={form.impact} disabled={!stored}>
          <option value={null}>{t("p.scoring.not_estimated")}</option>
          {#each IMPACTS as v (v)}<option value={v}>{t("p.scoring.impact_value", { value: formatNumber(v), word: t(IMPACT_KEY[v]) })}</option>{/each}
        </select>
      </label>
      <label>{t(INPUT_KEY.confidence_percent)}<input type="number" min="0" max="100" step="1" bind:value={form.confidence_percent} disabled={!stored} /></label>
      <label>{t(INPUT_KEY.effort)}<input type="number" min="0" step="any" bind:value={form.effort} disabled={!stored} /></label>
    </fieldset>
    <div class="row">
      <button class="button" type="submit" disabled={!stored || busy}>{t("p.scoring.save")}</button>
      <button class="button" type="button" onclick={onclose}>{t("p.scoring.close")}</button>
    </div>
  </form>

  {#each MODELS as model (model)}
    {@const s = scores[model]?.score}
    <section aria-labelledby={`score-${model}`}>
      <h3 id={`score-${model}`}>{t(MODEL_KEY[model])}</h3>
      <p class="small muted">{t(FORMULA_KEY[model])}</p>
      {#if s}
        {#if s.score !== null}
          <p><strong>{t("p.scoring.score_is", { score: formatDecimal(s.score, 2) })}</strong></p>
        {:else}
          <p>{t("p.scoring.no_score")}</p>
          <ul class="small">
            {#each s.missing as m (m.input)}<li>{t(inputKey(m.input))}: {t(missingKey(m.input))}</li>{/each}
          </ul>
        {/if}
        <dl class="components">
          {#each s.components as c (c.name)}
            {@const k = componentKeys(c.name)}
            <dt>{t(k.label)}</dt>
            <dd>{valueText(c.name, c.value)} <span class="small muted">· {t(k.means)}</span></dd>
          {/each}
        </dl>
      {/if}
    </section>
  {/each}
</section>

<style>
  .estimates { margin-block: 1.5rem; }
  fieldset { display: flex; flex-wrap: wrap; gap: 0.75rem; border: 1px solid var(--border, #ccc); padding: 0.5rem 0.75rem; margin: 0 0 0.75rem; }
  label { display: grid; gap: 0.2rem; }
  .row { display: flex; gap: 0.5rem; flex-wrap: wrap; }
  .components { display: grid; grid-template-columns: max-content 1fr; gap: 0.2rem 0.75rem; }
  .components dt { font-weight: 600; }
  .components dd { margin: 0; }
</style>
