<!--
  Work breakdown (`/plans/[pid]/wbs`): the plan's tasks as a tree with generated
  codes (1, 1.2, 1.2.3). A code is for reading only: it changes when a sibling is
  added or a task moves, so every link here goes by the task's own id. The tree is
  also a table (the same rows, in the same order); a very large plan shows the
  table alone. A task under a parent you may not see sits under "Restricted item".
-->
<script lang="ts">
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { PpmClient, type Task } from "#lib/api/ppm.js";
  import { WBS_TREE_LIMIT, buildWbs } from "#lib/wbs.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");
  let tasks = $state<Task[] | null>(null);
  let error = $state<string | null>(null);

  onMount(async () => {
    try {
      tasks = (await ppm.listTasks(pid)).tasks;
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.plans.pid.wbs.could_not_load_the_tasks");
    }
  });

  const rows = $derived(buildWbs(tasks ?? []));
  const tooBig = $derived(rows.length > WBS_TREE_LIMIT);
</script>

<svelte:head><title>{t("p.plans.pid.wbs.work_breakdown_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plans.pid.wbs.plan")}</a></p>
<h1>{t("p.plans.pid.wbs.work_breakdown")}</h1>
<p class="lede">{t("p.plans.pid.wbs.the_plan_s_tasks_each")}</p>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if tasks}
  {#if rows.length === 0}
    <p>{t("p.plans.pid.wbs.no_tasks_yet")}</p>
  {:else}
    {#if tooBig}
      <p class="banner" role="note">{t("p.plans.pid.wbs.tasks_is_too_many_to", { length: rows.length })}</p>
    {:else}
      <ul class="wbs" aria-label={t("p.plans.pid.wbs.work_breakdown_tree")}>
        {#each rows as r (r.key)}
          <li style={`margin-inline-start:${r.depth * 1.5}rem`}>
            <span class="code">{r.code}</span>
            {#if r.restricted}<em>{r.title}</em>{:else}<a href={localeHref(`/plans/${pid}/tasks/${r.key}`)}>{r.title}</a>{/if}
            {#if r.children > 0}<span class="small muted"> {(r.children === 1 ? t("p.plans.pid.wbs.sub_task_one", { children: r.children }) : t("p.plans.pid.wbs.sub_task_other", { children: r.children }))}</span>{/if}
          </li>
        {/each}
      </ul>
    {/if}
    <table>
      <caption class="sr-only">{t("p.plans.pid.wbs.work_breakdown_as_a_table")}</caption>
      <thead><tr><th scope="col">{t("p.plans.pid.wbs.code")}</th><th scope="col">{t("p.plans.pid.wbs.task")}</th><th scope="col">{t("p.plans.pid.wbs.level")}</th><th scope="col">{t("p.plans.pid.wbs.sub_tasks")}</th></tr></thead>
      <tbody>
        {#each rows as r (r.key)}
          <tr><th scope="row">{r.code}</th><td>{r.title}</td><td>{r.depth + 1}</td><td>{r.children}</td></tr>
        {/each}
      </tbody>
    </table>
  {/if}
{/if}

<style>
  .wbs { list-style: none; padding: 0; }
  .code { font-variant-numeric: tabular-nums; font-weight: 600; margin-inline-end: 0.5rem; }
</style>
