<!--
  CFO financial area (`/financials`): budget variance (by collection,
  portfolio, and category) and per-currency estate exposure. All
  numbers are server-derived minor units formatted with `money()`;
  currencies are never merged and never converted (the server's note
  is displayed verbatim). English-first, like the other PPM views.
-->
<script lang="ts">
  import { resource } from "#lib/resource.svelte.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import {
    PpmClient,
    money,
    type FinancialExposure,
    type FinancialVariance,
    type VarianceRow,
  } from "#lib/api/ppm.js";

  const ppm = PpmClient.withFetch();
  const varianceLoad = resource<FinancialVariance>(() => ppm.financialVariance(), "ppm.common.loadFailed");
  const exposureLoad = resource<FinancialExposure>(() => ppm.financialExposure(), "ppm.common.loadFailed");
  const variance = $derived(varianceLoad.data);
  const exposure = $derived(exposureLoad.data);
  const error = $derived(varianceLoad.error ?? exposureLoad.error);

  onMount(() => {
    void varianceLoad.reload();
    void exposureLoad.reload();
  });

  const overrun = (c: DataGridCellContext) => ((c.row as { overrun?: boolean }).overrun ? "cell-red" : "");
  const exposureColumns: DataGridColumn[] = $derived([
    { id: "currency", header: t("p.financials.currency"), rowHeader: true },
    { id: "planned_minor", header: t("p.financials.planned"), align: "end", format: (v, r) => money(Number(v), String(r.currency)) },
    { id: "actual_minor", header: t("p.financials.actual"), align: "end", format: (v, r) => money(Number(v), String(r.currency)) },
    { id: "remaining_minor", header: t("p.financials.remaining"), align: "end", format: (v, r) => money(Number(v), String(r.currency)), cellClass: overrun },
    { id: "line_count", header: t("p.financials.lines"), align: "end" },
    { id: "plans", header: t("p.financials.work_items"), align: "end" },
  ]);
  type Group = { key: string; variance: VarianceRow[] };
  /** One row for each currency of each group; the group's name only on its first row. */
  const flat = (groups: Group[]) => groups.flatMap((g) => g.variance.map((row, i) => ({ ...row, group: i === 0 ? g.key : "" })));
  const varianceColumns = (label: string): DataGridColumn[] => [
    { id: "group", header: label, rowHeader: true },
    { id: "currency", header: t("p.financials.currency") },
    { id: "planned_minor", header: t("p.financials.planned"), align: "end", format: (v, r) => money(Number(v), String(r.currency)) },
    { id: "actual_minor", header: t("p.financials.actual"), align: "end", format: (v, r) => money(Number(v), String(r.currency)) },
    { id: "remaining_minor", header: t("p.financials.remaining"), align: "end", format: (v, r) => money(Number(v), String(r.currency)), cellClass: overrun },
    { id: "line_count", header: t("p.financials.lines"), align: "end" },
  ];
</script>

<svelte:head><title>{t("p.financials.ppm", { t: t("ppm.nav.financials") })}</title></svelte:head>

<h1>{t("ppm.nav.financials")}</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if exposure}
  <h2>{t("p.financials.currency_exposure")}</h2>
  <p class="muted">{t("p.financials.as_of", { as_of: exposure.as_of, note: exposure.note })}</p>
  <DataTable label={t("p.financials.currency_exposure")} columns={exposureColumns} rows={exposure.currencies} rowId={(r) => String(r.currency)} emptyText={t("p.financials.no_budget_lines_recorded_yet")} testid="fin-exposure" />
{/if}

{#snippet varianceTable(label: string, rows: Array<{ key: string; variance: VarianceRow[] }>, testid: string)}
  <h2>{label}</h2>
  <DataTable {label} columns={varianceColumns(label)} rows={flat(rows)} rowId={(_r, i) => String(i)} emptyText={t("p.financials.no_budget_lines_recorded_yet")} sortable={false} {testid} />
{/snippet}

{#if variance}
  {@render varianceTable(
    "By category",
    variance.by_category.map((g) => ({ key: g.category, variance: g.variance })),
    "fin-by-category",
  )}
  {@render varianceTable(
    "By collection",
    variance.by_collection.map((g) => ({ key: g.collection, variance: g.variance })),
    "fin-by-collection",
  )}
  {@render varianceTable(
    "By portfolio",
    variance.by_portfolio.map((g) => ({
      key: g.portfolio?.name ?? "(unassigned)",
      variance: g.variance,
    })),
    "fin-by-portfolio",
  )}
{/if}
