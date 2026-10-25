<!--
  A plan's tolerances (`/plans/[pid]/tolerances`): for time, cost, risk and resources, the limit in force (this plan's
  own or inherited from a plan above), what was measured and how, and the colour. Each limit can be set here or
  removed (then the plan inherits again). A colour is always a word and a shape too.
-->
<script lang="ts">
  import { errorReason, errorText } from "#lib/error-text.js";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { PpmClient, type ToleranceDimension, type TolerancePicture } from "#lib/api/ppm.js";
  import {
    amountText,
    dimensionLabel,
    fromApiLimit,
    limitFieldLabel,
    sourceText,
    STATUS_SYMBOL,
    STATUS_TEXT,
    toApiLimit,
    usageText,
  } from "#lib/tolerance-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");
  let picture = $state<TolerancePicture | null>(null);
  let error = $state<string | null>(null);
  let notice = $state("");
  /** What is typed in each form, by dimension. */
  let limits = $state<Record<string, string>>({});
  let ambers = $state<Record<string, string>>({});
  let notes = $state<Record<string, string>>({});

  function fill(p: TolerancePicture) {
    for (const d of p.dimensions) {
      limits[d.dimension] = d.limit === null ? "" : String(fromApiLimit(d.dimension, d.limit));
      ambers[d.dimension] = d.amber_from_percent ? String(d.amber_from_percent) : "";
      notes[d.dimension] = d.note ?? "";
    }
  }

  async function load() {
    try {
      picture = await ppm.planTolerances(pid);
      fill(picture);
      error = null;
    } catch (err) {
      error = errorText(err, "p.tolerances.could_not_load");
    }
  }
  onMount(load);

  async function save(e: SubmitEvent, d: ToleranceDimension) {
    e.preventDefault();
    // A number input binds a number (or nothing), so go through a string before judging it.
    const raw = String(limits[d.dimension] ?? "").trim();
    const typed = Number(raw);
    if (raw === "" || !Number.isFinite(typed)) {
      notice = t("p.tolerances.limit_problem");
      return;
    }
    try {
      const amber = Number(String(ambers[d.dimension] ?? ""));
      picture = await ppm.setTolerance(pid, d.dimension, {
        limit: toApiLimit(d.dimension, typed),
        amber_from_percent: Number.isFinite(amber) && amber > 0 ? Math.round(amber) : undefined,
        note: String(notes[d.dimension] ?? "").trim() || null,
      });
      fill(picture);
      notice = t("p.tolerances.saved", { dimension: dimensionLabel(d.dimension) });
    } catch (err) {
      notice = t("p.tolerances.could_not", { reason: errorReason(err) });
    }
  }

  async function escalateAll() {
    try {
      const r = await ppm.escalateTolerances(pid);
      picture = r.tolerances;
      fill(picture);
      notice = t("p.tolerances.escalated", { n: r.raised.length });
    } catch (err) {
      notice = t("p.tolerances.could_not", { reason: errorReason(err) });
    }
  }

  async function remove(d: ToleranceDimension) {
    try {
      picture = await ppm.removeTolerance(pid, d.dimension);
      fill(picture);
      notice = t("p.tolerances.removed", { dimension: dimensionLabel(d.dimension) });
    } catch (err) {
      notice = t("p.tolerances.could_not", { reason: errorReason(err) });
    }
  }
</script>

<svelte:head><title>{t("p.tolerances.tolerances_ppm")}</title></svelte:head>

<p class="small">
  <a href={localeHref(`/plans/${pid}`)}>{t("p.tolerances.back")}</a> ·
  <a href={localeHref("/tolerances")}>{t("p.tolerances.board_link")}</a>
