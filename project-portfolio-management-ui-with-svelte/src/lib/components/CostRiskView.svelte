<!--
  The cost-risk result, per currency (never added across): budget, mean, cost at
  P10 / P50 / P80 / P90, the chance of staying within budget, a histogram with
  P50, P80, P90 and the budget marked by labelled lines — never colour alone — and
  the same figures as a table; which lines drive it; which risks struck how often;
  and every assumption, including that the inputs are independent.
-->
<script lang="ts">
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t } from "#lib/i18n.svelte.js";
  import { money, type CostRisk } from "#lib/api/ppm.js";
  import { budgetConfidenceText, costHistogramGeometry, costSkippedText, eventHitText, lineSourceText, sensitivityText } from "#lib/scheduling-view.js";

  let { result }: { result: CostRisk } = $props();
  const a = $derived(result.assumptions);

  type Cur = CostRisk["currencies"][number];
  type Line = Cur["lines"][number];
  const outcomeColumns: DataGridColumn[] = $derived([
    { id: "label", header: t("c.CostRiskView.outcome"), rowHeader: true },
    { id: "value", header: t("c.CostRiskView.cost"), align: "end" },
  ]);
  const outcomeRows = (c: Cur) => [
    { label: t("c.CostRiskView.budget_sum_of_planned_amounts"), value: money(c.budget_minor, c.currency) },
    { label: t("c.CostRiskView.already_spent"), value: money(c.spent_minor, c.currency) },
    { label: t("c.CostRiskView.mean_simulated_cost"), value: money(c.mean_minor, c.currency) },
    ...c.percentiles.map((p) => ({ label: `P${p.p}`, value: money(p.minor, c.currency) })),
  ];
  const histogramColumns = (cur: string): DataGridColumn[] => [
    { id: "from_minor", header: t("c.CostRiskView.cost_from"), rowHeader: true, align: "end", format: (v) => money(Number(v), cur) },
    { id: "to_minor", header: t("c.CostRiskView.to"), align: "end", format: (v) => money(Number(v), cur) },
    { id: "count", header: t("c.CostRiskView.simulations"), align: "end" },
  ];
  const planColumns = (cur: string): DataGridColumn[] => [
    { id: "name", header: t("c.CostRiskView.plan"), rowHeader: true, accessor: (r) => (r as { name?: string | null; plan: string }).name ?? (r as { plan: string }).plan },
    { id: "budget_minor", header: t("c.CostRiskView.budget_2"), align: "end", format: (v) => money(Number(v), cur) },
    { id: "mean_minor", header: t("c.CostRiskView.mean_simulated_cost"), align: "end", format: (v) => money(Number(v), cur) },
  ];
  const lineColumns = (cur: string): DataGridColumn[] => [
    { id: "description", header: t("c.CostRiskView.line"), rowHeader: true, accessor: (r) => (r as unknown as Line).description ?? (r as unknown as Line).pid, cell: lineNameCell },
    { id: "range", header: t("c.CostRiskView.low_planned_high"), sortable: false, accessor: (r) => { const l = r as unknown as Line; return `${money(l.low, cur)} / ${money(l.likely, cur)} / ${money(l.high, cur)}`; } },
    { id: "estimate", header: t("c.CostRiskView.estimate"), accessor: (r) => lineSourceText(r as unknown as Line), cell: lineSourceCell },
    { id: "actual", header: t("c.CostRiskView.spent"), align: "end", format: (v) => money(Number(v), cur) },
    { id: "sensitivity", header: t("c.CostRiskView.sensitivity"), accessor: (r) => sensitivityText((r as unknown as Line).sensitivity) },
  ];
</script>

{#snippet lineNameCell({ row, formatted }: DataGridCellContext)}{formatted}{#if row.category} <span class="muted small">({row.category})</span>{/if}{/snippet}

{#snippet lineSourceCell({ row, formatted }: DataGridCellContext)}{#if row.source === "fallback"}<span aria-hidden="true">▲</span> {/if}{formatted}{/snippet}


<p class="small muted">
  {result.scope.kind === "portfolio" ? (result.scope.plans === 1 ? t("c.CostRiskView.scope_all_one", { plans: result.scope.plans }) : t("c.CostRiskView.scope_all_other", { plans: result.scope.plans })) : t("c.CostRiskView.scope_this_plan")} ·
  {t("c.CostRiskView.run_summary", { seed: a.seed, iterations: a.iterations, distribution: a.distribution === "pert" ? t("c.CostRiskView.dist_pert") : t("c.CostRiskView.dist_triangular") })}
</p>

{#each result.currencies as c (c.currency)}
  {@const g = costHistogramGeometry(c.histogram, c.percentiles, c.budget_minor)}
  <section aria-labelledby={`cur-${c.currency}`}>
    <h2 id={`cur-${c.currency}`}>{c.currency}</h2>
      <DataTable label={t("c.CostRiskView.cost_at_each_percentile", { currency: c.currency })} columns={outcomeColumns} rows={outcomeRows(c)} rowId={(r) => String(r.label)} sortable={false} />
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
          <DataTable label={t("c.CostRiskView.cost_from")} columns={histogramColumns(c.currency)} rows={c.histogram} rowId={(r) => String(r.from_minor)} sortable={false} />
        </details>
        <figcaption class="small muted">{t("c.CostRiskView.each_bar_is_a_cost")}</figcaption>
      </figure>
    {/if}

    {#if result.scope.kind === "portfolio"}
      <h3>{t("c.CostRiskView.by_plan")}</h3>
      <DataTable label={t("c.CostRiskView.each_plan_s_budget_and", { currency: c.currency })} columns={planColumns(c.currency)} rows={c.plans} rowId={(r) => String(r.plan)} />
    {/if}

    <h3>{t("c.CostRiskView.what_drives_it")}</h3>
      <DataTable label={t("c.CostRiskView.per_line_estimate_and_sensitivity", { currency: c.currency })} columns={lineColumns(c.currency)} rows={c.lines} rowId={(r) => String(r.pid)} />

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
</style>
