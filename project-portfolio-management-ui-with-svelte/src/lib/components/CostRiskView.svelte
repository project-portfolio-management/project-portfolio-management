<!--
  The cost-risk result, per currency (never added across): budget, mean, cost at
  P10 / P50 / P80 / P90, the chance of staying within budget, a histogram with
  P50, P80, P90 and the budget marked by labelled lines — never colour alone — and
  the same figures as a table; which lines drive it; which risks struck how often;
  and every assumption, including that the inputs are independent.
-->
<script lang="ts">
  import { t } from "#lib/i18n.svelte.js";
  import { money, type CostRisk } from "#lib/api/ppm.js";
  import { budgetConfidenceText, costHistogramGeometry, costSkippedText, eventHitText, lineSourceText, sensitivityText } from "#lib/scheduling-view.js";

  let { result }: { result: CostRisk } = $props();
  const a = $derived(result.assumptions);
</script>

<p class="small muted">
  {result.scope.kind === "portfolio" ? (result.scope.plans === 1 ? t("c.CostRiskView.scope_all_one", { plans: result.scope.plans }) : t("c.CostRiskView.scope_all_other", { plans: result.scope.plans })) : t("c.CostRiskView.scope_this_plan")} ·
  {t("c.CostRiskView.run_summary", { seed: a.seed, iterations: a.iterations, distribution: a.distribution === "pert" ? t("c.CostRiskView.dist_pert") : t("c.CostRiskView.dist_triangular") })}
</p>

