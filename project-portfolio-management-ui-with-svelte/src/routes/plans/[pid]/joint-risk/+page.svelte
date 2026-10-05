<!--
  Cost and schedule together (`/plans/[pid]/joint-risk`): one simulation in which
  the finish and the cost move together. A risk with both a schedule and a cost
  impact strikes once and drives both. A late finish adds a delay cost you set
  (zero by default). The headline is the joint confidence — both within budget
  and on time — which is never above either on its own. The quadrants are a table
  and the scatter has the same numbers beneath it.
-->
<script lang="ts">
  import { localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { PpmClient, money, type JointRisk } from "#lib/api/ppm.js";
  import {
    budgetConfidenceText,
    confidenceText,
    costSkippedText,
    delayCostText,
    jointConfidenceText,
    lineSourceText,
    percentileText,
    quadrantRows,
    scatterGeometry,
    skippedReasonText,
  } from "#lib/scheduling-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");
  let result = $state<JointRisk | null>(null);
  let error = $state<string | null>(null);
  let iterations = $state(2000);
  let seed = $state(1);
  let distribution = $state<"triangular" | "pert">("triangular");
  let band = $state(20);
  let currency = $state("");
  let delayCost = $state(0);
  let busy = $state(false);

  async function run() {
    busy = true;
    try {
      result = await ppm.jointRisk(pid, {
        iterations,
        seed,
        distribution,
        fallbackBandPercent: band,
        currency: currency.trim().toUpperCase() || undefined,
        delayCostPerDayMinor: Math.round(delayCost * 100),
      });
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : "Could not run the simulation";
    } finally {
      busy = false;
    }
  }
  onMount(() => void run());

  const scatter = $derived(result ? scatterGeometry(result.points, result.cost.budget_minor) : null);
</script>

<svelte:head><title>Cost and schedule risk — PPM</title></svelte:head>

<h1>Cost and schedule together</h1>
<p class="small"><a href={localeHref(`/plans/${pid}`)}>← Plan</a> · <a href={localeHref(`/plans/${pid}/schedule-risk`)}>Schedule risk</a> · <a href={localeHref(`/plans/${pid}/cost-risk`)}>Cost risk</a></p>

