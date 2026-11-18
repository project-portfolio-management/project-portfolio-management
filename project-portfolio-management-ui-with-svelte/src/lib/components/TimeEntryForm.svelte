<!--
  Recording time (`/me/timesheet`, NC-1 slice 12 part h, LT-11): an entry against the signed-in person's own reference
  (the `person` their week names) in a plan (`POST /api/plans/{pid}/time-entries`), opened on asking. The plans come
  from `GET /api/plans` and a plan's tasks from `GET /api/plans/{pid}/tasks` (each list disabled until read); the
  person's own entries in the chosen plan are read back from `GET /api/plans/{pid}/time-entries` (which lists
  everyone's; the page keeps the person's). Every rule is said and checked first, every problem at once; an entry
  cannot be changed or removed once recorded, so it is asked first and Cancel sends nothing. Rules in
  `src/lib/time-entry-view.ts`.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { t, type StringKey } from "#lib/i18n.svelte.js";
  import { formatDate, formatInteger } from "#lib/locale-format.js";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { rowOf } from "#lib/row-of.js";
  import { PpmClient, type Task, type TimeEntry } from "#lib/api/ppm.js";
  import { PlanRepository } from "#lib/api/plans.js";
  import type { PlanRef } from "#lib/api/types.js";
  import { durationText } from "#lib/timesheet-view.js";
  import {
    CATEGORIES,
    CATEGORY_KEY,
    ENTRY_MAX_MINUTES,
    categoryKey,
    emptyEntryForm,
    entryMinutes,
    ownEntries,
    timeEntryBody,
    timeRefusalKey,
    type TimeAction,
    type TimeEntryForm,
  } from "#lib/time-entry-view.js";

  let {
    me,
    today,
    lockedWeeks,
    onrecorded,
  }: {
    /** The caller's own person reference, from their week; `null` until it is read. */
    me: string | null;
    today: string;
    lockedWeeks: ReadonlySet<string>;
    onrecorded: (day: string) => void;
  } = $props();

  const ppm = PpmClient.withFetch();
  const planRepo = PlanRepository.withFetch();
  let open = $state(false);
  let plans = $state<PlanRef[] | null>(null);
  let tasks = $state<Task[] | null>(null);
  let entries = $state<TimeEntry[] | null>(null);
  let entriesFor = $state("");
  let form = $state<TimeEntryForm>(emptyEntryForm(""));
  let loadError = $state<string | null>(null);
  let problem = $state<string | null>(null);
  let notice = $state("");
  let busy = $state(false);

  function why(action: TimeAction, err: unknown, fallback: StringKey): string {
    const key = timeRefusalKey(action, err);
    return key ? t(key) : errorText(err, fallback);
  }

  async function openForm() {
    open = true;
    form = emptyEntryForm(today);
    problem = null;
    notice = "";
    if (plans) return;
    try {
      plans = await planRepo.list();
      loadError = null;
    } catch (err) {
      loadError = errorText(err, "p.timeentry.could_not_load_plans");
    }
  }

  const planName = (pid: string) => plans?.find((p) => p.pid === pid)?.name ?? "";
  const taskName = (pid: string | null) => (pid ? tasks?.find((x) => x.pid === pid)?.title ?? t("p.timeentry.task_unnamed") : t("p.timeentry.no_task"));

  async function loadPlan(pid: string) {
    tasks = null;
    entries = null;
    entriesFor = pid;
    form.taskPid = "";
    if (!pid) return;
    try {
      const [ts, es] = await Promise.all([ppm.listTasks(pid), ppm.timeEntries(pid)]);
      if (entriesFor !== pid) return;
      tasks = ts.tasks;
      entries = me ? ownEntries(es, me) : [];
      loadError = null;
    } catch (err) {
      loadError = why("entries", err, "p.timeentry.could_not_load_plan");
    }
  }

  async function record(e: SubmitEvent) {
    e.preventDefault();
    notice = "";
    if (!me) return;
    const r = timeEntryBody(form, me, lockedWeeks);
    if ("problems" in r) {
      problem = r.problems.map((k) => t(k, { max: formatInteger(ENTRY_MAX_MINUTES) })).join(" ");
      return;
    }
    problem = null;
    const words = {
      time: durationText(r.body.minutes),
      day: formatDate(r.body.spent_on),
      plan: planName(r.planPid),
    };
    if (!(await askConfirm(t("p.timeentry.confirm", words)))) return;
    busy = true;
    try {
      await ppm.recordTime(r.planPid, r.body);
      notice = t("p.timeentry.recorded", words);
      const day = r.body.spent_on;
      form = { ...emptyEntryForm(day), planPid: form.planPid, taskPid: form.taskPid, category: form.category, billable: form.billable };
      onrecorded(day);
      entries = me ? ownEntries(await ppm.timeEntries(r.planPid), me) : [];
    } catch (err) {
      problem = why("record", err, "p.timeentry.could_not_record");
    } finally {
      busy = false;
    }
  }

  const typed = $derived(entryMinutes(form.hours, form.minutes));
  const entryColumns: DataGridColumn[] = $derived([
    { id: "spent_on", header: t("p.timeentry.col_date"), rowHeader: true, format: (v) => formatDate(String(v)) },
    { id: "minutes", header: t("p.timeentry.col_time"), align: "end", format: (v) => durationText(Number(v)) },
    { id: "task_pid", header: t("p.timeentry.col_task"), accessor: (r) => taskName(rowOf<TimeEntry>(r).task_pid) },
    { id: "category", header: t("p.timeentry.col_category"), format: (v) => t(categoryKey(String(v))) },
    { id: "billable", header: t("p.timeentry.col_billable"), format: (v) => (v ? t("p.timeentry.billable_yes") : t("p.timeentry.billable_no")) },
    { id: "note", header: t("p.timeentry.col_note"), sortable: false, format: (v) => (v ? String(v) : "") },
  ]);
