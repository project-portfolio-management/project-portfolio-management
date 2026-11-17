<!--
  The canvases of one plan, or of every scope (LT-1, NC-1 slice 9), and the form that starts a new one from a built-in
  template. Used by `/canvases` (no `planPid`: every canvas the caller may read, and a new one is free-standing) and
  `/plans/[pid]/canvases` (that plan's, and a new one is on the plan). Every problem the service would refuse is said
  before anything is sent; a refusal is the catalogue's sentence, never the service's text.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { errorText } from "#lib/error-text.js";
  import { localeHref, t } from "#lib/i18n.svelte.js";
  import { PpmClient, type Canvas, type CanvasTemplate } from "#lib/api/ppm.js";
  import { CANVAS_TEXT_MAX, CANVAS_TITLE_MAX, TEMPLATE_LABELS, canvasProblems, labelFor } from "#lib/canvas-view.js";

  let { planPid = null }: { planPid?: string | null } = $props();

  const ppm = PpmClient.withFetch();

  let canvases = $state<Canvas[] | null>(null);
  let templates = $state<CanvasTemplate[]>([]);
  let loadError = $state<string | null>(null);
  let templateError = $state<string | null>(null);
  let filter = $state("");

  let template = $state("");
  let title = $state("");
  let description = $state("");
  let problems = $state<string[]>([]);
  let createError = $state<string | null>(null);
  let saving = $state(false);

  const templateName = (key: string): string =>
    labelFor(TEMPLATE_LABELS, key, templates.find((x) => x.key === key)?.title, t);
  const shown = $derived((canvases ?? []).filter((c) => filter === "" || c.template === filter));

  async function load() {
    loadError = null;
    try {
      canvases = await ppm.listCanvases(planPid ?? undefined);
    } catch (err) {
      loadError = errorText(err, "p.canvas.could_not_load_list");
    }
  }
  async function loadTemplates() {
    templateError = null;
    try {
      templates = await ppm.canvasTemplates();
      if (!template && templates[0]) template = templates[0].key;
    } catch (err) {
      templateError = errorText(err, "p.canvas.could_not_load_templates");
    }
  }
  onMount(() => {
    void load();
    void loadTemplates();
  });

  async function create(e: SubmitEvent) {
    e.preventDefault();
    createError = null;
    problems = canvasProblems(title, description).map((k) => t(k));
    if (problems.length > 0 || !template) return;
    saving = true;
    try {
      const d = description.trim();
      const made = await ppm.createCanvas({
        template,
        title: title.trim(),
        ...(d ? { description: d } : {}),
        ...(planPid ? { plan_pid: planPid } : {}),
      });
      await goto(localeHref(`/canvases/${made.pid}`));
    } catch (err) {
      createError = errorText(err, "p.canvas.could_not_create");
    } finally {
      saving = false;
    }
  }
</script>

{#if loadError}<p class="banner" role="alert">{loadError}</p>{/if}

<section aria-labelledby="canvas-list-h">
  <h2 id="canvas-list-h">{t("p.canvas.list_heading")}</h2>
  {#if canvases && canvases.length > 0}
    <label class="filter">
      {t("p.canvas.filter")}
      <select bind:value={filter}>
        <option value="">{t("p.canvas.filter_all")}</option>
        {#each templates as tpl (tpl.key)}<option value={tpl.key}>{templateName(tpl.key)}</option>{/each}
      </select>
    </label>
  {/if}
  {#if canvases && shown.length === 0}
    <p class="muted">{canvases.length === 0 ? t("p.canvas.none_yet") : t("p.canvas.none_of_kind")}</p>
  {:else if canvases}
    <ul class="canvases">
      {#each shown as c (c.pid)}
        <li>
          <a href={localeHref(`/canvases/${c.pid}`)}>{c.title}</a>
          <span class="small muted">{templateName(c.template)}{c.plan_pid ? "" : ` · ${t("p.canvas.free_standing")}`}</span>
        </li>
      {/each}
    </ul>
  {/if}
</section>

<section aria-labelledby="canvas-new-h">
  <h2 id="canvas-new-h">{t("p.canvas.new_heading")}</h2>
  <p class="small muted">{planPid ? t("p.canvas.new_on_plan") : t("p.canvas.new_free_standing")}</p>
  {#if templateError}<p class="banner" role="alert">{templateError}</p>{/if}
  {#if createError}<p class="banner" role="alert">{createError}</p>{/if}
  {#if problems.length > 0}
    <ul class="banner" role="alert">{#each problems as p (p)}<li>{p}</li>{/each}</ul>
  {/if}
  <form class="stack" onsubmit={create}>
    <label>
      {t("p.canvas.f_template")}
      <select bind:value={template} required disabled={templates.length === 0}>
        {#each templates as tpl (tpl.key)}<option value={tpl.key}>{templateName(tpl.key)}</option>{/each}
      </select>
    </label>
    <label>
      {t("p.canvas.f_title")}
      <input type="text" bind:value={title} maxlength={CANVAS_TITLE_MAX} />
    </label>
    <label>
      {t("p.canvas.f_description")}
      <textarea bind:value={description} rows="3" maxlength={CANVAS_TEXT_MAX}></textarea>
    </label>
    <div class="row">
      <button class="button primary" type="submit" disabled={saving || templates.length === 0}>
        {t("p.canvas.create")}
      </button>
    </div>
  </form>
</section>

<style>
  section {
    margin-block-start: 1.5rem;
  }
  .canvases {
    display: grid;
    gap: 0.5rem;
    padding-inline-start: 1.25rem;
  }
  .canvases li {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    align-items: baseline;
  }
  .filter {
    display: inline-flex;
    gap: 0.5rem;
    align-items: center;
    margin-block-end: 0.75rem;
  }
</style>
