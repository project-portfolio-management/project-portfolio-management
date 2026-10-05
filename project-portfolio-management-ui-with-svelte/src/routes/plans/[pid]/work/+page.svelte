<!--
  Work and assignments (`/plans/[pid]/work`): for each task, how long it takes,
  how much of the assignee's day it asks for, and how much work that is — and
  the rule that decides which of the three moves when you change one. Below, the
  work spread over days, weeks or months by each task's contour, as a table that
  always adds up. Over-capacity is said in words, never hidden; work that cannot
  be placed is listed, never dropped.
-->
<script lang="ts">
  import { localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import {
    PpmClient,
    type Contour,
    type CpmNode,
    type Grain,
    type SchedulingRule,
    type WorkProfile,
  } from "#lib/api/ppm.js";
  import {
    CONTOUR_TEXT,
    RULE_HELP,
    RULE_TEXT,
    UNPLACED_TEXT,
    assignmentBody,
    capacityText,
    customWeights,
    hoursText,
    resultText,
  } from "#lib/work-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");
  let nodes = $state<CpmNode[]>([]);
  let profile = $state<WorkProfile | null>(null);
  let error = $state<string | null>(null);
  let grain = $state<Grain>("week");
  /** What the last change did, per task. */
  let said = $state<Record<string, string>>({});
  let form = $state<Record<string, { rule: SchedulingRule; changing: "duration" | "units" | "work"; duration: string; units: string; hours: string; contour: Contour; custom: string }>>({});

  async function load() {
    try {
      nodes = (await ppm.planCpm(pid)).nodes;
      for (const n of nodes)
        form[n.pid] ??= {
          rule: n.scheduling_rule ?? "fixed_units",
          changing: n.units_percent == null ? "units" : "work",
          duration: String(n.duration_days ?? ""),
          units: String(n.units_percent ?? ""),
          hours: n.work_minutes ? String(n.work_minutes / 60) : "",
          contour: n.contour ?? "flat",
          custom: "",
        };
      await loadProfile();
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : "Could not load the plan";
    }
  }
  async function loadProfile() {
    try {
      profile = await ppm.workProfile(pid, { period: grain });
    } catch (err) {
      profile = null;
      error = err instanceof Error ? err.message : "Could not load the work profile";
    }
  }
  onMount(load);

  async function save(n: CpmNode, e: SubmitEvent) {
    e.preventDefault();
    const f = form[n.pid];
    if (!f) return;
    const body = assignmentBody({ ...f, first: n.units_percent == null });
    if ("error" in body) {
      said[n.pid] = body.error;
      return;
    }
    try {
      const r = await ppm.setAssignment(pid, n.pid, body);
      said[n.pid] = [resultText(r), capacityText(r.capacity)].filter(Boolean).join(" ");
      await load();
    } catch (err) {
      said[n.pid] = err instanceof Error ? err.message : "Could not save";
    }
  }
  async function saveContour(n: CpmNode) {
    const f = form[n.pid];
    if (!f) return;
    let custom: number[] | undefined;
    if (f.contour === "custom") {
      const w = customWeights(f.custom);
      if (!Array.isArray(w)) {
        said[n.pid] = w.error;
        return;
      }
      custom = w;
    }
    try {
      await ppm.setContour(pid, n.pid, f.contour, custom);
      said[n.pid] = "Contour saved.";
      await loadProfile();
    } catch (err) {
      said[n.pid] = err instanceof Error ? err.message : "Could not save";
    }
  }
  const day = (iso: string) => iso.slice(5);
  const label = (b: { from: string; to: string }) => (b.from === b.to ? b.from : `${day(b.from)}–${day(b.to)}`);
</script>

<svelte:head><title>Work and assignments — PPM</title></svelte:head>

<h1>Work and assignments</h1>
<p class="lede">How long each task takes, how much of a person's day it uses, and how much work that adds up to.</p>
<p class="small"><a href={localeHref(`/plans/${pid}`)}>← Plan</a> · <a href={localeHref(`/plans/${pid}/costs`)}>Costs</a></p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}

