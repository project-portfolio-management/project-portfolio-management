<!--
  A plan's goals (`/plans/[pid]/goals`, FR-12; NC-1 slice 12 part n): every goal in its place with its description,
  target date and status (`GET …/goals`), added (`POST`), changed (`PUT …/goals/{index}`) or removed after a question
  (`DELETE`). A goal is addressed by its position, which moves when one before it is removed, so every change sends
  the list's `ETag` from the last read as `If-Match` (CF-2): a list someone else changed meanwhile is refused with
  412, said in the page's words, and read again, so the wrong goal is never edited. Every limit is checked before
  anything is sent. Rules in `src/lib/goal-view.ts`.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import { errorText } from "#lib/error-text.js";
  import { localeHref, t, tDynamic } from "#lib/i18n.svelte.js";
  import { formatDate } from "#lib/locale-format.js";
  import { ApiError } from "#lib/api/client.js";
  import { PpmClient, type GoalList, type IndexedGoal } from "#lib/api/ppm.js";
  import {
    EMPTY_FORM,
    GOALS_MAX,
    STATUSES,
    TITLE_MAX,
    formOf,
    goalBody,
    refusalKey,
    sameGoal,
    statusLabel,
    type GoalForm,
  } from "#lib/goal-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");

  let list = $state<GoalList | null>(null);
  let loadError = $state<string | null>(null);
  let error = $state<string | null>(null);
  let notice = $state<string | null>(null);
  let busy = $state(false);
  let editing = $state<number | null>(null);
  let edit = $state<GoalForm>({ ...EMPTY_FORM });
  let draft = $state<GoalForm>({ ...EMPTY_FORM });

  async function load() {
    try {
      list = await ppm.goals(pid);
      loadError = null;
    } catch (err) {
      loadError = errorText(err, "p.goals.could_not_load");
    }
  }
  onMount(load);

  const goals = $derived(list?.goals ?? []);
  const full = $derived(goals.length >= GOALS_MAX);

  /** A refusal in the page's words; a changed list or a missing goal is read again after it is said. */
  async function refused(action: "save" | "remove", err: unknown) {
    const s = err instanceof ApiError ? err.status : undefined;
    if (s === 412 || s === 404) {
      editing = null;
      await load();
      error = tDynamic(refusalKey(action, s));
    } else {
      error = errorText(err, action === "save" ? "p.goals.could_not_save" : "p.goals.could_not_remove");
    }
  }

  async function add(e: SubmitEvent) {
    e.preventDefault();
    notice = null;
    if (full) {
      error = t("p.goals.full");
      return;
    }
    const r = goalBody(draft);
    if ("problem" in r) {
      error = tDynamic(r.problem);
      return;
    }
    busy = true;
    try {
      list = await ppm.addGoal(pid, r.body);
      notice = t("p.goals.added", { title: r.body.title });
      error = null;
      draft = { ...EMPTY_FORM };
    } catch (err) {
      await refused("save", err);
    } finally {
      busy = false;
    }
  }

  function startEdit(g: IndexedGoal) {
    editing = g.index;
    edit = formOf(g);
    error = notice = null;
  }
  async function save(e: SubmitEvent, g: IndexedGoal) {
    e.preventDefault();
    const r = goalBody(edit);
    if ("problem" in r) {
      error = tDynamic(r.problem);
      return;
    }
    if (sameGoal(r.body, g)) {
      editing = null; // nothing changed: nothing sent
      return;
    }
    busy = true;
    try {
      list = await ppm.replaceGoal(pid, g.index, r.body, list?.etag ?? null);
      notice = t("p.goals.saved", { title: r.body.title });
      error = null;
      editing = null;
    } catch (err) {
      await refused("save", err);
    } finally {
      busy = false;
    }
  }
  async function remove(g: IndexedGoal) {
    if (!(await askConfirm(t("p.goals.remove_ask", { title: g.title })))) return;
    busy = true;
    notice = null;
    try {
      list = await ppm.removeGoal(pid, g.index, list?.etag ?? null);
      notice = t("p.goals.removed", { title: g.title });
      error = null;
      editing = null;
    } catch (err) {
      await refused("remove", err);
    } finally {
      busy = false;
    }
  }
