<!--
  Change requests (`/change-requests`, IR-4): every change request across the plans you can read, newest
  first, with the count by status and how many await a decision. Read-only here: drafting, submitting and
  deciding are done through the API.
-->
<script lang="ts">
  import { rowOf } from "#lib/row-of.js";
  import { resource } from "#lib/resource.svelte.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { enumLabel } from "#lib/enum-labels.js";
  import { onMount } from "svelte";
  import { PpmClient, type ChangeRequestList } from "#lib/api/ppm.js";

  const ppm = PpmClient.withFetch();
  let status = $state("");
  let openOnly = $state(false);
  const loaded = resource<ChangeRequestList>(
    () => ppm.changeRequests({ status: status || undefined, open: openOnly }),
    "p.change_requests.could_not_load",
  );
  const list = $derived(loaded.data);
  const error = $derived(loaded.error);
  const refresh = () => loaded.reload();
  onMount(refresh);

  const STATUSES = ["draft", "submitted", "under_review", "approved", "rejected", "implemented", "withdrawn"] as const;

  type CR = ChangeRequestList["change_requests"][number];
  const impactText = (cr: CR): string => {
    if (cr.impact.schedule_business_days === null && cr.impact.cost_minor === null) return t("p.change_requests.no_impact");
    const days = cr.impact.schedule_business_days !== null ? t("p.change_requests.impact_days", { days: cr.impact.schedule_business_days }) : "";
    const cost = cr.impact.cost_minor !== null ? `${cr.impact.schedule_business_days !== null ? " · " : ""}${(cr.impact.cost_minor / 100).toFixed(2)} ${cr.impact.cost_currency}` : "";
    return `${days}${cost}`;
  };
  const columns: DataGridColumn[] = $derived([
    { id: "title", header: t("p.change_requests.title_col"), rowHeader: true, cell: crTitle },
    { id: "kind", header: t("p.change_requests.kind_col"), format: (v) => enumLabel("cr_kind", String(v)) },
    { id: "status", header: t("p.change_requests.status_col"), format: (v) => enumLabel("cr_status", String(v)) },
    { id: "impact", header: t("p.change_requests.impact_col"), sortable: false, accessor: (r) => impactText(rowOf<CR>(r)) },
    { id: "approver_ref", header: t("p.change_requests.approver_col"), format: (v) => (v ? String(v) : t("p.change_requests.no_approver")) },
    { id: "age_calendar_days", header: t("p.change_requests.age_col"), align: "end" },
  ]);
</script>

{#snippet crTitle({ row, formatted }: DataGridCellContext)}
  <a href={localeHref(`/plans/${row.plan_pid}/governance`)} aria-label={`${formatted}: ${t("p.change_requests.plan_link")}`}>{formatted}</a>
{/snippet}


<svelte:head><title>{t("p.change_requests.change_requests_ppm")}</title></svelte:head>

<h1>{t("p.change_requests.change_requests")}</h1>
<p class="small muted">{t("p.change_requests.intro")}</p>

{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if list}
  <ul class="summary" aria-label={t("p.change_requests.by_status")}>
    {#each STATUSES as s (s)}
      <li>{enumLabel("cr_status", s)}: <strong>{list.summary.by_status[s] ?? 0}</strong></li>
    {/each}
    <li>{t("p.change_requests.awaiting")}: <strong>{list.summary.awaiting_decision}</strong></li>
  </ul>
  {#if list.summary.oldest_open}
    <p class="small muted">
      {t("p.change_requests.oldest_open", { title: list.summary.oldest_open.title, days: list.summary.oldest_open.age_calendar_days })}
    </p>
  {/if}
{/if}

<form class="filter" onsubmit={(e) => { e.preventDefault(); void refresh(); }}>
  <label>{t("p.change_requests.show")}
    <select bind:value={status} onchange={refresh}>
      <option value="">{t("p.change_requests.all_statuses")}</option>
      {#each STATUSES as s (s)}<option value={s}>{enumLabel("cr_status", s)}</option>{/each}
    </select>
  </label>
  <label>
    <input type="checkbox" bind:checked={openOnly} onchange={refresh} />
    {t("p.change_requests.open_only")}
  </label>
</form>

{#if list}
  {#if list.change_requests.length === 0}
    <p>{!status && !openOnly ? t("p.change_requests.none") : t("p.change_requests.none_match")}</p>
  {:else}
    <p class="small muted" aria-live="polite">
      {list.change_requests.length === 1 ? t("p.change_requests.count_one", { n: 1 }) : t("p.change_requests.count_other", { n: list.change_requests.length })}
    </p>
    <DataTable label={t("p.change_requests.title_col")} columns={columns} rows={list.change_requests} rowId={(r) => String(r.pid)} searchable />
  {/if}
{:else if !error}
  <p class="muted">{t("p.change_requests.loading")}</p>
{/if}

<style>
  .summary {
    display: flex;
    flex-wrap: wrap;
    gap: 1.25rem;
    list-style: none;
    padding: 0;
  }
  .filter {
    display: flex;
    flex-wrap: wrap;
    gap: 1rem;
    align-items: end;
    margin: 1rem 0;
  }
</style>
