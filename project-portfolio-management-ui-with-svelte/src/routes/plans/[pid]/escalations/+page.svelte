<!--
  A plan's escalations (`/plans/[pid]/escalations`): raise one at a level of the governance escalation path, and work
  the ones already raised: acknowledge, take up a level (with a reason), resolve (an outcome and a written response)
  or withdraw. Each state is a word and a shape; "overdue" is said in words. Opened from a red tolerance with
  `?reason=tolerance&dimension=…` it starts a tolerance escalation.
-->
<script lang="ts">
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { PpmClient, type Escalation } from "#lib/api/ppm.js";
  import {
    DIMENSIONS,
    OUTCOMES,
    REASONS,
    STATUS_SYMBOL,
    STATUS_TEXT,
    ageText,
    eventLabel,
    isLive,
    levelText,
    noteOk,
    outcomeLabel,
    raiseProblems,
    reasonLabel,
  } from "#lib/escalation-view.js";
  import { dimensionLabel } from "#lib/tolerance-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");
  let list = $state<Escalation[] | null>(null);
  let error = $state<string | null>(null);
  let notice = $state("");
  let shown = $state<Record<string, Escalation>>({});

  let title = $state("");
  let description = $state("");
  let reason = $state(page.url.searchParams.get("reason") ?? "other");
  let dimension = $state(page.url.searchParams.get("dimension") ?? "");
  let level = $state("");
  let to = $state("");
  let owner = $state("");
  let respondBy = $state("");
  let problems = $state<string[]>([]);

  /** What is typed against each escalation's actions, by escalation. */
  let upNote = $state<Record<string, string>>({});
  let outcome = $state<Record<string, string>>({});
  let response = $state<Record<string, string>>({});

  async function load() {
    try {
      list = (await ppm.planEscalations(pid)).escalations;
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.escalations.could_not_load");
    }
  }
  onMount(load);

  const message = (err: unknown) => t("p.escalations.could_not", { reason: err instanceof Error ? err.message : "" });

  async function raise(e: SubmitEvent) {
    e.preventDefault();
    problems = raiseProblems({ title, reason_kind: reason, tolerance_dimension: dimension, level, owner_ref: owner });
    if (problems.length > 0) return;
    const body: Record<string, unknown> = { title: title.trim(), reason_kind: reason };
    if (description.trim()) body.description = description.trim();
    if (reason === "tolerance") body.tolerance_dimension = dimension;
    if (level.trim()) body.level = Number(level);
    if (to.trim()) body.escalated_to = to.trim();
    if (owner.trim()) body.owner_ref = owner.trim();
    if (respondBy) body.respond_by = respondBy;
    try {
      await ppm.raiseEscalation(pid, body);
      notice = t("p.escalations.raised", { title: title.trim() });
      title = description = level = to = owner = respondBy = "";
      await load();
    } catch (err) {
      notice = message(err);
    }
  }

  async function toggle(e: Escalation) {
    if (shown[e.pid]) {
      const { [e.pid]: _gone, ...rest } = shown;
      shown = rest;
      return;
    }
    try {
      shown = { ...shown, [e.pid]: await ppm.escalation(pid, e.pid) };
    } catch (err) {
      notice = message(err);
    }
  }

  /** Do something to an escalation, reload the list, and refresh its history if it is open. */
  async function act(e: Escalation, action: "acknowledge" | "raise-level" | "resolve" | "withdraw", body: Record<string, unknown> = {}) {
    try {
      await ppm.escalationAction(pid, e.pid, action, body);
      notice = t("p.escalations.done");
      await load();
      if (shown[e.pid]) shown = { ...shown, [e.pid]: await ppm.escalation(pid, e.pid) };
    } catch (err) {
      notice = message(err);
    }
  }

  function takeUp(e: Escalation) {
    if (!noteOk(upNote[e.pid] ?? "")) {
      notice = t("p.escalations.note_problem");
      return;
    }
    void act(e, "raise-level", { note: upNote[e.pid]!.trim() });
  }

  function resolve(e: Escalation) {
    if (!noteOk(response[e.pid] ?? "")) {
      notice = t("p.escalations.note_problem");
      return;
    }
    void act(e, "resolve", { outcome: outcome[e.pid] || "decided", response: response[e.pid]!.trim() });
  }
</script>

<svelte:head><title>{t("p.escalations.escalations_ppm")}</title></svelte:head>

<p class="small">
  <a href={localeHref(`/plans/${pid}`)}>{t("p.escalations.back")}</a> ·
  <a href={localeHref("/escalations")}>{t("p.escalations.register_link")}</a>
