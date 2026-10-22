<!--
  Work breakdown (`/plans/[pid]/wbs`): the plan's tasks as a tree with generated
  codes (1, 1.2, 1.2.3). A code is for reading only: it changes when a sibling is
  added or a task moves, so every link here goes by the task's own id. The tree is
  also a table (the same rows, in the same order); a very large plan shows the
  table alone. A task under a parent you may not see sits under "Restricted item".
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { isAbsent } from "#lib/absent.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { PpmClient, type Task } from "#lib/api/ppm.js";
  import { WBS_TREE_LIMIT, buildWbs, siblingMove } from "#lib/wbs.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");
  let tasks = $state<Task[] | null>(null);
  let error = $state<string | null>(null);
  /** Links per task; empty when they could not be loaded, and then the page says so. */
  let linkCounts = $state<Record<string, number>>({});
  let linkCountsFailed = $state(false);

  onMount(async () => {
    try {
      tasks = (await ppm.listTasks(pid)).tasks;
    } catch (err) {
      error = errorText(err, "p.plans.pid.wbs.could_not_load_the_tasks");
    }
    try {
      linkCounts = (await ppm.taskLinkCounts(pid)).counts;
    } catch (err) {
      linkCounts = {};
      linkCountsFailed = !isAbsent(err);
    }
  });

  let notice = $state("");

  /** Move a task one place among its siblings, then reload so the codes are the service's order. */
  async function move(taskPid: string, title: string, direction: "up" | "down") {
    const change = siblingMove(tasks ?? [], taskPid, direction);
    if (!change) return;
    try {
      await ppm.setTaskOrder(pid, change.parent_task_pid, change.order);
      tasks = (await ppm.listTasks(pid)).tasks;
      notice = t("p.plans.pid.wbs.moved", { title });
    } catch (err) {
      notice = t("p.plans.pid.wbs.could_not_move", { reason: err instanceof Error ? err.message : "" });
    }
  }

  const rows = $derived(buildWbs(tasks ?? []).map((r) => ({ ...r, links: r.restricted ? 0 : (linkCounts[r.key] ?? 0) })));
  const tooBig = $derived(rows.length > WBS_TREE_LIMIT);

  const columns: DataGridColumn[] = $derived([
    { id: "code", header: t("p.plans.pid.wbs.code"), rowHeader: true },
    { id: "title", header: t("p.plans.pid.wbs.task") },
    { id: "depth", header: t("p.plans.pid.wbs.level"), align: "end", format: (v) => String(Number(v) + 1) },
    { id: "children", header: t("p.plans.pid.wbs.sub_tasks"), align: "end" },
    { id: "links", header: t("p.plans.pid.wbs.links"), align: "end" },
  ]);
</script>

<svelte:head><title>{t("p.plans.pid.wbs.work_breakdown_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plans.pid.wbs.plan")}</a></p>
<h1>{t("p.plans.pid.wbs.work_breakdown")}</h1>
<p class="lede">{t("p.plans.pid.wbs.the_plan_s_tasks_each")}</p>
{#if error}<p class="banner" role="alert">{error}</p>{/if}
{#if linkCountsFailed}<p class="banner" role="note" data-testid="link-counts-failed">{t("p.plans.pid.wbs.link_counts_failed")}</p>{/if}

{#if tasks}
  <p class="small" role="status">{notice}</p>
  {#if rows.length === 0}
    <p>{t("p.plans.pid.wbs.no_tasks_yet")}</p>
  {:else}
    {#if tooBig}
      <p class="banner" role="note">{t("p.plans.pid.wbs.tasks_is_too_many_to", { length: rows.length })}</p>
    {:else}
      <ul class="wbs" aria-label={t("p.plans.pid.wbs.work_breakdown_tree")}>
        {#each rows as r (r.key)}
          <li style:margin-inline-start="{r.depth * 1.5}rem">
            <span class="code">{r.code}</span>
            {#if r.restricted}<em>{r.title}</em>{:else}<a href={localeHref(`/plans/${pid}/tasks/${r.key}`)}>{r.title}</a>{/if}
            {#if r.children > 0}<span class="small muted"> {(r.children === 1 ? t("p.plans.pid.wbs.sub_task_one", { children: r.children }) : t("p.plans.pid.wbs.sub_task_other", { children: r.children }))}</span>{/if}
            {#if r.links > 0}<span class="small muted"> {(r.links === 1 ? t("p.plans.pid.wbs.link_one", { links: r.links }) : t("p.plans.pid.wbs.link_other", { links: r.links }))}</span>{/if}
            {#if !r.restricted}
              <button type="button" class="move" disabled={!siblingMove(tasks ?? [], r.key, "up")} aria-label={t("p.plans.pid.wbs.move_up", { title: r.title })} onclick={() => move(r.key, r.title, "up")}><span aria-hidden="true">↑</span></button>
              <button type="button" class="move" disabled={!siblingMove(tasks ?? [], r.key, "down")} aria-label={t("p.plans.pid.wbs.move_down", { title: r.title })} onclick={() => move(r.key, r.title, "down")}><span aria-hidden="true">↓</span></button>
            {/if}
          </li>
        {/each}
      </ul>
    {/if}
    <DataTable label={t("p.plans.pid.wbs.work_breakdown_as_a_table")} {columns} rows={rows} rowId={(r) => String(r.key)} sortable={false} />
  {/if}
{/if}

<style>
  .wbs { list-style: none; padding: 0; }
  .move { margin-inline-start: 0.25rem; min-inline-size: 2rem; min-block-size: 2rem; }
  .code { font-variant-numeric: tabular-nums; font-weight: 600; margin-inline-end: 0.5rem; }
</style>
