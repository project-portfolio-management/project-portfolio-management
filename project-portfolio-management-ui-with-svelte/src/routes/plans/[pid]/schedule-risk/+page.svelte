<!--
  Schedule risk (`/plans/[pid]/schedule-risk`): a seeded simulation of the plan's
  task network. The histogram shows how often each finish date came up, with P50,
  P80 and P90 marked by labelled lines — never colour alone — and the same numbers
  in a table beneath. Every assumption is stated, including that the inputs are
  treated as independent. A task with no estimate is marked as using the
  fallback band; a risk that could not be used is named with the reason.
-->
<script lang="ts">
  import { localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { PpmClient, type ScheduleRisk } from "#lib/api/ppm.js";
  import {
    confidenceText,
    criticalityText,
    distributionText,
    histogramGeometry,
    percentileText,
    sensitivityText,
    skippedReasonText,
    sourceText,
  } from "#lib/scheduling-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");
  let run = $state<ScheduleRisk | null>(null);
  let ranOn = $state("");
  let error = $state<string | null>(null);
  let iterations = $state(2000);
  let seed = $state(1);
  let distribution = $state<"triangular" | "pert">("triangular");
  let band = $state(20);
  let busy = $state(false);

  async function simulate() {
    busy = true;
    try {
      run = await ppm.scheduleRisk(pid, { iterations, seed, distribution, fallbackBandPercent: band });
      ranOn = new Date().toISOString().slice(0, 10);
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : "Could not run the simulation";
    } finally {
      busy = false;
    }
  }
  onMount(() => void simulate());

  const g = $derived(run ? histogramGeometry(run.histogram, run.percentiles, run.cpm_finish) : null);
  const pName = (p: number) => `P${p}`;
</script>

<svelte:head><title>Schedule risk — PPM</title></svelte:head>

<h1>Schedule risk</h1>
<p class="lede">Many what-if runs of the schedule show the range of finish dates and how likely each one is.</p>
<p class="small"><a href={localeHref(`/plans/${pid}`)}>← Plan</a> · <a href={localeHref(`/plans/${pid}/critical-path`)}>Critical path</a></p>

