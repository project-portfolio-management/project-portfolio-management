<!--
  Per-item task board (`/{collection}/{pid}/board`) — the spec-§13
  operational Kanban on Lily's KanbanBoard: five status columns, a card
  moves by drag or by its "Move to" menu (the PATCH board move; the service
  stamps status_changed_at / first done_at, so flow data stays true), each
  card shows its assignee and points as typed fields and a blocked task says
  so in words, plus sprint create/select with the honest burndown
  (real completions only — the server's derivation note is shown) and
  the last-24h standup digest. English-first, like the other PPM views.
-->
<script lang="ts">
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { page } from "$app/state";
  import KanbanBoard from "@lilydesignsystem/svelte-kanban-board";
  import type { KanbanCard, KanbanColumn, KanbanField } from "@lilydesignsystem/svelte-kanban-board";
  import {
    PpmClient,
    type Burndown,
    type Sprint,
    type SprintNote,
    type Standup,
    type Task,
    type Velocity,
  } from "#lib/api/ppm.js";
  import { t } from "#lib/i18n.svelte.js";
  import { exportLabels, searchAnnouncement } from "#lib/table-labels.js";
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
  let error = $state<string | null>(null);
  let title = $state("");
  let sprintName = $state("");
  let sprintStart = $state("");
  let sprintEnd = $state("");

  /** Set when the plan has more tasks than one answer carries (a very large plan). */
  let shownOf = $state<{ shown: number; total: number } | null>(null);

  async function load() {
    try {
      const listed = await ppm.listTasks(pid);
      tasks = listed.tasks;
      shownOf = listed.truncated ? { shown: listed.tasks.length, total: listed.total ?? listed.tasks.length } : null;
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

  const STATUSES = [
    { id: "todo", key: "p.plans.pid.board.todo" },
    { id: "in_progress", key: "p.plans.pid.board.in_progress" },
    { id: "in_review", key: "p.plans.pid.board.in_review" },
    { id: "done", key: "p.plans.pid.onboarding.done" },
    { id: "blocked", key: "p.plans.pid.flow.blocked" },
  ] as const;
  const columns = $derived<KanbanColumn[]>(STATUSES.map((c) => ({ id: c.id, title: t(c.key) })));
  const fields = $derived<KanbanField[]>([
    { id: "assignee", header: t("p.plans.pid.board.assignee"), type: "text" },
    { id: "points", header: t("p.plans.pid.board.story_points"), type: "number" },
  ]);

  const cards = $derived<KanbanCard[]>(
    (tasks ?? []).map((task) => ({
      id: task.pid,
      columnId: STATUSES.some((c) => c.id === task.status) ? task.status : "todo",
      title: task.title,
      // A non-colour status: said in words, and announced with the card.
      flag:
        task.blocked_days !== null
          ? t(task.blocked_days === 1 ? "p.plans.pid.board.blocked_calendar_day_one" : "p.plans.pid.board.blocked_calendar_day_other", { blocked_days: task.blocked_days })
          : undefined,
      values: { assignee: task.assignee_ref ?? t("p.plans.pid.board.unassigned"), points: task.points ?? null },
    })),
  );

  // A move is the PATCH board move; the board shows the card as pending, and if the service refuses it puts the card back
  // (and `onMoveError` says so). The reload then restores the stored truth either way.
  async function move(cardId: string, toColumn: string) {
    await ppm.moveTask(pid, cardId, toColumn);
    void load();
  }

  const boardLabels = $derived({
    cardCount: (n: number) => t("p.kanban.card_count", { n }),
    moveButton: (card: KanbanCard) => t("p.kanban.move_button", { title: card.title }),
    moveMenuLabel: t("p.kanban.move_menu"),
    moveAnnouncement: (title: string, column: string) => t("p.kanban.moved", { title, column }),
    movePending: (title: string, column: string) => t("p.plans.pid.board.moving", { title, column }),
    moveFailed: (title: string, column: string) => t("p.plans.pid.board.move_failed", { title, column }),
    emptyColumn: () => t("p.plans.pid.board.empty_column"),
    search: t("p.plans.pid.board.search"),
    ...exportLabels(),
    searchAnnouncement,
    exportHeaders: { title: t("p.plans.pid.board.task"), column: t("p.plans.pid.board.status"), flag: t("p.plans.pid.flow.blocked") },
  });

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

  const burndownColumns = (total: number): DataGridColumn[] => [
    { id: "date", header: t("p.plans.pid.board.date"), rowHeader: true },
    { id: "remaining", header: t("p.plans.pid.board.remaining_of", { total_tasks: total }), align: "end" },
  ];
  type VRow = Velocity["sprints"][number];
  const velocityColumns: DataGridColumn[] = $derived([
    { id: "sprint", header: t("p.plans.pid.board.sprint"), rowHeader: true, accessor: (r) => (r as unknown as VRow).sprint.name },
    { id: "tasks_done", header: t("p.plans.pid.board.tasks_done"), align: "end" },
    { id: "points_done", header: t("p.plans.pid.board.points_done"), align: "end" },
    { id: "unpointed_done", header: t("p.plans.pid.board.unpointed"), align: "end" },
  ]);
</script>

<svelte:head><title>{t("p.plans.pid.board.ppm", { t: t("ppm.common.board") })}</title></svelte:head>

<h1>{t("ppm.common.board")}</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}
{#if shownOf}<p class="banner" role="status">{t("p.plans.pid.board.only_first_tasks", { shown: shownOf.shown, total: shownOf.total })}</p>{/if}

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
  <div class="board-wrap" data-testid="task-board">
    <KanbanBoard
      label={t("ppm.common.board")}
      {columns}
      {cards}
      {fields}
      labels={boardLabels}
      exportFilename="tasks"
      onMove={move}
      onMoveError={() => (error = t("p.kanban.could_not_move"))}
    />
  </div>
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
  <DataTable label={t("p.plans.pid.board.remaining_of", { total_tasks: burndown.total_tasks })} columns={burndownColumns(burndown.total_tasks)} rows={burndown.points} rowId={(r) => String(r.date)} sortable={false} testid="burndown" />
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
  <DataTable label={t("p.plans.pid.board.sprint")} columns={velocityColumns} rows={velocity.sprints} rowId={(r) => String((r as unknown as { sprint: { pid: string } }).sprint.pid)} testid="velocity" />
{/if}

<style>
  .row { display: flex; gap: 0.75rem; align-items: end; flex-wrap: wrap; margin-bottom: 1rem; }
  .board-wrap { overflow-x: auto; margin-block: 1rem; }
  input.points { width: 4rem; }
</style>
