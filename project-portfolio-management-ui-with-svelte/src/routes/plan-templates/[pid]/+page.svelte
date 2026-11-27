<!--
  One plan template (`/plan-templates/[pid]`, LT-2, NC-1 slice 9): what it holds (its plans as an outline, each with
  its number of tasks and its milestones as days after the start), and starting new plans from it: a name for the top
  plan (the template's own when left blank), a start date when the template carries dates, and an optional plan to put
  it under. A saved template can be removed; plans already made from it stay. A built-in starter cannot be removed.
  Every problem the service would refuse is said before anything is sent; a refusal is the catalogue's sentence.
  NC-12 (NC-1 slice 12 part k): the plans made from it, each with how many of the template's changes it has waiting and a
  link to its Template changes page; a saved template is revised from a plan tree after a question.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import { errorText } from "#lib/error-text.js";
  import { localeHref, t, tn } from "#lib/i18n.svelte.js";
  import { PpmClient, type InstantiatedTemplate, type PlanTemplate, type TemplatePlan, type TemplateRevised } from "#lib/api/ppm.js";
  import { ApiError } from "#lib/api/client.js";
  import { formatInteger } from "#lib/locale-format.js";
  import { reviseRefusalKey } from "#lib/template-sync-view.js";
  import { tDynamic } from "#lib/i18n.svelte.js";
  import { PlanRepository } from "#lib/api/plans.js";
  import type { PlanRef } from "#lib/api/types.js";
  import { PLAN_NAME_MAX, instantiateBody, outline } from "#lib/template-view.js";

  const ppm = PpmClient.withFetch();
  const plansRepo = PlanRepository.withFetch();
  const pid = $derived(page.params.pid ?? "");

  let tpl = $state<PlanTemplate | null>(null);
  let loadError = $state<string | null>(null);
  let plans = $state<PlanRef[]>([]);
  let error = $state<string | null>(null);
  let busy = $state(false);
  let made = $state<InstantiatedTemplate | null>(null);

  let name = $state("");
  let startDate = $state("");
  let parentPid = $state("");

  const rows = $derived(tpl ? outline(tpl.snapshot.nodes) : []);
  const needsStart = $derived(tpl?.snapshot.has_offsets ?? false);
  const starter = $derived(tpl?.created_by === "system:starter");
  const rootName = $derived(rows[0]?.name ?? "");

  async function load() {
    loadError = null;
    try {
      tpl = await ppm.planTemplate(pid);
    } catch (err) {
      loadError = errorText(err, "p.plan_templates.could_not_load");
    }
  }
  async function loadPlans() {
    try {
      plans = await plansRepo.list();
    } catch {
      plans = []; // no plan to put it under: it becomes a top-level plan, which the form still offers
    }
  }
  // The plans made from it, and revising it from a plan tree (NC-12).
  let madeFrom = $state<TemplatePlan[] | null>(null);
  let madeError = $state<string | null>(null);
  async function loadMade() {
    try {
      madeFrom = await ppm.templatePlans(pid);
      madeError = null;
    } catch (err) {
      madeError = errorText(err, "p.template_sync.could_not_load_plans");
    }
  }
  let reviseFrom = $state("");
  let revised = $state<TemplateRevised | null>(null);
  let reviseError = $state<string | null>(null);
  async function revise(e: SubmitEvent) {
    e.preventDefault();
    revised = null;
    reviseError = null;
    const from = plans.find((p) => p.pid === reviseFrom);
    if (!tpl || !from) {
      reviseError = t("p.template_sync.revise_pick");
      return;
    }
    if (!(await askConfirm(t("p.template_sync.revise_ask", { name: from.name, template: tpl.name })))) return;
    busy = true;
    try {
      revised = await ppm.reviseTemplate(pid, from.pid);
      await Promise.all([load(), loadMade()]);
    } catch (err) {
      const status = err instanceof ApiError ? err.status : undefined;
      reviseError =
        status === 422 ? tDynamic(reviseRefusalKey(status)) : errorText(err, "p.template_sync.could_not_revise");
    } finally {
      busy = false;
    }
  }

  onMount(() => {
    void load();
    void loadPlans();
    void loadMade();
  });

  async function start(e: SubmitEvent) {
    e.preventDefault();
    error = null;
    made = null;
    const r = instantiateBody({ name, startDate, parentPid }, needsStart);
    if ("problem" in r) {
      error = t(r.problem);
      return;
    }
    busy = true;
    try {
      made = await ppm.instantiateTemplate(pid, r.body);
    } catch (err) {
      error = errorText(err, "p.plan_templates.could_not_start");
    } finally {
      busy = false;
    }
  }

  async function remove() {
    if (!tpl || !(await askConfirm(t("p.plan_templates.confirm_remove", { name: tpl.name })))) return;
    error = null;
    busy = true;
    try {
      await ppm.deletePlanTemplate(pid);
      await goto(localeHref("/plan-templates"));
    } catch (err) {
      error = errorText(err, "p.plan_templates.could_not_remove");
    } finally {
      busy = false;
    }
  }
</script>

<svelte:head><title>{t("p.plan_templates.one_title_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref("/plan-templates")}>{t("p.plan_templates.back")}</a></p>
{#if !tpl}
  <h1>{t("p.plan_templates.one_title")}</h1>
  {#if loadError}<p class="banner" role="alert">{loadError}</p>{/if}
{:else}
  <h1>{tpl.name}</h1>
  <p class="small muted">
    {starter ? t("p.plan_templates.is_starter") : t("p.plan_templates.is_saved", { version: tpl.version })}
  </p>
  {#if tpl.description}<p class="description">{tpl.description}</p>{/if}

  <section aria-labelledby="holds-h">
    <h2 id="holds-h">{t("p.plan_templates.holds_heading")}</h2>
    <p class="small muted">{t("p.plan_templates.holds_says")}</p>
    <ul class="outline">
      {#each rows as row (row.key)}
        <li class="depth" data-depth={Math.min(row.depth, 6)}>
          <strong>{row.name}</strong>
          <span class="small muted">{tn("p.plan_templates.tasks_count", row.tasks, { count: row.tasks })}</span>
          {#if row.milestones.length > 0}
            <ul class="small">
              {#each row.milestones as m, i (i)}
                <li>{t("p.plan_templates.milestone_at", { name: m.name, days: m.offset_days })}</li>
              {/each}
            </ul>
          {/if}
        </li>
      {/each}
    </ul>
  </section>

  <section aria-labelledby="start-h">
    <h2 id="start-h">{t("p.plan_templates.start_heading")}</h2>
    <p class="small muted">{t("p.plan_templates.start_says")}</p>
    {#if error}<p class="banner" role="alert">{error}</p>{/if}
    {#if made}
      <div class="banner success" role="status">
        <p>
          {t("p.plan_templates.started", {
            plans: made.plans.length,
            tasks: made.tasks_created,
            milestones: made.milestones_created,
          })}
        </p>
        <p><a href={localeHref(`/plans/${made.root_pid}`)}>{t("p.plan_templates.open_new")}</a></p>
      </div>
    {/if}
    <form class="stack" onsubmit={start}>
      <label>
        {t("p.plan_templates.f_plan_name")}
        <input type="text" bind:value={name} maxlength={PLAN_NAME_MAX} placeholder={rootName} />
      </label>
      {#if needsStart}
        <label>
          {t("p.plan_templates.f_start")}
          <input type="date" bind:value={startDate} />
        </label>
      {:else}
        <p class="small muted">{t("p.plan_templates.no_dates")}</p>
      {/if}
      <label>
        {t("p.plan_templates.f_parent")}
        <select bind:value={parentPid}>
          <option value="">{t("p.plan_templates.no_parent")}</option>
          {#each plans as p (p.pid)}<option value={p.pid}>{p.name}</option>{/each}
        </select>
      </label>
      <div class="row">
        <button class="button primary" type="submit" disabled={busy}>{t("p.plan_templates.start")}</button>
      </div>
    </form>
  </section>

  <section aria-labelledby="made-h">
    <h2 id="made-h">{t("p.template_sync.plans_h")}</h2>
    {#if madeError}<p class="banner" role="alert">{madeError}</p>{/if}
    {#if madeFrom && madeFrom.length === 0}
      <p class="small muted" data-testid="made-none">{t("p.template_sync.plans_none")}</p>
    {:else if madeFrom}
      <ul class="stack" data-testid="made-from">
        {#each madeFrom as m (m.plan_pid)}
          <li>
            <a href={localeHref(`/plans/${m.plan_pid}`)}>{m.name ?? m.plan_pid}</a>
            <span class="small muted">· {t("p.template_sync.plans_row", { synced: formatInteger(m.synced_version), version: formatInteger(m.template_version) })} ·
              {m.changes_waiting > 0 ? t("p.template_sync.waiting", { n: formatInteger(m.changes_waiting) }) : t("p.template_sync.plan_up_to_date")}</span>
            {#if m.changes_waiting > 0}<a class="small" href={localeHref(`/plans/${m.plan_pid}/template`)}>{t("p.template_sync.open_changes")}</a>{/if}
          </li>
        {/each}
      </ul>
    {/if}
  </section>

  {#if !starter}
    <section aria-labelledby="revise-h">
      <h2 id="revise-h">{t("p.template_sync.revise_h")}</h2>
      <p class="small muted">{t("p.template_sync.revise_says")}</p>
      {#if reviseError}<p class="banner" role="alert">{reviseError}</p>{/if}
      {#if revised}
        <p class="banner success" role="status" data-testid="revised">
          {t("p.template_sync.revised", { version: formatInteger(revised.version) })}
          {#if revised.omitted_unreadable > 0}{t("p.template_sync.revised_omitted", { n: formatInteger(revised.omitted_unreadable) })}{/if}
        </p>
      {/if}
      <form class="stack" onsubmit={revise}>
        <label>
          {t("p.template_sync.revise_plan")}
          <select bind:value={reviseFrom}>
            <option value="">{t("p.template_sync.revise_choose")}</option>
            {#each plans as p (p.pid)}<option value={p.pid}>{p.name}</option>{/each}
          </select>
        </label>
        <div class="row"><button class="button" type="submit" disabled={busy}>{t("p.template_sync.revise")}</button></div>
      </form>
    </section>

    <section aria-labelledby="remove-h">
      <h2 id="remove-h">{t("p.plan_templates.remove_heading")}</h2>
      <p class="small muted">{t("p.plan_templates.remove_says")}</p>
      <button class="button" type="button" disabled={busy} onclick={remove}>{t("p.plan_templates.remove")}</button>
    </section>
  {/if}
{/if}

<style>
  section {
    margin-block-start: 1.5rem;
  }
  .description {
    white-space: pre-wrap;
    overflow-wrap: anywhere;
  }
  .outline,
  .outline ul {
    list-style: none;
    padding: 0;
    margin: 0;
    display: grid;
    gap: 0.4rem;
  }
  .outline ul {
    padding-inline-start: 1rem;
    margin-block-start: 0.25rem;
  }
  .depth[data-depth="1"] {
    padding-inline-start: 1.25rem;
  }
  .depth[data-depth="2"] {
    padding-inline-start: 2.5rem;
  }
  .depth[data-depth="3"] {
    padding-inline-start: 3.75rem;
  }
  .depth[data-depth="4"],
  .depth[data-depth="5"],
  .depth[data-depth="6"] {
    padding-inline-start: 5rem;
  }
  .outline strong {
    margin-inline-end: 0.5rem;
  }
</style>