</p>
<h1>{t("p.escalations.escalations")}</h1>
<p class="small muted">{t("p.escalations.intro")}</p>
{#if error}<p class="banner" role="alert">{error}</p>{/if}
<p class="small" role="status">{notice}</p>

<section aria-labelledby="raise-h">
  <h2 id="raise-h">{t("p.escalations.raise_heading")}</h2>
  <form onsubmit={raise} aria-labelledby="raise-h">
    <label>{t("p.escalations.f_title")} <input bind:value={title} maxlength="200" /></label>
    <label>{t("p.escalations.f_reason")}
      <select bind:value={reason}>{#each REASONS as r (r)}<option value={r}>{reasonLabel(r)}</option>{/each}</select>
    </label>
    {#if reason === "tolerance"}
      <label>{t("p.escalations.f_dimension")}
        <select bind:value={dimension}>
          <option value=""></option>
          {#each DIMENSIONS as d (d)}<option value={d}>{dimensionLabel(d)}</option>{/each}
        </select>
      </label>
    {/if}
    <label>{t("p.escalations.f_level")} <input type="number" min="1" max="9" bind:value={level} /></label>
    <label>{t("p.escalations.f_to")} <input bind:value={to} maxlength="120" /></label>
    <label>{t("p.escalations.f_owner")} <input bind:value={owner} /></label>
    <label>{t("p.escalations.f_respond")} <input type="date" bind:value={respondBy} /></label>
    <label>{t("p.escalations.f_description")} <textarea bind:value={description} rows="3"></textarea></label>
    {#if problems.length > 0}<ul class="banner" role="alert">{#each problems as m (m)}<li>{m}</li>{/each}</ul>{/if}
    <button type="submit">{t("p.escalations.raise")}</button>
  </form>
</section>

<section aria-labelledby="list-h">
  <h2 id="list-h">{t("p.escalations.escalations")}</h2>
  {#if list === null}
    <p class="muted">{t("p.escalations.loading")}</p>
  {:else if list.length === 0}
    <p>{t("p.escalations.none")}</p>
  {:else}
    <ul class="escalations">
      {#each list as e (e.pid)}
        {@const open = shown[e.pid]}
        <li class={`esc esc-${e.status}`}>
          <strong>{e.title}</strong>
          — <span aria-hidden="true">{STATUS_SYMBOL[e.status]}</span> {STATUS_TEXT[e.status]},
          {levelText(e)}, <span class="small">{ageText(e)}</span>
          {#if e.reason_kind === "tolerance" && e.tolerance_dimension}<span class="small muted"> · {dimensionLabel(e.tolerance_dimension)}</span>{/if}
          <button type="button" aria-expanded={open !== undefined} onclick={() => toggle(e)}>{open ? t("p.escalations.hide", { title: e.title }) : t("p.escalations.show", { title: e.title })}</button>
          {#if open}
            <div role="region" aria-label={t("p.escalations.show", { title: e.title })}>
              {#if open.description}<p>{open.description}</p>{/if}
              <p class="small">{reasonLabel(open.reason_kind)}{open.respond_by ? ` · ${t("p.escalations.respond_by")}: ${open.respond_by}` : ""}</p>
              {#if open.outcome}<p><strong>{outcomeLabel(open.outcome)}:</strong> {open.response}</p>{/if}
              <h3>{t("p.escalations.history")}</h3>
              <ol>
                {#each open.events ?? [] as ev, i (i)}
                  <li class="small">{ev.at} — {eventLabel(ev.event)}, {t("l.escalation.level", { level: ev.level })}{ev.actor ? ` (${ev.actor})` : ""}{ev.note ? `: ${ev.note}` : ""}</li>
                {/each}
              </ol>
              {#if isLive(open)}
                <div class="actions">
                  {#if open.status === "open"}<button type="button" onclick={() => act(e, "acknowledge")}>{t("p.escalations.acknowledge")}</button>{/if}
                  <button type="button" onclick={() => act(e, "withdraw")}>{t("p.escalations.withdraw")}</button>
                </div>
                <label>{t("p.escalations.take_up_note")} <input bind:value={upNote[e.pid]} /></label>
                <button type="button" onclick={() => takeUp(e)}>{t("p.escalations.take_up")}</button>
                <label>{t("p.escalations.outcome")}
                  <select bind:value={outcome[e.pid]}>{#each OUTCOMES as o (o)}<option value={o}>{outcomeLabel(o)}</option>{/each}</select>
                </label>
                <label>{t("p.escalations.response")} <textarea bind:value={response[e.pid]} rows="2"></textarea></label>
                <button type="button" onclick={() => resolve(e)}>{t("p.escalations.resolve")}</button>
              {/if}
            </div>
          {/if}
        </li>
      {/each}
    </ul>
  {/if}
</section>

<style>
  form { display: grid; gap: 0.5rem; max-inline-size: 32rem; margin-block: 1rem; }
  label { display: grid; gap: 0.2rem; }
  .escalations { list-style: none; padding: 0; }
  .esc { border-inline-start: 0.4rem solid #5a5a5a; padding: 0.4rem 0.75rem; margin-block: 0.75rem; }
  .esc-open { border-inline-start-color: #b00020; }
  .esc-acknowledged { border-inline-start-color: #c77700; }
  .esc-resolved { border-inline-start-color: #1b7a3a; }
  .esc-withdrawn { border-inline-start-style: dashed; }
  .actions { display: flex; flex-wrap: wrap; gap: 0.5rem; margin-block: 0.5rem; }
  [role="region"] { display: grid; gap: 0.5rem; max-inline-size: 36rem; }
</style>
