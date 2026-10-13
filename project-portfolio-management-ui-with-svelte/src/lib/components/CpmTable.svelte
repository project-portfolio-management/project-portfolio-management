<!--
  The table twin of the critical-path chart (SC-1): every number the chart
  draws, as a real table a screen reader or a keyboard can walk, plus the
  columns the chart cannot show (free float, constraint, deadline, calendar).
-->
<script lang="ts">
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t } from "#lib/i18n.svelte.js";
  import type { CpmNode } from "#lib/api/ppm.js";
  import { floatLabel, floatState } from "#lib/cpm-view.js";

  let { nodes }: { nodes: CpmNode[] } = $props();

  const columns: DataGridColumn[] = $derived([
    { id: "title", header: t("c.CpmTable.task"), rowHeader: true, format: (v) => (v ? String(v) : t("c.CpmTable.untitled")) },
    { id: "early_start", header: t("c.CpmTable.starts") },
    { id: "early_finish", header: t("c.CpmTable.finishes") },
    { id: "total_float", header: t("c.CpmTable.total_float"), align: "end" },
    { id: "free_float", header: t("c.CpmTable.free_float"), align: "end" },
    { id: "state", header: t("c.CpmTable.state"), accessor: (r) => floatLabel(r as unknown as CpmNode), cell: stateCell },
    { id: "constraint", header: t("c.CpmTable.constraint"), accessor: (r) => { const n = r as unknown as CpmNode; return n.constraint ? `${n.constraint.toUpperCase()} ${n.constraint_date}` : "—"; }, cell: constraintCell },
    { id: "deadline", header: t("c.CpmTable.deadline"), format: (v) => (v ? String(v) : "—") },
    { id: "calendar", header: t("c.CpmTable.calendar"), format: (v) => (v === "plan+person" ? t("c.CpmTable.plan_assignee") : t("c.CpmTable.plan")) },
  ]);
</script>

{#snippet stateCell({ row, formatted }: DataGridCellContext)}<span aria-hidden="true">{floatState(row as unknown as CpmNode).marker}</span> {formatted}{/snippet}

{#snippet constraintCell({ row, formatted }: DataGridCellContext)}
  {formatted}{#if (row as unknown as CpmNode).hard_conflict}<strong> {t("c.CpmTable.overrides_its_predecessors")}</strong>{/if}
{/snippet}


  <DataTable label={t("c.CpmTable.task_schedule_with_float_in")} columns={columns} rows={nodes} rowId={(r) => String(r.pid)} rowAttrs={(r) => ({ id: `task-${r.pid}` })} rowClass={(r) => floatState(r as unknown as CpmNode).key} testid="cpm-table" />

<style>
  /* A critical task is marked at the start of its row, and a late one with a dashed mark: the row also says so in words. */
  :global(.data-table tr.critical th[scope="row"]) { border-inline-start: 4px solid #a4262c; }
  :global(.data-table tr.late th[scope="row"]) { border-inline-start: 4px dashed #7f1d1d; }
</style>
