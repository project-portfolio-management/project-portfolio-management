<!--
  CFO financial area (`/financials`): budget variance (by collection,
  portfolio, and category) and per-currency estate exposure. All
  numbers are server-derived minor units formatted with `money()`;
  currencies are never merged and never converted (the server's note
  is displayed verbatim). English-first, like the other PPM views.
-->
<script lang="ts">
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
  let variance = $state<FinancialVariance | null>(null);
  let exposure = $state<FinancialExposure | null>(null);
  let error = $state<string | null>(null);

  onMount(async () => {
    try {
      variance = await ppm.financialVariance();
      exposure = await ppm.financialExposure();
    } catch (err) {
      error = err instanceof Error ? err.message : t("ppm.common.loadFailed");
    }
  });
</script>

<svelte:head><title>{t("p.financials.ppm", { t: t("ppm.nav.financials") })}</title></svelte:head>

<h1>{t("ppm.nav.financials")}</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if exposure}
  <h2>{t("p.financials.currency_exposure")}</h2>
  <p class="muted">{t("p.financials.as_of", { as_of: exposure.as_of, note: exposure.note })}</p>
  <table data-testid="fin-exposure">
    <thead>
      <tr>
        <th>{t("p.financials.currency")}</th><th>{t("p.financials.planned")}</th><th>{t("p.financials.actual")}</th><th>{t("p.financials.remaining")}</th>
        <th>{t("p.financials.lines")}</th><th>{t("p.financials.work_items")}</th>
      </tr>
    </thead>
    <tbody>
      {#each exposure.currencies as row (row.currency)}
        <tr class:overrun={row.overrun}>
          <td><strong>{row.currency}</strong></td>
          <td>{money(row.planned_minor, row.currency)}</td>
          <td>{money(row.actual_minor, row.currency)}</td>
          <td>{money(row.remaining_minor, row.currency)}</td>
          <td>{row.line_count}</td>
          <td>{row.plans}</td>
        </tr>
      {:else}
        <tr><td colspan="6" class="muted">{t("p.financials.no_budget_lines_recorded_yet")}</td></tr>
      {/each}
    </tbody>
  </table>
{/if}

{#snippet varianceTable(label: string, rows: Array<{ key: string; variance: VarianceRow[] }>, testid: string)}
  <h2>{label}</h2>
  <table data-testid={testid}>
    <thead>
      <tr>
        <th>{label}</th><th>{t("p.financials.currency")}</th><th>{t("p.financials.planned")}</th><th>{t("p.financials.actual")}</th>
        <th>{t("p.financials.remaining")}</th><th>{t("p.financials.lines")}</th>
      </tr>
    </thead>
    <tbody>
      {#each rows as group, gi (`${gi}:${group.key}`)}
        {#each group.variance as row, index (row.currency)}
          <tr class:overrun={row.overrun}>
            <td>{#if index === 0}<strong>{group.key}</strong>{/if}</td>
            <td>{row.currency}</td>
            <td>{money(row.planned_minor, row.currency)}</td>
            <td>{money(row.actual_minor, row.currency)}</td>
            <td>{money(row.remaining_minor, row.currency)}</td>
            <td>{row.line_count}</td>
          </tr>
        {/each}
      {:else}
        <tr><td colspan="6" class="muted">{t("p.financials.no_budget_lines_recorded_yet")}</td></tr>
      {/each}
    </tbody>
  </table>
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

<style>
  tr.overrun td { color: #b91c1c; }
</style>
