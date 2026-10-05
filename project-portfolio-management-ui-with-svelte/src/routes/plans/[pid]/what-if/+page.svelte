<!--
  What-if (`/plans/[pid]/what-if`): stage schedule edits, preview what they would
  do on a copy of the plan (nothing is saved), then commit exactly that or
  discard. A preview is tied to the plan as it was; if the plan changes first,
  the commit is refused and you preview again — it is never merged. Cost is shown
  only to someone allowed to see it, and who does the work only to someone who
  may manage people.
-->
<script lang="ts">
  import { localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { ApiError } from "#lib/api/client.js";
  import {
    PpmClient,
    type CpmEdge,
    type CpmNode,
    type SandboxEdit,
    type SandboxPreview,
  } from "#lib/api/ppm.js";
  import {
    CONSTRAINTS,
    EDIT_TEXT,
    MAX_EDITS,
    buildEdit,
    costReasonText,
    deltaText,
    describeEdit,
    finishText,
    isStale,
    type EditForm,
    type EditKind,
  } from "#lib/whatif-view.js";
  import { hoursText } from "#lib/work-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");
  let nodes = $state<CpmNode[]>([]);
  let edges = $state<CpmEdge[]>([]);
  let edits = $state<SandboxEdit[]>([]);
  let preview = $state<SandboxPreview | null>(null);
  let error = $state<string | null>(null);
  let status = $state("");
  let reason = $state("");
  let busy = $state(false);
  let form = $state<EditForm>({
    kind: "duration" as EditKind, task: "", start: "", days: "", constraint: "", constraintDate: "",
    reason: "", from: "", to: "", linkKind: "fs", lag: "", link: "",
  });

  const name = (p: string) => nodes.find((n) => n.pid === p)?.title ?? p.slice(0, 8);

  async function load() {
    try {
      const c = await ppm.planCpm(pid);
      nodes = c.nodes;
      edges = c.edges;
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : "Could not load the plan";
    }
  }
  onMount(load);

  function stage(e: SubmitEvent) {
    e.preventDefault();
    const edit = buildEdit(form);
    if ("error" in edit) {
      status = edit.error;
      return;
    }
    if (edits.length >= MAX_EDITS) {
      status = `At most ${MAX_EDITS} edits at a time.`;
      return;
    }
    edits = [...edits, edit];
    preview = null; // a preview describes the edits it was made from
    status = "Edit staged. Preview to see what it does.";
  }
  function unstage(i: number) {
    edits = edits.filter((_, j) => j !== i);
    preview = null;
  }
  function discard() {
    edits = [];
    preview = null;
    status = "Discarded. Nothing was changed.";
  }
  async function run() {
    busy = true;
    try {
      preview = await ppm.sandboxPreview(pid, edits);
      status = "";
    } catch (err) {
      preview = null;
      status = err instanceof Error ? err.message : "Could not preview";
    } finally {
      busy = false;
    }
  }
  async function commit() {
    if (!preview) return;
    busy = true;
    try {
      const r = await ppm.sandboxCommit(pid, edits, preview.plan_etag, reason);
      status = `Committed: ${r.audit_entries} change${r.audit_entries === 1 ? "" : "s"} recorded.`;
      edits = [];
      preview = null;
      reason = "";
      await load();
    } catch (err) {
      if (err instanceof ApiError && isStale(err.status)) {
        preview = null;
        status = "The plan changed since this preview, so nothing was applied. Preview again, then commit.";
        await load();
      } else status = err instanceof Error ? err.message : "Could not commit";
    } finally {
      busy = false;
    }
  }
  const dates = (d: { early_start: string; early_finish: string; total_float: number; critical: boolean } | null) =>
    d ? `${d.early_start} to ${d.early_finish}` : "not scheduled";
</script>

<svelte:head><title>What-if — PPM</title></svelte:head>

<h1>What-if</h1>
<p class="small"><a href={localeHref(`/plans/${pid}`)}>← Plan</a> · <a href={localeHref(`/plans/${pid}/critical-path`)}>Critical path</a></p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}
<p class="muted small">Try changes on a copy. Nothing is saved until you commit.</p>

