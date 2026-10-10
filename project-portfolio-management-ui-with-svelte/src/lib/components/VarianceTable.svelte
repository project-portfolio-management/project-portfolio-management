<!--
  Schedule variance against a baseline (SC-3). Every row says what happened in
  words and a marker (▲ slipped, ▼ pulled in, = on baseline, + added, − removed,
  ? cannot say), because a bare number or a colour is how a status report lies.
  A task **added** since the baseline has no slip; one **removed** stays listed
  with its baselined dates; one unscheduled on either side has no variance.
-->
<script lang="ts">
  import { t } from "#lib/i18n.svelte.js";
  import type { ScheduleVariance } from "#lib/api/ppm.js";
  import { standingText } from "#lib/cpm-view.js";

  let { variance }: { variance: NonNullable<ScheduleVariance["variance"]> } = $props();
  const s = $derived(variance.summary);
  const sign = (n: number | null) => (n === null ? "—" : n > 0 ? `+${n}` : String(n));
</script>

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
  <div class="scroll">
    <table>
      <caption class="sr-only">{t("c.VarianceTable.schedule_variance_against_the_baseli")}</caption>
      <thead>
        <tr>
          <th scope="col">{t("c.VarianceTable.task")}</th>
          <th scope="col">{t("c.VarianceTable.standing")}</th>
          <th scope="col">{t("c.VarianceTable.baselined")}</th>
          <th scope="col">{t("c.VarianceTable.now")}</th>
          <th scope="col">{t("c.VarianceTable.start")}</th>
          <th scope="col">{t("c.VarianceTable.finish")}</th>
          <th scope="col">{t("c.VarianceTable.duration")}</th>
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
