<!--
  Critical path (`/plans/[pid]/critical-path`, SC-1 / SC-2 / PJ-12): the plan's
  tasks with their float. Three views of one result — a Gantt with the float
  drawn as a tail, a precedence network, and a table — plus the critical chains,
  what could not be scheduled (and why), and an editor for a task's start,
  duration, constraint and deadline that answers with **what moved**.
  English-first, like the other recent views.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { ApiError } from "#lib/api/client.js";
  import { PpmClient, type Cpm, type CpmNode, type ScheduleMoved } from "#lib/api/ppm.js";
  import CpmGantt from "#lib/components/CpmGantt.svelte";
  import CpmTable from "#lib/components/CpmTable.svelte";
  import NetworkDiagram from "#lib/components/NetworkDiagram.svelte";
  import { floatLabel, unscheduledReason } from "#lib/cpm-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");

  let cpm = $state<Cpm | null>(null);
  let error = $state<string | null>(null);
  let view = $state<"gantt" | "network" | "table">("gantt");
  let near = $state(5);
  let selected = $state<string | null>(null);
  let result = $state<ScheduleMoved | null>(null);
  let saving = $state(false);

  // The editor's fields (an absent field clears it, so the form is always complete).
  let start = $state("");
  let duration = $state("");
  let ckind = $state("");
  let cdate = $state("");
  let deadline = $state("");
  let reason = $state("");

  async function load() {
    try {
      cpm = await ppm.planCpm(pid, near);
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : "Could not load the schedule";
    }
  }
  onMount(() => void load());

  const titleOf = $derived(new Map((cpm?.nodes ?? []).map((n) => [n.pid, n.title ?? "(untitled)"])));
  const node = $derived<CpmNode | null>(cpm?.nodes.find((n) => n.pid === selected) ?? null);
  const unscheduledSelected = $derived(cpm?.unscheduled.find((u) => u.pid === selected) ?? null);

  function select(p: string) {
    selected = p;
    result = null;
    const n = cpm?.nodes.find((x) => x.pid === p);
    start = n?.planned_start ?? "";
    duration = n?.duration_days === null || n?.duration_days === undefined ? "" : String(n.duration_days);
    ckind = n?.constraint ?? "";
    cdate = n?.constraint_date ?? "";
    deadline = n?.deadline ?? "";
    reason = "";
  }

  const hard = $derived(ckind === "mso" || ckind === "mfo");

  async function save(event: SubmitEvent) {
    event.preventDefault();
    if (!selected) return;
    saving = true;
    error = null;
    try {
      result = await ppm.setTaskSchedule(pid, selected, {
        start_date: start || null,
        duration_days: duration === "" ? null : Number(duration),
        constraint_kind: ckind || null,
        constraint_date: ckind ? cdate || null : null,
        deadline: deadline || null,
        reason: hard ? reason : null,
      });
      await load();
    } catch (err) {
      error = err instanceof ApiError || err instanceof Error ? err.message : "Could not save";
    } finally {
      saving = false;
    }
  }
</script>

<svelte:head><title>Critical path — PPM</title></svelte:head>