</script>

<section class="record" aria-labelledby="record-h" data-testid="record-time">
  <h2 id="record-h">{t("p.timeentry.heading")}</h2>
  <p class="small muted">{t("p.timeentry.says", { max: formatInteger(ENTRY_MAX_MINUTES) })}</p>
  {#if !open}
    <button class="button" type="button" disabled={!me} onclick={() => void openForm()}>{t("p.timeentry.open")}</button>
  {:else}
    {#if loadError}<p class="banner" role="alert">{loadError}</p>{/if}
    {#if problem}<p class="banner error" role="alert">{problem}</p>{/if}
    <p class="small" role="status">{notice}</p>
    <form class="entry-form" novalidate onsubmit={record} aria-label={t("p.timeentry.form")}>
      <label>{t("p.timeentry.f_plan")}
        <select bind:value={form.planPid} disabled={!plans} onchange={() => void loadPlan(form.planPid)}>
          <option value="">{plans ? t("p.timeentry.choose_plan") : t("p.timeentry.loading")}</option>
          {#each plans ?? [] as p (p.pid)}<option value={p.pid}>{p.name}</option>{/each}
        </select>
      </label>
      <label>{t("p.timeentry.f_task")}
        <select bind:value={form.taskPid} disabled={!tasks}>
          <option value="">{t("p.timeentry.no_task")}</option>
          {#each (tasks ?? []).filter((x) => x.status !== "done") as task (task.pid)}<option value={task.pid}>{task.title}</option>{/each}
        </select>
      </label>
      <label>{t("p.timeentry.f_date")}<input type="date" bind:value={form.spentOn} required /></label>
      <fieldset>
        <legend>{t("p.timeentry.f_duration")}</legend>
        <label class="inline">{t("p.timeentry.f_hours")}<input type="number" min="0" max="24" step="1" bind:value={form.hours} /></label>
        <label class="inline">{t("p.timeentry.f_minutes")}<input type="number" min="0" max="59" step="1" bind:value={form.minutes} /></label>
        {#if typed !== null}<p class="small" data-testid="typed-duration">{t("p.timeentry.typed", { time: durationText(typed) })}</p>{/if}
      </fieldset>
      <label>{t("p.timeentry.f_category")}
        <select bind:value={form.category}>
          {#each CATEGORIES as c (c)}<option value={c}>{t(CATEGORY_KEY[c])}</option>{/each}
        </select>
      </label>
      <label class="inline"><input type="checkbox" bind:checked={form.billable} /> {t("p.timeentry.f_billable")}</label>
      <label>{t("p.timeentry.f_note")}<textarea rows="2" bind:value={form.note}></textarea></label>
      <div><button class="button" type="submit" disabled={busy || !me}>{t("p.timeentry.record")}</button></div>
    </form>

    {#if entriesFor && entries}
      <h3>{t("p.timeentry.yours_heading", { plan: planName(entriesFor) })}</h3>
      {#if entries.length === 0}
        <p class="small">{t("p.timeentry.yours_none")}</p>
      {:else}
        <DataTable label={t("p.timeentry.yours_table", { plan: planName(entriesFor) })} columns={entryColumns} rows={entries} rowId={(r) => String(rowOf<TimeEntry>(r).pid)} testid="own-entries" />
      {/if}
    {/if}
  {/if}
</section>

<style>
  .record {
    margin-block: 1.5rem;
  }
  .entry-form {
    display: grid;
    gap: 0.5rem;
    max-inline-size: 32rem;
    margin-block: 1rem;
  }
  .entry-form label {
    display: grid;
    gap: 0.2rem;
  }
  .entry-form label.inline {
    display: inline-flex;
    gap: 0.35rem;
    align-items: center;
    margin-inline-end: 1rem;
  }
</style>
