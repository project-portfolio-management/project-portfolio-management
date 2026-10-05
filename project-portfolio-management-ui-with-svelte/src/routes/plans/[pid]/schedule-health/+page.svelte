<!--
  Schedule health (`/plans/[pid]/schedule-health`): the fourteen DCMA-style
  checks over the plan's task network, each with what it counted, what it is held
  to, and the tasks to fix — each linked to the task. A self-check, not a DCMA
  compliance assessment. A check that cannot be computed says "Not enough data"
  with the reason and is never a pass. Pass / fail / not computable are a word and
  a symbol, never colour alone.
-->
<script lang="ts">
  import { localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { PpmClient, type ScheduleHealth } from "#lib/api/ppm.js";
  import { healthReason, healthResult, healthSummary, healthValue, offenderHref, thresholdSentence } from "#lib/scheduling-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");
  let report = $state<ScheduleHealth | null>(null);
  let error = $state<string | null>(null);
  let asOf = $state("");

  async function load() {
    try {
      report = await ppm.scheduleHealth(pid, asOf || undefined);
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : "Could not run the schedule health checks";
    }
  }
  onMount(() => void load());
</script>

<svelte:head><title>Schedule health — PPM</title></svelte:head>

<h1>Schedule health</h1>
<p class="lede">Checks the schedule for common problems, such as tasks with no links or no estimate.</p>
<p class="small"><a href={localeHref(`/plans/${pid}`)}>← Plan</a> · <a href={localeHref(`/plans/${pid}/critical-path`)}>Critical path</a></p>

{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if report}
  <p class="banner" role="note">{report.disclaimer}</p>
  <p class="small muted">
    As of {report.as_of} ·
    {#if report.baseline}baseline version {report.baseline.version}{:else}no baseline taken{/if} ·
    preset {report.preset} (thresholds: {report.thresholds.source}) · {report.tasks} task{report.tasks === 1 ? "" : "s"}
  </p>
  <p aria-live="polite"><strong>{healthSummary(report)}.</strong></p>
  <form class="filter" onsubmit={(e) => { e.preventDefault(); void load(); }}>
    <label>Status date <input type="date" bind:value={asOf} /></label>
    <button class="button small" type="submit">Re-run</button>
  </form>

  <table>
    <caption class="sr-only">The fourteen schedule health checks</caption>
    <thead>
      <tr>
        <th scope="col">Check</th>
        <th scope="col">Result</th>
        <th scope="col">What was found</th>
        <th scope="col">Held to</th>
        <th scope="col">To fix</th>
      </tr>
    </thead>
    <tbody>
      {#each report.checks as c (c.id)}
        {@const r = healthResult(c)}
        <tr class={r.kind}>
          <th scope="row">{c.id}. {c.name}</th>
          <td><span aria-hidden="true">{r.symbol}</span> {r.text}</td>
          <td>
            {#if c.pass === null}{healthReason(c.reason)}{:else}{healthValue(c)}{/if}
          </td>
          <td>{thresholdSentence(c)}</td>
          <td>
            {#if c.offenders.length === 0}
              {c.pass === false ? "See the note" : "—"}
            {:else}
              <details>
                <summary>{c.offenders.length} task{c.offenders.length === 1 ? "" : "s"}</summary>
                <ul>
                  {#each c.offenders as o (o.pid)}
                    <li><a href={localeHref(offenderHref(pid, o.pid))}>{o.title ?? o.pid}</a></li>
                  {/each}
                </ul>
              </details>
            {/if}
            {#if c.note}<p class="small muted">{c.note}</p>{/if}
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
  <p class="small muted">{report.note}</p>
{:else if !error}
  <p class="muted">Loading…</p>
{/if}

<style>
  table {
    width: 100%;
    border-collapse: collapse;
  }
  th,
  td {
    text-align: left;
    padding: 0.4rem 0.5rem;
    border-bottom: 1px solid var(--mxi-color-border, #ddd);
    vertical-align: top;
  }
  tr.fail td:nth-child(2) {
    font-weight: 700;
    color: var(--danger, #b00020);
  }
  tr.na td:nth-child(2) {
    font-style: italic;
  }
  .filter {
    display: flex;
    gap: 0.75rem;
    align-items: end;
    margin: 0.75rem 0;
  }
  .filter label {
    display: grid;
    gap: 0.2rem;
    font-size: 0.85rem;
  }
</style>
