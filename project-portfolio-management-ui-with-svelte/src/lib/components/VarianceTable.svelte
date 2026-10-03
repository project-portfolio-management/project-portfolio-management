<!--
  Schedule variance against a baseline (SC-3). Every row says what happened in
  words and a marker (▲ slipped, ▼ pulled in, = on baseline, + added, − removed,
  ? cannot say), because a bare number or a colour is how a status report lies.
  A task **added** since the baseline has no slip; one **removed** stays listed
  with its baselined dates; one unscheduled on either side has no variance.
-->
<script lang="ts">
  import type { ScheduleVariance } from "#lib/api/ppm.js";
  import { standingText } from "#lib/cpm-view.js";

  let { variance }: { variance: NonNullable<ScheduleVariance["variance"]> } = $props();
  const s = $derived(variance.summary);
  const sign = (n: number | null) => (n === null ? "—" : n > 0 ? `+${n}` : String(n));
</script>

<div data-testid="variance">
  <p class="small">
    {s.matched} of {s.baseline_tasks} baselined tasks are still in the plan:
    {s.slipped} slipped, {s.pulled_in} pulled in, {s.on_baseline} on baseline{s.unmeasurable > 0 ? `, ${s.unmeasurable} cannot be measured` : ""}.
    {s.added} added and {s.removed} removed since.
    {#if s.plan_finish_slip_days === null}
      The plan's own finish cannot be compared.
    {:else if s.plan_finish_slip_days === 0}
      The plan's finish is unchanged.
    {:else}
      The plan's finish is {Math.abs(s.plan_finish_slip_days)} working day{Math.abs(s.plan_finish_slip_days) === 1 ? "" : "s"}
      {s.plan_finish_slip_days > 0 ? "later" : "earlier"} than baselined{s.removed > 0 && s.plan_finish_slip_days < 0 ? " (check the removed tasks: deleting work is not an improvement)" : ""}.
    {/if}
  </p>
  <div class="scroll">
    <table>
      <caption class="sr-only">Schedule variance against the baseline, in working days</caption>
      <thead>
        <tr>
          <th scope="col">Task</th>
          <th scope="col">Standing</th>
          <th scope="col">Baselined</th>
          <th scope="col">Now</th>
          <th scope="col">Start Δ</th>
          <th scope="col">Finish Δ</th>
          <th scope="col">Duration Δ</th>
        </tr>
      </thead>
      <tbody>
        {#each variance.rows as r (r.task_pid)}
          {@const t = standingText(r)}
          <tr class={r.standing}>
            <th scope="row">{r.title}</th>
            <td><span aria-hidden="true">{t.marker}</span> {t.word}: {t.detail}</td>
            <td>{r.baseline_start ?? "—"} → {r.baseline_finish ?? "—"}</td>
            <td>{r.start ?? "—"} → {r.finish ?? "—"}</td>
            <td>{sign(r.start_variance_days)}</td>
            <td>{sign(r.finish_variance_days)}</td>
            <td>{sign(r.duration_variance_days)}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
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
  tr.slipped th[scope="row"] {
    border-inline-start: 4px solid #a4262c;
  }
  tr.removed th[scope="row"] {
    border-inline-start: 4px dashed #555;
    text-decoration: line-through;
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
