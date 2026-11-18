<!--
  Planned against recorded hours (`/resources`, NC-1 slice 12 part j, SC-7), read on asking: `GET
  /api/resource-plan/hours` (a blank window not sent: the service reads the 28 days to today; the dates checked first).
  The totals, a table by plan (planned, of which unassigned, recorded or "nothing recorded", the difference), time
  recorded by people with no booking on the plan, people hidden from a reader who may not manage them, and a list cut
  short. The service's note is not shown. Rules in `src/lib/resource-actions-view.ts`.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { t, tn } from "#lib/i18n.svelte.js";
  import { formatDate, formatNumber, formatInteger } from "#lib/locale-format.js";
  import { rowOf } from "#lib/row-of.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { PpmClient, type PlannedHours } from "#lib/api/ppm.js";
  import { HOURS_MAX_DAYS, hoursOf, hoursQuery, hoursRefusalKey } from "#lib/resource-actions-view.js";

  let { planName }: { planName: (pid: string) => string } = $props();

  const ppm = PpmClient.withFetch();
  let from = $state("");
  let to = $state("");
  let data = $state<PlannedHours | null>(null);
  let problem = $state<string | null>(null);
  let busy = $state(false);

  const hours = (m: number | null | undefined) => (m === null || m === undefined ? t("p.hours.nothing_recorded") : t("p.hours.h", { h: formatNumber(hoursOf(m), { maximumFractionDigits: 1 }) }));

  async function read(e: SubmitEvent) {
    e.preventDefault();
    const r = hoursQuery(from, to);
    if ("problems" in r) {
      problem = r.problems.map((k) => t(k, { max: formatInteger(HOURS_MAX_DAYS) })).join(" ");
      return;
    }
    problem = null;
    busy = true;
    try {
      data = await ppm.plannedHours(r.query);
    } catch (err) {
      const key = hoursRefusalKey(err);
      problem = key ? t(key) : errorText(err, "p.hours.could_not");
    } finally {
      busy = false;
    }
  }

  type Row = PlannedHours["plans"][number];
  const columns: DataGridColumn[] = $derived([
    { id: "plan", header: t("p.hours.col_plan"), rowHeader: true, accessor: (r) => planName(rowOf<Row>(r).plan_pid) },
    { id: "planned", header: t("p.hours.col_planned"), align: "end", accessor: (r) => hours(rowOf<Row>(r).planned_minutes) },
    { id: "placeholder", header: t("p.hours.col_placeholder"), align: "end", accessor: (r) => hours(rowOf<Row>(r).placeholder_planned_minutes) },
    { id: "actual", header: t("p.hours.col_actual"), align: "end", accessor: (r) => hours(rowOf<Row>(r).actual_minutes) },
    { id: "variance", header: t("p.hours.col_variance"), align: "end", accessor: (r) => { const v = rowOf<Row>(r).variance_minutes; return v === null ? "—" : t("p.hours.h", { h: formatNumber(hoursOf(v), { maximumFractionDigits: 1 }) }); } },
  ]);
</script>

<section aria-labelledby="hours-h" data-testid="planned-hours">
  <h2 id="hours-h">{t("p.hours.heading")}</h2>
  <p class="small muted">{t("p.hours.says", { max: formatInteger(HOURS_MAX_DAYS) })}</p>
  <form class="row" novalidate onsubmit={read} aria-label={t("p.hours.form")}>
    <label>{t("p.hours.f_from")}<input type="date" bind:value={from} /></label>
    <label>{t("p.hours.f_to")}<input type="date" bind:value={to} /></label>
    <button class="button" type="submit" disabled={busy}>{t("p.hours.read")}</button>
  </form>
  {#if problem}<p class="banner error" role="alert">{problem}</p>{/if}
  {#if data}
    <p>{t("p.hours.window", { from: formatDate(data.from), to: formatDate(data.to) })}</p>
    <p>{t("p.hours.totals", { planned: hours(data.total_planned_minutes), actual: hours(data.total_actual_minutes) })}</p>
    {#if data.masked}<p class="small">{t("p.hours.masked")}</p>{/if}
    {#if data.truncated}<p class="banner" role="note">{t("p.hours.truncated")}</p>{/if}
    {#if data.plans.length === 0}
      <p>{t("p.hours.none")}</p>
    {:else}
      <DataTable label={t("p.hours.table")} {columns} rows={data.plans} rowId={(r) => String(rowOf<Row>(r).plan_pid)} />
    {/if}
    {#if data.unplanned_actuals.length > 0}
      <h3>{t("p.hours.unplanned_heading")}</h3>
      <ul>
        {#each data.unplanned_actuals as u (u.plan_pid)}
          <li>{t("p.hours.unplanned", { plan: planName(u.plan_pid), hours: hours(u.minutes) })}{#if u.people_count !== undefined}{` ${tn("p.hours.unplanned_people", u.people_count, { n: formatInteger(u.people_count) })}`}{/if}</li>
        {/each}
      </ul>
    {/if}
  {/if}
</section>

<style>
  section { margin-block: 1.5rem; }
  .row { display: flex; gap: 0.75rem; align-items: end; flex-wrap: wrap; }
  .row label { display: grid; gap: 0.2rem; }
</style>
