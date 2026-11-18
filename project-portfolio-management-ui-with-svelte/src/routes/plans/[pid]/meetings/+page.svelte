<!--
  A plan's meetings (`/plans/[pid]/meetings`, NC-14, BR-3, NC-1 slice 12 part e). A meeting is planned with a title, a
  date and an agenda; then, once, recorded as held with its minutes and outcomes (decisions, actions, notes), or
  cancelled. Agenda items and outcomes may point at the plan's tasks, risks, milestones and issues, each named as the
  record is now. A held or cancelled meeting is a record: minutes cannot be corrected and no attendees are kept (both
  said first). Each meeting downloads as a PDF. From a held meeting's actions, tasks are proposed, checked by the
  service against the minutes and created once (`MeetingTasks.svelte`). Recording and cancelling ask first. Rules in
  `src/lib/meeting-view.ts`; refusals are the catalogue's sentences, never the service's text.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { API_BASE_URL } from "#lib/config.js";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import { errorText } from "#lib/error-text.js";
  import { localeHref, t } from "#lib/i18n.svelte.js";
  import { formatDate, formatInteger } from "#lib/locale-format.js";
  import { PpmClient, type Meeting, type MeetingItem, type MeetingSummary } from "#lib/api/ppm.js";
  import RecordPicker from "#lib/components/RecordPicker.svelte";
  import MeetingTasks from "#lib/components/MeetingTasks.svelte";
  import { loadPlanRecords, recordWords, type PlanRecords } from "#lib/plan-records.js";
  import {
    ITEMS_MAX,
    MINUTES_MAX,
    OUTCOME_KEY,
    OUTCOME_KINDS,
    STATUS_MARK,
    TEXT_MAX,
    TITLE_MAX,
    heldBody,
    meetingRefusalKey,
    planBody,
    type AgendaForm,
    type LineProblem,
    type MeetingAction,
    type OutcomeForm,
  } from "#lib/meeting-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");

  let meetings = $state<MeetingSummary[] | null>(null);
  let loadError = $state<string | null>(null);
  let records = $state<PlanRecords>({});
  let recordsLoaded = $state(false);
  let refusal = $state("");
  let notice = $state("");
  let busy = $state(false);

  // Planning a meeting.
  let title = $state("");
  let scheduledOn = $state("");
  let agenda = $state<Array<AgendaForm & { id: number }>>([]);
  let planProblems = $state<string[]>([]);
  let nextId = 0;

  // The one meeting open, and recording it as held.
  let open = $state<Meeting | null>(null);
  let minutes = $state("");
  let heldOn = $state("");
  let outcomes = $state<Array<OutcomeForm & { id: number }>>([]);
  let heldProblems = $state<string[]>([]);

  const limits = { title: TITLE_MAX, items: ITEMS_MAX, chars: TEXT_MAX, minutes: formatInteger(MINUTES_MAX) };
  const words = (ps: LineProblem[], line: (n: number) => string): string[] =>
    ps.map((p) => (p.line === 0 ? t(p.key, limits) : t("p.meetings.problem_on_line", { line: line(p.line), problem: t(p.key, limits) })));

  async function load() {
    try {
      meetings = await ppm.meetings(pid);
      loadError = null;
    } catch (err) {
      loadError = errorText(err, "p.meetings.could_not_load");
    }
  }

  onMount(async () => {
    await load();
    records = await loadPlanRecords(ppm, pid);
    recordsLoaded = true;
  });

  /** Say a refusal in the catalogue's words, and read the list (and the open meeting) again. */
  async function refused(action: MeetingAction, err: unknown, fallback: Parameters<typeof t>[0]) {
    const k = meetingRefusalKey(action, err);
    refusal = k ? t(k) : errorText(err, fallback);
    notice = "";
    await load();
    if (open) await reopen(open.pid);
  }

  async function reopen(mid: string) {
    try {
      open = await ppm.meeting(pid, mid);
    } catch {
      open = null;
    }
  }

  async function plan(e: SubmitEvent) {
    e.preventDefault();
    refusal = "";
    notice = "";
    const r = planBody({ title, scheduledOn, agenda });
    if ("problems" in r) {
      planProblems = words(r.problems, (n) => t("p.meetings.agenda_item", { n }));
      return;
    }
    planProblems = [];
    busy = true;
    try {
      await ppm.planMeeting(pid, r.body);
      notice = t("p.meetings.planned", { title: r.body.title, date: formatDate(r.body.scheduled_on) });
      title = "";
      scheduledOn = "";
      agenda = [];
      await load();
    } catch (err) {
      await refused("plan", err, "p.meetings.could_not_save");
    } finally {
      busy = false;
    }
  }

  async function toggle(m: MeetingSummary) {
    if (open?.pid === m.pid) {
      open = null;
      return;
    }
    refusal = "";
    try {
      open = await ppm.meeting(pid, m.pid);
      minutes = "";
      heldOn = "";
      outcomes = [];
      heldProblems = [];
    } catch (err) {
      await refused("held", err, "p.meetings.could_not_load");
    }
  }

  async function hold(e: SubmitEvent) {
    e.preventDefault();
    if (!open) return;
    refusal = "";
    notice = "";
    const r = heldBody({ minutes, heldOn, outcomes });
    if ("problems" in r) {
      heldProblems = words(r.problems, (n) => t("p.meetings.outcome_n", { n }));
      return;
    }
    heldProblems = [];
    const m = open;
    if (!(await askConfirm(t("p.meetings.confirm_held", { title: m.title })))) return;
    busy = true;
    try {
      await ppm.holdMeeting(pid, m.pid, r.body);
      notice = t("p.meetings.held_done", { title: m.title });
      await load();
      await reopen(m.pid);
    } catch (err) {
      await refused("held", err, "p.meetings.could_not_save");
    } finally {
      busy = false;
    }
  }

  async function cancel(m: Meeting) {
    refusal = "";
    notice = "";
    if (!(await askConfirm(t("p.meetings.confirm_cancel", { title: m.title })))) return;
    busy = true;
    try {
      await ppm.cancelMeeting(pid, m.pid);
      notice = t("p.meetings.cancelled_done", { title: m.title });
      await load();
      await reopen(m.pid);
    } catch (err) {
      await refused("cancel", err, "p.meetings.could_not_save");
    } finally {
      busy = false;
    }
  }

  async function tasksMade() {
    if (open) await reopen(open.pid);
  }
