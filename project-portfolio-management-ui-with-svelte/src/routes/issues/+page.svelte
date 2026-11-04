<!--
  Issues register (`/issues`, IR-2): every issue across the plans you can read,
  most serious first, then oldest. An issue is something that has already
  happened; a risk is something that might. Age is in calendar days.
-->
<script lang="ts">
  import { rowOf } from "#lib/row-of.js";
  import { resource } from "#lib/resource.svelte.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t, localeHref, tn } from "#lib/i18n.svelte.js";
  import { enumLabel } from "#lib/enum-labels.js";
  import { onMount } from "svelte";
  import { PpmClient, type IssueRegister, type IssueType } from "#lib/api/ppm.js";
  import { statusLabel, typeLabel } from "#lib/issue-view.js";

  const ppm = PpmClient.withFetch();
  let status = $state("");
  let severity = $state("");
  let openOnly = $state(false);
  let minAge = $state<number | undefined>(undefined);

  const loaded = resource<IssueRegister>(
    () =>
      ppm.issues({
        status: status || undefined,
        severity: severity || undefined,
        open: openOnly,
        min_age_days: minAge === undefined || Number.isNaN(minAge) ? undefined : minAge,
      }),
    "p.issues.could_not_load",
  );
  const register = $derived(loaded.data);
  const error = $derived(loaded.error);
  const refresh = () => loaded.reload();
  // The organization's type names (OC-1). Without them a built-in type is still translated and any other reads as words.
  let types = $state<IssueType[] | null>(null);
  onMount(() => {
    void refresh();
    ppm.issueTypes(true).then(
      (found) => (types = Array.isArray(found) ? found : null),
      () => (types = null),
    );
  });

  const SEVERITIES = ["critical", "high", "med", "low"] as const;
  // Each plan can have its own statuses, so the filter offers the ones in use, and the built-ins are always there.
  const STATUSES = $derived(Object.keys(register?.summary.by_status ?? { open: 0, in_progress: 0, resolved: 0, closed: 0 }));

  type Issue = IssueRegister["issues"][number];
  const columns: DataGridColumn[] = $derived([
    { id: "title", header: t("p.issues.title_col"), rowHeader: true, cell: titleCell },
    { id: "kind", header: t("p.issues.kind_col"), format: (v) => typeLabel(types, String(v)) },
    { id: "severity", header: t("p.issues.severity_col"), format: (v) => enumLabel("severity", String(v)) },
    { id: "status", header: t("p.issues.status_col"), format: (v) => statusLabel(null, String(v)) },
    { id: "owner_ref", header: t("p.issues.owner_col"), format: (v) => (v ? String(v) : t("p.issues.no_owner")) },
    { id: "age_calendar_days", header: t("p.issues.age_col"), align: "end" },
    { id: "due_date", header: t("p.issues.due_col"), format: (v) => (v ? String(v) : t("p.issues.no_due")), cell: dueCell },
  ]);
</script>

{#snippet titleCell({ row, formatted }: DataGridCellContext)}
  <a href={localeHref(`/plans/${row.plan_pid}/issues`)} aria-label={`${formatted}: ${t("p.issues.plan_link")}`}>{formatted}</a>
{/snippet}

{#snippet dueCell({ row, formatted }: DataGridCellContext)}
  {formatted}{#if rowOf<Issue>(row).overdue}<strong> · {t("p.issues.overdue_flag")}</strong>{/if}
{/snippet}


<svelte:head><title>{t("p.issues.issues_ppm")}</title></svelte:head>

<h1>{t("p.issues.issues")}</h1>
<p class="small muted">{t("p.issues.intro")}</p>

{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if register}
  <ul class="summary" aria-label={t("p.issues.by_severity")}>
    {#each SEVERITIES as s (s)}
      <li>{enumLabel("severity", s)}: <strong>{register.summary.by_severity[s] ?? 0}</strong></li>
    {/each}
    <li>{t("p.issues.overdue_count")}: <strong>{register.summary.overdue}</strong></li>
  </ul>
  {#if register.summary.oldest_open}
    <p class="small muted">
      {t("p.issues.oldest_open", {
        title: register.summary.oldest_open.title,
        days: register.summary.oldest_open.age_calendar_days,
      })}
    </p>
  {/if}
{/if}

<form class="filter" onsubmit={(e) => { e.preventDefault(); void refresh(); }}>
  <label>{t("p.issues.show")}
    <select bind:value={status} onchange={refresh}>
      <option value="">{t("p.issues.all_statuses")}</option>
      {#each STATUSES as s (s)}<option value={s}>{statusLabel(null, s)}</option>{/each}
    </select>
  </label>
  <label>{t("p.issues.severity_filter")}
    <select bind:value={severity} onchange={refresh}>
      <option value="">{t("p.issues.all_severities")}</option>
      {#each SEVERITIES as s (s)}<option value={s}>{enumLabel("severity", s)}</option>{/each}
    </select>
  </label>
  <label>
    <input type="checkbox" bind:checked={openOnly} onchange={refresh} />
    {t("p.issues.open_only")}
  </label>
  <label>{t("p.issues.older_than")}
    <input type="number" min="0" step="1" bind:value={minAge} onchange={refresh} />
  </label>
</form>

{#if register}
  {#if register.issues.length === 0}
    <p>{register.matching === 0 && register.summary.oldest_open === null && !status && !severity && !openOnly && minAge === undefined ? t("p.issues.none") : t("p.issues.none_match")}</p>
  {:else}
    <p class="small muted" aria-live="polite">
      {tn("p.issues.count", register.matching, { n: register.matching })}
    </p>
    {#if register.truncated}<p class="small muted">{t("p.issues.truncated", { n: register.issues.length })}</p>{/if}
    <DataTable label={t("p.issues.title_col")} columns={columns} rows={register.issues} rowId={(r) => String(r.pid)} searchable />
  {/if}
{:else if !error}
  <p class="muted">{t("p.issues.loading")}</p>
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
