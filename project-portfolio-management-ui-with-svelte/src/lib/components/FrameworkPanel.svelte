<!--
  A plan's ways of working or governance framework: what is in force and where it came from (this plan's own, or
  inherited from a plan above), follow a different shared one, stop following (then inherit again), or write one for
  this plan and edit it, with every list on it (ceremonies, escalation path, decision rights, forums).
-->
<script lang="ts">
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient, type Framework, type FrameworkFamily, type FrameworkInForce } from "#lib/api/ppm.js";
  import {
    DECISION_MODELS,
    decisionModelLabel,
    methodologyLabel,
    METHODOLOGIES,
    formOf,
    frameworkBody,
    frameworkProblems,
    recordOf,
    sourceText,
    summary,
    type FrameworkForm,
  } from "#lib/framework-view.js";

  let { family, pid, heading }: { family: FrameworkFamily; pid: string; heading: string } = $props();

  const ppm = PpmClient.withFetch();
  let inForce = $state<FrameworkInForce | null>(null);
  let all = $state<Framework[]>([]);
  let error = $state<string | null>(null);
  let notice = $state("");
  let chosen = $state("");
  let editing = $state(false);
  let creating = $state(false);
  let form = $state<FrameworkForm>(formOf(null));
  let problems = $state<string[]>([]);

  const record = $derived(recordOf(family, inForce));
  /** A record this plan owns, or an organisation-wide one (which other plans may follow too: see `sharedWith`). */
  const editable = $derived(!!record && (record.owner_plan_pid === pid || record.scope === "organization"));
  /** How many other plans an edit would also change. */
  const sharedWith = $derived(record ? Math.max(0, record.plans_using - (inForce?.source === "own" ? 1 : 0)) : 0);
  let confirmShared = $state(false);

  async function load() {
    try {
      [inForce, all] = await Promise.all([ppm.frameworkInForce(family, pid), ppm.frameworks(family)]);
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.frameworks.could_not_load");
    }
  }
  onMount(load);

  const message = (err: unknown) => t("p.frameworks.could_not", { reason: err instanceof Error ? err.message : "" });

  async function follow() {
    if (!chosen) return;
    try {
      inForce = await ppm.linkFramework(family, pid, chosen);
      chosen = "";
      notice = t("p.frameworks.followed");
    } catch (err) {
      notice = message(err);
    }
  }

  async function unfollow() {
    try {
      inForce = await ppm.linkFramework(family, pid, null);
      notice = t("p.frameworks.unfollowed");
    } catch (err) {
      notice = message(err);
    }
  }

  function startNew() {
    form = formOf(null);
    problems = [];
    creating = true;
    editing = true;
  }

  function startEdit() {
    confirmShared = false;
    form = formOf(record);
    problems = [];
    creating = false;
    editing = true;
  }

  async function save(e: SubmitEvent, isNew: boolean) {
    e.preventDefault();
    problems = frameworkProblems(family, form);
    if (!isNew && sharedWith > 0 && !confirmShared) problems = [...problems, t("p.frameworks.confirm_problem", { n: sharedWith })];
    if (problems.length > 0) return;
    try {
      if (isNew) {
        await ppm.createFramework(family, frameworkBody(family, form, null, pid));
      } else if (record) {
        await ppm.updateFramework(family, record.pid, frameworkBody(family, form, record, null));
      }
      editing = false;
      notice = t("p.frameworks.saved");
      await load();
    } catch (err) {
      notice = message(err);
    }
  }
  const others = $derived(all.filter((f) => f.pid !== record?.pid));
  const chosenRecord = $derived(others.find((f) => f.pid === chosen));
  /** Nobody follows it, and it is the organisation's or this plan's own. */
  const deletable = $derived(!!chosenRecord && chosenRecord.plans_using === 0 && (chosenRecord.scope === "organization" || chosenRecord.owner_plan_pid === pid));

  async function destroy() {
    if (!chosenRecord) return;
    try {
      await ppm.deleteFramework(family, chosenRecord.pid);
      notice = t("p.frameworks.deleted", { name: chosenRecord.name });
      chosen = "";
      await load();
    } catch (err) {
      notice = message(err);
    }
  }
</script>

