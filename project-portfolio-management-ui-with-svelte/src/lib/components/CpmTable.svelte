<!--
  The table twin of the critical-path chart (SC-1): every number the chart
  draws, as a real table a screen reader or a keyboard can walk, plus the
  columns the chart cannot show (free float, constraint, deadline, calendar).
-->
<script lang="ts">
  import type { CpmNode } from "#lib/api/ppm.js";
  import { floatLabel, floatState } from "#lib/cpm-view.js";

  let { nodes }: { nodes: CpmNode[] } = $props();
</script>

<div class="scroll">
  <table data-testid="cpm-table">
    <caption class="sr-only">Task schedule with float, in the plan's working days</caption>
    <thead>
      <tr>
        <th scope="col">Task</th>
        <th scope="col">Starts</th>
        <th scope="col">Finishes</th>
        <th scope="col">Total float</th>
        <th scope="col">Free float</th>
        <th scope="col">State</th>
        <th scope="col">Constraint</th>
        <th scope="col">Deadline</th>
        <th scope="col">Calendar</th>
      </tr>
    </thead>
    <tbody>
      {#each nodes as n (n.pid)}
        {@const st = floatState(n)}
        <tr class={st.key}>
          <th scope="row">{n.title ?? "(untitled)"}</th>
          <td>{n.early_start}</td>
          <td>{n.early_finish}</td>
          <td>{n.total_float}</td>
          <td>{n.free_float}</td>
          <td><span aria-hidden="true">{st.marker}</span> {floatLabel(n)}</td>
          <td>
            {#if n.constraint}{n.constraint.toUpperCase()} {n.constraint_date}{:else}—{/if}
            {#if n.hard_conflict}<strong> (overrides its predecessors)</strong>{/if}
          </td>
          <td>{n.deadline ?? "—"}</td>
          <td>{n.calendar === "plan+person" ? "plan + assignee" : "plan"}</td>
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
