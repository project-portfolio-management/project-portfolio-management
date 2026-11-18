<!--
  A scorecard's KPIs by finished quarter (`/deescalation`, EX-10, NC-1 slice 11): each KPI's target and, per quarter,
  met, not met or nothing recorded, as a word and a symbol (never a colour alone), with the reading. A KPI is met in a
  quarter when the latest reading dated in it is at or better than its target; with no reading it is missing, which is
  not the same as not met. A KPI made from the scorecard template is named from the catalogue by its `template_key`
  (NC-1 slice 12 part a); any other keeps its stored name.
-->
<script lang="ts">
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import DataTable from "#lib/components/DataTable.svelte";
  import { t, tDynamic } from "#lib/i18n.svelte.js";
  import { rowOf } from "#lib/row-of.js";
  import type { ScorecardKpi } from "#lib/api/ppm-types.js";
  import { STATE_MARK, kpiName, quarterName, stateOf } from "#lib/deescalation-view.js";

  let { kpis, quarters, label }: { kpis: ScorecardKpi[]; quarters: string[]; label: string } = $props();

  const cellOf = (r: ScorecardKpi, q: string) => r.quarters.find((x) => x.quarter === q);

  const columns: DataGridColumn[] = $derived([
    {
      id: "name",
      header: t("p.deescalation.col_kpi"),
      rowHeader: true,
      accessor: (r) => kpiName(rowOf<ScorecardKpi>(r), (key) => tDynamic(`p.deescalation.measure_${key}`)),
    },
    {
      id: "target_value",
      header: t("p.deescalation.col_target"),
      accessor: (r) => {
        const k = rowOf<ScorecardKpi>(r);
        return t(k.direction === "lower_better" ? "p.deescalation.target_at_most" : "p.deescalation.target_at_least", {
          value: k.target_value,
          unit: k.unit ? unitWord(k.unit) : "",
        }).trim();
      },
    },
    ...quarters.map(
      (q): DataGridColumn => ({
        id: `q-${q}`,
        header: quarterName(q, t),
        accessor: (r) => stateOf(cellOf(rowOf<ScorecardKpi>(r), q)?.met),
        format: (v, r) => {
          const c = cellOf(rowOf<ScorecardKpi>(r), q);
          const word = t(STATE_MARK[v as keyof typeof STATE_MARK].key);
          return c?.value === null || c?.value === undefined ? word : t("p.deescalation.state_with_value", { state: word, value: c.value });
        },
        cell: stateCell,
      }),
    ),
  ]);

  function unitWord(unit: string): string {
    if (unit === "percent") return t("p.deescalation.unit_percent");
    // A count needs no word after the number.
    if (unit === "count") return "";
    return unit;
  }
</script>

{#snippet stateCell({ value, formatted }: DataGridCellContext)}
  <span aria-hidden="true">{STATE_MARK[value as keyof typeof STATE_MARK].symbol}</span>
  {formatted}
{/snippet}

<DataTable {label} {columns} rows={kpis} rowId={(r) => String(r.kpi_pid)} emptyText={t("p.deescalation.no_kpis")} />
