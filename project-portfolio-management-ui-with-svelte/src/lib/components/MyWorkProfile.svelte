<!--
  The work over time (`/me`, NC-1 slice 12 part h, PJ-5): the work carried by tasks in every plan the reader can
  read, spread over periods (`GET /api/work-profile`), on asking, as two tables: each plan's work and the part not
  placed, and the work in each period. Days, weeks (the default, not sent) or months, and an optional window; every
  rule said and checked first. The service's `note` is English and is not shown. Rules in `src/lib/my-work-view.ts`.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { formatDateRange } from "#lib/locale-format.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { rowOf } from "#lib/row-of.js";
  import { PpmClient, type Bucket, type PortfolioWorkProfile } from "#lib/api/ppm.js";
  import { durationText } from "#lib/timesheet-view.js";
  import { PROFILE_MAX_PERIODS, meRefusalKey, workProfileQuery, type WorkProfileForm } from "#lib/my-work-view.js";

  const ppm = PpmClient.withFetch();
  let form = $state<WorkProfileForm>({ period: "", from: "", to: "" });
  let profile = $state<PortfolioWorkProfile | null>(null);
  let problem = $state<string | null>(null);
  let busy = $state(false);

  async function show(e: SubmitEvent) {
    e.preventDefault();
    const r = workProfileQuery(form);
    if ("problems" in r) {
      problem = r.problems.map((k) => t(k, { max: PROFILE_MAX_PERIODS })).join(" ");
      return;
    }
    problem = null;
    busy = true;
    try {
      profile = await ppm.portfolioWorkProfile(r.query);
    } catch (err) {
      profile = null;
      const key = meRefusalKey("work_profile", err);
      problem = key ? t(key) : errorText(err, "p.workprofile.could_not_load");
    } finally {
      busy = false;
    }
  }

  type PlanRow = PortfolioWorkProfile["plans"][number];
  const planColumns: DataGridColumn[] = $derived([
    { id: "name", header: t("p.workprofile.col_plan"), rowHeader: true, cell: planCell, accessor: (r) => rowOf<PlanRow>(r).name ?? "" },
    { id: "work_minutes", header: t("p.workprofile.col_work"), align: "end", format: (v) => durationText(Number(v)) },
    { id: "unplaced_work_minutes", header: t("p.workprofile.col_unplaced"), align: "end", format: (v) => durationText(Number(v)) },
  ]);
  const bucketColumns: DataGridColumn[] = $derived([
    { id: "from", header: t("p.workprofile.col_period"), rowHeader: true, accessor: (r) => formatDateRange(rowOf<Bucket>(r).from, rowOf<Bucket>(r).to) },
    { id: "minutes", header: t("p.workprofile.col_work"), align: "end", format: (v) => durationText(Number(v)) },
  ]);
</script>

{#snippet planCell({ row }: DataGridCellContext)}
  {@const p = rowOf<PlanRow>(row)}
  <a href={localeHref(`/plans/${p.plan_pid}`)}>{p.name ?? t("p.workprofile.plan_unnamed")}</a>
{/snippet}

<section class="profile" aria-labelledby="profile-h" data-testid="work-profile">
  <h2 id="profile-h">{t("p.workprofile.heading")}</h2>
  <p class="small muted">{t("p.workprofile.says")}</p>
  <form class="profile-form" novalidate onsubmit={show} aria-label={t("p.workprofile.form")}>
    <label>{t("p.workprofile.f_period")}
      <select bind:value={form.period}>
        <option value="">{t("p.workprofile.period_week")}</option>
        <option value="day">{t("p.workprofile.period_day")}</option>
        <option value="month">{t("p.workprofile.period_month")}</option>
      </select>
    </label>
    <label>{t("p.workprofile.f_from")}<input type="date" bind:value={form.from} /></label>
    <label>{t("p.workprofile.f_to")}<input type="date" bind:value={form.to} /></label>
    <div><button class="button" type="submit" disabled={busy}>{t("p.workprofile.show")}</button></div>
  </form>
  <p class="small muted">{t("p.workprofile.rules", { max: PROFILE_MAX_PERIODS })}</p>
  {#if problem}<p class="banner error" role="alert">{problem}</p>{/if}
  {#if profile}
    {#if profile.reason === "no_assignments" || profile.plans.length === 0}
      <p>{t("p.workprofile.none")}</p>
    {:else}
      <p data-testid="work-profile-summary">
        {t("p.workprofile.summary", {
          range: formatDateRange(profile.from, profile.to),
          work: durationText(profile.totals.work_minutes),
          unplaced: durationText(profile.totals.unplaced_work_minutes ?? 0),
        })}
      </p>
      <DataTable label={t("p.workprofile.plans_table")} columns={planColumns} rows={profile.plans} rowId={(r) => String(rowOf<PlanRow>(r).plan_pid)} testid="work-profile-plans" />
      <DataTable label={t("p.workprofile.periods_table")} columns={bucketColumns} rows={profile.totals.buckets ?? []} rowId={(_r, i) => String(i)} sortable={false} testid="work-profile-periods" />
    {/if}
  {/if}
</section>

<style>
  .profile {
    margin-block: 1.5rem;
  }
  .profile-form {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    align-items: end;
  }
  .profile-form label {
    display: grid;
    gap: 0.2rem;
  }
</style>
