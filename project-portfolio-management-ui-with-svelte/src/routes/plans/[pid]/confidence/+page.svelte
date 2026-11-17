<!--
  A plan's delivery confidence (`/plans/[pid]/confidence`, DP-2, NC-1 slice 11): what the portfolio map shows (the
  latest independent assessment, else the latest self-assessment, else "not assessed", never green), its whole history,
  and a form to add one. A value is a word and a symbol as well as a colour. An assessment never replaces another; it
  is kept beside the derived RAG and never blended with it. Amber/red or red opens an exception, which the page says.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { errorText } from "#lib/error-text.js";
  import { enumLabel } from "#lib/enum-labels.js";
  import { localeHref, t, tn } from "#lib/i18n.svelte.js";
  import { formatDate } from "#lib/locale-format.js";
  import { todayIn } from "#lib/timesheet-view.js";
  import { CONFIDENCE_SCALE, PpmClient, type ConfidenceView } from "#lib/api/ppm.js";
  import {
    CONFIDENCE_MARK,
    RATIONALE_MIN,
    assessRefusalKey,
    assessmentProblems,
    newestFirst,
    opensException,
  } from "#lib/confidence-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");
  const tz = Intl.DateTimeFormat().resolvedOptions().timeZone || "UTC";

  let view = $state<ConfidenceView | null>(null);
  let loadError = $state<string | null>(null);
  let problems = $state<string[]>([]);
  let error = $state<string | null>(null);
  let notice = $state("");
  let busy = $state(false);

  let value = $state("");
  let kind = $state("self");
  let rationale = $state("");
  let assessedOn = $state("");

  const word = (v: string): string => enumLabel("pk_conf", v);
  const mark = (v: string) => CONFIDENCE_MARK[v as keyof typeof CONFIDENCE_MARK] ?? CONFIDENCE_MARK.not_assessed;
  const history = $derived(view ? newestFirst(view.history) : []);

  async function load() {
    loadError = null;
    try {
      view = await ppm.deliveryConfidence(pid);
    } catch (err) {
      loadError = errorText(err, "p.confidence.could_not_load");
    }
  }
  onMount(load);

  async function save(e: SubmitEvent) {
    e.preventDefault();
    error = null;
    notice = "";
    problems = assessmentProblems({ value, kind, rationale, assessedOn }, todayIn(tz)).map((k) =>
      t(k, { min: RATIONALE_MIN }),
    );
    if (problems.length > 0) return;
    busy = true;
    try {
      const r = await ppm.assessDeliveryConfidence(pid, {
        value: value as (typeof CONFIDENCE_SCALE)[number],
        kind: kind as "self" | "independent",
        rationale: rationale.trim(),
        ...(assessedOn ? { assessed_on: assessedOn } : {}),
      });
      notice = r.exception
        ? tn("p.confidence.saved_exception", r.exception.window_hours, { hours: r.exception.window_hours })
        : t("p.confidence.saved");
      value = "";
      rationale = "";
      assessedOn = "";
      await load();
    } catch (err) {
      const k = assessRefusalKey(err);
      error = k ? t(k) : errorText(err, "p.confidence.could_not_save");
    } finally {
      busy = false;
    }
  }
</script>

<svelte:head><title>{t("p.confidence.title_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plan_modules.back")}</a></p>
<h1>{t("p.confidence.title")}</h1>
<p class="small muted">{t("p.confidence.intro")}</p>
{#if loadError}<p class="banner error" role="alert">{loadError}</p>{/if}

{#if view}
  {@const shown = view.shown}
  <section aria-labelledby="shown-h">
    <h2 id="shown-h">{t("p.confidence.shown_heading")}</h2>
    <p class="reading">
      <span aria-hidden="true">{mark(shown.value).symbol}</span>
      <strong>{word(shown.value)}</strong>
    </p>
    {#if shown.value !== "not_assessed" && shown.assessed_on}
      <p class="small">
        {shown.kind === "independent"
          ? t("p.confidence.shown_independent", { when: formatDate(shown.assessed_on) })
          : t("p.confidence.shown_self", { when: formatDate(shown.assessed_on) })}
        {#if shown.stale} · {t("p.confidence.stale")}{/if}
      </p>
    {:else}
      <p class="small">{t("p.confidence.none_yet")}</p>
    {/if}
  </section>

  <section aria-labelledby="assess-h">
    <h2 id="assess-h">{t("p.confidence.assess_heading")}</h2>
    <p class="small muted">{t("p.confidence.assess_says")}</p>
    {#if error}<p class="banner error" role="alert">{error}</p>{/if}
    {#if problems.length > 0}
      <ul class="banner error" role="alert">{#each problems as p (p)}<li>{p}</li>{/each}</ul>
    {/if}
    <p class="small" role="status">{notice}</p>
    <p class="small"><a href={localeHref("/exceptions")}>{t("p.exceptions.see_register")}</a></p>
    <form class="stack" novalidate onsubmit={save}>
      <label>
        {t("p.confidence.f_value")}
        <select bind:value>
          <option value="">{t("p.confidence.choose_value")}</option>
          {#each CONFIDENCE_SCALE as v (v)}<option value={v}>{mark(v).symbol} {word(v)}</option>{/each}
        </select>
      </label>
      {#if opensException(value)}<p class="small" role="note">{t("p.confidence.opens_exception")}</p>{/if}
      <fieldset>
        <legend>{t("p.confidence.f_kind")}</legend>
        <label class="check"><input type="radio" name="kind" value="self" bind:group={kind} /> {t("p.confidence.kind_self")}</label>
        <label class="check"><input type="radio" name="kind" value="independent" bind:group={kind} /> {t("p.confidence.kind_independent")}</label>
      </fieldset>
      <label>
        {t("p.confidence.f_rationale", { min: RATIONALE_MIN })}
        <textarea bind:value={rationale} rows="3"></textarea>
      </label>
      <label>
        {t("p.confidence.f_assessed_on")}
        <input type="date" bind:value={assessedOn} />
      </label>
      <div class="row">
        <button class="button primary" type="submit" disabled={busy}>{t("p.confidence.save")}</button>
      </div>
    </form>
  </section>

  <section aria-labelledby="history-h">
    <h2 id="history-h">{t("p.confidence.history_heading")}</h2>
    {#if history.length === 0}
      <p class="muted">{t("p.confidence.no_history")}</p>
    {:else}
      <ol class="history" reversed>
        {#each history as a (a.pid)}
          <li>
            <p>
              <span aria-hidden="true">{mark(a.value).symbol}</span>
              <strong>{word(a.value)}</strong>
              · {a.kind === "independent" ? t("p.confidence.kind_independent") : t("p.confidence.kind_self")}
              · {formatDate(a.assessed_on)}
            </p>
            <p class="rationale">{a.rationale}</p>
          </li>
        {/each}
      </ol>
    {/if}
  </section>
{/if}

<style>
  section {
    margin-block-start: 1.5rem;
  }
  .reading {
    font-size: 1.25rem;
    display: flex;
    gap: 0.5rem;
    align-items: baseline;
  }
  .check {
    display: flex;
    align-items: center;
    gap: 0.35rem;
  }
  fieldset {
    display: grid;
    gap: 0.35rem;
  }
  .history {
    display: grid;
    gap: 0.75rem;
    padding-inline-start: 1.5rem;
  }
  .history p {
    margin-block: 0 0.25rem;
  }
  .rationale {
    white-space: pre-wrap;
    overflow-wrap: anywhere;
  }
</style>
