<!--
  My kanban board (`/kanban`): the signed-in person's open tasks across every plan, in columns by status, on
  Lily's KanbanBoard. A card moves by drag or by the card's "Move to" menu (never drag-only); a move PATCHes the
  task's status on its own plan. Open tasks only, so a task moved to Done stays on the board for this visit and is
  gone on the next load.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { ApiError } from "#lib/api/client.js";
  import KanbanBoard from "@lilydesignsystem/svelte-kanban-board";
  import type { KanbanCard, KanbanColumn } from "@lilydesignsystem/svelte-kanban-board";
  import { t, tDynamic } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient, MY_WORK_BUCKETS, type MyTask } from "#lib/api/ppm.js";

  const STATUSES = ["todo", "in_progress", "in_review", "blocked", "done"] as const;
  const ppm = PpmClient.withFetch();

  let tasks = $state<MyTask[] | null>(null);
  let omitted = $state(0);
  let error = $state<string | null>(null);
  let notice = $state<string | null>(null);

  async function load() {
    try {
      const tz = Intl.DateTimeFormat().resolvedOptions().timeZone || "UTC";
      const work = await ppm.myTasks(tz);
      const seen = new Set<string>();
      tasks = MY_WORK_BUCKETS.flatMap((b) => work.buckets[b] ?? []).filter((x) => !seen.has(x.pid) && seen.add(x.pid));
      omitted = work.omitted_unreadable;
      error = null;
    } catch (err) {
      error = err instanceof ApiError && err.status === 401 ? t("p.kanban.sign_in") : errorText(err, "p.kanban.could_not_load");
    }
  }
  onMount(() => void load());

  const columns = $derived<KanbanColumn[]>(STATUSES.map((s) => ({ id: s, title: tDynamic(`p.kanban.${s}`) })));
  const cards = $derived<KanbanCard[]>(
    (tasks ?? []).map((x) => ({
      id: x.pid,
      columnId: (STATUSES as readonly string[]).includes(x.status) ? x.status : "todo",
      title: [x.title, x.plan_name, x.due_date ? t("p.kanban.due", { date: x.due_date.slice(0, 10) }) : null, x.points != null ? t("p.kanban.points", { n: x.points }) : null]
        .filter(Boolean)
        .join(" · "),
    })),
  );

  async function move(cardId: string, toColumn: string) {
    const task = tasks?.find((x) => x.pid === cardId);
    if (!task || task.status === toColumn) return;
    const before = task.status;
    task.status = toColumn;
    notice = null;
    try {
      await ppm.moveTask(task.plan_pid, task.pid, toColumn);
    } catch {
      task.status = before;
      notice = t("p.kanban.could_not_move");
    }
  }
</script>

<svelte:head><title>{t("p.kanban.title")}</title></svelte:head>

<h1>{t("p.kanban.h1")}</h1>
<p class="lede">{t("p.kanban.lede")}</p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}
{#if notice}<p class="banner error" role="alert">{notice}</p>{/if}
{#if !tasks && !error}<p class="muted">{t("p.kanban.loading")}</p>{/if}

{#if tasks}
  {#if omitted > 0}<p class="muted">{t("p.kanban.hidden", { n: omitted })}</p>{/if}
  {#if tasks.length === 0}<p>{t("p.kanban.none")}</p>{/if}
  <div class="board">
    <KanbanBoard
      label={t("p.kanban.board_label")}
      {columns}
      {cards}
      onMove={(id: string, to: string) => void move(id, to)}
      labels={{
        cardCount: (n: number) => t("p.kanban.card_count", { n }),
        moveButton: (card: KanbanCard) => t("p.kanban.move_button", { title: card.title }),
        moveMenuLabel: t("p.kanban.move_menu"),
        moveAnnouncement: (title: string, column: string) => t("p.kanban.moved", { title, column }),
      }}
    />
  </div>
{/if}

<style>
  .board { overflow-x: auto; margin-block: 1rem; }
</style>
