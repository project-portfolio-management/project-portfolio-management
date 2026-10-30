<!--
  Gantt route (`/gantt`) — the selected portfolio's schedule (PPM-6)
  on Lily's GanttChart: dated work items as task bars, dependency edges
  as a text summary per task, the critical path marked in colour and in
  words. Read-only in v1: schedule edits stay with the schedule
  endpoints; the Gantt is a derived visualization, not a second editor.
-->
<script lang="ts">
  import { errorReason } from "#lib/error-text.js";
  import { isAbsent } from "#lib/absent.js";
  import { onMount } from "svelte";
  import GanttView from "#lib/components/GanttView.svelte";
  import { chartTasks } from "#lib/gantt-view.js";
  import { PpmClient } from "#lib/api/ppm.js";
  import { PlanRepository } from "#lib/api/plans.js";
  import type { PlanRef } from "#lib/api/types.js";
  import type { ScheduleView } from "#lib/api/ppm.js";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { formatDateRange } from "#lib/locale-format.js";

  const client = PpmClient.withFetch();

  let portfolios = $state<PlanRef[]>([]);
  let selected = $state<string>("");
  let schedule = $state<ScheduleView | null>(null);
  let mask = $state("");
  let holidays = $state<string[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);
  /** The calendar read failed for a reason other than "none", so the chart is not shaded. */
  let calendarFailed = $state(false);

  async function loadSchedule(pid: string) {
    if (!pid) return;
    loading = true;
    error = null;
    calendarFailed = false;
    try {
      schedule = await client.schedule(pid);
      // The plan's own working days shade the chart; without a calendar nothing is shaded.
      try {
        const cal = await client.planCalendar(pid);
        mask = cal.weekdays;
        holidays = cal.holidays;
      } catch (err) {
        mask = "";
        holidays = [];
        calendarFailed = !isAbsent(err);
      }
    } catch (err) {
      error = errorReason(err);
      schedule = null;
    } finally {
      loading = false;
    }
  }

  onMount(async () => {
    try {
      portfolios = await PlanRepository.withFetch().list();
      selected = portfolios[0]?.pid ?? "";
      if (selected) await loadSchedule(selected);
      else loading = false;
    } catch (err) {
      error = errorReason(err);
      loading = false;
    }
  });

  // Every item is a row; dated ones get a bar, undated ones are listed with none.
  const chart = $derived(schedule ? chartTasks(schedule) : []);
  const tasks = $derived(
    chart.flatMap((c) => (c.start && c.end ? [{ id: c.id, text: c.label, start: c.start, end: c.end }] : [])),
  );

  const undated = $derived(
    (schedule?.items ?? []).filter((i) => !i.start || !i.end),
  );
</script>

<svelte:head><title>{t("p.gantt.main_x", { t: t("ppm.nav.gantt") })}</title></svelte:head>

<h1>{t("ppm.nav.gantt")}</h1>

<p>
  <label>
    {t("detail.opened")}
    <select
      bind:value={selected}
      onchange={() => void loadSchedule(selected)}
    >
      {#each portfolios as p (p.pid)}
        <option value={p.pid}>{p.name}</option>
      {/each}
    </select>
  </label>
</p>

{#if calendarFailed}
  <p class="banner" role="note" data-testid="calendar-failed">{t("p.gantt.calendar_failed")}</p>
{/if}
{#if loading}
  <p>{t("list.loading")}</p>
{:else if error}
  <p class="banner" role="alert">{error}</p>
{:else if !schedule || tasks.length === 0}
  <p class="surface">{t("list.empty")}</p>
{:else}
  <!-- T-28k (repo tasks.md EV-1): a horizontal-scrolling timeline needs real width under a phone viewport;
       `.mobile-list` below is its read-only stand-in, switched purely by `@media (max-width: 600px)`. -->
  <div class="gantt-wrap">
    <GanttView tasks={chart} label={t("ppm.nav.gantt")} {mask} {holidays} />
  </div>
  <ul class="mobile-list">
    {#each tasks as task (task.id)}
      <li>
        <a href={localeHref(`/plans/${task.id}`)}>{task.text}</a>
        <span class="dates"
          >{formatDateRange(task.start, task.end)}</span
        >
      </li>
    {/each}
  </ul>
  {#if undated.length > 0}
    <p class="muted small">
      {t("list.empty")}:
      {#each undated as item (item.pid)}
        <span class="chip">{item.name}</span>
      {/each}
    </p>
  {/if}
{/if}

<style>
  /* T-28k: read-only stand-in for the Gantt under a phone viewport. */
  .mobile-list {
    display: none;
  }

  @media (max-width: 600px) {
    .gantt-wrap {
      display: none;
    }
    .mobile-list {
      display: block;
      list-style: none;
      margin: 0;
      padding: 0;
    }
    .mobile-list li {
      display: flex;
      flex-direction: column;
      gap: 0.15rem;
      padding: 0.75rem 0.25rem;
      border-bottom: 1px solid var(--mxi-color-border);
    }
    .mobile-list a {
      color: inherit;
      text-decoration: none;
      font-weight: 600;
    }
    .mobile-list .dates {
      font-size: 0.85em;
      opacity: 0.75;
    }
  }
</style>
