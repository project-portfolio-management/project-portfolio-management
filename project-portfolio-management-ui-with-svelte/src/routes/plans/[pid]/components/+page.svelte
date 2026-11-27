<!--
  A plan's components (`/plans/[pid]/components`, PL-11; NC-1 slice 12 part n): every component with its open, done and
  total tasks and its lead, and the tasks in none as unassigned (`GET …/components/summary`, so the counts add up to the
  plan's tasks), added, renamed or given a lead (`POST` / `PUT`), and removed after a question when no task belongs to
  it (`DELETE`; the service refuses otherwise, and the page says how many tasks to move first). Every limit is said
  and checked before anything is sent; refusals are the catalogue's sentences. Rules in `src/lib/component-view.ts`.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import { errorText } from "#lib/error-text.js";
  import { localeHref, t, tDynamic } from "#lib/i18n.svelte.js";
  import { formatInteger } from "#lib/locale-format.js";
  import { ApiError } from "#lib/api/client.js";
  import { PpmClient, type Component, type ComponentSummary } from "#lib/api/ppm.js";
  import { NAME_MAX, componentBody, nameTaken, refusalKey } from "#lib/component-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");

  let list = $state<Component[] | null>(null);
  let summary = $state<ComponentSummary | null>(null);
  let loadError = $state<string | null>(null);
  let error = $state<string | null>(null);
  let notice = $state<string | null>(null);
  let busy = $state(false);
  let editing = $state<string | null>(null);
  let editName = $state("");
  let editLead = $state("");
  let name = $state("");
  let lead = $state("");

  async function load() {
    try {
      [list, summary] = await Promise.all([ppm.components(pid), ppm.componentSummary(pid)]);
      loadError = null;
    } catch (err) {
      loadError = errorText(err, "p.components.could_not_load");
    }
  }
  onMount(load);

  const byPid = $derived(new Map((list ?? []).map((c) => [c.pid, c])));
  const statusOf = (err: unknown) => (err instanceof ApiError ? err.status : undefined);
  const refusal = (action: "save" | "remove", err: unknown) => {
    const s = statusOf(err);
    return s !== undefined && s < 500 && refusalKey(action, s) !== `p.components.could_not_${action}`
      ? tDynamic(refusalKey(action, s))
      : errorText(err, action === "save" ? "p.components.could_not_save" : "p.components.could_not_remove");
  };

  async function add(e: SubmitEvent) {
    e.preventDefault();
    notice = null;
    const r = componentBody(name, lead);
    if ("problem" in r) {
      error = tDynamic(r.problem);
      return;
    }
    if (nameTaken(r.body.name, list ?? [])) {
      error = t("p.components.taken");
      return;
    }
    busy = true;
    try {
      await ppm.createComponent(pid, r.body);
      notice = t("p.components.added", { name: r.body.name });
      error = null;
      name = lead = "";
      await load();
    } catch (err) {
      error = refusal("save", err);
    } finally {
      busy = false;
    }
  }

  function startEdit(c: Component) {
    editing = c.pid;
    editName = c.name;
    editLead = c.lead_ref ?? "";
    error = notice = null;
  }
  async function save(e: SubmitEvent, c: Component) {
    e.preventDefault();
    const r = componentBody(editName, editLead);
    if ("problem" in r) {
      error = tDynamic(r.problem);
      return;
    }
    if (nameTaken(r.body.name, list ?? [], c.pid)) {
      error = t("p.components.taken");
      return;
    }
    if (r.body.name === c.name && (r.body.lead_ref ?? null) === (c.lead_ref ?? null)) {
      editing = null; // nothing changed: nothing sent
      return;
    }
    busy = true;
    try {
      await ppm.updateComponent(pid, c.pid, r.body);
      notice = t("p.components.saved", { name: r.body.name });
      error = null;
      editing = null;
      await load();
    } catch (err) {
      error = refusal("save", err);
    } finally {
      busy = false;
    }
  }
  async function remove(c: Component) {
    if (!(await askConfirm(t("p.components.remove_ask", { name: c.name })))) return;
    busy = true;
    try {
      await ppm.deleteComponent(pid, c.pid);
      notice = t("p.components.removed", { name: c.name });
      error = null;
      await load();
    } catch (err) {
      error = refusal("remove", err);
      await load();
    } finally {
      busy = false;
    }
  }
</script>

<svelte:head><title>{t("p.components.title_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plan_modules.back")}</a></p>
<h1>{t("p.components.title")}</h1>
<p class="lede">{t("p.components.intro")}</p>
{#if loadError}<p class="banner" role="alert">{loadError}</p>{/if}
{#if error}<p class="banner error" role="alert">{error}</p>{/if}
{#if notice}<p class="banner" role="status" data-testid="components-notice">{notice}</p>{/if}

{#if !summary && !loadError}
  <p class="muted">{t("p.components.loading")}</p>
{:else if summary}
  {#if (list ?? []).length === 0}<p data-testid="components-none">{t("p.components.none")}</p>{/if}
  <ul class="stack items" data-testid="components">
    {#each summary.buckets as b (b.component_pid ?? "unassigned")}
      {@const c = b.component_pid ? byPid.get(b.component_pid) : undefined}
      <li class="surface">
        <h2 id={`comp-${b.component_pid ?? "none"}`}>{c ? c.name : t("p.components.unassigned")}</h2>
        <p class="small">{t("p.components.counts", { open: formatInteger(b.open), done: formatInteger(b.done), total: formatInteger(b.total) })}</p>
        {#if c}
          <p class="small muted">{c.lead_ref ? t("p.components.lead", { lead: c.lead_ref }) : t("p.components.no_lead")}</p>
          {#if editing === c.pid}
            <form class="stack" onsubmit={(e) => save(e, c)}>
              <label>{t("p.components.name")} <input bind:value={editName} maxlength={NAME_MAX} /></label>
              <label>{t("p.components.lead_field")} <input bind:value={editLead} /></label>
              <div class="row">
                <button class="button small" type="submit" disabled={busy}>{t("p.components.save")}</button>
                <button class="button small" type="button" onclick={() => (editing = null)}>{t("p.components.cancel")}</button>
              </div>
            </form>
          {:else}
            <div class="row">
              <button class="button small" type="button" onclick={() => startEdit(c)} aria-describedby={`comp-${c.pid}`}>{t("p.components.edit")}</button>
              {#if b.total === 0}
                <button class="button small danger" type="button" disabled={busy} onclick={() => remove(c)} aria-describedby={`comp-${c.pid}`}>{t("p.components.remove")}</button>
              {:else}
                <span class="small muted">{t("p.components.in_use", { n: formatInteger(b.total) })}</span>
              {/if}
            </div>
          {/if}
        {/if}
      </li>
    {/each}
  </ul>

  <form class="surface stack" onsubmit={add} aria-labelledby="add-h">
    <h2 id="add-h">{t("p.components.add_h")}</h2>
    <label>{t("p.components.name")} <input bind:value={name} maxlength={NAME_MAX} /></label>
    <label>{t("p.components.lead_field")} <input bind:value={lead} /></label>
    <button class="button primary" type="submit" disabled={busy}>{t("p.components.add")}</button>
  </form>
{/if}

<style>
  .items {
    list-style: none;
    padding: 0;
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
