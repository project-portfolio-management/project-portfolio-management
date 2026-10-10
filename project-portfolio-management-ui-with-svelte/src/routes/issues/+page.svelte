<!--
  Issues register (`/issues`, IR-2): every issue across the plans you can read,
  most serious first, then oldest. An issue is something that has already
  happened; a risk is something that might. Age is in calendar days.
-->
<script lang="ts">
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { enumLabel } from "#lib/enum-labels.js";
  import { onMount } from "svelte";
  import { PpmClient, type IssueRegister } from "#lib/api/ppm.js";

  const ppm = PpmClient.withFetch();
  let register = $state<IssueRegister | null>(null);
  let error = $state<string | null>(null);
  let status = $state("");
  let severity = $state("");
  let openOnly = $state(false);
  let minAge = $state<number | undefined>(undefined);

  async function refresh() {
    try {
      register = await ppm.issues({
        status: status || undefined,
        severity: severity || undefined,
        open: openOnly,
        min_age_days: minAge === undefined || Number.isNaN(minAge) ? undefined : minAge,
      });
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.issues.could_not_load");
    }
  }
  onMount(refresh);

  const SEVERITIES = ["critical", "high", "med", "low"] as const;
  const STATUSES = ["open", "in_progress", "resolved", "closed"] as const;
</script>

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
      {#each STATUSES as s (s)}<option value={s}>{enumLabel("issue_status", s)}</option>{/each}
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
      {register.matching === 1 ? t("p.issues.count_one", { n: register.matching }) : t("p.issues.count_other", { n: register.matching })}
    </p>
    {#if register.truncated}<p class="small muted">{t("p.issues.truncated", { n: register.issues.length })}</p>{/if}
    <table>
      <thead>
        <tr>
          <th scope="col">{t("p.issues.title_col")}</th>
          <th scope="col">{t("p.issues.kind_col")}</th>
          <th scope="col">{t("p.issues.severity_col")}</th>
          <th scope="col">{t("p.issues.status_col")}</th>
          <th scope="col">{t("p.issues.owner_col")}</th>
          <th scope="col">{t("p.issues.age_col")}</th>
          <th scope="col">{t("p.issues.due_col")}</th>
        </tr>
      </thead>
      <tbody>
        {#each register.issues as issue (issue.pid)}
          <tr>
            <th scope="row"><a href={localeHref(`/plans/${issue.plan_pid}/governance`)} aria-label={t("p.issues.plan_link")}>{issue.title}</a></th>
            <td>{enumLabel("issue_kind", issue.kind)}</td>
            <td>{enumLabel("severity", issue.severity)}</td>
            <td>{enumLabel("issue_status", issue.status)}</td>
            <td>{issue.owner_ref ?? t("p.issues.no_owner")}</td>
            <td>{issue.age_calendar_days}</td>
            <td>
              {issue.due_date ?? t("p.issues.no_due")}
              {#if issue.overdue}<strong> · {t("p.issues.overdue_flag")}</strong>{/if}
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
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
