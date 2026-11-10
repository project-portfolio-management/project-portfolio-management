<!--
  One person's week as a table (NC-1 slice 6, LT-11): a row per plan, a column per day (Monday to Sunday, the
  service's ISO week), each row's total with its billable and non-billable halves beside it, and the day totals
  below. Times are whole minutes in words; an empty week says so in a sentence, never as zeros. A plan the page
  could not name is shown by the start of its id.
-->
<script lang="ts">
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { formatWeekdayDate } from "#lib/locale-format.js";
  import { durationText } from "#lib/timesheet-view.js";
  import type { TimesheetWeek } from "#lib/api/ppm.js";

  let {
    week,
    planNames,
    caption,
  }: {
    week: TimesheetWeek;
    planNames: Record<string, string>;
    caption: string;
  } = $props();

  function planLabel(pid: string): string {
    return planNames[pid] ?? t("p.timesheet.plan_unnamed", { id: pid.split("-")[0] ?? pid });
  }
</script>

{#if week.grid.total === null}
  <p data-testid="timesheet-empty">{t("p.timesheet.no_entries")}</p>
{:else}
  <div class="scroll">
    <table class="grid" data-testid="timesheet-grid">
      <caption>{caption}</caption>
      <thead>
        <tr>
          <th scope="col">{t("p.timesheet.col_plan")}</th>
          {#each week.grid.days as day (day)}<th scope="col">{formatWeekdayDate(day)}</th>{/each}
          <th scope="col">{t("p.timesheet.col_total")}</th>
          <th scope="col">{t("p.timesheet.col_billable")}</th>
          <th scope="col">{t("p.timesheet.col_non_billable")}</th>
        </tr>
      </thead>
      <tbody>
        {#each week.grid.rows as row (row.plan_pid)}
          <tr>
            <th scope="row"><a href={localeHref(`/plans/${row.plan_pid}`)}>{planLabel(row.plan_pid)}</a></th>
            {#each row.by_day as minutes, i (i)}
              <td>{minutes === 0 ? "" : durationText(minutes)}</td>
            {/each}
            <td class="total">{durationText(row.total)}</td>
            <td>{durationText(row.split.billable)}</td>
            <td>{durationText(row.split.non_billable)}</td>
          </tr>
        {/each}
      </tbody>
      <tfoot>
        <tr>
          <th scope="row">{t("p.timesheet.row_day_totals")}</th>
          {#each week.grid.day_totals as minutes, i (i)}
            <td>{durationText(minutes)}</td>
          {/each}
          <td class="total">{durationText(week.grid.total)}</td>
          <td>{durationText(week.grid.split.billable)}</td>
          <td>{durationText(week.grid.split.non_billable)}</td>
        </tr>
      </tfoot>
    </table>
  </div>
  <p class="small" data-testid="timesheet-split">
    {t("p.timesheet.split_line", {
      billable: durationText(week.grid.split.billable),
      non_billable: durationText(week.grid.split.non_billable),
      total: durationText(week.grid.total),
    })}
  </p>
{/if}
{#if week.omitted_unreadable > 0}
  <p class="small muted">{t("p.timesheet.omitted", { n: week.omitted_unreadable })}</p>
{/if}

<style>
  .scroll {
    overflow-x: auto;
    max-inline-size: 100%;
  }
  .grid {
    border-collapse: collapse;
    min-inline-size: 40rem;
  }
  .grid caption {
    text-align: start;
    font-weight: 600;
    padding-block-end: 0.35rem;
  }
  .grid th,
  .grid td {
    padding: 0.3rem 0.5rem;
    border-block-end: 1px solid var(--border, #ddd);
    text-align: end;
    white-space: nowrap;
  }
  .grid th[scope="row"],
  .grid thead th:first-child {
    text-align: start;
  }
  .grid .total,
  .grid tfoot th,
  .grid tfoot td {
    font-weight: 600;
  }
</style>
