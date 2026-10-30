<!--
  The funding ledger (`/ledger`, EX-12): each allocation and what it bought. Untraced money is shown with its
  amount and why.
-->
<script lang="ts">
  import { rowOf } from "#lib/row-of.js";
  import { errorText } from "#lib/error-text.js";
  import { t } from "#lib/i18n.svelte.js";
  import { formatDate } from "#lib/locale-format.js";
  import { onMount } from "svelte";
  import { resource } from "#lib/resource.svelte.js";
  import { PpmClient, money, type FundingAllocation, type AllocationTrace } from "#lib/api/ppm.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";

  const ppm = PpmClient.withFetch();
  const loaded = resource(async () => (await ppm.fundingAllocations()).allocations, "p.ledger.could_not_load");
  const allocations = $derived(loaded.data);
  let trace = $state<AllocationTrace | null>(null);
  let traceError = $state<string | null>(null);
  const error = $derived(loaded.error ?? traceError);

  onMount(() => void loaded.reload());

  async function open(a: FundingAllocation) {
    try {
      trace = await ppm.traceAllocation(a.pid);
      traceError = null;
    } catch (err) {
      traceError = errorText(err, "p.ledger.could_not_trace");
    }
  }
  const cur = () => trace?.currency ?? "";
  const label = (a: FundingAllocation) => `${a.funding_source} ${formatDate(a.allocated_on)}`;
  const num = (a: unknown, b: unknown) => Number(a) - Number(b);
  const allocColumns: DataGridColumn[] = $derived([
    { id: "funding_source", header: t("p.ledger.col_source"), rowHeader: true },
    { id: "reference", header: t("p.ledger.col_ref"), format: (v) => (v ? String(v) : "") },
    { id: "allocated_on", header: t("p.ledger.col_on") },
    { id: "amount_minor", header: t("p.ledger.col_amount"), align: "end", compare: num, format: (v, r) => money(Number(v), String(r.currency)) },
    { id: "planned_on_lines_minor", header: t("p.ledger.col_planned"), align: "end", compare: num, format: (v, r) => money(Number(v), String(r.currency)) },
    { id: "untraced_minor", header: t("p.ledger.col_untraced"), align: "end", compare: num, format: (v, r) => money(Number(v), String(r.currency)), cellClass: (c) => (Number(c.value) !== 0 ? "cell-amber" : "") },
    { id: "lines", header: t("p.ledger.col_lines"), align: "end" },
    { id: "pid", header: t("p.ledger.col_act"), sortable: false, cell: traceCell },
  ]);
  const lineColumns: DataGridColumn[] = $derived([
    { id: "plan", header: t("p.ledger.line_plan"), rowHeader: true, format: (v) => (v ? String(v) : "") },
    { id: "description", header: t("p.ledger.line_desc") },
    { id: "planned_minor", header: t("p.ledger.line_planned"), align: "end", compare: num, format: (v) => money(Number(v), cur()) },
    { id: "actual_minor", header: t("p.ledger.line_actual"), align: "end", compare: num, format: (v) => money(Number(v), cur()) },
    { id: "gate", header: t("p.ledger.line_gate"), format: (v, r) => (v ? `${v}: ${r.released ? t("p.ledger.released") : t("p.ledger.held")}` : t("p.ledger.no_gate")) },
  ]);
</script>

{#snippet traceCell({ row }: DataGridCellContext)}
  {@const a = rowOf<FundingAllocation>(row)}
  <button class="button" type="button" aria-label={t("p.ledger.trace", { name: label(a) })} onclick={() => void open(a)}>{t("p.ledger.trace_short")}</button>
{/snippet}

<svelte:head><title>{t("p.ledger.title")}</title></svelte:head>

<h1>{t("p.ledger.h1")}</h1>
<p class="lede">{t("p.ledger.lede")}</p>
<p class="muted">{t("p.ledger.scope")}</p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}
{#if !allocations && !error}<p class="muted">{t("p.ledger.loading")}</p>{/if}

{#if allocations}
  {#if allocations.length === 0}
    <p>{t("p.ledger.none")}</p>
  {:else}
    <DataTable label={t("p.ledger.h1")} columns={allocColumns} rows={allocations} rowId={(r) => String(r.pid)} searchable />
  {/if}
{/if}

{#if trace}
  <section aria-labelledby="lg-trace">
    <h2 id="lg-trace">{t("p.ledger.trace_h2", { name: label(trace.allocation) })}</h2>
    <p>{t("p.ledger.money", { allocated: money(trace.money.allocated_minor, trace.currency), planned: money(trace.money.planned_on_lines_minor, trace.currency), spent: money(trace.money.spent_minor, trace.currency), untraced: money(trace.money.untraced_minor, trace.currency), currency: trace.currency })}</p>
    {#if trace.money.untraced_reason}<p class="banner" role="note">{trace.money.untraced_reason}</p>{/if}
    <p>{t("p.ledger.gates", { gated: trace.gates.lines_with_a_gate, released: trace.gates.released })}</p>
    <p>{t("p.ledger.milestones_met", { met: trace.milestones.met, total: trace.milestones.total })}</p>
    <p>{t("p.ledger.benefits", { n: trace.benefits.benefits })}
      {#each trace.benefits.validated_by_currency as v (v.currency)}{t("p.ledger.validated", { currency: v.currency, amount: money(v.validated_minor, v.currency) })}. {/each}
    </p>
    {#if trace.lines.length > 0}
      <h3>{t("p.ledger.lines_h3")}</h3>
      <DataTable label={t("p.ledger.lines_h3")} columns={lineColumns} rows={trace.lines} rowId={(r) => String(r.line_pid)} />
    {/if}
  </section>
{/if}
