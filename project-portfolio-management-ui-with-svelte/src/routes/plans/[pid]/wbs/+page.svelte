<!--
  Work breakdown (`/plans/[pid]/wbs`): the plan's tasks as a tree with generated
  codes (1, 1.2, 1.2.3). A code is for reading only: it changes when a sibling is
  added or a task moves, so every link here goes by the task's own id. The tree is
  also a table (the same rows, in the same order); a very large plan shows the
  table alone. A task under a parent you may not see sits under "Restricted item".
-->
<script lang="ts">
  import { localeHref } from "#lib/i18n.svelte.js";
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
      error = err instanceof Error ? err.message : "Could not load the tasks";
    }
  });

  const rows = $derived(buildWbs(tasks ?? []));
  const tooBig = $derived(rows.length > WBS_TREE_LIMIT);
</script>

<svelte:head><title>Work breakdown — PPM</title></svelte:head>

<p class="small"><a href={localeHref(`/plans/${pid}`)}>← Plan</a></p>
<h1>Work breakdown</h1>
<p class="lede">The plan's tasks, each broken into its sub-tasks. The code on each line is for reading: it changes when work is added or moved.</p>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if tasks}
  {#if rows.length === 0}
    <p>No tasks yet.</p>
  {:else}
    {#if tooBig}
      <p class="banner" role="note">{rows.length} tasks is too many to nest usefully; the table shows them all.</p>
    {:else}
      <ul class="wbs" aria-label="Work breakdown tree">
        {#each rows as r (r.key)}
          <li style={`margin-inline-start:${r.depth * 1.5}rem`}>
            <span class="code">{r.code}</span>
            {#if r.restricted}<em>{r.title}</em>{:else}<a href={localeHref(`/plans/${pid}/tasks/${r.key}`)}>{r.title}</a>{/if}
            {#if r.children > 0}<span class="small muted"> ({r.children} sub-task{r.children === 1 ? "" : "s"})</span>{/if}
          </li>
        {/each}
      </ul>
    {/if}
    <table>
      <caption class="sr-only">Work breakdown as a table</caption>
      <thead><tr><th scope="col">Code</th><th scope="col">Task</th><th scope="col">Level</th><th scope="col">Sub-tasks</th></tr></thead>
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
