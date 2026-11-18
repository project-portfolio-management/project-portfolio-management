<!--
  Tasks from a held meeting's actions (`/plans/[pid]/meetings`, BR-3, NC-1 slice 12 part e). For each action with no
  task yet, a person may propose one: a title, a passage of the minutes that supports it, and optionally an assignee
  (a reference already on the plan's tasks or RACI), a due date and points. Every rule is said first and checked
  before anything is sent. "Check the proposals" asks the service, which writes nothing and says, for each, whether it
  is ready or why not (in the catalogue's words by the service's reason code). Only a check that is still current and
  has every proposal ready lets "Create the tasks" go, after a question; they are created all or none, once per
  action, under one idempotency key kept across a retry. Rules in `src/lib/meeting-view.ts`.
-->
<script lang="ts">
  import { askConfirm } from "#lib/confirm.svelte.js";
  import { errorText } from "#lib/error-text.js";
  import { t, tn } from "#lib/i18n.svelte.js";
  import { formatDate } from "#lib/locale-format.js";
  import { PpmClient, type Meeting, type TaskProposalPreview } from "#lib/api/ppm.js";
  import {
    EXCERPT_MIN,
    POINTS_MAX,
    PROPOSALS_MAX,
    TASK_TITLE_MAX,
    allReady,
    droppedKey,
    dueWindow,
    meetingDay,
    meetingRefusalKey,
    openActions,
    proposalFor,
    proposalsBody,
    type ProposalForm,
  } from "#lib/meeting-view.js";

  let { pid, meeting, onMade }: { pid: string; meeting: Meeting; onMade: () => Promise<void> } = $props();

  const ppm = PpmClient.withFetch();
  const actions = $derived(openActions(meeting));
  const dueRange = $derived(dueWindow(meetingDay(meeting)));

  let rows = $state<ProposalForm[]>([]);
  let preview = $state<TaskProposalPreview | null>(null);
  /** The body the preview judged, so a change after it is seen; and the outcomes it was for, in order. */
  let previewed = $state("");
  let previewedOutcomes = $state<number[]>([]);
  let key = $state("");
  let problems = $state<string[]>([]);
  let error = $state<string | null>(null);
  let notice = $state("");
  let busy = $state(false);

  // A new set of actions (another meeting, or tasks made) starts the proposals again.
  $effect(() => {
    const fresh = actions.map(proposalFor);
    rows = fresh;
    preview = null;
    previewed = "";
    key = "";
  });

  const built = $derived(proposalsBody(meeting, rows));
  const current = $derived(
    preview !== null && built !== null && "body" in built && JSON.stringify(built.body) === previewed,
  );
  const fill = { min: EXCERPT_MIN, max: TASK_TITLE_MAX, points: POINTS_MAX, count: PROPOSALS_MAX };

  function newKey(): string {
    const c = globalThis.crypto;
    return c && typeof c.randomUUID === "function" ? c.randomUUID() : `k-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 12)}`;
  }

  /** The sentence for the check of the proposal for outcome `outcome`, when there is one. */
  function verdict(outcome: number): string | null {
    if (!preview || !current) return null;
    const at = previewedOutcomes.indexOf(outcome);
    const item = preview.items.find((i) => i.index === at);
    if (!item) return null;
    if (item.status === "ready")
      return item.assignee ? t("p.meetings.ready_for", { who: item.assignee }) : t("p.meetings.ready");
    return t(droppedKey(item.reason));
  }

  async function check(e: SubmitEvent) {
    e.preventDefault();
    error = null;
    notice = "";
    const r = proposalsBody(meeting, rows);
    if (r === null) {
      problems = [];
      notice = t("p.meetings.none_chosen");
      return;
    }
    if ("problems" in r) {
      problems = r.problems.map((p) =>
        p.outcome === 0 ? t(p.key, fill) : t("p.meetings.problem_on_line", { line: t("p.meetings.outcome_n", { n: p.outcome }), problem: t(p.key, fill) }),
      );
      return;
    }
    problems = [];
    busy = true;
    try {
      preview = await ppm.previewMeetingTasks(pid, meeting.pid, r.body);
      previewed = JSON.stringify(r.body);
      previewedOutcomes = r.body.items.map((i) => i.outcome);
      key = newKey();
      notice = tn("p.meetings.checked", preview.counts.ready, { n: preview.counts.ready, dropped: preview.counts.dropped });
    } catch (err) {
      const k = meetingRefusalKey("preview", err);
      error = k ? t(k, fill) : errorText(err, "p.meetings.could_not_check");
    } finally {
      busy = false;
    }
  }

  async function create() {
    if (!preview || !current || !allReady(preview)) return;
    const body = JSON.parse(previewed) as { items: Array<{ outcome: number }> };
    const n = body.items.length;
    if (!(await askConfirm(tn("p.meetings.confirm_tasks", n, { n, title: meeting.title })))) return;
    error = null;
    busy = true;
    try {
      const made = await ppm.commitMeetingTasks(pid, meeting.pid, JSON.parse(previewed), key);
      notice = tn("p.meetings.tasks_made", made.created.length, { n: made.created.length });
      key = "";
      await onMade();
    } catch (err) {
      const k = meetingRefusalKey("commit", err);
      error = k ? t(k) : errorText(err, "p.meetings.could_not_create");
      preview = null;
    } finally {
      busy = false;
    }
  }
</script>

{#if actions.length > 0}
  <section class="tasks" aria-labelledby={`meeting-tasks-h-${meeting.pid}`}>
    <h3 id={`meeting-tasks-h-${meeting.pid}`}>{t("p.meetings.tasks_heading")}</h3>
    <p class="small" data-testid="tasks-rules">
      {t("p.meetings.tasks_says", { min: EXCERPT_MIN, max: TASK_TITLE_MAX, points: POINTS_MAX, count: PROPOSALS_MAX, first: formatDate(dueRange.first), last: formatDate(dueRange.last) })}
    </p>
    {#if error}<p class="banner error" role="alert">{error}</p>{/if}
    {#if problems.length > 0}<ul class="banner error" role="alert">{#each problems as p (p)}<li>{p}</li>{/each}</ul>{/if}
    <p class="small" role="status">{notice}</p>
    <form class="stack" novalidate onsubmit={check}>
      {#each rows as r (r.outcome)}
        {@const o = meeting.outcomes.find((x) => x.index === r.outcome)}
        {@const n = r.outcome + 1}
        <fieldset class="proposal">
          <legend>{t("p.meetings.proposal_legend", { n, text: o?.text ?? "" })}</legend>
          <label class="check"><input type="checkbox" bind:checked={r.chosen} />{t("p.meetings.f_propose", { n })}</label>
          {#if r.chosen}
            <label>{t("p.meetings.f_task_title", { n, max: TASK_TITLE_MAX })}<input bind:value={r.title} /></label>
            <label>{t("p.meetings.f_excerpt", { n, min: EXCERPT_MIN })}<textarea bind:value={r.excerpt} rows="2"></textarea></label>
            <label>{t("p.meetings.f_assignee", { n })}<input bind:value={r.assignee} /></label>
            <label>{t("p.meetings.f_due", { n })}<input type="date" min={dueRange.first} max={dueRange.last} bind:value={r.due} /></label>
            <label>{t("p.meetings.f_points", { n, max: POINTS_MAX })}<input type="number" step="1" min="0" max={POINTS_MAX} inputmode="numeric" bind:value={r.points} /></label>
            {@const v = verdict(r.outcome)}
            {#if v}<p class="small" data-testid={`verdict-${n}`}>{v}</p>{/if}
          {/if}
        </fieldset>
      {/each}
      <div class="row">
        <button class="button" type="submit" disabled={busy}>{t("p.meetings.check")}</button>
        <button class="button" type="button" disabled={busy || !current || !preview || !allReady(preview)} onclick={create}>{t("p.meetings.create_tasks")}</button>
      </div>
      {#if preview && current && !allReady(preview)}<p class="small">{t("p.meetings.not_all_ready")}</p>{/if}
      {#if preview && !current}<p class="small">{t("p.meetings.check_again")}</p>{/if}
    </form>
  </section>
{/if}

<style>
  .tasks {
    margin-block-start: 1rem;
  }
  fieldset.proposal {
    display: grid;
    gap: 0.35rem;
  }
  .check {
    display: inline-flex;
    gap: 0.35rem;
    align-items: center;
    padding-block: 0.3rem;
  }
  .row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    margin-block: 0.25rem;
  }
</style>