</p>
<h1>{t("p.tolerances.tolerances")}{picture ? `: ${picture.plan_name}` : ""}</h1>
<p class="small muted">{t("p.tolerances.intro")}</p>
{#if error}<p class="banner" role="alert">{error}</p>{/if}
<p class="small" role="status">{notice}</p>

{#if picture}
  <p class={`overall rag rag-${picture.overall}`}>
    <strong>{t("p.tolerances.overall")}:</strong>
    <span aria-hidden="true">{STATUS_SYMBOL[picture.overall]}</span> {STATUS_TEXT[picture.overall]}
  </p>
  {#if picture.escalation_needed}
    <p class="needed" role="note">
      {t("p.tolerances.escalate_all_intro")}
      <button type="button" onclick={escalateAll}>{t("p.tolerances.escalate_all")}</button>
    </p>
  {/if}
  {#each picture.dimensions as d (d.dimension)}
    <section class={`surface rag rag-${d.status}`} aria-labelledby={`d-${d.dimension}`}>
      <h2 id={`d-${d.dimension}`}>
        {dimensionLabel(d.dimension)}:
        <span aria-hidden="true">{STATUS_SYMBOL[d.status]}</span> {STATUS_TEXT[d.status]}
      </h2>
      <dl>
        <dt>{t("p.tolerances.limit")}</dt>
        <dd>
          {d.limit === null ? "—" : amountText(d.dimension, d.limit)}
          {#if sourceText(d)}<span class="small muted"> ({sourceText(d)})</span>{/if}
        </dd>
        {#if d.status !== "not_set"}
          <dt>{t("p.tolerances.measured")}</dt>
          <dd>{d.measured === null || d.measured === undefined ? "—" : amountText(d.dimension, d.measured)} <span class="small muted">— {usageText(d)}</span></dd>
        {:else}
          <dd class="small muted">{usageText(d)}</dd>
        {/if}
        <dt>{t("p.tolerances.how")}</dt>
        <dd class="small">{d.basis}</dd>
        {#if d.note}<dt>{t("p.tolerances.note")}</dt><dd>{d.note}</dd>{/if}
        {#if d.open_escalations}<dt>{t("p.escalations.escalations")}</dt><dd><a href={localeHref(`/plans/${pid}/escalations`)}>{d.open_escalations === 1 ? t("p.tolerances.escalations_one", { n: 1 }) : t("p.tolerances.escalations_other", { n: d.open_escalations })}</a></dd>{/if}
      </dl>
      {#if d.escalation_needed}
        <p class="needed" role="note"><strong>{t("p.tolerances.needs_escalation")}</strong> <a href={localeHref(`/plans/${pid}/escalations?reason=tolerance&dimension=${d.dimension}`)}>{t("p.tolerances.raise_one")}</a></p>
      {/if}
      <form onsubmit={(e) => save(e, d)} aria-label={dimensionLabel(d.dimension)}>
        <label>{limitFieldLabel(d.dimension)} <input type="number" step="any" min="0" bind:value={limits[d.dimension]} /></label>
        <label>{t("p.tolerances.amber_from")} <input type="number" min="1" max="99" bind:value={ambers[d.dimension]} /></label>
        <label>{t("p.tolerances.note")} <input bind:value={notes[d.dimension]} maxlength="1000" /></label>
        <span class="actions">
          <button type="submit">{t("p.tolerances.save")}</button>
          {#if d.source === "own"}<button type="button" onclick={() => remove(d)}>{t("p.tolerances.remove")}</button>{/if}
        </span>
      </form>
    </section>
  {/each}
{:else if !error}
  <p class="muted">{t("p.tolerances.loading")}</p>
{/if}

<style>
  .rag { border-inline-start: 0.4rem solid transparent; padding-inline-start: 0.75rem; }
  .rag-red { border-inline-start-color: #b00020; }
  .rag-amber { border-inline-start-color: #c77700; }
  .rag-green { border-inline-start-color: #1b7a3a; }
  .rag-unknown { border-inline-start-color: #5a5a5a; border-inline-start-style: dotted; }
  .rag-not_set { border-inline-start-color: #9a9a9a; border-inline-start-style: dashed; }
  section { margin-block: 1rem; }
  form { display: grid; gap: 0.5rem; max-inline-size: 28rem; margin-block-start: 0.75rem; }
  label { display: grid; gap: 0.2rem; }
  .actions { display: flex; flex-wrap: wrap; gap: 0.5rem; }
  dl { display: grid; grid-template-columns: max-content 1fr; gap: 0.25rem 1rem; }
  dt { font-weight: 600; }
  dd { margin: 0; }
</style>
