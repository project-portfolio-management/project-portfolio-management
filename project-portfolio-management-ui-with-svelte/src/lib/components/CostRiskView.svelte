<!--
  The cost-risk result, per currency (never added across): budget, mean, cost at
  P10 / P50 / P80 / P90, the chance of staying within budget, a histogram with
  P50, P80, P90 and the budget marked by labelled lines — never colour alone — and
  the same figures as a table; which lines drive it; which risks struck how often;
  and every assumption, including that the inputs are independent.
-->
<script lang="ts">
  import { money, type CostRisk } from "#lib/api/ppm.js";
  import { budgetConfidenceText, costHistogramGeometry, costSkippedText, eventHitText, lineSourceText, sensitivityText } from "#lib/scheduling-view.js";

  let { result }: { result: CostRisk } = $props();
  const a = $derived(result.assumptions);
</script>

<p class="small muted">
  {result.scope.kind === "portfolio" ? `All ${result.scope.plans} plan${result.scope.plans === 1 ? "" : "s"} you can read` : "This plan"} ·
  seed {a.seed} · {a.iterations} iterations · {a.distribution === "pert" ? "PERT" : "Triangular"}
</p>

{#each result.currencies as c (c.currency)}
  {@const g = costHistogramGeometry(c.histogram, c.percentiles, c.budget_minor)}
  <section aria-labelledby={`cur-${c.currency}`}>
    <h2 id={`cur-${c.currency}`}>{c.currency}</h2>
    <table>
      <caption class="sr-only">{c.currency} cost at each percentile</caption>
      <thead><tr><th scope="col">Outcome</th><th scope="col">Cost</th></tr></thead>
      <tbody>
        <tr><th scope="row">Budget (sum of planned amounts)</th><td>{money(c.budget_minor, c.currency)}</td></tr>
        <tr><th scope="row">Already spent</th><td>{money(c.spent_minor, c.currency)}</td></tr>
        <tr><th scope="row">Mean simulated cost</th><td>{money(c.mean_minor, c.currency)}</td></tr>
        {#each c.percentiles as p (p.p)}
          <tr><th scope="row">P{p.p}</th><td>{money(p.minor, c.currency)}</td></tr>
        {/each}
      </tbody>
    </table>
    <p>{budgetConfidenceText(c.budget_confidence_bp, money(c.budget_minor, c.currency))}</p>

    {#if g}
      <figure>
        <svg viewBox="0 0 {g.width} {g.height}" role="img" aria-label={`Histogram of simulated ${c.currency} cost, with P50, P80, P90 and the budget marked`} width="100%" style="max-width: 44rem">
          {#each g.bars as b (b.from)}
            <rect x={b.x} y={b.y} width={b.w} height={b.h} class="bar"><title>{money(b.from, c.currency)} to {money(b.to, c.currency)}: {b.count} of {a.iterations}</title></rect>
          {/each}
          <line x1="24" x2={g.width - 24} y1={g.baseline} y2={g.baseline} class="axis" />
          {#if g.budgetX !== null}
            <line x1={g.budgetX} x2={g.budgetX} y1="14" y2={g.baseline} class="budget" />
            <text x={g.budgetX} y="10" text-anchor="middle" class="label">budget</text>
          {/if}
          {#each g.markers as m (m.p)}
            <line x1={m.x} x2={m.x} y1="14" y2={g.baseline} class={`pct p${m.p}`} />
            <text x={m.x} y={g.baseline + 14} text-anchor="middle" class="label">P{m.p}</text>
          {/each}
        </svg>
        <details>
          <summary>The same figures as a table</summary>
          <table>
            <thead><tr><th scope="col">Cost from</th><th scope="col">to</th><th scope="col">Simulations</th></tr></thead>
            <tbody>
              {#each c.histogram as b (b.from_minor)}
                <tr><th scope="row">{money(b.from_minor, c.currency)}</th><td>{money(b.to_minor, c.currency)}</td><td>{b.count}</td></tr>
              {/each}
            </tbody>
          </table>
        </details>
        <figcaption class="small muted">Each bar is a cost range; taller means more simulations ended there. Solid lines are P50, P80 and P90 (labelled); the dashed line is the budget.</figcaption>
      </figure>
    {/if}

    {#if result.scope.kind === "portfolio"}
      <h3>By plan</h3>
      <table>
        <caption class="sr-only">Each plan's budget and mean simulated cost in {c.currency}</caption>
        <thead><tr><th scope="col">Plan</th><th scope="col">Budget</th><th scope="col">Mean simulated cost</th></tr></thead>
        <tbody>
          {#each c.plans as p (p.plan)}
            <tr><th scope="row">{p.name ?? p.plan}</th><td>{money(p.budget_minor, c.currency)}</td><td>{money(p.mean_minor, c.currency)}</td></tr>
          {/each}
        </tbody>
      </table>
    {/if}

    <h3>What drives it</h3>
    <table>
      <caption class="sr-only">Per-line estimate and sensitivity in {c.currency}</caption>
      <thead>
        <tr><th scope="col">Line</th><th scope="col">Low / planned / high</th><th scope="col">Estimate</th><th scope="col">Spent</th><th scope="col">Sensitivity</th></tr>
      </thead>
      <tbody>
        {#each c.lines as l (l.pid)}
          <tr class:fallback={l.source === "fallback"}>
            <th scope="row">{l.description ?? l.pid}{#if l.category} <span class="muted small">({l.category})</span>{/if}</th>
            <td>{money(l.low, c.currency)} / {money(l.likely, c.currency)} / {money(l.high, c.currency)}</td>
            <td>{#if l.source === "fallback"}<span aria-hidden="true">▲</span> {/if}{lineSourceText(l)}</td>
            <td>{money(l.actual, c.currency)}</td>
            <td>{sensitivityText(l.sensitivity)}</td>
          </tr>
        {/each}
      </tbody>
    </table>

    {#if c.events.length > 0}
      <h3>Risks that added cost</h3>
      <ul>
        {#each c.events as e (e.risk)}
          <li>{e.title ?? e.risk}: adds {money(e.impact_minor, c.currency)} — {eventHitText(e.probability_bp, e.hit_bp)}</li>
        {/each}
      </ul>
    {/if}
  </section>
{/each}

{#if result.events.skipped.length > 0}
  <p class="small">Left out:</p>
  <ul class="small">
    {#each result.events.skipped as e (e.risk_pid)}<li>{e.title}: {costSkippedText(e.reason)}</li>{/each}
  </ul>
{/if}
{#if result.events.used.length === 0}
  <p class="small muted">No register risk was used as a cost event.</p>
{/if}
{#if result.events.risks_without_cost_fields > 0}
  <p class="small muted">{result.events.risks_without_cost_fields} register risk{result.events.risks_without_cost_fields === 1 ? " has" : "s have"} no cost fields and {result.events.risks_without_cost_fields === 1 ? "is" : "are"} not part of this.</p>
{/if}

<h2>Assumptions</h2>
<ul class="small">
  <li><strong>Independence.</strong> {a.independence}</li>
  <li>{a.floor}</li>
  <li>{a.currencies}</li>
  <li>{a.budget}</li>
  <li>{a.fallback}</li>
  <li>Seed {a.seed}: the same seed gives the same result. Up to {a.max_iterations} iterations.</li>
</ul>
<p class="small muted">{result.note}</p>

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
  .bar {
    fill: var(--mxi-color-accent, #4a6fa5);
  }
  .axis,
  .pct,
  .budget {
    stroke: currentColor;
  }
  .axis {
    stroke-width: 1;
  }
  .pct {
    stroke-width: 2;
  }
  .pct.p80 {
    stroke-dasharray: 8 3;
  }
  .pct.p90 {
    stroke-dasharray: 2 3;
  }
  .budget {
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
</style>
