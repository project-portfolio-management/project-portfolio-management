<!--
  Earned value (`/plans/[pid]/earned-value`, SC-4): planned value, earned value and
  actual cost per currency, with schedule and cost performance (SPI, CPI) and
  earned-schedule SPI(t) in plain sentences. Percent complete is physical — work
  finished, not time elapsed — and how it is counted is said up front: the plan's
  default method, any task counted by a person-entered percent (always flagged,
  always with a note), and any task that carries no weight. Measured as of today.
  English-first, like the other recent views.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { PpmClient, money, type EarnedValue, type Task } from "#lib/api/ppm.js";
  import EvBars from "#lib/components/EvBars.svelte";
  import {
    costSentence,
    earnedScheduleSentence,
    health,
    marker,
    methodSentence,
    ratioOrReason,
    scheduleSentence,
  } from "#lib/ev-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");

  let ev = $state<EarnedValue | null>(null);
  let tasks = $state<Task[]>([]);
  let error = $state<string | null>(null);
  let editing = $state<string | null>(null);
  let method = $state("");
  let percent = $state(50);
  let note = $state("");

  async function load() {
    try {
      ev = await ppm.earnedValue(pid);
      tasks = (await ppm.listTasks(pid)).tasks;
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : "Could not load earned value";
    }
  }
  onMount(() => void load());

  const titleOf = $derived(new Map(tasks.map((t) => [t.pid, t.title])));

  async function setDefault(value: "zero_hundred" | "fifty_fifty" | "points") {
    error = null;
    try {
      await ppm.setEarnedValueMethod(pid, value);
      await load();
    } catch (err) {
      error = err instanceof Error ? err.message : "Could not save";
    }
  }

  function edit(taskPid: string) {
    editing = taskPid;
    const share = ev?.percent_complete?.tasks.find((t) => t.pid === taskPid);
    method = share?.method === "manual" ? "manual" : "";
    percent = share?.method === "manual" ? Math.round(share.percent_bp / 100) : 50;
    note = "";
  }

  async function save(event: SubmitEvent) {
    event.preventDefault();
    if (!editing) return;
    error = null;
    try {
      await ppm.setTaskPercent(
        pid,
        editing,
        method === "manual"
          ? { method: "manual", percent, note }
          : method
            ? { method: method as "zero_hundred" | "fifty_fifty" }
            : {},
      );
      editing = null;
      await load();
    } catch (err) {
      error = err instanceof Error ? err.message : "Could not save";
    }
  }
</script>

<svelte:head><title>Earned value — PPM</title></svelte:head>