<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.frameworks.back")}</a></p>
<h1>{heading}</h1>
<p class="small muted">{t(family === "ways-of-working" ? "p.frameworks.intro_wow" : "p.frameworks.intro_gov")}</p>
{#if error}<p class="banner" role="alert">{error}</p>{/if}
<p class="small" role="status">{notice}</p>

{#if inForce}
  <section aria-labelledby="inforce-h">
    <h2 id="inforce-h">{t("p.frameworks.in_force")}</h2>
    {#if record}
      <p>
        <strong>{record.name}</strong> — {sourceText(inForce)}
        {#if inForce.source === "inherited" && inForce.from_plan_pid}
          (<a href={localeHref(`/plans/${inForce.from_plan_pid}`)}>{t("p.frameworks.from_plan")}</a>)
        {/if}
        <span class="small muted"> · {record.scope === "plan" ? t("p.frameworks.scope_plan") : t("p.frameworks.scope_org")}</span>
      </p>
      {#if record.description}<p>{record.description}</p>{/if}
      <dl>
        {#each summary(family, record) as [label, value] (label)}<dt>{label}</dt><dd>{value}</dd>{/each}
      </dl>
      <p class="actions">
        {#if editable}<button type="button" onclick={startEdit}>{t("p.frameworks.edit")}</button>{/if}
        {#if inForce.source === "own"}<button type="button" onclick={unfollow}>{t("p.frameworks.unfollow")}</button>{/if}
      </p>
    {:else}
      <p>{inForce.reason ?? t("p.frameworks.none")}</p>
    {/if}
  </section>

  <section aria-labelledby="follow-h">
    <h2 id="follow-h">{t("p.frameworks.follow_heading")}</h2>
    {#if others.length === 0}
      <p class="muted">{t("p.frameworks.no_others")}</p>
    {:else}
      <label>{t("p.frameworks.choose")}
        <select bind:value={chosen}>
          <option value=""></option>
          {#each others as f (f.pid)}<option value={f.pid}>{f.name}</option>{/each}
        </select>
      </label>
      <button type="button" disabled={!chosen} onclick={follow}>{t("p.frameworks.follow")}</button>
      {#if deletable}<button type="button" onclick={destroy}>{t("p.frameworks.delete", { name: chosenRecord?.name ?? "" })}</button>{/if}
    {/if}
    <p><button type="button" onclick={startNew}>{t("p.frameworks.write_new")}</button></p>
  </section>

  {#if editing}
    <section aria-labelledby="form-h">
      <h2 id="form-h">{creating ? t("p.frameworks.new_heading") : t("p.frameworks.edit_heading")}</h2>
      <form onsubmit={(e) => save(e, creating)} aria-labelledby="form-h">
        <label>{t("p.frameworks.f_name")} <input bind:value={form.name} maxlength="120" /></label>
        {#if family === "ways-of-working"}
          <label>{t("p.frameworks.methodology")}
            <select bind:value={form.methodology}>{#each METHODOLOGIES as m (m)}<option value={m}>{methodologyLabel(m)}</option>{/each}</select>
          </label>
          <label>{t("p.frameworks.cadence_days")} <input type="number" min="1" max="366" bind:value={form.cadence_days} /></label>
          <label>{t("p.frameworks.wip_limit")} <input type="number" min="1" max="1000" bind:value={form.wip_limit} /></label>
          <label>{t("p.frameworks.ready")} <textarea bind:value={form.ready} rows="2"></textarea></label>
          <label>{t("p.frameworks.done")} <textarea bind:value={form.done} rows="2"></textarea></label>
          <fieldset>
            <legend>{t("p.frameworks.ceremonies")}</legend>
            {#each form.ceremonies as row, i (i)}
              <div class="path-row">
                <label>{t("p.frameworks.practice_name")} <input bind:value={row.name} maxlength="80" /></label>
                <label>{t("p.frameworks.practice_cadence")} <input bind:value={row.cadence} maxlength="80" /></label>
                <label>{t("p.frameworks.practice_purpose")} <input bind:value={row.purpose} maxlength="300" /></label>
                <button type="button" onclick={() => form.ceremonies.splice(i, 1)}>{t("p.frameworks.row_remove", { n: i + 1 })}</button>
              </div>
            {/each}
            <button type="button" onclick={() => form.ceremonies.push({ name: "", cadence: "", purpose: "" })}>{t("p.frameworks.ceremony_add")}</button>
          </fieldset>
        {:else}
          <label>{t("p.frameworks.decision_model")}
            <select bind:value={form.decision_model}>{#each DECISION_MODELS as m (m)}<option value={m}>{decisionModelLabel(m)}</option>{/each}</select>
          </label>
          <label>{t("p.frameworks.review_cadence")} <input type="number" min="1" max="366" bind:value={form.review_cadence_days} /></label>
          <label>{t("p.frameworks.reporting_cadence")} <input type="number" min="1" max="366" bind:value={form.reporting_cadence_days} /></label>
          <fieldset>
            <legend>{t("p.frameworks.path")}</legend>
            {#each form.path as row, i (i)}
              <div class="path-row">
                <label>{t("p.frameworks.path_level")} <input type="number" min="1" max="9" bind:value={row.level} /></label>
                <label>{t("p.frameworks.path_body")} <input bind:value={row.body} maxlength="120" /></label>
                <label>{t("p.frameworks.path_trigger")} <input bind:value={row.trigger} maxlength="300" /></label>
                <button type="button" onclick={() => form.path.splice(i, 1)}>{t("p.frameworks.path_remove", { n: i + 1 })}</button>
              </div>
            {/each}
            <button type="button" onclick={() => form.path.push({ level: String(form.path.length + 1), body: "", trigger: "" })}>{t("p.frameworks.path_add")}</button>
          </fieldset>
          <fieldset>
            <legend>{t("p.frameworks.rights")}</legend>
            {#each form.rights as row, i (i)}
              <div class="path-row">
                <label>{t("p.frameworks.right_decision")} <input bind:value={row.decision} maxlength="120" /></label>
                <label>{t("p.frameworks.right_decides")} <input bind:value={row.decides} maxlength="120" /></label>
                <label>{t("p.frameworks.right_consulted")} <input bind:value={row.consulted} maxlength="120" /></label>
                <button type="button" onclick={() => form.rights.splice(i, 1)}>{t("p.frameworks.row_remove", { n: i + 1 })}</button>
              </div>
            {/each}
            <button type="button" onclick={() => form.rights.push({ decision: "", decides: "", consulted: "" })}>{t("p.frameworks.right_add")}</button>
          </fieldset>
          <fieldset>
            <legend>{t("p.frameworks.forums")}</legend>
            {#each form.forums as row, i (i)}
              <div class="path-row">
                <label>{t("p.frameworks.practice_name")} <input bind:value={row.name} maxlength="80" /></label>
                <label>{t("p.frameworks.practice_cadence")} <input bind:value={row.cadence} maxlength="80" /></label>
                <label>{t("p.frameworks.practice_purpose")} <input bind:value={row.purpose} maxlength="300" /></label>
                <button type="button" onclick={() => form.forums.splice(i, 1)}>{t("p.frameworks.row_remove", { n: i + 1 })}</button>
              </div>
            {/each}
            <button type="button" onclick={() => form.forums.push({ name: "", cadence: "", purpose: "" })}>{t("p.frameworks.forum_add")}</button>
          </fieldset>
        {/if}
        {#if !creating && sharedWith > 0}
          <label class="shared"><input type="checkbox" bind:checked={confirmShared} /> {t("p.frameworks.confirm_shared", { n: sharedWith })}</label>
        {/if}
        {#if problems.length > 0}<ul class="banner" role="alert">{#each problems as m (m)}<li>{m}</li>{/each}</ul>{/if}
        <span class="actions">
          <button type="submit">{t("p.frameworks.save")}</button>
          <button type="button" onclick={() => (editing = false)}>{t("p.frameworks.cancel")}</button>
        </span>
      </form>
    </section>
  {/if}
{:else if !error}
  <p class="muted">{t("p.frameworks.loading")}</p>
{/if}

<style>
  .path-row { margin-block-end: 0.5rem; }
  .path-row button,
  fieldset > button { min-block-size: 2rem; margin-block: 0.25rem; }
</style>
