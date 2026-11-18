<!--
  Effort against capacity (`/capacity`, NC-1 slice 12 part j, PJ-3), read on asking: `GET /api/capacity/utilization`
  for the team or by person, over a window of 1 to 366 days (checked first, as the service would quietly read 28
  instead; 28 and the team not sent) and optionally one plan. Each figure is effort over the capacity declared less
  time away, a percentage, or why there is none in words (no capacity declared, all of it away, too little to report,
  too few people to show); at or over capacity is a warning ("▲"), not an achievement. The service's note is not
  shown. Rules in `src/lib/resource-actions-view.ts`.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { t, tn } from "#lib/i18n.svelte.js";
  import { formatBasisPoints, formatNumber, formatInteger } from "#lib/locale-format.js";
  import { rowOf } from "#lib/row-of.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { PlanRepository } from "#lib/api/plans.js";
  import type { PlanRef } from "#lib/api/types.js";
  import { PpmClient, type Utilization, type UtilizationFigure } from "#lib/api/ppm.js";
  import {
    UTIL_DEFAULT_DAYS,
    UTIL_MAX_DAYS,
    figuresOf,
    hoursOf,
    unavailableKey,
    utilizationQuery,
    utilizationRefusalKey,
  } from "#lib/resource-actions-view.js";

  const ppm = PpmClient.withFetch();
  const planRepo = PlanRepository.withFetch();
  let by = $state<"team" | "person">("team");
  let windowDays = $state<number | null>(UTIL_DEFAULT_DAYS);
  let plan = $state("");
  let plans = $state<PlanRef[] | null>(null);
  let data = $state<Utilization | null>(null);
  let problem = $state<string | null>(null);
  let busy = $state(false);

  async function opened(e: Event) {
    if (!(e.currentTarget as HTMLDetailsElement).open || plans !== null) return;
    try {
      plans = await planRepo.list();
    } catch {
      plans = [];
    }
  }

  async function read(e: SubmitEvent) {
    e.preventDefault();
    const r = utilizationQuery(by, windowDays, plan);
    if ("problems" in r) {
      problem = r.problems.map((k) => t(k, { max: formatInteger(UTIL_MAX_DAYS) })).join(" ");
      return;
    }
    problem = null;
    busy = true;
    try {
      data = await ppm.utilization(r.query);
    } catch (err) {
      data = null;
      const key = utilizationRefusalKey(err);
      problem = key ? t(key) : errorText(err, "p.util.could_not");
    } finally {
      busy = false;
    }
  }

  const hours = (m: number) => t("p.hours.h", { h: formatNumber(hoursOf(m), { maximumFractionDigits: 1 }) });
  const figure = (f: UtilizationFigure) =>
    f.basis_points === null
      ? t(unavailableKey(f.unavailable))
      : `${f.at_or_over_capacity ? "▲ " : ""}${formatBasisPoints(f.basis_points, 0)}${f.at_or_over_capacity ? ` ${t("p.util.over")}` : ""}`;
  const figures = $derived(data ? figuresOf(data.utilization) : []);
  const columns: DataGridColumn[] = $derived([
    { id: "actor_ref", header: t("p.util.col_person"), rowHeader: true },
    { id: "figure", header: t("p.util.col_figure"), accessor: (r) => figure(rowOf<UtilizationFigure>(r)) },
    { id: "effort", header: t("p.util.col_effort"), align: "end", accessor: (r) => hours(rowOf<UtilizationFigure>(r).effort_minutes) },
    { id: "available", header: t("p.util.col_available"), align: "end", accessor: (r) => hours(rowOf<UtilizationFigure>(r).available_minutes) },
    { id: "away", header: t("p.util.col_away"), align: "end", accessor: (r) => hours(rowOf<UtilizationFigure>(r).non_working_minutes) },
  ]);
</script>

<details class="surface" data-testid="utilization" ontoggle={opened}>
  <summary>{t("p.util.heading")}</summary>
  <p class="small muted">{t("p.util.says", { max: formatInteger(UTIL_MAX_DAYS), days: formatInteger(UTIL_DEFAULT_DAYS) })}</p>
  <form class="row" novalidate onsubmit={read} aria-label={t("p.util.form")}>
    <label>{t("p.util.f_by")}
      <select bind:value={by}>
        <option value="team">{t("p.util.by_team")}</option>
        <option value="person">{t("p.util.by_person")}</option>
      </select>
    </label>
    <label>{t("p.util.f_window")}<input type="number" min="1" max={UTIL_MAX_DAYS} step="1" bind:value={windowDays} /></label>
    <label>{t("p.util.f_plan")}
      <select bind:value={plan} disabled={plans === null}>
        <option value="">{t("p.util.every_plan")}</option>
        {#each plans ?? [] as p (p.pid)}<option value={p.pid}>{p.name}</option>{/each}
      </select>
    </label>
    <button class="button" type="submit" disabled={busy}>{t("p.util.read")}</button>
  </form>
  {#if problem}<p class="banner error" role="alert">{problem}</p>{/if}
  {#if data}
    <p>{tn("p.util.window", data.window_days, { n: formatInteger(data.window_days) })}</p>
    {#if data.by === "person"}
      {#if figures.length === 0}
        <p>{t("p.util.none")}</p>
      {:else}
        <DataTable label={t("p.util.table")} {columns} rows={figures} rowId={(r) => String(rowOf<UtilizationFigure>(r).actor_ref)} />
      {/if}
    {:else if figures[0]}
      {@const f = figures[0]}
      <p data-testid="utilization-team"><strong>{figure(f)}</strong>{#if f.basis_points !== null}{` · ${t("p.util.team_detail", { effort: hours(f.effort_minutes), available: hours(f.available_minutes), away: hours(f.non_working_minutes) })}`}{/if}</p>
      {#if data.people_counted !== undefined}<p class="small">{tn("p.util.people_counted", data.people_counted, { n: formatInteger(data.people_counted) })}</p>{/if}
    {/if}
    <p class="small muted">{t("p.util.suppression", { people: formatInteger(data.suppression.min_group_people), hours: hours(data.suppression.floor_minutes) })}</p>
  {/if}
</details>

<style>
  details { margin-block: 1rem; }
  .row { display: flex; gap: 0.75rem; align-items: end; flex-wrap: wrap; }
  .row label { display: grid; gap: 0.2rem; }
</style>
