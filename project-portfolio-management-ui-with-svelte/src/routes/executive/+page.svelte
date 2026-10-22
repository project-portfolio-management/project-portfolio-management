<!--
  CEO executive area (`/executive`): the portfolio-health briefing
  (server-derived RAG rollup — the derivation string is displayed, not
  re-computed here), the decision log, and benefits realization.
  All three views are ETag-conditional server derivations carrying
  `as_of`; the client formats, it does not compute. English-first
  (locale catalogues extend as a follow-up, matching the other PPM
  views).
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import {
    PpmClient,
    money,
    type DecisionEntry,
    type ExecutiveBenefits,
    type ExecutiveHealth,
    type AlignmentCoverage,
  } from "#lib/api/ppm.js";

  const ppm = PpmClient.withFetch();
  let health = $state<ExecutiveHealth | null>(null);
  let decisions = $state<DecisionEntry[] | null>(null);
  let benefits = $state<ExecutiveBenefits | null>(null);
  let alignment = $state<AlignmentCoverage | null>(null);
  let error = $state<string | null>(null);

  onMount(async () => {
    try {
      health = await ppm.executiveHealth();
      decisions = (await ppm.executiveDecisions(30)).decisions;
      benefits = await ppm.executiveBenefits();
      alignment = await ppm.executiveAlignment();
    } catch (err) {
      error = errorText(err, "ppm.common.loadFailed");
    }
  });

  const bucketName = (p: { name: string } | null) => p?.name ?? "(unassigned)";

  type Health = ExecutiveHealth["portfolios"][number];
  type BenefitPortfolio = ExecutiveBenefits["portfolios"][number];
  type Dec = DecisionEntry;
  const healthColumns: DataGridColumn[] = $derived([
    { id: "portfolio", header: t("p.executive.portfolio"), rowHeader: true, accessor: (r) => bucketName((r as unknown as Health).portfolio) },
    { id: "status", header: t("p.executive.status"), cell: statusCell, cellClass: (c) => ({ red: "cell-red", amber: "cell-amber", green: "cell-green" })[String(c.value)] ?? "" },
    { id: "members", header: t("p.executive.members"), align: "end" },
    { id: "red", header: t("ppm.dashboard.red"), align: "end", accessor: (r) => (r as unknown as Health).rag.red },
    { id: "amber", header: t("ppm.dashboard.amber"), align: "end", accessor: (r) => (r as unknown as Health).rag.amber },
    { id: "green", header: t("ppm.dashboard.green"), align: "end", accessor: (r) => (r as unknown as Health).rag.green },
    { id: "overdue_milestones", header: t("p.executive.overdue_milestones"), align: "end" },
    { id: "escalated_risks", header: t("p.executive.escalated_risks"), align: "end" },
    { id: "open_risk_exposure", header: t("p.executive.open_exposure"), align: "end" },
    { id: "overrun_currencies", header: t("p.executive.overrun_currencies"), accessor: (r) => (r as unknown as Health).overrun_currencies.join(", ") || "—" },
    { id: "days_since_last_update", header: t("p.executive.calendar_days_since_update"), align: "end" },
  ]);
  type BRow = { portfolio: string; benefits: number | string; currency: string; target: string; realized: string; ratio: number | null | undefined; nonFinancial: number | null };
  /** One row for each currency line of each portfolio; a portfolio with no money lines gets a single row that says so. */
  const benefitRows = (ps: BenefitPortfolio[]): BRow[] =>
    ps.flatMap((row): BRow[] =>
      row.financial.length === 0
        ? [{ portfolio: bucketName(row.portfolio), benefits: row.benefits, currency: "", target: "", realized: "", ratio: undefined, nonFinancial: row.non_financial }]
        : row.financial.map((line, i) => ({ portfolio: i === 0 ? bucketName(row.portfolio) : "", benefits: i === 0 ? row.benefits : "", currency: line.currency, target: money(line.target_minor, line.currency), realized: money(line.realized_minor, line.currency), ratio: line.realization_ratio, nonFinancial: null })),
    );
  const benefitColumns: DataGridColumn[] = $derived([
    { id: "portfolio", header: t("p.executive.portfolio"), rowHeader: true },
    { id: "benefits", header: t("p.executive.benefits"), align: "end" },
    { id: "currency", header: t("p.executive.currency") },
    { id: "target", header: t("p.executive.target"), align: "end" },
    { id: "realized", header: t("p.executive.realized"), align: "end" },
    { id: "ratio", header: t("p.executive.realization"), align: "end", cell: ratioCell },
  ]);
  const alignmentColumns: DataGridColumn[] = $derived([
    { id: "collection", header: t("p.executive.collection"), rowHeader: true },
    { id: "total", header: t("p.executive.total"), align: "end" },
    { id: "aligned", header: t("p.executive.aligned"), align: "end" },
    { id: "unaligned", header: t("p.executive.unaligned"), align: "end", cellClass: (c) => (Number(c.value) > 0 ? "cell-amber" : "") },
  ]);
  const decisionColumns: DataGridColumn[] = $derived([
    { id: "at", header: t("p.executive.when"), rowHeader: true },
    { id: "kind", header: t("p.executive.kind"), accessor: (r) => { const e = r as unknown as Dec; return `${e.kind}${e.gate ? ` · ${e.gate}` : ""}`; } },
    { id: "decision", header: t("p.executive.decision") },
    { id: "subject", header: t("p.executive.subject"), accessor: (r) => { const s = (r as unknown as Dec).subject; return s.name ?? s.pid; } },
    { id: "actor", header: t("p.executive.actor"), accessor: (r) => { const e = r as unknown as Dec; return e.actor ?? e.sponsor ?? "—"; } },
  ]);
