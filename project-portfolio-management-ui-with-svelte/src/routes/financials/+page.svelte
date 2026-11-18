<!--
  CFO financial area (`/financials`): budget variance (by collection,
  portfolio, and category) and per-currency estate exposure. All
  numbers are server-derived minor units formatted with `money()`;
  currencies are never merged and never converted (the server's note
  is displayed verbatim). English-first, like the other PPM views.
  The estimate at completion rolled up from a chosen plan over the plans beneath it (NC-1 slice 12 part f), a row per
  currency, in the catalogue's words (the roll-up's own `note` is not shown).
-->
<script lang="ts">
  import { resource } from "#lib/resource.svelte.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t, tn } from "#lib/i18n.svelte.js";
  import { formatDate } from "#lib/locale-format.js";
  import { onMount } from "svelte";
  import {
    PpmClient,
    money,
    type FinancialExposure,
    type FinancialVariance,
    type ForecastRollup,
    type VarianceRow,
  } from "#lib/api/ppm.js";
  import { PlanRepository } from "#lib/api/plans.js";
  import type { PlanRef } from "#lib/api/types.js";
  import { errorText } from "#lib/error-text.js";
  import { DEPTH_MAX, depthProblem, rollupRefusalKey } from "#lib/forecast-view.js";

  const ppm = PpmClient.withFetch();
  const plansRepo = PlanRepository.withFetch();

  // The estimate at completion rolled up from one plan over the plans beneath it (NC-1 slice 12 part f).
  let plans = $state<PlanRef[] | null>(null);
  let rollPlan = $state("");
  let depth = $state<number | null>(null);
  let rollup = $state<ForecastRollup | null>(null);
  let rollError = $state<string | null>(null);
  let rolling = $state(false);
  async function showRollup(e: SubmitEvent) {
    e.preventDefault();
    rollError = null;
    if (!rollPlan) return void (rollError = t("p.forecast.problem_plan"));
    const problem = depthProblem(depth);
    if (problem) return void (rollError = t(problem, { max: DEPTH_MAX }));
    rolling = true;
    try {
      rollup = await ppm.forecastRollup(rollPlan, depth ?? undefined);
    } catch (err) {
      rollup = null;
      const k = rollupRefusalKey(err);
      rollError = k ? t(k) : errorText(err, "p.forecast.could_not_read_rollup");
    } finally {
      rolling = false;
    }
  }
  const rollColumns: DataGridColumn[] = $derived([
    { id: "currency", header: t("p.financials.currency"), rowHeader: true },
    { id: "actual_minor", header: t("p.forecast.c_actual"), align: "end", format: (v, r) => money(Number(v), String(r.currency)) },
    { id: "estimate_to_complete_minor", header: t("p.forecast.c_etc"), align: "end", format: (v, r) => money(Number(v), String(r.currency)) },
    { id: "estimate_at_completion_minor", header: t("p.forecast.c_eac"), align: "end", format: (v, r) => money(Number(v), String(r.currency)) },
    { id: "plans", header: t("p.forecast.c_plans"), align: "end" },
  ]);
  const varianceLoad = resource<FinancialVariance>(() => ppm.financialVariance(), "ppm.common.loadFailed");
  const exposureLoad = resource<FinancialExposure>(() => ppm.financialExposure(), "ppm.common.loadFailed");
  const variance = $derived(varianceLoad.data);
  const exposure = $derived(exposureLoad.data);
  const error = $derived(varianceLoad.error ?? exposureLoad.error);

  onMount(() => {
    void varianceLoad.reload();
    void exposureLoad.reload();
    plansRepo.list().then((p) => (plans = p)).catch(() => (plans = []));
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
  <p class="muted">{t("p.financials.as_of", { as_of: formatDate(exposure.as_of), note: exposure.note })}</p>
  <DataTable label={t("p.financials.currency_exposure")} columns={exposureColumns} rows={exposure.currencies} rowId={(r) => String(r.currency)} emptyText={t("p.financials.no_budget_lines_recorded_yet")} testid="fin-exposure" />
{/if}

<section aria-labelledby="rollup-h" data-testid="fin-rollup">
  <h2 id="rollup-h">{t("p.forecast.rollup_heading")}</h2>
  <p class="small muted">{t("p.forecast.rollup_says", { max: DEPTH_MAX })}</p>
  <form class="row rollup" novalidate onsubmit={showRollup}>
    <label>
      {t("p.forecast.f_plan")}
      <select bind:value={rollPlan} disabled={plans === null}>
        <option value="">{t("p.forecast.choose_plan")}</option>
        {#each plans ?? [] as p (p.pid)}<option value={p.pid}>{p.name}</option>{/each}
      </select>
    </label>
    <label>
      {t("p.forecast.f_depth", { max: DEPTH_MAX })}
      <input type="number" min="1" max={DEPTH_MAX} step="1" bind:value={depth} />
    </label>
    <button class="button primary" type="submit" disabled={rolling || plans === null}>{t("p.forecast.show_rollup")}</button>
  </form>
  {#if rollError}<p class="banner error" role="alert">{rollError}</p>{/if}
  {#if rollup}
    <p class="small">{tn("p.forecast.rollup_line", rollup.tree.plans, { name: rollup.root.name, n: rollup.tree.plans, date: formatDate(rollup.as_of) })}</p>
    {#if rollup.tree.truncated}<p class="small">{t("p.forecast.rollup_truncated")}</p>{/if}
    {#if rollup.tree.revisits > 0}<p class="small">{tn("p.forecast.rollup_revisits", rollup.tree.revisits, { n: rollup.tree.revisits })}</p>{/if}
    {#if rollup.rollup.no_currency_signal > 0}<p class="small">{tn("p.forecast.rollup_no_currency", rollup.rollup.no_currency_signal, { n: rollup.rollup.no_currency_signal })}</p>{/if}
    <DataTable label={t("p.forecast.rollup_heading")} columns={rollColumns} rows={rollup.rollup.rows} rowId={(r) => String(r.currency)} emptyText={t("p.forecast.rollup_empty")} sortable={false} testid="fin-rollup-table" />
  {/if}
</section>

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