<h2>Tasks</h2>
{#if nodes.length === 0 && !error}
  <p class="muted">This plan has no scheduled tasks yet.</p>
{/if}
<ul class="stack tasks">
  {#each nodes as n (n.pid)}
    {@const f = form[n.pid]}
    <li class="surface">
      <h3>{n.title ?? n.pid}</h3>
      <p class="small">
        {#if n.units_percent != null && n.work_minutes != null}
          {n.duration_days} working days · {n.units_percent}% of a day · {hoursText(n.work_minutes)} of work · {RULE_TEXT[n.scheduling_rule ?? "fixed_units"]}
        {:else}
          <span class="muted">No assignment yet. Give a duration and units and the work is worked out.</span>
        {/if}
      </p>
      {#if f}
        <details>
          <summary>Edit assignment</summary>
          <form class="stack" onsubmit={(e) => save(n, e)}>
            <label>
              Rule
              <select bind:value={f.rule}>
                {#each Object.entries(RULE_TEXT) as [k, v]}<option value={k}>{v}</option>{/each}
              </select>
            </label>
            <p class="small muted">{RULE_HELP[f.rule]}</p>
            {#if n.units_percent == null}
              <label>Duration (working days) <input type="number" min="1" max="3650" step="1" bind:value={f.duration} /></label>
              <label>Units (% of the day) <input type="number" min="1" max="1000" step="1" bind:value={f.units} /></label>
            {:else}
              <label>
                Change
                <select bind:value={f.changing}>
                  <option value="work">Work (hours)</option>
                  <option value="duration">Duration (days)</option>
                  <option value="units">Units (%)</option>
                </select>
              </label>
              {#if f.changing === "work"}
                <label>Work (hours) <input type="number" min="0" step="0.25" bind:value={f.hours} /></label>
              {:else if f.changing === "duration"}
                <label>Duration (working days) <input type="number" min="1" max="3650" step="1" bind:value={f.duration} /></label>
              {:else}
                <label>Units (% of the day) <input type="number" min="1" max="1000" step="1" bind:value={f.units} /></label>
              {/if}
            {/if}
            <button class="button" type="submit">Save assignment</button>
          </form>
          {#if n.units_percent != null}
            <div class="row">
              <label>
                Spread
                <select bind:value={f.contour}>
                  {#each Object.entries(CONTOUR_TEXT) as [k, v]}<option value={k}>{v}</option>{/each}
                </select>
              </label>
              {#if f.contour === "custom"}
                <label>Percentages <input bind:value={f.custom} placeholder="10, 20, 70" /></label>
              {/if}
              <button class="button" type="button" onclick={() => saveContour(n)}>Save contour</button>
            </div>
          {/if}
        </details>
        {#if said[n.pid]}<p class="small" role="status">{said[n.pid]}</p>{/if}
      {/if}
    </li>
  {/each}
</ul>

<h2>Work over time</h2>
<label>Period
  <select bind:value={grain} onchange={loadProfile}>
    <option value="day">Day</option>
    <option value="week">Week</option>
    <option value="month">Month</option>
  </select>
</label>
{#if profile && profile.reason === "no_assignments"}
  <p class="muted">No task carries work yet, so there is nothing to spread.</p>
{:else if profile && profile.totals.buckets}
  <table data-testid="work-profile">
    <caption class="small muted">Hours per {grain}; each row adds up to the task's work, the last row to the plan's.</caption>
    <thead>
      <tr><th scope="col">Task</th>{#each profile.totals.buckets as b}<th scope="col">{label(b)}</th>{/each}<th scope="col">Total</th></tr>
    </thead>
    <tbody>
      {#each profile.tasks as t (t.pid)}
        <tr>
          <th scope="row">{t.title} <span class="muted small">({t.contour.replace("_", " ")})</span></th>
          {#each t.buckets as b}<td>{b.minutes ? hoursText(b.minutes) : "–"}</td>{/each}
          <td>{hoursText(t.work_minutes)}{#if !t.placed} <span class="small">(not placed)</span>{/if}</td>
        </tr>
      {/each}
    </tbody>
    <tfoot>
      <tr>
        <th scope="row">Plan</th>
        {#each profile.totals.buckets as b}<td>{b.minutes ? hoursText(b.minutes) : "–"}</td>{/each}
        <td>{hoursText(profile.totals.work_minutes)}</td>
      </tr>
    </tfoot>
  </table>
  {#if profile.unplaced.length}
    <ul>
      {#each profile.unplaced as u}
        <li>▲ {u.title} ({hoursText(u.work_minutes)}) {UNPLACED_TEXT[u.reason] ?? u.reason}, so its work is not in the buckets.</li>
      {/each}
    </ul>
  {/if}
  {#if profile.totals.outside_window_minutes}
    <p class="small muted">{hoursText(profile.totals.outside_window_minutes)} falls outside this window.</p>
  {/if}
{/if}

<style>
  .tasks {
    list-style: none;
    padding: 0;
  }
  h3 {
    margin: 0 0 0.25rem;
    font-size: 1rem;
  }
  label {
    display: block;
  }
</style>
