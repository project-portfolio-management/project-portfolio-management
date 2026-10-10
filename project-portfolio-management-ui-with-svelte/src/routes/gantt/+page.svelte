<!--
  Gantt route (`/gantt`) — the selected portfolio's schedule (PPM-6)
  in the SVAR Gantt: dated work items as task bars, dependency edges
  as links, the critical path highlighted via task CSS. Read-only in
  v1: schedule edits stay with the schedule endpoints; the Gantt is a
  derived visualization, not a second editor.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { Gantt, Willow } from "@svar-ui/svelte-gantt";
  import LilyGanttTable from "#lib/components/LilyGanttTable.svelte";
  import { PpmClient } from "#lib/api/ppm.js";
  import { PlanRepository } from "#lib/api/plans.js";
  import type { PlanRef } from "#lib/api/types.js";
  import type { ScheduleView } from "#lib/api/ppm.js";
  import { t, localeHref } from "#lib/i18n.svelte.js";

  const client = PpmClient.withFetch();

  let portfolios = $state<PlanRef[]>([]);
  let selected = $state<string>("");
  let schedule = $state<ScheduleView | null>(null);
  let view = $state<"chart" | "table">("chart");
  let loading = $state(true);
  let error = $state<string | null>(null);

  async function loadSchedule(pid: string) {
    if (!pid) return;
    loading = true;
    error = null;
    try {
      schedule = await client.schedule(pid);
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
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
      error = err instanceof Error ? err.message : String(err);
      loading = false;
    }
  });

  // Dated schedule items become task bars; the critical path gets a
  // CSS marker class. Undated items are listed below the chart rather
  // than invented onto the timeline.
  const tasks = $derived(
    (schedule?.items ?? [])
      .filter((i) => i.start && i.end)
      .map((i) => ({
        id: i.pid,
        text: i.name,
        start: new Date(i.start ?? ""),
        end: new Date(i.end ?? ""),
        type: "task",
        css: i.on_critical_path ? "ppm-critical" : "",
      })),
  );

  const links = $derived(
    (schedule?.edges ?? []).map((e) => ({
      id: e.pid,
      source: e.predecessor_pid,
      target: e.successor_pid,
      type: "e2s",
    })),
  );

  const lilyItems = $derived(
    (schedule?.items ?? []).map((i) => ({
      pid: i.pid,
      name: i.name,
      start: i.start ?? null,
      end: i.end ?? null,
      critical: i.on_critical_path,
    })),
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

{#if loading}
  <p>{t("list.loading")}</p>
{:else if error}
  <p class="banner" role="alert">{error}</p>
{:else if !schedule || tasks.length === 0}
  <p class="surface">{t("list.empty")}</p>
{:else}
  <!-- T-28k (repo tasks.md EV-1): the SVAR Gantt is unusable under a
       phone viewport (a horizontal-scrolling timeline needs real
       width); `.mobile-list` below is its read-only stand-in,
       switched purely by `@media (max-width: 600px)`. -->
  <div class="view-toggle" role="group" aria-label={t("p.gantt.schedule_view")}>
    <button type="button" aria-pressed={view === "chart"} onclick={() => (view = "chart")}>{t("p.gantt.chart")}</button>
    <button type="button" aria-pressed={view === "table"} onclick={() => (view = "table")}>
      {t("p.gantt.accessible_table")}
    </button>
  </div>
  {#if view === "table"}
    <LilyGanttTable items={lilyItems} label={t("ppm.nav.gantt")} />
  {:else}
  <div class="gantt-wrap" data-testid="ppm-gantt">
    <Willow fonts={false}>
      <Gantt {tasks} {links} readonly />
    </Willow>
  </div>
  {/if}
  <ul class="mobile-list">
    {#each tasks as task (task.id)}
      <li>
        <a href={localeHref(`/plans/${task.id}`)}>{task.text}</a>
        <span class="dates"
          >{task.start.toLocaleDateString()} – {task.end.toLocaleDateString()}</span
        >
      </li>
    {/each}
  </ul>
  {#if view === "chart" && undated.length > 0}
    <p class="muted small">
      {t("list.empty")}:
      {#each undated as item (item.pid)}
        <span class="chip">{item.name}</span>
      {/each}
    </p>
  {/if}
{/if}

<style>
  .view-toggle {
    display: flex;
    gap: 0.5rem;
    margin-block: 0.5rem;
  }
  .view-toggle button[aria-pressed="true"] {
    font-weight: 700;
    text-decoration: underline;
  }
  .gantt-wrap {
    height: 560px;
  }
  .gantt-wrap :global(.ppm-critical .wx-bar) {
    --wx-gantt-task-color: #c0392b;
  }

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
