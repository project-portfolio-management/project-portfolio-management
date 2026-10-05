<!--
  Critical path (`/plans/[pid]/critical-path`, SC-1 / SC-2 / PJ-12): the plan's
  tasks with their float. Three views of one result — a Gantt with the float
  drawn as a tail, a precedence network, and a table — plus the critical chains,
  what could not be scheduled (and why), and an editor for a task's start,
  duration, constraint and deadline that answers with **what moved**.
  English-first, like the other recent views.
-->
<script lang="ts">
  import { taskEstimateProblems, taskRangeText } from "#lib/simulation-inputs.js";
  import { localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { ApiError } from "#lib/api/client.js";
  import {
    PpmClient,
    type Cpm,
    type CpmNode,
    type ScheduleBaseline,
    type ScheduleMoved,
    type ScheduleVariance,
  } from "#lib/api/ppm.js";
  import CpmGantt from "#lib/components/CpmGantt.svelte";
  import CpmTable from "#lib/components/CpmTable.svelte";
  import NetworkDiagram from "#lib/components/NetworkDiagram.svelte";
  import VarianceTable from "#lib/components/VarianceTable.svelte";
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

  // Baselines (SC-3): append-only; variance is against a chosen version.
  let baselines = $state<ScheduleBaseline[]>([]);
  let chosen = $state<number | null>(null);
  let variance = $state<ScheduleVariance | null>(null);
  let baselineReason = $state("");
  let showBaseline = $state(true);

  // The editor's fields (an absent field clears it, so the form is always complete).
  let start = $state("");
  let duration = $state("");
  let optimistic = $state("");
  let pessimistic = $state("");
  let estimateProblem = $state<string[]>([]);
  let estimateSaved = $state(false);
  let ckind = $state("");
  let cdate = $state("");
  let deadline = $state("");
  let reason = $state("");

  async function loadBaselines() {
    try {
      baselines = await ppm.scheduleBaselines(pid);
      variance = await ppm.scheduleVariance(pid, chosen ?? undefined);
      chosen = variance.baseline?.version ?? null;
    } catch (err) {
      error = err instanceof Error ? err.message : "Could not load the baselines";
    }
  }

  async function takeBaseline(event: SubmitEvent) {
    event.preventDefault();
    error = null;
    try {
      await ppm.takeScheduleBaseline(pid, baselineReason);
      baselineReason = "";
      chosen = null;
      await loadBaselines();
    } catch (err) {
      error = err instanceof Error ? err.message : "Could not take the baseline";
    }
  }

  async function load() {
    try {
      cpm = await ppm.planCpm(pid, near);
      error = null;
      await loadBaselines();
    } catch (err) {
      error = err instanceof Error ? err.message : "Could not load the schedule";
    }
  }
  onMount(() => void load());

  // Where each task was baselined to sit, for the outline under its Gantt bar.
  const ghosts = $derived(
    new Map(
      (showBaseline ? (variance?.variance?.rows ?? []) : [])
        .filter((r) => r.baseline_start && r.baseline_finish)
        .map((r) => [r.task_pid, { start: r.baseline_start, finish: r.baseline_finish }] as const),
    ),
  );

  const titleOf = $derived(new Map((cpm?.nodes ?? []).map((n) => [n.pid, n.title ?? "(untitled)"])));
  const node = $derived<CpmNode | null>(cpm?.nodes.find((n) => n.pid === selected) ?? null);
  const unscheduledSelected = $derived(cpm?.unscheduled.find((u) => u.pid === selected) ?? null);

  function select(p: string) {
    selected = p;
    result = null;
    const n = cpm?.nodes.find((x) => x.pid === p);
    start = n?.planned_start ?? "";
    duration = n?.duration_days === null || n?.duration_days === undefined ? "" : String(n.duration_days);
    optimistic = n?.duration_optimistic_days === null || n?.duration_optimistic_days === undefined ? "" : String(n.duration_optimistic_days);
    pessimistic = n?.duration_pessimistic_days === null || n?.duration_pessimistic_days === undefined ? "" : String(n.duration_pessimistic_days);
    estimateProblem = [];
    estimateSaved = false;
    ckind = n?.constraint ?? "";
    cdate = n?.constraint_date ?? "";
    deadline = n?.deadline ?? "";
    reason = "";
  }

  const hard = $derived(ckind === "mso" || ckind === "mfo");

  async function saveEstimate(event: SubmitEvent) {
    event.preventDefault();
    if (!selected) return;
    estimateSaved = false;
    estimateProblem = taskEstimateProblems(optimistic, pessimistic, node?.duration_days ?? null);
    if (estimateProblem.length > 0) return;
    try {
      await ppm.setTaskEstimate(pid, selected, {
        optimistic_days: optimistic === "" ? null : Number(optimistic),
        pessimistic_days: pessimistic === "" ? null : Number(pessimistic),
      });
      estimateSaved = true;
      await load();
    } catch (err) {
      error = err instanceof ApiError || err instanceof Error ? err.message : "Could not save the estimate";
    }
  }

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

<p class="small"><a href={localeHref(`/plans/${pid}`)}>← Plan</a> · <a href={localeHref(`/plans/${pid}/calendar`)}>Working calendar</a></p>
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
    <CpmGantt nodes={cpm.nodes} baseline={ghosts} {selected} onselect={select} />
  {:else if view === "network"}
    <NetworkDiagram nodes={cpm.nodes} edges={cpm.edges} chains={cpm.critical_chains} {selected} onselect={select} />
  {:else}
    <CpmTable nodes={cpm.nodes} />
  {/if}

  <section aria-labelledby="bl-h" class="stack">
    <h2 id="bl-h">Baselines</h2>
    <p class="small muted">
      A baseline is a numbered snapshot of the schedule, kept for good: re-baselining makes the next version and never replaces one.
    </p>
    <form onsubmit={takeBaseline} class="row">
      <label>Why a baseline now? <input type="text" bind:value={baselineReason} required maxlength="500" /></label>
      <button type="submit">Take baseline</button>
    </form>
    {#if baselines.length > 0}
      <label>
        Compare with
        <select bind:value={chosen} onchange={() => void loadBaselines()}>
          {#each baselines as b (b.pid)}
            <option value={b.version}>Version {b.version}: {b.reason} ({b.created_at.slice(0, 10)})</option>
          {/each}
        </select>
      </label>
      <label><input type="checkbox" bind:checked={showBaseline} /> Show the baseline under each bar</label>
    {/if}
    {#if variance?.variance}
      <VarianceTable variance={variance.variance} />
    {:else if variance}
      <p class="muted">No baseline yet, so nothing can be said about slip.</p>
    {/if}
  </section>

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
      <form onsubmit={saveEstimate} class="stack" aria-labelledby="est-h">
        <h3 id="est-h">Range for schedule risk</h3>
        <p class="small muted">{taskRangeText(node?.duration_optimistic_days, node?.duration_days ?? null, node?.duration_pessimistic_days)}. The duration above is the likely value.</p>
        <label>Optimistic (working days) <input type="number" min="0" max="3650" step="1" bind:value={optimistic} /></label>
        <label>Pessimistic (working days) <input type="number" min="0" max="3650" step="1" bind:value={pessimistic} /></label>
        <button type="submit">Save range</button>
        {#each estimateProblem as p (p)}<p class="banner" role="alert">{p}</p>{/each}
        {#if estimateSaved}<p role="status" class="small">Range saved.</p>{/if}
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
  .row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem 1rem;
    align-items: end;
  }
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
