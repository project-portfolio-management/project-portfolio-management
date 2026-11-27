<!--
  Roadmap categories (`/roadmap-categories`, RE-7; NC-1 slice 12 part l): the categories a product roadmap is filed
  under, defined once so renaming one renames it wherever it is used (`GET /api/roadmap-categories`, active first). A
  category is added (`POST`) or changed (`PUT`: its name and the whole set of its translations, one `code: text` to a
  line), every problem said first and a name already taken refused before sending (and a 409 meanwhile in the same
  sentence); a category is retired after a question (no longer chosen; a roadmap that names it keeps it) or restored.
  Rules in `src/lib/roadmap-category-view.ts`.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import { errorText } from "#lib/error-text.js";
  import { i18n, localeHref, t, tDynamic } from "#lib/i18n.svelte.js";
  import { ApiError } from "#lib/api/client.js";
  import { PpmClient } from "#lib/api/ppm.js";
  import {
    NAME_MAX,
    categoryChange,
    nameTaken,
    shownName,
    translationsText,
    type RoadmapCategory,
  } from "#lib/roadmap-category-view.js";

  const ppm = PpmClient.withFetch();
  let list = $state<RoadmapCategory[] | null>(null);
  let loadError = $state<string | null>(null);
  let error = $state<string | null>(null);
  let problems = $state<string[]>([]);
  let notice = $state<string | null>(null);
  let busy = $state(false);
  let name = $state("");
  let translations = $state("");
  let editing = $state<string | null>(null);
  let editName = $state("");
  let editTranslations = $state("");

  async function load() {
    try {
      list = await ppm.roadmapCategories();
      loadError = null;
    } catch (err) {
      loadError = errorText(err, "p.roadmap_categories.could_not_load");
    }
  }
  onMount(load);

  const refused = (err: unknown) =>
    err instanceof ApiError && err.status === 409
      ? t("p.roadmap_categories.taken")
      : err instanceof ApiError && err.status === 403
        ? t("p.roadmap_categories.forbidden")
        : errorText(err, "p.roadmap_categories.could_not_save");

  async function send(work: () => Promise<unknown>, done: string) {
    busy = true;
    error = notice = null;
    try {
      await work();
      notice = done;
      await load();
      return true;
    } catch (err) {
      error = refused(err);
      return false;
    } finally {
      busy = false;
    }
  }

  async function add(e: SubmitEvent) {
    e.preventDefault();
    const r = categoryChange(name, translations, null);
    if (!r) return;
    if ("problems" in r) return void (problems = r.problems);
    problems = [];
    if (nameTaken(name, list ?? [])) return void (error = t("p.roadmap_categories.taken"));
    if (await send(() => ppm.addRoadmapCategory({ name: r.body.name!, translations: r.body.translations }), t("p.roadmap_categories.added", { name: r.body.name! })))
      name = translations = "";
  }

  async function save(e: SubmitEvent, c: RoadmapCategory) {
    e.preventDefault();
    const r = categoryChange(editName, editTranslations, c);
    if (r === null) return void (editing = null);
    if ("problems" in r) return void (problems = r.problems);
    problems = [];
    if (r.body.name && nameTaken(r.body.name, list ?? [], c.pid)) return void (error = t("p.roadmap_categories.taken"));
    if (await send(() => ppm.changeRoadmapCategory(c.pid, r.body), t("p.roadmap_categories.saved", { name: r.body.name ?? c.name })))
      editing = null;
  }

  async function setActive(c: RoadmapCategory, active: boolean) {
    if (!active && !(await askConfirm(t("p.roadmap_categories.retire_ask", { name: c.name })))) return;
    await send(
      () => ppm.changeRoadmapCategory(c.pid, { active }),
      active ? t("p.roadmap_categories.restored", { name: c.name }) : t("p.roadmap_categories.retired", { name: c.name }),
    );
  }
</script>

