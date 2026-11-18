<!--
  The day's plan (`/me`, NC-1 slice 12 part h, PL-4): the signed-in person's time blocked for tasks on one local day
  (`GET /api/me/plan?day=&tz=`, in the browser's IANA zone), with the slots that clash marked in words and a shape,
  what is due that day, and the slots left out because their task can no longer be read counted. A slot is planned
  for one of the person's own open tasks (`POST /api/me/planned-slots`; the task list disabled until read; every
  rule said and checked first, every problem at once) and removed after a question (`POST …/{pid}/delete`; Cancel
  sends nothing). Rules in `src/lib/planner-view.ts`.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { errorText } from "#lib/error-text.js";
  import { t, tn, localeHref, type StringKey } from "#lib/i18n.svelte.js";
  import { formatDate, formatDateTimeRange, formatInteger } from "#lib/locale-format.js";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import { PpmClient, type DayPlan, type DayPlanSlot, type MyTask } from "#lib/api/ppm.js";
  import { todayIn } from "#lib/timesheet-view.js";
  import {
    SLOTS_MAX,
    SLOT_PROBLEM_KEY,
    clashCount,
    isDay,
    plannerRefusalKey,
    shiftDay,
    slotBody,
    type PlannerAction,
  } from "#lib/planner-view.js";

  let { tasks, tz }: { tasks: MyTask[] | null; tz: string } = $props();

  const ppm = PpmClient.withFetch();
  const today = $derived(todayIn(tz));
  // The shown day; set to today when the section opens.
  let day = $state("");
  let dayInput = $state("");
  let plan = $state<DayPlan | null>(null);
  let loadError = $state<string | null>(null);
  let form = $state({ taskPid: "", start: "", end: "" });
  let problem = $state<string | null>(null);
  let notice = $state("");
  let busy = $state(false);

  function why(action: PlannerAction, err: unknown, fallback: StringKey): string {
    const key = plannerRefusalKey(action, err);
    return key ? t(key, { max: formatInteger(SLOTS_MAX) }) : errorText(err, fallback);
  }

  async function load() {
    try {
      plan = await ppm.myDayPlan(day, tz);
      loadError = null;
    } catch (err) {
      plan = null;
      loadError = why("read", err, "p.planner.could_not_load");
    }
  }
  onMount(() => {
    day = today;
    dayInput = today;
    void load();
  });

  function goTo(next: string | null) {
    if (!next) return;
    day = next;
    dayInput = next;
    problem = null;
    notice = "";
    plan = null;
    void load();
  }
  function showTyped(e: SubmitEvent) {
    e.preventDefault();
    if (!isDay(dayInput)) {
      problem = t("p.planner.problem_day");
      notice = "";
      return;
    }
    goTo(dayInput.trim());
  }

  const taskTitle = (pid: string) => tasks?.find((x) => x.pid === pid)?.title ?? "";
  const slotTime = (s: { starts_at: string; ends_at: string }) => formatDateTimeRange(s.starts_at, s.ends_at);

  async function add(e: SubmitEvent) {
    e.preventDefault();
    notice = "";
    const r = slotBody(form, day, tz);
    if ("problems" in r) {
      problem = r.problems.map((p) => t(SLOT_PROBLEM_KEY[p])).join(" ");
      return;
    }
    problem = null;
    busy = true;
    try {
      await ppm.planSlot(r.body);
      notice = t("p.planner.planned", { title: taskTitle(r.body.task_pid), time: slotTime(r.body) });
      form = { taskPid: form.taskPid, start: "", end: "" };
    } catch (err) {
      problem = why("plan", err, "p.planner.could_not_plan");
    } finally {
      busy = false;
      await load();
    }
  }

  async function remove(s: DayPlanSlot) {
    notice = "";
    problem = null;
    const time = slotTime(s);
    if (!(await askConfirm(t("p.planner.confirm_remove", { title: s.task.title, time })))) return;
    busy = true;
    try {
      await ppm.removeSlot(s.pid);
      notice = t("p.planner.removed", { title: s.task.title, time });
    } catch (err) {
      problem = why("remove", err, "p.planner.could_not_remove");
    } finally {
      busy = false;
      await load();
    }
  }
</script>

