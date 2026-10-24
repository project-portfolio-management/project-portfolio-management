<!--
  Schedule variance against a baseline (SC-3). Every row says what happened in
  words and a marker (▲ slipped, ▼ pulled in, = on baseline, + added, − removed,
  ? cannot say), because a bare number or a colour is how a status report lies.
  A task **added** since the baseline has no slip; one **removed** stays listed
  with its baselined dates; one unscheduled on either side has no variance.
-->
<script lang="ts">
  import { rowOf } from "#lib/row-of.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t } from "#lib/i18n.svelte.js";
  import type { ScheduleVariance } from "#lib/api/ppm.js";
  import { standingText } from "#lib/cpm-view.js";

  let { variance }: { variance: NonNullable<ScheduleVariance["variance"]> } = $props();
  const s = $derived(variance.summary);
  const sign = (n: number | null) => (n === null ? "—" : n > 0 ? `+${n}` : String(n));

  type VRow = typeof variance.rows[number];
  const columns: DataGridColumn[] = $derived([
    { id: "title", header: t("c.VarianceTable.task"), rowHeader: true },
    { id: "standing", header: t("c.VarianceTable.standing"), accessor: (r) => { const x = standingText(rowOf<VRow>(r)); return `${x.word}: ${x.detail}`; }, cell: standingCell },
    { id: "baseline", header: t("c.VarianceTable.baselined"), accessor: (r) => `${rowOf<VRow>(r).baseline_start ?? "—"} → ${rowOf<VRow>(r).baseline_finish ?? "—"}` },
    { id: "now", header: t("c.VarianceTable.now"), accessor: (r) => `${rowOf<VRow>(r).start ?? "—"} → ${rowOf<VRow>(r).finish ?? "—"}` },
    { id: "start_variance_days", header: t("c.VarianceTable.start"), align: "end", format: (v) => sign(v as number | null) },
    { id: "finish_variance_days", header: t("c.VarianceTable.finish"), align: "end", format: (v) => sign(v as number | null) },
    { id: "duration_variance_days", header: t("c.VarianceTable.duration"), align: "end", format: (v) => sign(v as number | null) },
  ]);
</script>

{#snippet standingCell({ row, formatted }: DataGridCellContext)}<span aria-hidden="true">{standingText(rowOf<VRow>(row)).marker}</span> {formatted}{/snippet}


<div data-testid="variance">
  <p class="small">
    {t("c.VarianceTable.matched_of_baselined", { matched: s.matched, baseline_tasks: s.baseline_tasks })}
    {t("c.VarianceTable.slipped_pulled_on", { slipped: s.slipped, pulled_in: s.pulled_in, on_baseline: s.on_baseline, unmeasurable: s.unmeasurable > 0 ? t("c.VarianceTable.cannot_be_measured", { unmeasurable: s.unmeasurable }) : "" })}
    {t("c.VarianceTable.added_removed", { added: s.added, removed: s.removed })}
    {#if s.plan_finish_slip_days === null}
      {t("c.VarianceTable.the_plan_s_own_finish")}
    {:else if s.plan_finish_slip_days === 0}
      {t("c.VarianceTable.the_plan_s_finish_is")}
    {:else}
      {t(s.plan_finish_slip_days > 0 ? (Math.abs(s.plan_finish_slip_days) === 1 ? "c.VarianceTable.finish_later_one" : "c.VarianceTable.finish_later_other") : (Math.abs(s.plan_finish_slip_days) === 1 ? "c.VarianceTable.finish_earlier_one" : "c.VarianceTable.finish_earlier_other"), { days: Math.abs(s.plan_finish_slip_days), note: s.removed > 0 && s.plan_finish_slip_days < 0 ? t("c.VarianceTable.check_removed") : "" })}
    {/if}
  </p>
    <DataTable label={t("c.VarianceTable.schedule_variance_against_the_baseli")} columns={columns} rows={variance.rows} rowId={(r) => String(r.task_pid)} />
</div>
