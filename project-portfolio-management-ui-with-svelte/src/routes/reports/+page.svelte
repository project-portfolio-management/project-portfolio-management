<!--
  Saved reports (`/reports`, PPM-9): definitions (collection +
  filters + projected fields), synchronous runs previewed as a
  table, and the CSV download.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t } from "#lib/i18n.svelte.js";
  import { enumLabel } from "#lib/enum-labels.js";
  import { onMount } from "svelte";
  import { PpmClient, type ReportDefinition, type ReportRun } from "#lib/api/ppm.js";
  import { COLLECTIONS, type Collection } from "#lib/api/types.js";

  const ppm = PpmClient.withFetch();
  let reports = $state<ReportDefinition[]>([]);
  let runs = $state<Record<string, ReportRun>>({});
  let error = $state<string | null>(null);

  let name = $state("");
  let collection = $state<Collection>("projects");
  let nameLike = $state("");
  let fields = $state("name,stage,status,target_date");

  async function refresh() {
    try {
      reports = await ppm.listReports();
      error = null;
    } catch (err) {
      error = errorText(err, "ppm.common.loadFailed");
    }
  }
  onMount(refresh);

  async function create(event: SubmitEvent) {
    event.preventDefault();
    try {
      await ppm.createReport({
        name,
        collection,
        filters: nameLike.trim() ? { name_like: nameLike.trim() } : {},
        fields: fields.split(",").map((field) => field.trim()).filter(Boolean),
      });
      name = "";
      nameLike = "";
      await refresh();
    } catch (err) {
      error = errorText(err, "p.reports.save_failed");
    }
  }

  async function run(pid: string) {
    try {
      runs = { ...runs, [pid]: await ppm.runReport(pid) };
    } catch (err) {
      error = errorText(err, "p.reports.run_failed");
    }
  }

  /** One column per field of the first row (a report's fields are the author's choice). */
  const reportColumns = (first: Record<string, unknown>): DataGridColumn[] =>
    Object.keys(first).map((k, i) => ({ id: k, header: k, rowHeader: i === 0, format: (v) => (v === null || v === undefined ? "" : String(v)) }));
</script>

<svelte:head><title>{t("p.reports.reports_ppm")}</title></svelte:head>

<h1>{t("ppm.reports.title")}</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

<form class="row" onsubmit={create}>
  <input placeholder={t("ppm.reports.name")} bind:value={name} required />
  <select bind:value={collection} aria-label={t("p.reports.collection")}>
    {#each COLLECTIONS as segment (segment)}<option value={segment}>{enumLabel("collection", segment)}</option>{/each}
  </select>
  <input placeholder={t("p.reports.name_contains")} bind:value={nameLike} />
  <input placeholder={t("p.reports.fields_comma_separated")} bind:value={fields} size="34" />
  <button class="button primary" type="submit">{t("ppm.reports.save")}</button>
</form>

{#each reports as report (report.pid)}
  <section class="report">
    <p>
      <strong>{report.name}</strong>
      <span class="chip">{report.collection}</span>
      <button class="button small" onclick={() => run(report.pid)}>{t("ppm.reports.run")}</button>
      <a class="button small" href={ppm.reportCsvUrl(report.pid)} download>{t("p.reports.csv")}</a>
    </p>
    {#if runs[report.pid]}
      {@const result = runs[report.pid]}
      {#if result}
        <p class="small muted">{result.rows} {t("ppm.reports.rows")}</p>
        {#if result.data.length > 0}
          {@const first = result.data[0] ?? {}}
          <DataTable label={report.name} columns={reportColumns(first)} rows={result.data} rowId={(_r, i) => String(i)} searchable />
        {/if}
      {/if}
    {/if}
  </section>
{/each}

<style>
  .row { display: flex; gap: 0.5rem; flex-wrap: wrap; align-items: center; margin: 0.8rem 0; }
  .report { border-bottom: 1px solid var(--border, #ddd); padding: 0.5rem 0; }
  .chip {
    display: inline-block;
    border: 1px solid var(--border, #ccc);
    border-radius: 999px;
    padding: 0 0.5rem;
    margin: 0 0.3rem;
    font-size: 0.78rem;
  }
</style>
