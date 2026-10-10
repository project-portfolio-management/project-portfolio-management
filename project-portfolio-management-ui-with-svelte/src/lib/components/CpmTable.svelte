<!--
  The table twin of the critical-path chart (SC-1): every number the chart
  draws, as a real table a screen reader or a keyboard can walk, plus the
  columns the chart cannot show (free float, constraint, deadline, calendar).
-->
<script lang="ts">
  import { t } from "#lib/i18n.svelte.js";
  import type { CpmNode } from "#lib/api/ppm.js";
  import { floatLabel, floatState } from "#lib/cpm-view.js";

  let { nodes }: { nodes: CpmNode[] } = $props();
</script>

<div class="scroll">
  <table data-testid="cpm-table">
    <caption class="sr-only">{t("c.CpmTable.task_schedule_with_float_in")}</caption>
    <thead>
      <tr>
        <th scope="col">{t("c.CpmTable.task")}</th>
        <th scope="col">{t("c.CpmTable.starts")}</th>
        <th scope="col">{t("c.CpmTable.finishes")}</th>
        <th scope="col">{t("c.CpmTable.total_float")}</th>
        <th scope="col">{t("c.CpmTable.free_float")}</th>
        <th scope="col">{t("c.CpmTable.state")}</th>
        <th scope="col">{t("c.CpmTable.constraint")}</th>
        <th scope="col">{t("c.CpmTable.deadline")}</th>
        <th scope="col">{t("c.CpmTable.calendar")}</th>
      </tr>
    </thead>
    <tbody>
      {#each nodes as n (n.pid)}
        {@const st = floatState(n)}
        <tr class={st.key} id={`task-${n.pid}`}>
          <th scope="row">{n.title ?? t("c.CpmTable.untitled")}</th>
          <td>{n.early_start}</td>
          <td>{n.early_finish}</td>
          <td>{n.total_float}</td>
          <td>{n.free_float}</td>
          <td><span aria-hidden="true">{st.marker}</span> {floatLabel(n)}</td>
          <td>
            {#if n.constraint}{n.constraint.toUpperCase()} {n.constraint_date}{:else}—{/if}
            {#if n.hard_conflict}<strong> {t("c.CpmTable.overrides_its_predecessors")}</strong>{/if}
          </td>
          <td>{n.deadline ?? "—"}</td>
          <td>{n.calendar === "plan+person" ? t("c.CpmTable.plan_assignee") : t("c.CpmTable.plan")}</td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>

<style>
  .scroll {
    overflow-x: auto;
  }
  table {
    border-collapse: collapse;
    width: 100%;
    font-size: 0.9em;
  }
  th,
  td {
    border: 1px solid var(--border, #ccc);
    padding: 0.25rem 0.45rem;
    text-align: start;
  }
  tr.critical th[scope="row"] {
    border-inline-start: 4px solid #a4262c;
  }
  tr.late th[scope="row"] {
    border-inline-start: 4px dashed #7f1d1d;
  }
  .sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    overflow: hidden;
    clip-path: inset(50%);
    white-space: nowrap;
  }
</style>