<p class="small"><a href={`/plans/${pid}/governance`}>← Governance</a> · <a href={`/plans/${pid}/critical-path`}>Critical path</a></p>
<h1>Earned value</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if ev}
  <p class="small muted">Measured as of {ev.as_of}. Percent complete is work finished, not time elapsed.</p>

  {#if ev.blocks === null}
    <p class="banner" role="status" data-testid="ev-absent">{ev.reason}</p>
  {:else}
    {#if ev.percent_complete}
      {@const pc = ev.percent_complete}
      <section aria-labelledby="pc-h" class="stack">
        <h2 id="pc-h">{Math.round(pc.percent_bp / 100)}% complete</h2>
        <p class="small">{methodSentence(pc)}.</p>
        {#if pc.manual}
          <p class="banner" role="note">Some of this figure rests on a person-entered percent, not on finished work: {pc.manual_tasks.map((p) => titleOf.get(p) ?? "a task").join(", ")}.</p>
        {/if}
        <label>
          Count tasks
          <select value={pc.default_method} onchange={(e) => void setDefault(e.currentTarget.value as "zero_hundred" | "fifty_fifty" | "points")}>
            <option value="zero_hundred">only when finished</option>
            <option value="fifty_fifty">started work counts half</option>
            <option value="points">by story points, when finished</option>
          </select>
        </label>
      </section>
    {/if}

    {#each ev.blocks as b (b.currency)}
      {@const h = marker(health(b.spi.bp))}
      <section class="surface stack" aria-labelledby="blk-{b.currency}">
        <h2 id="blk-{b.currency}">{b.currency} <span class="small muted">baseline version {b.baseline_version}</span></h2>
        <EvBars block={b} />
        <p><span aria-hidden="true">{h}</span> {scheduleSentence(b, (n) => money(n, b.currency))}</p>
        <p><span aria-hidden="true">{marker(health(b.cpi.bp))}</span> {costSentence(b, (n) => money(n, b.currency))}</p>
        <p><span aria-hidden="true">{marker(health(b.spi_t.bp))}</span> {earnedScheduleSentence(b)}</p>
        <table>
          <caption class="sr-only">{b.currency} earned value figures</caption>
          <tbody>
            <tr><th scope="row">Budget at completion</th><td>{money(b.bac_minor, b.currency)}</td></tr>
            <tr><th scope="row">Planned value</th><td>{money(b.pv_minor, b.currency)}</td></tr>
            <tr><th scope="row">Earned value</th><td>{money(b.ev_minor, b.currency)}</td></tr>
            <tr><th scope="row">Actual cost</th><td>{b.ac_minor === null ? "unknown" : money(b.ac_minor, b.currency)}</td></tr>
            <tr><th scope="row">Schedule variance (EV − PV)</th><td>{money(b.sv_minor, b.currency)}</td></tr>
            <tr><th scope="row">Cost variance (EV − AC)</th><td>{b.cv_minor === null ? "unknown" : money(b.cv_minor, b.currency)}</td></tr>
            <tr><th scope="row">SPI</th><td>{ratioOrReason(b.spi)}</td></tr>
            <tr><th scope="row">CPI</th><td>{ratioOrReason(b.cpi)}</td></tr>
            <tr><th scope="row">SPI(t)</th><td>{ratioOrReason(b.spi_t)}</td></tr>
          </tbody>
        </table>
      </section>
    {/each}

    {#if ev.unbaselined_currencies && ev.unbaselined_currencies.length > 0}
      <p class="banner" role="note">
        Budget lines in {ev.unbaselined_currencies.join(", ")} have no baseline, so they are not part of any figure above and are never added to another currency.
      </p>
    {/if}

    <h2>Count a task another way</h2>
    <ul class="small">
      {#each ev.percent_complete?.tasks ?? [] as t (t.pid)}
        <li>
          {titleOf.get(t.pid) ?? "(task)"}: {Math.round(t.percent_bp / 100)}%
          ({t.method === "manual" ? "manual" : t.method === "fifty_fifty" ? "half for started" : "finished only"}{ev.percent_complete?.weighting === "points" ? `, ${t.weight} points` : ""})
          <button type="button" onclick={() => edit(t.pid)}>Change</button>
        </li>
      {/each}
    </ul>
    {#if editing}
      <form onsubmit={save} class="stack surface">
        <h3>{titleOf.get(editing) ?? "Task"}</h3>
        <label>
          Method
          <select bind:value={method}>
            <option value="">Use the plan's method</option>
            <option value="zero_hundred">Only when finished</option>
            <option value="fifty_fifty">Started work counts half</option>
            <option value="manual">A percent I enter (flagged)</option>
          </select>
        </label>
        {#if method === "manual"}
          <label>Percent complete <input type="number" min="0" max="100" bind:value={percent} required /></label>
          <label>Why (required) <input type="text" bind:value={note} required /></label>
        {/if}
        <button type="submit">Save</button>
        <button type="button" onclick={() => (editing = null)}>Cancel</button>
      </form>
    {/if}
    <p class="small muted">{ev.note}</p>
  {/if}
{:else if !error}
  <p>Loading…</p>
{/if}

<style>
  table {
    border-collapse: collapse;
  }
  th,
  td {
    border: 1px solid var(--border, #ccc);
    padding: 0.25rem 0.6rem;
    text-align: start;
  }
  td {
    font-variant-numeric: tabular-nums;
  }
  .sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    overflow: hidden;
    clip-path: inset(50%);
    white-space: nowrap;
  }
  label {
    display: flex;
    flex-direction: column;
    gap: 0.2rem;
    max-width: 24rem;
  }
</style>