{#if error}<p class="banner" role="alert">{error}</p>{/if}

<form class="filter" onsubmit={(e) => { e.preventDefault(); void simulate(); }}>
  <label>Iterations <input type="number" min="1" max="20000" step="1" bind:value={iterations} /></label>
  <label>Seed <input type="number" min="0" step="1" bind:value={seed} /></label>
  <label>Distribution
    <select bind:value={distribution}>
      <option value="triangular">Triangular</option>
      <option value="pert">Weighted towards the likely value (PERT)</option>
    </select>
  </label>
  <label>Band for tasks with no estimate (± %) <input type="number" min="0" max="100" step="1" bind:value={band} /></label>
  <button class="button small primary" type="submit" disabled={busy}>{busy ? "Running…" : "Run"}</button>
</form>

{#if run}
  <p class="small muted">
    Run on {ranOn} against the plan as it stands · seed {run.assumptions.seed} · {run.assumptions.iterations} iterations ·
    {distributionText(run.assumptions)} · calendar {run.calendar}
  </p>

  <h2>When it finishes</h2>
  <table>
    <caption class="sr-only">Finish date at each percentile</caption>
    <thead><tr><th scope="col">Outcome</th><th scope="col">Finish</th></tr></thead>
    <tbody>
      {#each run.percentiles as p (p.p)}
        <tr><th scope="row">{pName(p.p)}</th><td>{p.finish}</td></tr>
      {/each}
      <tr><th scope="row">Critical-path finish (the plan's own date)</th><td>{run.cpm_finish ?? "not enough data"}</td></tr>
    </tbody>
  </table>
  <p>{confidenceText(run.cpm_finish_confidence_bp, run.cpm_finish)}</p>
  <p class="small muted">{run.percentiles.map(percentileText).join(" · ")}. Earliest {run.earliest}, latest {run.latest}.</p>

  {#if g}
    <figure>
      <svg viewBox="0 0 {g.width} {g.height}" role="img" aria-label="Histogram of simulated finish dates, with P50, P80 and P90 marked" width="100%" style="max-width: 44rem">
        {#each g.bars as b (b.date)}
          <rect x={b.x} y={b.y} width={b.w} height={b.h} class="bar"><title>{b.date}: {b.count} of {run.assumptions.iterations}</title></rect>
        {/each}
        <line x1="24" x2={g.width - 24} y1={g.baseline} y2={g.baseline} class="axis" />
        {#if g.cpmX !== null}
          <line x1={g.cpmX} x2={g.cpmX} y1="14" y2={g.baseline} class="cpm" />
          <text x={g.cpmX} y="10" text-anchor="middle" class="label">plan</text>
        {/if}
        {#each g.markers as m (m.p)}
          <line x1={m.x} x2={m.x} y1="14" y2={g.baseline} class={`pct p${m.p}`} />
          <text x={m.x} y={g.baseline + 14} text-anchor="middle" class="label">P{m.p}</text>
        {/each}
      </svg>
      <details>
        <summary>The same figures as a table</summary>
        <table>
          <thead><tr><th scope="col">Finish date</th><th scope="col">Simulations</th><th scope="col">Share</th></tr></thead>
          <tbody>
            {#each run.histogram as h (h.date)}
              <tr><th scope="row">{h.date}</th><td>{h.count}</td><td>{((h.count / run.assumptions.iterations) * 100).toFixed(1)}%</td></tr>
            {/each}
          </tbody>
        </table>
      </details>
      <figcaption class="small muted">Each bar is a finish date; taller means more of the simulations ended there. Solid lines are P50, P80 and P90 (labelled); the dashed line is the plan's own date.</figcaption>
    </figure>
  {/if}

  <h2>What drives it</h2>
  <table>
    <caption class="sr-only">Per-task estimate, criticality and sensitivity</caption>
    <thead>
      <tr>
        <th scope="col">Task</th><th scope="col">Optimistic / likely / pessimistic (business days)</th>
        <th scope="col">Estimate</th><th scope="col">Critical in</th><th scope="col">Sensitivity</th>
      </tr>
    </thead>
    <tbody>
      {#each [...run.tasks].sort((a, b) => b.criticality_bp - a.criticality_bp) as t (t.pid)}
        <tr class:fallback={t.source === "fallback"}>
          <th scope="row">{t.title ?? t.pid}</th>
          <td>{t.optimistic} / {t.likely} / {t.pessimistic}</td>
          <td>{#if t.source === "fallback"}<span aria-hidden="true">▲</span> {/if}{sourceText(t)}</td>
          <td>{criticalityText(t.criticality_bp)} of runs</td>
          <td>{sensitivityText(t.sensitivity)}</td>
        </tr>
      {/each}
    </tbody>
  </table>

  <h2>Risks used</h2>
  {#if run.events.used.length === 0}
    <p>No register risk was used as a schedule event.</p>
  {:else}
    <ul>
      {#each run.events.used as e (e.risk_pid)}
        <li>{e.title}: {e.probability_percent}% chance of adding {e.impact_days} business day{e.impact_days === 1 ? "" : "s"}</li>
      {/each}
    </ul>
  {/if}
  {#if run.events.skipped.length > 0}
    <p class="small">Left out:</p>
    <ul class="small">
      {#each run.events.skipped as e (e.risk_pid)}
        <li>{e.title}: {skippedReasonText(e.reason)}</li>
      {/each}
    </ul>
  {/if}
  {#if run.events.risks_without_schedule_fields > 0}
    <p class="small muted">{run.events.risks_without_schedule_fields} register risk{run.events.risks_without_schedule_fields === 1 ? " has" : "s have"} no schedule fields and {run.events.risks_without_schedule_fields === 1 ? "is" : "are"} not part of this.</p>
  {/if}
  {#if run.unscheduled.length > 0}
    <p class="small">Not simulated, because the schedule cannot place them: {run.unscheduled.map((u) => `${u.title ?? u.pid} (${u.reason})`).join(", ")}.</p>
  {/if}

  <h2>Assumptions</h2>
  <ul class="small">
    <li><strong>Independence.</strong> {run.assumptions.independence}</li>
    <li>{run.assumptions.durations}</li>
    <li>{run.assumptions.fallback}</li>
    <li>Seed {run.assumptions.seed}: the same seed gives the same result. Up to {run.assumptions.max_iterations} iterations.</li>
  </ul>
  <p class="small muted">{run.note}</p>
{:else if !error}
  <p class="muted">Running…</p>
{/if}

<style>
  table {
    border-collapse: collapse;
    width: 100%;
  }
  th,
  td {
    text-align: left;
    padding: 0.4rem 0.5rem;
    border-bottom: 1px solid var(--mxi-color-border, #ddd);
  }
  .bar {
    fill: var(--mxi-color-accent, #4a6fa5);
  }
  .axis {
    stroke: currentColor;
    stroke-width: 1;
  }
  .pct {
    stroke: currentColor;
    stroke-width: 2;
  }
  .pct.p80 {
    stroke-dasharray: 8 3;
  }
  .pct.p90 {
    stroke-dasharray: 2 3;
  }
  .cpm {
    stroke: currentColor;
    stroke-width: 1;
    stroke-dasharray: 4 4;
  }
  .label {
    font-size: 11px;
    fill: currentColor;
  }
  tr.fallback th {
    font-style: italic;
  }
  .filter {
    display: flex;
    flex-wrap: wrap;
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