<section class="planner" aria-labelledby="planner-h" data-testid="planner">
  <h2 id="planner-h">{t("p.planner.heading")}</h2>
  <p class="small muted">{t("p.planner.says", { tz })}</p>

  <nav class="days" aria-label={t("p.planner.day_nav")}>
    <button class="button" type="button" onclick={() => goTo(shiftDay(day, -1))}>{t("p.planner.previous_day")}</button>
    <button class="button" type="button" disabled={day === today} onclick={() => goTo(today)}>{t("p.planner.back_to_today")}</button>
    <button class="button" type="button" onclick={() => goTo(shiftDay(day, 1))}>{t("p.planner.next_day")}</button>
    <form class="inline-form" onsubmit={showTyped} novalidate>
      <label>{t("p.planner.f_day")}
        <input type="date" bind:value={dayInput} required />
      </label>
      <button class="button" type="submit">{t("p.planner.show_day")}</button>
    </form>
  </nav>

  {#if problem}<p class="banner error" role="alert">{problem}</p>{/if}
  <p class="small" role="status">{notice}</p>

  {#if day}<h3>{t("p.planner.day_heading", { day: formatDate(day, "full") })}</h3>{/if}
  {#if loadError}<p class="banner" role="alert">{loadError}</p>{/if}
  {#if plan}
    {#if plan.window.hours !== 24}
      <p class="small">{t("p.planner.day_hours", { hours: formatInteger(plan.window.hours) })}</p>
    {/if}
    {#if plan.slots.length === 0}
      <p>{t("p.planner.nothing_planned")}</p>
    {:else}
      <ul class="slots" data-testid="slots">
        {#each plan.slots as s (s.pid)}
          {@const n = clashCount(s.pid, plan.clashes)}
          <li>
            <span>{slotTime(s)}</span>
            <a href={localeHref(`/plans/${s.task.plan_pid}/tasks/${s.task.pid}`)}>{s.task.title}</a>
            {#if s.task.plan_name}<span class="small muted">{s.task.plan_name}</span>{/if}
            {#if n > 0}
              <span class="clash"><span aria-hidden="true">⚠ </span>{tn("p.planner.clashes", n, { n: formatInteger(n) })}</span>
            {/if}
            <button class="button small" type="button" disabled={busy} onclick={() => void remove(s)}
              aria-label={t("p.planner.remove_named", { title: s.task.title, time: slotTime(s) })}>{t("p.planner.remove")}</button>
          </li>
        {/each}
      </ul>
    {/if}
    {#if plan.omitted_unreadable > 0}
      <p class="small muted">{tn("p.planner.omitted", plan.omitted_unreadable, { n: formatInteger(plan.omitted_unreadable) })}</p>
    {/if}
    <h4>{t("p.planner.due_heading")}</h4>
    {#if plan.due_on_day.length === 0}
      <p class="small">{t("p.planner.nothing_due")}</p>
    {:else}
      <ul>
        {#each plan.due_on_day as d (d.pid)}
          <li><a href={localeHref(`/plans/${d.plan_pid}/tasks/${d.pid}`)}>{d.title}</a></li>
        {/each}
      </ul>
    {/if}
  {:else if !loadError}
    <p class="muted">{t("p.planner.loading")}</p>
  {/if}

  <form class="slot-form" novalidate onsubmit={add} aria-label={t("p.planner.form")}>
    <p class="small muted">{t("p.planner.form_says", { day: formatDate(day, "full") })}</p>
    <label>{t("p.planner.f_task")}
      <select bind:value={form.taskPid} disabled={!tasks}>
        <option value="">{tasks ? t("p.planner.choose_task") : t("p.planner.loading_tasks")}</option>
        {#each tasks ?? [] as task (task.pid)}
          <option value={task.pid}>{t("p.planner.task_option", { title: task.title, plan: task.plan_name })}</option>
        {/each}
      </select>
    </label>
    {#if tasks && tasks.length === 0}<p class="small">{t("p.planner.no_tasks")}</p>{/if}
    <label>{t("p.planner.f_start")}<input type="time" bind:value={form.start} /></label>
    <label>{t("p.planner.f_end")}<input type="time" bind:value={form.end} /></label>
    <div><button class="button" type="submit" disabled={busy || !tasks}>{t("p.planner.plan_time")}</button></div>
  </form>
</section>

<style>
  .planner {
    margin-block: 1.5rem;
  }
  .days,
  .inline-form {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    align-items: end;
  }
  .inline-form label,
  .slot-form label {
    display: grid;
    gap: 0.2rem;
  }
  .slots {
    list-style: none;
    padding: 0;
  }
  .slots li {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    align-items: baseline;
    padding-block: 0.3rem;
  }
  .clash {
    font-weight: 600;
  }
  .slot-form {
    display: grid;
    gap: 0.5rem;
    max-inline-size: 32rem;
    margin-block: 1rem;
  }
</style>