<form class="surface stack" onsubmit={stage} aria-labelledby="stage-h">
  <h2 id="stage-h">Stage an edit</h2>
  <label>
    Edit
    <select bind:value={form.kind}>
      {#each Object.entries(EDIT_TEXT) as [k, v]}<option value={k}>{v}</option>{/each}
    </select>
  </label>
  {#if form.kind === "add_link"}
    <label>Predecessor
      <select bind:value={form.from}><option value="">Choose…</option>{#each nodes as n (n.pid)}<option value={n.pid}>{n.title ?? n.pid}</option>{/each}</select>
    </label>
    <label>Successor
      <select bind:value={form.to}><option value="">Choose…</option>{#each nodes as n (n.pid)}<option value={n.pid}>{n.title ?? n.pid}</option>{/each}</select>
    </label>
    <label>Type
      <select bind:value={form.linkKind}>
        <option value="fs">Finish to start</option><option value="ss">Start to start</option>
        <option value="ff">Finish to finish</option><option value="sf">Start to finish</option>
      </select>
    </label>
    <label>Lag (working days) <input type="number" step="1" bind:value={form.lag} /></label>
  {:else if form.kind === "remove_link"}
    <label>Link
      <select bind:value={form.link}>
        <option value="">Choose…</option>
        {#each edges as l (l.pid)}<option value={l.pid}>{name(l.successor_pid)} follows {name(l.predecessor_pid)} ({l.kind.toUpperCase()})</option>{/each}
      </select>
    </label>
  {:else}
    <label>Task
      <select bind:value={form.task}><option value="">Choose…</option>{#each nodes as n (n.pid)}<option value={n.pid}>{n.title ?? n.pid}</option>{/each}</select>
    </label>
    {#if form.kind === "move"}
      <label>New start <input type="date" bind:value={form.start} /></label>
    {:else if form.kind === "duration"}
      <label>Working days <input type="number" min="0" max="3650" step="1" bind:value={form.days} /></label>
    {:else}
      <label>Constraint
        <select bind:value={form.constraint}>
          <option value="">None (clear it)</option>
          {#each Object.entries(CONSTRAINTS) as [k, v]}<option value={k}>{v}</option>{/each}
        </select>
      </label>
      {#if form.constraint}
        <label>Date <input type="date" bind:value={form.constraintDate} /></label>
        {#if form.constraint === "mso" || form.constraint === "mfo"}
          <label>Reason <input bind:value={form.reason} /></label>
        {/if}
      {/if}
    {/if}
  {/if}
  <button class="button" type="submit">Stage edit</button>
</form>
{#if status}<p role="status" class="small">{status}</p>{/if}

<h2>Staged edits</h2>
{#if edits.length === 0}
  <p class="muted">None yet.</p>
{:else}
  <ol data-testid="staged">
    {#each edits as e, i}
      <li>
        {describeEdit(e, name)}
        <button class="button" type="button" onclick={() => unstage(i)} aria-label={`Remove edit ${i + 1}`}>Remove</button>
      </li>
    {/each}
  </ol>
  <div class="row">
    <button class="button primary" type="button" onclick={run} disabled={busy}>Preview</button>
    <button class="button" type="button" onclick={discard} disabled={busy}>Discard</button>
  </div>
{/if}

{#if preview}
  <section class="surface stack" aria-labelledby="pv-h" data-testid="preview">
    <h2 id="pv-h">Preview</h2>
    <p>{finishText(preview)}</p>
    {#if preview.tasks.length === 0}
      <p class="muted">No task's dates or float change.</p>
    {:else}
      <table>
        <caption class="small muted">Tasks whose dates, float or critical status change</caption>
        <thead><tr><th scope="col">Task</th><th scope="col">Before</th><th scope="col">After</th><th scope="col">Float</th></tr></thead>
        <tbody>
          {#each preview.tasks as t (t.pid)}
            <tr>
              <th scope="row">{t.title}{#if t.assignee} <span class="muted small">({t.assignee})</span>{/if}</th>
              <td>{dates(t.before)}</td>
              <td>{dates(t.after)}</td>
              <td>
                {t.before?.total_float ?? "–"} → {t.after?.total_float ?? "–"}
                {#if t.after?.critical}<span class="small"> ■ critical</span>{/if}
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    {/if}
    {#if preview.critical_path.added.length || preview.critical_path.removed.length}
      <p class="small">
        {#if preview.critical_path.added.length}Joins the critical path: {preview.critical_path.added.map(name).join(", ")}. {/if}
        {#if preview.critical_path.removed.length}Leaves it: {preview.critical_path.removed.map(name).join(", ")}.{/if}
      </p>
    {/if}
    {#each preview.assignments_recomputed as r}
      <p class="small">“{name(r.task_pid)}” now takes {r.units_percent}% of the day for {hoursText(r.work_minutes)} of work (worked out from its scheduling rule).</p>
    {/each}
    {#if preview.cost_impact.available}
      {#each preview.cost_impact.labour as c}
        <p>Planned labour cost: {deltaText(c.delta_minor, c.currency)} ({deltaText(c.before_minor, c.currency)} → {deltaText(c.after_minor, c.currency)}).</p>
      {/each}
      {#if preview.cost_impact.unrated_minutes_after}
        <p class="small">▲ {hoursText(preview.cost_impact.unrated_minutes_after)} of work has no rate, so it is not costed.</p>
      {/if}
    {:else}
      <p class="small muted">{costReasonText(preview.cost_impact)}</p>
    {/if}
    <label>Reason (recorded with each change) <input bind:value={reason} maxlength="500" /></label>
    <div class="row">
      <button class="button primary" type="button" onclick={commit} disabled={busy}>Commit these changes</button>
      <button class="button" type="button" onclick={discard} disabled={busy}>Discard</button>
    </div>
  </section>
{/if}

<style>
  label {
    display: block;
  }
  li {
    margin-bottom: 0.25rem;
  }
</style>