</script>

{#snippet statusCell({ value }: DataGridCellContext)}<span class={`rag rag-${value}`}>{value}</span>{/snippet}

{#snippet ratioCell({ row, value }: DataGridCellContext)}
  {@const r = row as unknown as BRow}
  {#if r.nonFinancial !== null}
    <span class="muted">{t("p.executive.non_financial_only", { non_financial: r.nonFinancial })}</span>
  {:else if value === null}
    <span class="muted">{t("p.executive.no_target")}</span>
  {:else if typeof value === "number"}
    {(value * 100).toFixed(0)}%
  {/if}
{/snippet}


<svelte:head><title>{t("p.executive.ppm", { t: t("ppm.nav.executive") })}</title></svelte:head>

<h1>{t("ppm.nav.executive")}</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if health}
  <h2>{t("p.executive.portfolio_health")}</h2>
  <p class="muted">{t("p.executive.as_of", { as_of: health.as_of, derivation: health.derivation })}</p>
  <DataTable label={t("p.executive.portfolio")} columns={healthColumns} rows={health.portfolios} rowId={(r) => String((r as unknown as Health).portfolio?.pid ?? "unassigned")} testid="exec-health" />
{/if}

{#if benefits}
  <h2>{t("p.executive.benefits_realization")}</h2>
  <p class="muted">{benefits.note}</p>
  <DataTable label={t("p.executive.benefits")} columns={benefitColumns} rows={benefitRows(benefits.portfolios)} rowId={(_r, i) => String(i)} sortable={false} testid="exec-benefits" />
{/if}

{#if alignment}
  <h2>{t("p.executive.strategic_alignment")}</h2>
  <p class="muted">{alignment.derivation}</p>
  <DataTable label={t("p.executive.collection")} columns={alignmentColumns} rows={alignment.by_collection} rowId={(r) => String(r.collection)} testid="exec-alignment" />
  {#if alignment.unaligned_spend.length > 0}
    <h3>{t("p.executive.unaligned_spend")}</h3>
    <ul data-testid="exec-unaligned-spend">
      {#each alignment.unaligned_spend as row (row.currency)}
        <li><strong>{money(row.planned_minor, row.currency)}</strong> {t("p.executive.planned_with_no_objective")}</li>
      {/each}
    </ul>
  {/if}
  {#if alignment.unaligned_items.length > 0}
    <h3>{t("p.executive.largest_unaligned_items")}</h3>
    <ul data-testid="exec-unaligned-items">
      {#each alignment.unaligned_items as row (row.item.pid)}
        <li>
          {row.item.name} <span class="muted">({row.item.kind})</span>
          {#each row.planned as line (line.currency)}
            · {money(line.planned_minor, line.currency)}
          {/each}
        </li>
      {/each}
    </ul>
  {/if}
{/if}

{#if decisions}
  <h2>{t("p.executive.decision_log")}</h2>
  <DataTable label={t("p.executive.decision")} columns={decisionColumns} rows={decisions} rowId={(r) => String(r.kind) + String(r.at) + String((r as unknown as Dec).subject.pid)} emptyText={t("p.executive.no_decisions_recorded_yet")} testid="exec-decisions" pageSize={50} searchable />
{/if}

<style>
  .rag { padding: 0.1rem 0.5rem; border-radius: 999px; color: #fff; }
  .rag-red { background: #b91c1c; }
  .rag-amber { background: #b45309; }
  .rag-green { background: #15803d; }
</style>