{#if error}<p class="banner" role="alert">{error}</p>{/if}

<form class="filter" onsubmit={(e) => { e.preventDefault(); void run(); }}>
  <label>Currency (if the plan has several) <input type="text" maxlength="3" bind:value={currency} placeholder="GBP" /></label>
  <label>Cost of each working day late (major units) <input type="number" min="0" step="0.01" bind:value={delayCost} /></label>
  <label>Iterations <input type="number" min="1" max="20000" step="1" bind:value={iterations} /></label>
  <label>Seed <input type="number" min="0" step="1" bind:value={seed} /></label>
  <label>Distribution
    <select bind:value={distribution}><option value="triangular">Triangular</option><option value="pert">PERT</option></select>
  </label>
  <label>Band when there is no estimate (± %) <input type="number" min="0" max="100" step="1" bind:value={band} /></label>
  <button class="button small" type="submit" disabled={busy}>{busy ? "Running…" : "Run"}</button>
</form>

{#if result}
  {@const a = result.assumptions}
  <p class="small muted">
    {result.currency} · seed {a.seed} · {a.iterations} iterations · {a.distribution === "pert" ? "PERT" : "Triangular"} · calendar {result.calendar}
  </p>
  {#if result.other_currencies_not_simulated.length > 0}
    <p class="banner" role="note">Budget lines in {result.other_currencies_not_simulated.join(", ")} are not part of this run; currencies are never added.</p>
  {/if}

  <h2>Both at once</h2>
  <p><strong>{jointConfidenceText(result.confidence)}</strong></p>
  <table>
    <caption class="sr-only">How the simulated outcomes fall around the budget and the critical-path finish</caption>
    <thead><tr><th scope="col">Outcome</th><th scope="col">Simulations</th><th scope="col">Share</th></tr></thead>
    <tbody>
      {#each quadrantRows(result.quadrants) as q (q.key)}
        <tr><th scope="row"><span aria-hidden="true">{q.symbol}</span> {q.label}</th><td>{q.count}</td><td>{q.share}</td></tr>
      {/each}
    </tbody>
  </table>

  {#if scatter}
    <figure>
      <svg viewBox="0 0 {scatter.width} {scatter.height}" role="img" aria-label="Scatter of simulated total cost against working days late, with the budget and the on-time line" width="100%" style="max-width: 32rem">
        {#each scatter.dots as d, i (i)}
          <circle cx={d.x} cy={d.y} r="2.5" class="dot"><title>{d.days} working days late, {money(d.cost, result.currency)}</title></circle>
        {/each}
        <line x1={scatter.onTimeX} x2={scatter.onTimeX} y1="16" y2={scatter.height - 32} class="ref" />
        <text x={scatter.onTimeX} y="12" text-anchor="middle" class="label">on time</text>
        <line x1="32" x2={scatter.width - 32} y1={scatter.budgetY} y2={scatter.budgetY} class="ref budget" />
        <text x={scatter.width - 32} y={scatter.budgetY - 4} text-anchor="end" class="label">budget</text>
        <text x={scatter.width / 2} y={scatter.height - 6} text-anchor="middle" class="label">working days late →</text>
      </svg>
      <figcaption class="small muted">Each dot is one simulation (a thinned sample). Right of the on-time line is late; above the budget line is over budget. The table above has the full counts.</figcaption>
    </figure>
  {/if}

  <h2>Cost</h2>
  <table>
    <caption class="sr-only">Total cost, delay cost included</caption>
    <thead><tr><th scope="col">Outcome</th><th scope="col">Cost</th></tr></thead>
    <tbody>
      <tr><th scope="row">Budget</th><td>{money(result.cost.budget_minor, result.currency)}</td></tr>
      <tr><th scope="row">Mean (delay cost included)</th><td>{money(result.cost.mean_minor, result.currency)}</td></tr>
      <tr><th scope="row">Of which delay cost, on average</th><td>{money(result.cost.mean_delay_cost_minor, result.currency)}</td></tr>
      {#each result.cost.percentiles as p (p.p)}<tr><th scope="row">P{p.p}</th><td>{money(p.minor, result.currency)}</td></tr>{/each}
    </tbody>
  </table>
  <p>{budgetConfidenceText(result.confidence.budget_bp, money(result.cost.budget_minor, result.currency))}</p>
  <p class="small">{delayCostText(a.delay_cost.per_day_minor, a.delay_cost.currency, money)}</p>

  <h2>Finish</h2>
  <table>
    <caption class="sr-only">Finish date at each percentile</caption>
    <thead><tr><th scope="col">Outcome</th><th scope="col">Finish</th></tr></thead>
    <tbody>
      {#each result.schedule.percentiles as p (p.p)}<tr><th scope="row">P{p.p}</th><td>{p.finish}</td></tr>{/each}
      <tr><th scope="row">Critical-path finish (the plan's own date)</th><td>{result.schedule.cpm_finish ?? "not enough data"}</td></tr>
    </tbody>
  </table>
  <p>{confidenceText(result.confidence.finish_bp, result.schedule.cpm_finish)}</p>
  <p class="small muted">{result.schedule.percentiles.map(percentileText).join(" · ")}</p>

  <h2>Cost lines</h2>
  <table>
    <caption class="sr-only">Per-line estimate in {result.currency}</caption>
    <thead><tr><th scope="col">Line</th><th scope="col">Low / planned / high</th><th scope="col">Estimate</th></tr></thead>
    <tbody>
      {#each result.cost.lines as l (l.pid)}
        <tr class:fallback={l.source === "fallback"}>
          <th scope="row">{l.description ?? l.pid}</th>
          <td>{money(l.low, result.currency)} / {money(l.likely, result.currency)} / {money(l.high, result.currency)}</td>
          <td>{#if l.source === "fallback"}<span aria-hidden="true">▲</span> {/if}{lineSourceText(l)}</td>
        </tr>
      {/each}
    </tbody>
  </table>

  <h2>Risks</h2>
  {#if result.assumptions.shared_events.length > 0}
    <p class="small">{result.assumptions.shared_events.length} risk{result.assumptions.shared_events.length === 1 ? "" : "s"} affect both the schedule and the cost: each strikes once and drives both.</p>
  {/if}
  {#if result.events.schedule.skipped.length + result.events.cost.skipped.length > 0}
    <ul class="small">
      {#each result.events.schedule.skipped as e (`s-${e.risk_pid}`)}<li>Schedule — {e.title}: {skippedReasonText(e.reason)}</li>{/each}
      {#each result.events.cost.skipped as e (`c-${e.risk_pid}`)}<li>Cost — {e.title}: {costSkippedText(e.reason)}</li>{/each}
    </ul>
  {:else if result.events.schedule.used.length + result.events.cost.used.length === 0}
    <p class="small muted">No register risk was used in this run.</p>
  {/if}

  <h2>Assumptions</h2>
  <ul class="small">
    <li><strong>Independence.</strong> {a.independence}</li>
    <li>{a.shared_events_rule}</li>
    <li>{a.delay_cost.rule}</li>
    <li>{a.floor}</li>
    <li>{a.fallback}</li>
    <li>Seed {a.seed}: the same seed gives the same result.</li>
  </ul>
  <p class="small muted">{result.note}</p>
{:else if !error}
  <p class="muted">Running…</p>
{/if}

<style>
  table {
    border-collapse: collapse;
    width: 100%;
    margin: 0.5rem 0;
  }
  th,
  td {
    text-align: left;
    padding: 0.4rem 0.5rem;
    border-bottom: 1px solid var(--mxi-color-border, #ddd);
  }
  .dot {
    fill: var(--mxi-color-accent, #4a6fa5);
    fill-opacity: 0.55;
  }
  .ref {
    stroke: currentColor;
    stroke-width: 1;
    stroke-dasharray: 5 4;
  }
  .ref.budget {
    stroke-dasharray: 2 3;
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