{#each result.currencies as c (c.currency)}
  {@const g = costHistogramGeometry(c.histogram, c.percentiles, c.budget_minor)}
  <section aria-labelledby={`cur-${c.currency}`}>
    <h2 id={`cur-${c.currency}`}>{c.currency}</h2>
    <table>
      <caption class="sr-only">{t("c.CostRiskView.cost_at_each_percentile", { currency: c.currency })}</caption>
      <thead><tr><th scope="col">{t("c.CostRiskView.outcome")}</th><th scope="col">{t("c.CostRiskView.cost")}</th></tr></thead>
      <tbody>
        <tr><th scope="row">{t("c.CostRiskView.budget_sum_of_planned_amounts")}</th><td>{money(c.budget_minor, c.currency)}</td></tr>
        <tr><th scope="row">{t("c.CostRiskView.already_spent")}</th><td>{money(c.spent_minor, c.currency)}</td></tr>
        <tr><th scope="row">{t("c.CostRiskView.mean_simulated_cost")}</th><td>{money(c.mean_minor, c.currency)}</td></tr>
        {#each c.percentiles as p (p.p)}
          <tr><th scope="row">P{p.p}</th><td>{money(p.minor, c.currency)}</td></tr>
        {/each}
      </tbody>
    </table>
    <p>{budgetConfidenceText(c.budget_confidence_bp, money(c.budget_minor, c.currency))}</p>

    {#if g}
      <figure>
        <svg viewBox="0 0 {g.width} {g.height}" role="img" aria-label={t("c.CostRiskView.histogram_label", { currency: c.currency })} width="100%" style="max-width: 44rem">
          {#each g.bars as b (b.from)}
            <rect x={b.x} y={b.y} width={b.w} height={b.h} class="bar"><title>{t("c.CostRiskView.to_of", { money: money(b.from, c.currency), money2: money(b.to, c.currency), count: b.count, iterations: a.iterations })}</title></rect>
          {/each}
          <line x1="24" x2={g.width - 24} y1={g.baseline} y2={g.baseline} class="axis" />
          {#if g.budgetX !== null}
            <line x1={g.budgetX} x2={g.budgetX} y1="14" y2={g.baseline} class="budget" />
            <text x={g.budgetX} y="10" text-anchor="middle" class="label">{t("c.CostRiskView.budget")}</text>
          {/if}
          {#each g.markers as m (m.p)}
            <line x1={m.x} x2={m.x} y1="14" y2={g.baseline} class={`pct p${m.p}`} />
            <text x={m.x} y={g.baseline + 14} text-anchor="middle" class="label">P{m.p}</text>
          {/each}
        </svg>
        <details>
          <summary>{t("c.CostRiskView.the_same_figures_as_a")}</summary>
          <table>
            <thead><tr><th scope="col">{t("c.CostRiskView.cost_from")}</th><th scope="col">{t("c.CostRiskView.to")}</th><th scope="col">{t("c.CostRiskView.simulations")}</th></tr></thead>
            <tbody>
              {#each c.histogram as b (b.from_minor)}
                <tr><th scope="row">{money(b.from_minor, c.currency)}</th><td>{money(b.to_minor, c.currency)}</td><td>{b.count}</td></tr>
              {/each}
            </tbody>
          </table>
        </details>
        <figcaption class="small muted">{t("c.CostRiskView.each_bar_is_a_cost")}</figcaption>
      </figure>
    {/if}

    {#if result.scope.kind === "portfolio"}
      <h3>{t("c.CostRiskView.by_plan")}</h3>
      <table>
        <caption class="sr-only">{t("c.CostRiskView.each_plan_s_budget_and", { currency: c.currency })}</caption>
        <thead><tr><th scope="col">{t("c.CostRiskView.plan")}</th><th scope="col">{t("c.CostRiskView.budget_2")}</th><th scope="col">{t("c.CostRiskView.mean_simulated_cost")}</th></tr></thead>
        <tbody>
          {#each c.plans as p (p.plan)}
            <tr><th scope="row">{p.name ?? p.plan}</th><td>{money(p.budget_minor, c.currency)}</td><td>{money(p.mean_minor, c.currency)}</td></tr>
          {/each}
        </tbody>
      </table>
    {/if}

    <h3>{t("c.CostRiskView.what_drives_it")}</h3>
    <table>
      <caption class="sr-only">{t("c.CostRiskView.per_line_estimate_and_sensitivity", { currency: c.currency })}</caption>
      <thead>
        <tr><th scope="col">{t("c.CostRiskView.line")}</th><th scope="col">{t("c.CostRiskView.low_planned_high")}</th><th scope="col">{t("c.CostRiskView.estimate")}</th><th scope="col">{t("c.CostRiskView.spent")}</th><th scope="col">{t("c.CostRiskView.sensitivity")}</th></tr>
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
      <h3>{t("c.CostRiskView.risks_that_added_cost")}</h3>
      <ul>
        {#each c.events as e (e.risk)}
          <li>{t("c.CostRiskView.adds", { risk: e.title ?? e.risk, money: money(e.impact_minor, c.currency), eventHitText: eventHitText(e.probability_bp, e.hit_bp) })}</li>
        {/each}
      </ul>
    {/if}
  </section>
{/each}

{#if result.events.skipped.length > 0}
  <p class="small">{t("c.CostRiskView.left_out")}</p>
  <ul class="small">
    {#each result.events.skipped as e (e.risk_pid)}<li>{e.title}: {costSkippedText(e.reason)}</li>{/each}
  </ul>
{/if}
{#if result.events.used.length === 0}
  <p class="small muted">{t("c.CostRiskView.no_register_risk_was_used")}</p>
{/if}
{#if result.events.risks_without_cost_fields > 0}
  <p class="small muted">{result.events.risks_without_cost_fields === 1 ? t("c.CostRiskView.risks_without_cost_one", { count: result.events.risks_without_cost_fields }) : t("c.CostRiskView.risks_without_cost_other", { count: result.events.risks_without_cost_fields })}</p>
{/if}

<h2>{t("c.CostRiskView.assumptions")}</h2>
<ul class="small">
  <li><strong>{t("c.CostRiskView.independence")}</strong> {a.independence}</li>
  <li>{a.floor}</li>
  <li>{a.currencies}</li>
  <li>{a.budget}</li>
  <li>{a.fallback}</li>
  <li>{t("c.CostRiskView.seed_the_same_seed_gives", { seed: a.seed, max_iterations: a.max_iterations })}</li>
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
