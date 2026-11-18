<!--
  The time report (`/reports`, NC-1 slice 12 part h, RM-10): recorded time grouped by plan, person, category, billable
  or month over the plans the reader can read (`GET /api/time-reports`), on asking, with billable and not billable
  always apart, no entries said (never a row of zeros), the entries left out counted, and the same report as the
  service's CSV (a link). The plans to filter by come from `GET /api/plans` (the list disabled until read; a plan is
  named from it). Every rule is said and checked first. Rules in `src/lib/time-entry-view.ts`.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { errorText } from "#lib/error-text.js";
  import { t, tn } from "#lib/i18n.svelte.js";
  import { formatDateRange, formatInteger } from "#lib/locale-format.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { rowOf } from "#lib/row-of.js";
  import { PpmClient, type TimeReport, type TimeReportQuery } from "#lib/api/ppm.js";
  import { PlanRepository } from "#lib/api/plans.js";
  import type { PlanRef } from "#lib/api/types.js";
  import { durationText } from "#lib/timesheet-view.js";
  import {
    GROUP_BYS,
    GROUP_BY_KEY,
    groupText,
    timeReportQuery,
    timeRefusalKey,
    type TimeReportForm,
  } from "#lib/time-entry-view.js";

  const ppm = PpmClient.withFetch();
  const planRepo = PlanRepository.withFetch();
  let plans = $state<PlanRef[] | null>(null);
  let plansError = $state<string | null>(null);
  let form = $state<TimeReportForm>({ groupBy: "plan", from: "", to: "", plan: "", person: "" });
  let report = $state<TimeReport | null>(null);
  let asked = $state<TimeReportQuery | null>(null);
  let problem = $state<string | null>(null);
  let busy = $state(false);

  onMount(async () => {
    try {
      plans = await planRepo.list();
    } catch (err) {
      plansError = errorText(err, "p.timereport.could_not_load_plans");
    }
  });

  const planNames = $derived(Object.fromEntries((plans ?? []).map((p) => [p.pid, p.name])));

  async function show(e: SubmitEvent) {
    e.preventDefault();
    const r = timeReportQuery(form);
    if ("problems" in r) {
      problem = r.problems.map((k) => t(k)).join(" ");
      return;
    }
    problem = null;
    busy = true;
    try {
      report = await ppm.timeReport(r.query);
      asked = r.query;
    } catch (err) {
      report = null;
      asked = null;
      const key = timeRefusalKey("report", err);
      problem = key ? t(key) : errorText(err, "p.timereport.could_not_load");
    } finally {
      busy = false;
    }
  }

  type Group = TimeReport["groups"][number];
  const columns: DataGridColumn[] = $derived([
    { id: "key", header: report ? t(GROUP_BY_KEY[asked?.groupBy ?? "plan"]) : "", rowHeader: true, accessor: (r) => groupText(report?.group_by ?? "", rowOf<Group>(r).key, planNames) },
    { id: "entries", header: t("p.timereport.col_entries"), align: "end", format: (v) => formatInteger(Number(v)) },
    { id: "billable", header: t("p.timereport.col_billable"), align: "end", accessor: (r) => durationText(rowOf<Group>(r).split.billable) },
    { id: "non_billable", header: t("p.timereport.col_not_billable"), align: "end", accessor: (r) => durationText(rowOf<Group>(r).split.non_billable) },
    { id: "minutes", header: t("p.timereport.col_total"), align: "end", format: (v) => durationText(Number(v)) },
  ]);
</script>

<section class="time-report" aria-labelledby="time-report-h" data-testid="time-report">
  <h2 id="time-report-h">{t("p.timereport.heading")}</h2>
  <p class="small muted">{t("p.timereport.says")}</p>
  {#if plansError}<p class="banner" role="alert">{plansError}</p>{/if}
  <form class="report-form" novalidate onsubmit={show} aria-label={t("p.timereport.form")}>
    <label>{t("p.timereport.f_group_by")}
      <select bind:value={form.groupBy}>
        {#each GROUP_BYS as g (g)}<option value={g}>{t(GROUP_BY_KEY[g])}</option>{/each}
      </select>
    </label>
    <label>{t("p.timereport.f_from")}<input type="date" bind:value={form.from} /></label>
    <label>{t("p.timereport.f_to")}<input type="date" bind:value={form.to} /></label>
    <label>{t("p.timereport.f_plan")}
      <select bind:value={form.plan} disabled={!plans}>
        <option value="">{plans ? t("p.timereport.every_plan") : t("p.timereport.loading_plans")}</option>
        {#each plans ?? [] as p (p.pid)}<option value={p.pid}>{p.name}</option>{/each}
      </select>
    </label>
    <label>{t("p.timereport.f_person")}
      <input bind:value={form.person} autocomplete="off" spellcheck="false" maxlength="200" />
    </label>
    <div><button class="button" type="submit" disabled={busy}>{t("p.timereport.show")}</button></div>
  </form>
  {#if problem}<p class="banner error" role="alert">{problem}</p>{/if}
  {#if report && asked}
    <p data-testid="time-report-window">{t("p.timereport.window", { range: formatDateRange(report.from, report.to) })}</p>
    {#if report.total_minutes === null || report.groups.length === 0}
      <p>{t("p.timereport.no_entries")}</p>
    {:else}
      <DataTable label={t("p.timereport.table")} columns={columns} rows={report.groups} rowId={(r) => String(rowOf<Group>(r).key)} testid="time-report-table" />
      <p data-testid="time-report-total">
        {t("p.timereport.total", {
          billable: durationText(report.split.billable),
          not_billable: durationText(report.split.non_billable),
          total: durationText(report.total_minutes),
        })}
      </p>
    {/if}
    {#if report.omitted_unreadable > 0}
      <p class="small muted">{tn("p.timereport.omitted", report.omitted_unreadable, { n: formatInteger(report.omitted_unreadable) })}</p>
    {/if}
    {#if report.truncated}<p class="small">{t("p.timereport.truncated", { max: formatInteger(20000) })}</p>{/if}
    <p class="small"><a href={ppm.timeReportCsvUrl(asked)} download>{t("p.timereport.csv")}</a></p>
  {/if}
</section>

<style>
  .time-report {
    margin-block: 1.5rem;
  }
  .report-form {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    align-items: end;
  }
  .report-form label {
    display: grid;
    gap: 0.2rem;
  }
</style>
