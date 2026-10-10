<!--
  Per-item task board (`/{collection}/{pid}/board`) — the spec-§13
  operational Kanban: five status columns, drag = the PATCH board move
  (the service stamps status_changed_at / first done_at, so flow data
  stays true), plus sprint create/select with the honest burndown
  (real completions only — the server's derivation note is shown) and
  the last-24h standup digest. English-first, like the other PPM views.
-->
<script lang="ts">
  import { page } from "$app/state";
  import { Kanban, Willow, getCardShape } from "@svar-ui/svelte-kanban";
  import type { KanbanInstanceApi } from "@svar-ui/svelte-kanban";
  import {
    PpmClient,
    type Burndown,
    type Sprint,
    type SprintNote,
    type Standup,
    type Task,
    type Velocity,
  } from "#lib/api/ppm.js";
  import LilyKanbanTable from "#lib/components/LilyKanbanTable.svelte";
  import { t } from "#lib/i18n.svelte.js";
  import { calendarDays } from "#lib/units.js";
  const pid = page.params.pid ?? "";
  const ppm = PpmClient.withFetch();

  let tasks = $state<Task[] | null>(null);
  let sprints = $state<Sprint[]>([]);
  let selectedSprint = $state("");
  let burndown = $state<Burndown | null>(null);
  let standup = $state<Standup | null>(null);
  let velocity = $state<Velocity | null>(null);
  let notes = $state<SprintNote[]>([]);
  let noteBody = $state("");
  let noteCategory = $state("action");
  let points = $state("");
  let view = $state<"board" | "table">("board");
  let error = $state<string | null>(null);
  let title = $state("");
  let sprintName = $state("");
  let sprintStart = $state("");
  let sprintEnd = $state("");

  async function load() {
    try {
      tasks = (await ppm.listTasks(pid)).tasks;
      sprints = await ppm.listSprints(pid);
      standup = await ppm.standup(pid);
      velocity = await ppm.velocity(pid);
      // Default to the latest sprint so the burndown shows up unprompted.
      if (!selectedSprint && sprints.length > 0) {
        selectedSprint = sprints[0]?.pid ?? "";
      }
      if (selectedSprint) {
        burndown = await ppm.burndown(pid, selectedSprint);
        notes = await ppm.listNotes(pid, selectedSprint);
      }
    } catch (err) {
      error = err instanceof Error ? err.message : t("ppm.common.loadFailed");
    }
  }
  $effect(() => {
    void load();
  });

  async function act(action: () => Promise<unknown>) {
    error = null;
    try {
      await action();
      await load();
    } catch (err) {
      error = err instanceof Error ? err.message : t("ppm.common.actionFailed");
    }
  }

  const COLUMNS = [
    { id: "todo", get label() { return t("p.plans.pid.board.todo"); }, addCard: false },
    { id: "in_progress", get label() { return t("p.plans.pid.board.in_progress"); }, addCard: false },
    { id: "in_review", get label() { return t("p.plans.pid.board.in_review"); }, addCard: false },
    { id: "done", get label() { return t("p.plans.pid.onboarding.done"); }, addCard: false },
    { id: "blocked", get label() { return t("p.plans.pid.flow.blocked"); }, addCard: false },
  ];

  const cards = $derived(
    (tasks ?? []).map((task) => ({
      id: task.pid,
      label: task.title,
      description:
        (task.assignee_ref ?? t("p.plans.pid.board.unassigned")) +
        (task.points !== null && task.points !== undefined ? t("p.plans.pid.board.pt", { points: task.points }) : "") +
        (task.blocked_days !== null ? t("p.plans.pid.board.blocked_for", { days: calendarDays(task.blocked_days) }) : ""),
      status: task.status,
    })),
  );

  // Drag = the PATCH board move; the service owns the stamps and the
  // reload restores the stored truth on refusal.
  function init(api: KanbanInstanceApi) {
    api.on("move-card", (raw) => {
      const ev = raw as { id: string | number; column?: string | number };
      if (!ev.column) return;
      void act(() => ppm.moveTask(pid, String(ev.id), String(ev.column)));
    });
  }

  async function selectSprint(value: string) {
    selectedSprint = value;
    burndown = null;
    if (value) {
      try {
        burndown = await ppm.burndown(pid, value);
      } catch (err) {
        error = err instanceof Error ? err.message : t("ppm.common.loadFailed");
      }
    }
  }
</script>

<svelte:head><title>{t("p.plans.pid.board.ppm", { t: t("ppm.common.board") })}</title></svelte:head>

<h1>{t("ppm.common.board")}</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

<form
  class="row"
  onsubmit={(event) => {
    event.preventDefault();
    if (!title.trim()) return;
    const body: Record<string, unknown> = { title: title.trim() };
    if (selectedSprint) body["sprint_pid"] = selectedSprint;
    const parsedPoints = Number.parseInt(points, 10);
    if (!Number.isNaN(parsedPoints)) body["points"] = parsedPoints;
    title = "";
    points = "";
    void act(() => ppm.createTask(pid, body));
  }}
>
  <input bind:value={title} placeholder={t("p.plans.pid.board.new_task_title")} aria-label={t("p.plans.pid.board.new_task_title")} />
  <input
    bind:value={points}
    placeholder={t("p.plans.pid.board.pts")}
    aria-label={t("p.plans.pid.board.story_points")}
    class="points"
    inputmode="numeric"
  />
  <button class="button primary" type="submit">{t("p.plans.pid.board.add_task")}</button>
  <label>
    {t("p.plans.pid.board.sprint")}
    <select
      value={selectedSprint}
      onchange={(event) => void selectSprint(event.currentTarget.value)}
    >
      <option value="">{t("p.plans.pid.board.none")}</option>
      {#each sprints as sprint (sprint.pid)}
        <option value={sprint.pid}>{sprint.name}</option>
      {/each}
    </select>
  </label>
</form>

{#if tasks !== null}
  <div class="view-toggle" role="group" aria-label={t("p.plans.pid.board.board_view")}>
    <button type="button" aria-pressed={view === "board"} onclick={() => (view = "board")}>{t("p.plans.pid.board.board")}</button>
    <button type="button" aria-pressed={view === "table"} onclick={() => (view = "table")}>
      {t("p.plans.pid.board.accessible_table")}
    </button>
  </div>
  {#if view === "table"}
    <LilyKanbanTable
      cards={tasks.map((task) => ({ pid: task.pid, title: task.title, status: task.status }))}
      columns={COLUMNS.map(({ id, label }) => ({ id, label }))}
      label={t("ppm.common.board")}
      onmove={(taskPid, status) => void act(() => ppm.moveTask(pid, taskPid, status))}
    />
  {:else}
    <div class="board-wrap" data-testid="task-board">
      <Willow fonts={false}>
        <Kanban
          {cards}
          columns={COLUMNS}
          columnAccessor="status"
          card={{ ...getCardShape(), menu: false }}
          {init}
        />
      </Willow>
    </div>
  {/if}
{/if}

<h2>{t("p.plans.pid.board.sprints")}</h2>
<form
  class="row"
  onsubmit={(event) => {
    event.preventDefault();
    if (!sprintName.trim() || !sprintStart || !sprintEnd) return;
    const body = { name: sprintName.trim(), starts_on: sprintStart, ends_on: sprintEnd };
    sprintName = "";
    void act(() => ppm.createSprint(pid, body));
  }}
>
  <input bind:value={sprintName} placeholder={t("p.plans.pid.board.sprint_name")} aria-label={t("p.plans.pid.board.sprint_name")} />
  <input type="date" bind:value={sprintStart} aria-label={t("p.plans.pid.board.sprint_start")} />
  <input type="date" bind:value={sprintEnd} aria-label={t("p.plans.pid.board.sprint_end")} />
  <button type="submit">{t("p.plans.pid.board.add_sprint")}</button>
</form>

{#if burndown}
  <h2>{t("p.plans.pid.board.burndown", { name: burndown.sprint.name })}</h2>
  <p class="muted">{burndown.derivation}</p>
  <table data-testid="burndown">
    <thead><tr><th>{t("p.plans.pid.board.date")}</th><th>{t("p.plans.pid.board.remaining_of", { total_tasks: burndown.total_tasks })}</th></tr></thead>
    <tbody>
      {#each burndown.points as point (point.date)}
        <tr><td>{point.date}</td><td>{point.remaining}</td></tr>
      {/each}
    </tbody>
  </table>
{/if}

{#if standup}
  <h2>{t("p.plans.pid.board.standup_last_24h")}</h2>
  <div data-testid="standup">
    <p>
      {t("p.plans.pid.board.created_moves_blocked_now", { length: standup.tasks_created.length, length2: standup.tasks_moved.length, length3: standup.blocked_now.length })}
    </p>
    {#if standup.blocked_now.length > 0}
      <ul>
        {#each standup.blocked_now as task (task.pid)}
          <li>{task.title} <span class="muted">{(task.blocked_days === 1 ? t("p.plans.pid.board.blocked_calendar_day_one", { blocked_days: task.blocked_days }) : t("p.plans.pid.board.blocked_calendar_day_other", { blocked_days: task.blocked_days }))}</span></li>
        {/each}
      </ul>
    {/if}
  </div>
{/if}

{#if selectedSprint}
  <h2>{t("p.plans.pid.board.retro_feedback_notes")}</h2>
  <form
    class="row"
    onsubmit={(event) => {
      event.preventDefault();
      if (!noteBody.trim()) return;
      const body = { category: noteCategory, body: noteBody.trim() };
      noteBody = "";
      void act(() => ppm.createNote(pid, selectedSprint, body));
    }}
  >
    <select bind:value={noteCategory} aria-label={t("p.plans.pid.board.note_category")}>
      <option value="went_well">{t("p.plans.pid.board.went_well")}</option>
      <option value="improve">{t("p.plans.pid.board.improve")}</option>
      <option value="action">{t("p.plans.pid.board.action")}</option>
      <option value="feedback">{t("p.plans.pid.board.feedback")}</option>
    </select>
    <input bind:value={noteBody} placeholder={t("p.plans.pid.board.note")} aria-label={t("p.plans.pid.board.note_body")} />
    <button type="submit">{t("p.plans.pid.board.add_note")}</button>
  </form>
  <ul data-testid="sprint-notes">
    {#each notes as note (note.pid)}
      <li>
        <strong>{note.category}</strong>: {note.body}
        {#if note.task_pid}
          <span class="muted">{t("p.plans.pid.board.task_created")}</span>
        {:else if note.category === "action" || note.category === "feedback"}
          <button
            onclick={() =>
              void act(() => ppm.convertNote(pid, selectedSprint, note.pid))}
          >
            {t("p.plans.pid.board.convert_to_task")}
          </button>
        {/if}
      </li>
    {:else}
      <li class="muted">{t("p.plans.pid.board.no_notes_for_this_sprint")}</li>
    {/each}
  </ul>
{/if}

{#if velocity && velocity.sprints.length > 0}
  <h2>{t("p.plans.pid.board.velocity")}</h2>
  <p class="muted">{velocity.note}</p>
  <table data-testid="velocity">
    <thead><tr><th>{t("p.plans.pid.board.sprint")}</th><th>{t("p.plans.pid.board.tasks_done")}</th><th>{t("p.plans.pid.board.points_done")}</th><th>{t("p.plans.pid.board.unpointed")}</th></tr></thead>
    <tbody>
      {#each velocity.sprints as row (row.sprint.pid)}
        <tr>
          <td>{row.sprint.name}</td>
          <td>{row.tasks_done}</td>
          <td>{row.points_done}</td>
          <td>{row.unpointed_done}</td>
        </tr>
      {/each}
    </tbody>
  </table>
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
  .row { display: flex; gap: 0.75rem; align-items: end; flex-wrap: wrap; margin-bottom: 1rem; }
  .board-wrap { height: 480px; overflow-x: auto; }
  input.points { width: 4rem; }
</style>