<svelte:head><title>{t("p.roadmap_categories.title_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref("/roadmaps")}>{t("p.roadmap_categories.back")}</a></p>
<h1>{t("p.roadmap_categories.title")}</h1>
<p class="lede">{t("p.roadmap_categories.intro")}</p>
{#if loadError}<p class="banner" role="alert">{loadError}</p>{/if}
{#if error}<p class="banner error" role="alert" data-testid="rc-error">{error}</p>{/if}
{#if notice}<p class="banner" role="status" data-testid="rc-notice">{notice}</p>{/if}
{#if problems.length > 0}
  <ul class="banner error" role="alert" data-testid="rc-problems">{#each problems as p (p)}<li>{tDynamic(p)}</li>{/each}</ul>
{/if}

{#if !list && !loadError}
  <p class="muted">{t("p.roadmap_categories.loading")}</p>
{:else if list}
  {#if list.length === 0}<p data-testid="rc-none">{t("p.roadmap_categories.none")}</p>{/if}
  <ul class="items" data-testid="categories">
    {#each list as c (c.pid)}
      <li class="surface stack">
        <p>
          <strong id={`cat-${c.pid}`}>{shownName(c, i18n.locale)}</strong>
          {#if shownName(c, i18n.locale) !== c.name}<span class="small muted">({c.name})</span>{/if}
          {#if !c.active}<span class="small"> · {t("p.roadmap_categories.is_retired")}</span>{/if}
        </p>
        {#if Object.keys(c.translations ?? {}).length > 0}
          <p class="small muted">{t("p.roadmap_categories.translations_are", { list: translationsText(c.translations).split("\n").join("; ") })}</p>
        {/if}
        {#if editing === c.pid}
          <form class="stack" onsubmit={(e) => save(e, c)} novalidate aria-label={t("p.roadmap_categories.change_h", { name: c.name })}>
            <label>{t("p.roadmap_categories.f_name")} <input bind:value={editName} maxlength={NAME_MAX} /></label>
            <label>{t("p.roadmap_categories.f_translations")} <textarea rows="3" bind:value={editTranslations}></textarea></label>
            <div class="row">
              <button class="button" type="submit" disabled={busy}>{t("p.roadmap_categories.save")}</button>
              <button class="button" type="button" onclick={() => (editing = null)}>{t("p.roadmap_categories.cancel")}</button>
            </div>
          </form>
        {:else}
          <div class="row">
            <button class="button small" type="button" onclick={() => ((editing = c.pid), (editName = c.name), (editTranslations = translationsText(c.translations ?? {})), (problems = []))} aria-describedby={`cat-${c.pid}`}>{t("p.roadmap_categories.change")}</button>
            {#if c.active}
              <button class="button small danger" type="button" disabled={busy} onclick={() => setActive(c, false)} aria-describedby={`cat-${c.pid}`}>{t("p.roadmap_categories.retire")}</button>
            {:else}
              <button class="button small" type="button" disabled={busy} onclick={() => setActive(c, true)} aria-describedby={`cat-${c.pid}`}>{t("p.roadmap_categories.restore")}</button>
            {/if}
          </div>
        {/if}
      </li>
    {/each}
  </ul>

  <form class="surface stack" onsubmit={add} aria-labelledby="add-h" novalidate>
    <h2 id="add-h">{t("p.roadmap_categories.add_h")}</h2>
    <label>{t("p.roadmap_categories.f_name")} <input bind:value={name} maxlength={NAME_MAX} /></label>
    <label>{t("p.roadmap_categories.f_translations")} <textarea rows="3" bind:value={translations}></textarea></label>
    <button class="button primary" type="submit" disabled={busy}>{t("p.roadmap_categories.add")}</button>
  </form>
{/if}

<style>
  .items {
    list-style: none;
    padding: 0;
    display: grid;
    gap: 0.75rem;
  }
  h2 {
    font-size: 1.05rem;
  }
  label {
    display: block;
  }
  .row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }
</style>