</script>

{#snippet linkLine(link: MeetingItem["link"])}
  {#if link}
    <span class="small muted">
      · {link.name === null ? t("p.meetings.link_gone") : t("p.meetings.about", { record: recordWords(link.kind, link.name) })}
    </span>
  {/if}
{/snippet}

<svelte:head><title>{t("p.meetings.title_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plan_modules.back")}</a> · <a href={localeHref(`/plans/${pid}/lessons`)}>{t("p.lessons.link")}</a></p>
<h1>{t("p.meetings.title")}</h1>
<p class="small muted" data-testid="meetings-rules">{t("p.meetings.intro")}</p>
{#if loadError}<p class="banner error" role="alert">{loadError}</p>{/if}
{#if refusal}<p class="banner error" role="alert" data-testid="meetings-refusal">{refusal}</p>{/if}
<p class="small" role="status" data-testid="page-notice">{notice}</p>

<section aria-labelledby="meeting-plan-h">
  <h2 id="meeting-plan-h">{t("p.meetings.plan_heading")}</h2>
  <p class="small muted">{t("p.meetings.plan_says", limits)}</p>
  <form class="stack" novalidate onsubmit={plan}>
    <label>{t("p.meetings.f_title", { max: TITLE_MAX })}<input bind:value={title} /></label>
    <label>{t("p.meetings.f_date")}<input type="date" bind:value={scheduledOn} /></label>
    {#each agenda as a, i (a.id)}
      <fieldset class="line">
        <legend>{t("p.meetings.agenda_item", { n: i + 1 })}</legend>
        <label>{t("p.meetings.f_item_text", { n: i + 1, max: TEXT_MAX })}<input bind:value={a.text} /></label>
        <RecordPicker label={t("p.meetings.f_item_link", { n: i + 1 })} bind:value={a.link} {records} loaded={recordsLoaded} none={t("p.meetings.no_record")} />
        <div class="row"><button class="button small" type="button" onclick={() => agenda.splice(i, 1)}>{t("p.meetings.remove_item", { n: i + 1 })}</button></div>
      </fieldset>
    {/each}
    {#if planProblems.length > 0}<ul class="banner error" role="alert">{#each planProblems as p (p)}<li>{p}</li>{/each}</ul>{/if}
    <div class="row">
      <button class="button" type="button" disabled={agenda.length >= ITEMS_MAX} onclick={() => agenda.push({ text: "", link: "", id: nextId++ })}>{t("p.meetings.add_item")}</button>
      <button class="button" type="submit" disabled={busy}>{t("p.meetings.plan")}</button>
    </div>
  </form>
</section>

<section aria-labelledby="meeting-list-h">
  <h2 id="meeting-list-h">{t("p.meetings.list_heading")}</h2>
  <p class="small muted">{t("p.meetings.pdf_says")}</p>
  {#if meetings === null}
    {#if !loadError}<p class="muted">{t("p.meetings.loading")}</p>{/if}
  {:else if meetings.length === 0}
    <p class="muted">{t("p.meetings.none")}</p>
  {:else}
    <ul class="meetings" aria-label={t("p.meetings.list_heading")}>
      {#each meetings as m (m.pid)}
        {@const shown = open?.pid === m.pid ? open : null}
        <li>
          <strong>{m.title}</strong>
          — <span aria-hidden="true">{STATUS_MARK[m.status].symbol}</span> {t(STATUS_MARK[m.status].key)},
          {m.held_on
            ? t("p.meetings.held_on", { date: formatDate(m.held_on) })
            : t("p.meetings.scheduled_on", { date: formatDate(m.scheduled_on) })}
          <span class="row">
            <button class="button small" type="button" aria-expanded={shown !== null} onclick={() => toggle(m)}>
              {shown ? t("p.meetings.hide", { title: m.title }) : t("p.meetings.show", { title: m.title })}
            </button>
            <a class="button small" href={`${API_BASE_URL}/api/plans/${pid}/meetings/${m.pid}/pdf`} aria-label={t("p.meetings.pdf_named", { title: m.title })}>{t("p.meetings.pdf")}</a>
          </span>
          {#if shown}
            <div class="meeting" role="region" aria-label={t("p.meetings.show", { title: m.title })}>
              <h3>{t("p.meetings.agenda_heading")}</h3>
              {#if shown.agenda.length === 0}
                <p class="small muted">{t("p.meetings.agenda_none")}</p>
              {:else}
                <ol data-testid="meeting-agenda">
                  {#each shown.agenda as a, i (i)}<li>{a.text} {@render linkLine(a.link)}</li>{/each}
                </ol>
              {/if}
              {#if shown.status === "held"}
                <h3>{t("p.meetings.minutes_heading")}</h3>
                <p class="minutes">{shown.minutes}</p>
                <h3>{t("p.meetings.outcomes_heading")}</h3>
                {#if shown.outcomes.length === 0}
                  <p class="small muted">{t("p.meetings.outcomes_none")}</p>
                {:else}
                  <ol data-testid="meeting-outcomes">
                    {#each shown.outcomes as o (o.index)}
                      <li>
                        <strong>{o.kind ? t(OUTCOME_KEY[o.kind]) : t("p.meetings.outcome_note")}:</strong> {o.text} {@render linkLine(o.link)}
                        {#if o.task_pid}
                          <span class="small"> · <a href={localeHref(`/plans/${pid}/tasks/${o.task_pid}`)}>{t("p.meetings.task_made", { n: o.index + 1 })}</a></span>
                        {/if}
                      </li>
                    {/each}
                  </ol>
                {/if}
                <p class="small muted">{t("p.meetings.held_kept")}</p>
                <MeetingTasks {pid} meeting={shown} onMade={tasksMade} />
              {:else if shown.status === "cancelled"}
                <p class="small muted">{t("p.meetings.cancelled_kept")}</p>
              {:else}
                <form class="stack" novalidate onsubmit={hold} aria-label={t("p.meetings.held_heading", { title: m.title })}>
                  <h3>{t("p.meetings.held_heading", { title: m.title })}</h3>
                  <p class="small">{t("p.meetings.held_says", limits)}</p>
                  <label>{t("p.meetings.f_minutes", { max: formatInteger(MINUTES_MAX) })}<textarea bind:value={minutes} rows="6"></textarea></label>
                  <label>{t("p.meetings.f_held_on")}<input type="date" bind:value={heldOn} /></label>
                  {#each outcomes as o, i (o.id)}
                    <fieldset class="line">
                      <legend>{t("p.meetings.outcome_n", { n: i + 1 })}</legend>
                      <label>
                        {t("p.meetings.f_outcome_kind", { n: i + 1 })}
                        <select bind:value={o.kind}>
                          <option value="">{t("p.meetings.choose_kind")}</option>
                          {#each OUTCOME_KINDS as k (k)}<option value={k}>{t(OUTCOME_KEY[k])}</option>{/each}
                        </select>
                      </label>
                      <label>{t("p.meetings.f_outcome_text", { n: i + 1, max: TEXT_MAX })}<input bind:value={o.text} /></label>
                      <RecordPicker label={t("p.meetings.f_outcome_link", { n: i + 1 })} bind:value={o.link} {records} loaded={recordsLoaded} none={t("p.meetings.no_record")} />
                      <div class="row"><button class="button small" type="button" onclick={() => outcomes.splice(i, 1)}>{t("p.meetings.remove_outcome", { n: i + 1 })}</button></div>
                    </fieldset>
                  {/each}
                  {#if heldProblems.length > 0}<ul class="banner error" role="alert">{#each heldProblems as p (p)}<li>{p}</li>{/each}</ul>{/if}
                  <div class="row">
                    <button class="button" type="button" disabled={outcomes.length >= ITEMS_MAX} onclick={() => outcomes.push({ kind: "", text: "", link: "", id: nextId++ })}>{t("p.meetings.add_outcome")}</button>
                    <button class="button" type="submit" disabled={busy}>{t("p.meetings.record_held")}</button>
                  </div>
                </form>
                <div class="row">
                  <button class="button" type="button" disabled={busy} onclick={() => cancel(shown)}>{t("p.meetings.cancel_meeting", { title: m.title })}</button>
                </div>
              {/if}
            </div>
          {/if}
        </li>
      {/each}
    </ul>
  {/if}
</section>

<style>
  section {
    margin-block-start: 2rem;
  }
  form {
    max-width: 44rem;
  }
  fieldset.line {
    display: grid;
    gap: 0.35rem;
  }
  .row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    align-items: end;
    margin-block: 0.25rem;
  }
  .meetings {
    list-style: none;
    padding: 0;
    display: grid;
    gap: 1rem;
  }
  .meeting {
    display: grid;
    gap: 0.5rem;
    max-width: 48rem;
    border-inline-start: 0.2rem solid currentColor;
    padding-inline-start: 0.75rem;
  }
  .minutes {
    white-space: pre-wrap;
    overflow-wrap: anywhere;
  }
</style>
