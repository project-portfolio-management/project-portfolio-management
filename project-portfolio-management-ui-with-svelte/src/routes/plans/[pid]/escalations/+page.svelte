<!--
  A plan's escalations (`/plans/[pid]/escalations`): raise one at a level of the governance escalation path, and work
  the ones already raised: acknowledge, take up a level (with a reason), resolve (an outcome and a written response)
  or withdraw. Each state is a word and a shape; "overdue" is said in words. Opened from a red tolerance with
  `?reason=tolerance&dimension=…` it starts a tolerance escalation.

  NC-1 slice 12 part (d), CF-4: acknowledging and resolving go through the records (`PUT …/acknowledgement`,
  `PUT …/resolution`), and an opened escalation shows them (who acknowledged at its current level, and the
  resolution, which is never overwritten). The rights each needs and the resolution's rules are said first; a
  resolution is asked about before it is recorded; each refusal is the catalogue's sentence, never the service's text.
-->
<script lang="ts">
  import { errorReason, errorText } from "#lib/error-text.js";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import {
    PpmClient,
    type Escalation,
    type EscalationAcknowledgement,
    type EscalationResolution,
  } from "#lib/api/ppm.js";
  import type { RaiseEscalationBody } from "#lib/api/ppm-types.js";
  import {
    DIMENSIONS,
    NOTE_MIN,
    OUTCOMES,
    RESPONSE_MAX,
    REASONS,
    STATUS_SYMBOL,
    STATUS_TEXT,
    acknowledgeBody,
    acknowledgementText,
    ageText,
    automaticText,
    eventLabel,
    isLive,
    isNoRecord,
    levelText,
    noteOk,
    outcomeLabel,
    raiseProblems,
    reasonLabel,
    recordRefusalKey,
    recordsToRead,
    resolutionProblems,
    resolutionText,
    resolveBody,
    type EscalationRecordAction,
  } from "#lib/escalation-view.js";
  import { dimensionLabel } from "#lib/tolerance-view.js";
  import { formatInteger } from "#lib/locale-format.js";

  /** The records read for an opened escalation: `null` is "none yet"; `failed` when a read did not work. */
  interface Records {
    ack: EscalationAcknowledgement | null;
    resolution: EscalationResolution | null;
    failed: boolean;
  }

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");
  let list = $state<Escalation[] | null>(null);
  let error = $state<string | null>(null);
  let notice = $state("");
  let refusal = $state("");
  let shown = $state<Record<string, Escalation>>({});
  let records = $state<Record<string, Records>>({});

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
  let ackNote = $state<Record<string, string>>({});
  let outcome = $state<Record<string, string>>({});
  let response = $state<Record<string, string>>({});
  let resolveProblemsOf = $state<Record<string, string[]>>({});

  async function load() {
    try {
      list = (await ppm.planEscalations(pid)).escalations;
      error = null;
    } catch (err) {
      error = errorText(err, "p.escalations.could_not_load");
    }
  }
  onMount(load);

  const message = (err: unknown) => t("p.escalations.could_not", { reason: errorReason(err) });

  async function raise(e: SubmitEvent) {
    e.preventDefault();
    problems = raiseProblems({ title, reason_kind: reason, tolerance_dimension: dimension, level, owner_ref: owner });
    if (problems.length > 0) return;
    const body: RaiseEscalationBody = { title: title.trim(), reason_kind: reason };
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

  /** The acknowledgement and the resolution of `e`, as far as its state says there are any. */
  async function readRecords(e: Escalation): Promise<Records> {
    const want = recordsToRead(e.status);
    const rec: Records = { ack: null, resolution: null, failed: false };
    const miss = (err: unknown) => {
      if (!isNoRecord(err)) rec.failed = true;
    };
    await Promise.all([
      want.acknowledgement
        ? ppm.escalationAcknowledgement(pid, e.pid).then((a) => { rec.ack = a; }, miss)
        : null,
      want.resolution
        ? ppm.escalationResolution(pid, e.pid).then((r) => { rec.resolution = r; }, miss)
        : null,
    ]);
    return rec;
  }

  /** Read one escalation with its history and its records, and show it. */
  async function openOne(epid: string) {
    const one = await ppm.escalation(pid, epid);
    const rec = await readRecords(one);
    shown = { ...shown, [epid]: one };
    records = { ...records, [epid]: rec };
  }

  function close(epid: string) {
    const { [epid]: _gone, ...rest } = shown;
    shown = rest;
  }

  async function toggle(e: Escalation) {
    if (shown[e.pid]) {
      close(e.pid);
      return;
    }
    try {
      await openOne(e.pid);
    } catch (err) {
      notice = message(err);
    }
  }

  /** Read the list again, and the escalation again if it is open (closed when it is gone). */
  async function refresh(epid: string) {
    await load();
    if (!shown[epid]) return;
    try {
      await openOne(epid);
    } catch {
      close(epid);
    }
  }

  /** Do something to an escalation, reload the list, and refresh its history if it is open. */
  async function act(e: Escalation, action: "raise-level" | "withdraw", body: Record<string, unknown> = {}) {
    refusal = "";
    try {
      await ppm.escalationAction(pid, e.pid, action, body);
      notice = t("p.escalations.done");
      await refresh(e.pid);
    } catch (err) {
      notice = message(err);
    }
  }

  /** Say a refused record in the catalogue's words, and read the escalation again. */
  async function refused(e: Escalation, action: EscalationRecordAction, err: unknown) {
    const key = recordRefusalKey(action, err);
    refusal = key ? t(key) : errorText(err, "p.escalations.could_not_save");
    notice = "";
    await refresh(e.pid);
  }

  async function acknowledge(e: Escalation, ev: SubmitEvent) {
    ev.preventDefault();
    refusal = "";
    try {
      await ppm.acknowledgeEscalation(pid, e.pid, acknowledgeBody(ackNote[e.pid] ?? ""));
      ackNote = { ...ackNote, [e.pid]: "" };
      notice = t("p.escalations.ack_done");
      await refresh(e.pid);
    } catch (err) {
      await refused(e, "acknowledge", err);
    }
  }

  function takeUp(e: Escalation) {
    if (!noteOk(upNote[e.pid] ?? "")) {
      notice = t("p.escalations.note_problem");
      return;
    }
    void act(e, "raise-level", { note: upNote[e.pid]!.trim() });
  }

  async function resolve(e: Escalation, ev: SubmitEvent) {
    ev.preventDefault();
    refusal = "";
    const chosen = outcome[e.pid] ?? "decided";
    const written = response[e.pid] ?? "";
    const wrong = resolutionProblems(chosen, written);
    resolveProblemsOf = { ...resolveProblemsOf, [e.pid]: wrong };
    if (wrong.length > 0) return;
    if (!(await askConfirm(t("p.escalations.confirm_resolve", { title: e.title })))) return;
    try {
      await ppm.resolveEscalation(pid, e.pid, resolveBody(chosen, written));
      response = { ...response, [e.pid]: "" };
      notice = t("p.escalations.resolution_done");
      await refresh(e.pid);
    } catch (err) {
      await refused(e, "resolve", err);
    }
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
{#if refusal}<p class="banner" role="alert" data-testid="esc-refusal">{refusal}</p>{/if}
<p class="small" role="status">{notice}</p>

<section aria-labelledby="raise-h">
  <h2 id="raise-h">{t("p.escalations.raise_heading")}</h2>
  <form onsubmit={raise} aria-labelledby="raise-h" novalidate>
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
        {@const rec = records[e.pid]}
        <li class={`esc esc-${e.status}`}>
          <strong>{e.title}</strong>
          — <span aria-hidden="true">{STATUS_SYMBOL[e.status]}</span> {STATUS_TEXT[e.status]},
          {levelText(e)}, <span class="small">{ageText(e)}</span>
          {#if e.reason_kind === "tolerance" && e.tolerance_dimension}<span class="small muted"> · {dimensionLabel(e.tolerance_dimension)}</span>{/if}
          {#if automaticText(e)}<span class="small muted"> · {automaticText(e)}</span>{/if}
          <button type="button" aria-expanded={open !== undefined} onclick={() => toggle(e)}>{open ? t("p.escalations.hide", { title: e.title }) : t("p.escalations.show", { title: e.title })}</button>
          {#if open}
            <div role="region" aria-label={t("p.escalations.show", { title: e.title })}>
              {#if open.description}<p>{open.description}</p>{/if}
              <p class="small">{reasonLabel(open.reason_kind)}{open.respond_by ? ` · ${t("p.escalations.respond_by")}: ${open.respond_by}` : ""}</p>

              <h3>{t("p.escalations.records_heading")}</h3>
              {#if rec?.failed}<p class="small" data-testid="esc-records-failed">{t("p.escalations.records_failed")}</p>{/if}
              <div data-testid="esc-ack">
                {#if rec?.ack}
                  <p>{acknowledgementText(rec.ack)}</p>
                  {#if rec.ack.note}<p class="small">{t("p.escalations.ack_note_line", { note: rec.ack.note })}</p>{/if}
                {:else if !rec?.failed}
                  <p class="small muted">{t("p.escalations.ack_none")}</p>
                {/if}
              </div>
              {#if rec?.resolution}
                <div data-testid="esc-resolution">
                  <p>{resolutionText(rec.resolution)}</p>
                  {#if rec.resolution.response}<blockquote>{rec.resolution.response}</blockquote>{/if}
                  <p class="small muted">{t("p.escalations.resolution_kept")}</p>
                </div>
              {/if}

              <h3>{t("p.escalations.history")}</h3>
              <ol>
                {#each open.events ?? [] as ev, i (i)}
                  <li class="small">{ev.at} — {eventLabel(ev.event)}, {t("l.escalation.level", { level: ev.level })}{ev.actor ? ` (${ev.actor})` : ""}{ev.note ? `: ${ev.note}` : ""}</li>
                {/each}
              </ol>
              {#if isLive(open)}
                <p class="small" data-testid="esc-rules">{t("p.escalations.records_rules")}</p>
                {#if open.status === "open"}
                  <form class="act" novalidate onsubmit={(ev) => acknowledge(open, ev)} aria-label={t("p.escalations.ack_heading")}>
                    <label>{t("p.escalations.ack_note")} <input bind:value={ackNote[e.pid]} /></label>
                    <button type="submit">{t("p.escalations.acknowledge")}</button>
                  </form>
                {/if}
                <div class="actions">
                  <button type="button" onclick={() => act(e, "withdraw")}>{t("p.escalations.withdraw")}</button>
                </div>
                <label>{t("p.escalations.take_up_note")} <input bind:value={upNote[e.pid]} /></label>
                <button type="button" onclick={() => takeUp(e)}>{t("p.escalations.take_up")}</button>
                <form class="act" novalidate onsubmit={(ev) => resolve(open, ev)} aria-label={t("p.escalations.resolve_heading")}>
                  <p class="small">{t("p.escalations.resolve_says", { min: formatInteger(NOTE_MIN), max: formatInteger(RESPONSE_MAX) })}</p>
                  <label>{t("p.escalations.outcome")}
                    <select bind:value={outcome[e.pid]}>{#each OUTCOMES as o (o)}<option value={o}>{outcomeLabel(o)}</option>{/each}</select>
                  </label>
                  <label>{t("p.escalations.response")} <textarea bind:value={response[e.pid]} rows="2"></textarea></label>
                  {#if (resolveProblemsOf[e.pid] ?? []).length > 0}
                    <ul class="banner" role="alert">{#each resolveProblemsOf[e.pid] ?? [] as m (m)}<li>{m}</li>{/each}</ul>
                  {/if}
                  <button type="submit">{t("p.escalations.resolve")}</button>
                </form>
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
  form.act { margin-block: 0.5rem; }
  label { display: grid; gap: 0.2rem; }
  .escalations { list-style: none; padding: 0; }
  .esc { border-inline-start: 0.4rem solid #5a5a5a; padding: 0.4rem 0.75rem; margin-block: 0.75rem; }
  .esc-open { border-inline-start-color: #b00020; }
  .esc-acknowledged { border-inline-start-color: #c77700; }
  .esc-resolved { border-inline-start-color: #1b7a3a; }
  .esc-withdrawn { border-inline-start-style: dashed; }
  .actions { display: flex; flex-wrap: wrap; gap: 0.5rem; margin-block: 0.5rem; }
  blockquote { margin: 0; padding-inline-start: 0.75rem; border-inline-start: 0.2rem solid currentColor; }
  [role="region"] { display: grid; gap: 0.5rem; max-inline-size: 36rem; }
</style>
