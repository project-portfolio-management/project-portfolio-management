<!--
  Schedule health (`/plans/[pid]/schedule-health`): the fourteen DCMA-style
  checks over the plan's task network, each with what it counted, what it is held
  to, and the tasks to fix — each linked to the task. A self-check, not a DCMA
  compliance assessment. A check that cannot be computed says "Not enough data"
  with the reason and is never a pass. Pass / fail / not computable are a word and
  a symbol, never colour alone.
-->
<script lang="ts">
  import { rowOf } from "#lib/row-of.js";
  import { errorText } from "#lib/error-text.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { PpmClient, type ScheduleHealth } from "#lib/api/ppm.js";
  import { healthReason, healthResult, healthSummary, healthValue, offenderHref, thresholdSentence } from "#lib/scheduling-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");
  let report = $state<ScheduleHealth | null>(null);
  let error = $state<string | null>(null);
  let asOf = $state("");

  async function load() {
    try {
      report = await ppm.scheduleHealth(pid, asOf || undefined);
      error = null;
    } catch (err) {
      error = errorText(err, "p.plans.pid.schedule_health.could_not_run_the_schedule");
    }
  }
  onMount(() => void load());

  type Check = ScheduleHealth["checks"][number];
  const columns: DataGridColumn[] = $derived([
    { id: "name", header: t("p.plans.pid.schedule_health.check"), rowHeader: true, format: (v, r) => `${r.id}. ${v}` },
    { id: "result", header: t("p.plans.pid.schedule_health.result"), accessor: (r) => healthResult(rowOf<Check>(r)).text, cell: resultCell, cellClass: (c) => ({ fail: "cell-red health-fail", na: "cell-grey health-na", pass: "" })[healthResult(rowOf<Check>(c.row)).kind] },
    { id: "found", header: t("p.plans.pid.schedule_health.what_was_found"), accessor: (r) => { const c = rowOf<Check>(r); return c.pass === null ? healthReason(c.reason) : healthValue(c); } },
    { id: "held", header: t("p.plans.pid.schedule_health.held_to"), accessor: (r) => thresholdSentence(rowOf<Check>(r)) },
    { id: "fix", header: t("p.plans.pid.schedule_health.to_fix"), sortable: false, accessor: (r) => rowOf<Check>(r).offenders.length, cell: fixCell },
  ]);
</script>

{#snippet resultCell({ row }: DataGridCellContext)}
  {@const r = healthResult(rowOf<Check>(row))}
  <span aria-hidden="true">{r.symbol}</span> {r.text}
{/snippet}

{#snippet fixCell({ row }: DataGridCellContext)}
  {@const c = rowOf<Check>(row)}
  {#if c.offenders.length === 0}
    {c.pass === false ? t("p.plans.pid.schedule_health.see_the_note") : "—"}
  {:else}
    <details>
      <summary>{(c.offenders.length === 1 ? t("p.plans.pid.schedule_health.task_one", { length: c.offenders.length }) : t("p.plans.pid.schedule_health.task_other", { length: c.offenders.length }))}</summary>
      <ul>
        {#each c.offenders as o (o.pid)}
          <li><a href={localeHref(offenderHref(pid, o.pid))}>{o.title ?? o.pid}</a></li>
        {/each}
      </ul>
    </details>
  {/if}
  {#if c.note}<p class="small muted">{c.note}</p>{/if}
{/snippet}


<svelte:head><title>{t("p.plans.pid.schedule_health.schedule_health_ppm")}</title></svelte:head>

<h1>{t("p.plans.pid.schedule_health.schedule_health")}</h1>
<p class="lede">{t("p.plans.pid.schedule_health.checks_the_schedule_for_common")}</p>
<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plans.pid.schedule_health.plan")}</a> · <a href={localeHref(`/plans/${pid}/critical-path`)}>{t("p.plans.pid.schedule_health.critical_path")}</a></p>

{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if report}
  <p class="banner" role="note">{report.disclaimer}</p>
  <p class="small muted">
    {t("p.plans.pid.schedule_health.as_of", { as_of: report.as_of })}
    {#if report.baseline}{t("p.plans.pid.schedule_health.baseline_version", { version: report.baseline.version })}{:else}{t("p.plans.pid.schedule_health.no_baseline_taken")}{/if} {(report.tasks === 1 ? t("p.plans.pid.schedule_health.preset_thresholds_task_one", { preset: report.preset, source: report.thresholds.source, tasks: report.tasks }) : t("p.plans.pid.schedule_health.preset_thresholds_task_other", { preset: report.preset, source: report.thresholds.source, tasks: report.tasks }))}
  </p>
  <p aria-live="polite"><strong>{healthSummary(report)}.</strong></p>
  <form class="filter" onsubmit={(e) => { e.preventDefault(); void load(); }}>
    <label>{t("p.plans.pid.schedule_health.status_date")} <input type="date" bind:value={asOf} /></label>
    <button class="button small" type="submit">{t("p.plans.pid.schedule_health.re_run")}</button>
  </form>

    <DataTable label={t("p.plans.pid.schedule_health.the_fourteen_schedule_health_checks")} {columns} rows={report.checks} rowId={(r) => String(r.id)} sortable={false} />
  <p class="small muted">{report.note}</p>
{:else if !error}
  <p class="muted">{t("p.plans.pid.schedule_health.loading")}</p>
{/if}

<style>
  :global(.health-fail) { font-weight: 700; color: var(--danger, #b00020); }
  :global(.health-na) { font-style: italic; }
  .filter {
    display: flex;
    gap: 0.75rem;
    align-items: end;
    margin: 0.75rem 0;
  }
  .filter label {
    display: grid;
    gap: 0.2rem;
    font-size: 0.85rem;
  }
</style>