<p class="small"><a href={`/plans/${pid}`}>← Plan</a> · <a href={`/plans/${pid}/calendar`}>Working calendar</a></p>
<h1>Critical path</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if cpm}
  <p class="small muted">
    {cpm.project_finish ? `Finishes ${cpm.project_finish}.` : "Nothing is scheduled yet."}
    Calendar: {cpm.calendar === "plan" ? "this plan's" : "default (every day works)"}.
    {#if cpm.critical_chains.length > 1}
      {cpm.critical_chains.length} critical chains{cpm.chains_truncated ? " (more not shown)" : ""}.
    {/if}
  </p>
  {#if cpm.calendar_conflicts.length > 0}
    <p class="banner" role="alert">
      These assignees share no working day with the plan's calendar, so the plan's calendar was used for them:
      {cpm.calendar_conflicts.join(", ")}.
    </p>
  {/if}

  <div class="toolbar">
    <div role="group" aria-label="View">
      <button type="button" aria-pressed={view === "gantt"} onclick={() => (view = "gantt")}>Gantt</button>
      <button type="button" aria-pressed={view === "network"} onclick={() => (view = "network")}>Network</button>
      <button type="button" aria-pressed={view === "table"} onclick={() => (view = "table")}>Table</button>
    </div>
    <label>
      Near-critical within
      <input type="number" min="0" max="3650" bind:value={near} onchange={() => void load()} class="short" />
      working days
    </label>
  </div>

  {#if view === "gantt"}
    <CpmGantt nodes={cpm.nodes} {selected} onselect={select} />
  {:else if view === "network"}
    <NetworkDiagram nodes={cpm.nodes} edges={cpm.edges} chains={cpm.critical_chains} {selected} onselect={select} />
  {:else}
    <CpmTable nodes={cpm.nodes} />
  {/if}

  {#if cpm.critical_chains.length > 0}
    <h2>Critical chains</h2>
    <ol class="small">
      {#each cpm.critical_chains as chain, i (i)}
        <li>{chain.map((p) => titleOf.get(p) ?? "?").join(" → ")}</li>
      {/each}
    </ol>
  {/if}

  {#if cpm.unscheduled.length > 0}
    <h2>Not scheduled</h2>
    <ul class="small">
      {#each cpm.unscheduled as u (u.pid)}
        <li>
          <button type="button" class="link" onclick={() => select(u.pid)}>{u.title ?? "(untitled)"}</button>
          {unscheduledReason(u.reason)}
        </li>
      {/each}
    </ul>
  {/if}

  {#if selected}
    <section class="surface stack" aria-labelledby="edit-h">
      <h2 id="edit-h">{node?.title ?? unscheduledSelected?.title ?? "Task"}</h2>
      {#if node}<p>{floatLabel(node)}.{node.pushed_days > 0 ? ` Its predecessors push it ${node.pushed_days} working days later than planned.` : ""}</p>{/if}
      <form onsubmit={save} class="stack">
        <label>Planned start <input type="date" bind:value={start} /></label>
        <label>Duration (working days; 0 is a milestone) <input type="number" min="0" max="3650" bind:value={duration} /></label>
        <label>
          Constraint
          <select bind:value={ckind}>
            <option value="">None (as soon as possible)</option>
            <option value="snet">Start no earlier than</option>
            <option value="snlt">Start no later than</option>
            <option value="fnet">Finish no earlier than</option>
            <option value="fnlt">Finish no later than</option>
            <option value="mso">Must start on (overrides logic)</option>
            <option value="mfo">Must finish on (overrides logic)</option>
          </select>
        </label>
        {#if ckind}<label>Constraint date <input type="date" bind:value={cdate} required /></label>{/if}
        {#if hard}
          <label>
            Reason (required: this overrides the schedule's logic)
            <input type="text" bind:value={reason} required />
          </label>
        {/if}
        <label>Deadline (a goal: it shows as negative float, it never moves the task) <input type="date" bind:value={deadline} /></label>
        <button type="submit" disabled={saving}>{saving ? "Saving…" : "Save schedule"}</button>
      </form>
      {#if result}
        <div role="status" data-testid="moved">
          {#if result.moved === null}
            <p>{result.note}</p>
          {:else if result.moved.length === 0}
            <p>Saved. Nothing else moved.</p>
          {:else}
            <p>Saved. Moved:</p>
            <ul>
              {#each result.moved as m (m.pid)}
                <li>
                  {titleOf.get(m.pid) ?? "?"}:
                  {m.moved === "both" ? "start and finish" : m.moved}
                  {m.finish_days !== 0 ? ` finish ${m.finish_days > 0 ? "+" : ""}${m.finish_days}` : ""}
                  {m.start_days !== 0 ? ` start ${m.start_days > 0 ? "+" : ""}${m.start_days}` : ""} working days
                </li>
              {/each}
            </ul>
          {/if}
        </div>
      {/if}
    </section>
  {/if}
{:else if !error}
  <p>Loading…</p>
{/if}

<style>
  .toolbar {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem 1.5rem;
    align-items: center;
    margin-block: 0.75rem;
  }
  .toolbar button[aria-pressed="true"] {
    font-weight: 700;
    text-decoration: underline;
  }
  .short {
    width: 4.5rem;
  }
  .link {
    background: none;
    border: 0;
    padding: 0;
    font: inherit;
    text-decoration: underline;
    cursor: pointer;
    color: inherit;
  }
  form label {
    display: flex;
    flex-direction: column;
    gap: 0.2rem;
    max-width: 28rem;
  }
</style>