</script>

{#snippet fields(f: GoalForm, idp: string)}
  <label>{t("p.goals.title_field")} <input bind:value={f.title} maxlength={TITLE_MAX} /></label>
  <label>{t("p.goals.description_field")} <textarea bind:value={f.description} rows="2"></textarea></label>
  <label>{t("p.goals.target_field")} <input type="date" bind:value={f.target_date} /></label>
  <label
    >{t("p.goals.status_field")}
    <select bind:value={f.status}>
      <option value="">{t("p.goals.status_none")}</option>
      {#each STATUSES as s (s)}<option value={s}>{tDynamic(`p.goals.st.${s}`)}</option>{/each}
      <option value="Custom">{t("p.goals.status_custom")}</option>
    </select></label
  >
  {#if f.status === "Custom"}
    <label for={`${idp}-custom`}>{t("p.goals.custom_field")}</label>
    <input id={`${idp}-custom`} bind:value={f.custom} />
  {/if}
{/snippet}

<svelte:head><title>{t("p.goals.title_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plan_modules.back")}</a></p>
<h1>{t("p.goals.title")}</h1>
<p class="lede">{t("p.goals.intro")}</p>
{#if loadError}<p class="banner" role="alert">{loadError}</p>{/if}
{#if error}<p class="banner error" role="alert" data-testid="goals-error">{error}</p>{/if}
{#if notice}<p class="banner" role="status" data-testid="goals-notice">{notice}</p>{/if}

{#if !list && !loadError}
  <p class="muted">{t("p.goals.loading")}</p>
{:else if list}
  {#if goals.length === 0}<p data-testid="goals-none">{t("p.goals.none")}</p>{/if}
  <ol class="stack items" data-testid="goals">
    {#each goals as g (g.index)}
      {@const st = statusLabel(g.status)}
      <li class="surface">
        <h2 id={`goal-${g.index}`}>{g.title}</h2>
        {#if g.description}<p>{g.description}</p>{/if}
        {#if g.target_date}<p class="small">{t("p.goals.target", { date: formatDate(g.target_date) })}</p>{/if}
        {#if st}
          <p class="small muted">
            {t("p.goals.status", { status: "key" in st ? tDynamic(st.key) : st.custom })}
          </p>
        {/if}
        {#if editing === g.index}
          <form class="stack" onsubmit={(e) => save(e, g)}>
            {@render fields(edit, `edit-${g.index}`)}
            <div class="row">
              <button class="button small" type="submit" disabled={busy}>{t("p.goals.save")}</button>
              <button class="button small" type="button" onclick={() => (editing = null)}>{t("p.goals.cancel")}</button>
            </div>
          </form>
        {:else}
          <div class="row">
            <button class="button small" type="button" onclick={() => startEdit(g)} aria-describedby={`goal-${g.index}`}
              >{t("p.goals.edit")}</button
            >
            <button
              class="button small danger"
              type="button"
              disabled={busy}
              onclick={() => remove(g)}
              aria-describedby={`goal-${g.index}`}>{t("p.goals.remove")}</button
            >
          </div>
        {/if}
      </li>
    {/each}
  </ol>

  <form class="surface stack" onsubmit={add} aria-labelledby="add-h">
    <h2 id="add-h">{t("p.goals.add_h")}</h2>
    {#if full}
      <p class="small muted">{t("p.goals.full")}</p>
    {:else}
      {@render fields(draft, "add")}
      <button class="button primary" type="submit" disabled={busy}>{t("p.goals.add")}</button>
    {/if}
  </form>
{/if}

<style>
  .items {
    padding-inline-start: 1.5rem;
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
    align-items: center;
  }
</style>
