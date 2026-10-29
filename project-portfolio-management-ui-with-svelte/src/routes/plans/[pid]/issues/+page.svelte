<!--
  A plan's issues (`/plans/[pid]/issues`, NC-1 slice 1): the plan's issues most serious first, raise one, edit one
  (the service replaces all its fields; closing needs a resolution), and raise one from a risk that has happened
  (IR-3: one open issue per risk). Failures say the catalogue's sentence, never the service's own text.
-->
<script lang="ts">
  import { rowOf } from "#lib/row-of.js";
  import { errorText } from "#lib/error-text.js";
  import { resource } from "#lib/resource.svelte.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { enumLabel } from "#lib/enum-labels.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { PpmClient, type Issue, type PlanIssueList, type Risk } from "#lib/api/ppm.js";
  import {
    ISSUE_KINDS,
    ISSUE_SEVERITIES,
    ISSUE_STATUSES,
    emptyIssueForm,
    formFromIssue,
    issueBody,
    issueProblem,
  } from "#lib/issue-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");

  const loaded = resource<PlanIssueList>(() => ppm.planIssues(pid), "p.plan_issues.could_not_load");
  const list = $derived(loaded.data);
  const loadError = $derived(loaded.error);

  let risks = $state<Risk[]>([]);
  async function loadRisks() {
    try {
      risks = await ppm.listRisks(pid);
    } catch {
      risks = []; // the raise-from-risk form is then empty; the register above still works
    }
  }
  onMount(() => {
    void loaded.reload();
    void loadRisks();
  });

  let form = $state(emptyIssueForm());
  let editing = $state<string | null>(null);
  let riskPid = $state("");
  let notice = $state("");
  let actionError = $state<string | null>(null);
  let saving = $state(false);
  const error = $derived(actionError ?? loadError);

  function problemText(p: NonNullable<ReturnType<typeof issueProblem>>): string {
    return p === "title_required"
      ? t("p.plan_issues.problem_title_required")
      : p === "resolution_required"
        ? t("p.plan_issues.problem_resolution_required")
        : t("p.plan_issues.problem_due_before_raised");
  }

  function startEdit(issue: Issue) {
    editing = issue.pid;
    form = formFromIssue(issue);
    notice = "";
    actionError = null;
  }
  function reset() {
    editing = null;
    form = emptyIssueForm();
  }

  async function save(e: SubmitEvent) {
    e.preventDefault();
    actionError = null;
    notice = "";
    const problem = issueProblem(form);
    if (problem) {
      actionError = problemText(problem);
      return;
    }
    saving = true;
    try {
      if (editing) await ppm.updateIssue(pid, editing, issueBody(form));
      else await ppm.raiseIssue(pid, issueBody(form));
      notice = editing ? t("p.plan_issues.saved") : t("p.plan_issues.raised");
      reset();
      await loaded.reload();
    } catch (err) {
      actionError = errorText(err, "p.plan_issues.could_not_save");
    } finally {
      saving = false;
    }
  }

  async function fromRisk(e: SubmitEvent) {
    e.preventDefault();
    if (!riskPid) return;
    actionError = null;
    notice = "";
    saving = true;
    try {
      await ppm.raiseIssueFromRisk(pid, riskPid);
      notice = t("p.plan_issues.raised_from_risk");
      riskPid = "";
      await loaded.reload();
    } catch (err) {
      actionError = errorText(err, "p.plan_issues.could_not_save");
    } finally {
      saving = false;
    }
  }

  const columns: DataGridColumn[] = $derived([
    { id: "title", header: t("p.issues.title_col"), rowHeader: true },
    { id: "kind", header: t("p.issues.kind_col"), format: (v) => enumLabel("issue_kind", String(v)) },
    { id: "severity", header: t("p.issues.severity_col"), format: (v) => enumLabel("severity", String(v)) },
    { id: "status", header: t("p.issues.status_col"), format: (v) => enumLabel("issue_status", String(v)) },
    { id: "owner_ref", header: t("p.issues.owner_col"), format: (v) => (v ? String(v) : t("p.issues.no_owner")) },
    { id: "age_calendar_days", header: t("p.issues.age_col"), align: "end" },
    { id: "due_date", header: t("p.issues.due_col"), format: (v) => (v ? String(v) : t("p.issues.no_due")), cell: dueCell },
    { id: "pid", header: t("p.plan_issues.actions_col"), sortable: false, cell: actionsCell },
  ]);
</script>

{#snippet dueCell({ row, formatted }: DataGridCellContext)}
  {formatted}{#if rowOf<Issue>(row).overdue}<strong> · {t("p.issues.overdue_flag")}</strong>{/if}
{/snippet}

{#snippet actionsCell({ row }: DataGridCellContext)}
  {@const issue = rowOf<Issue>(row)}
  <button class="button small" type="button" onclick={() => startEdit(issue)} aria-label={`${t("p.plan_issues.edit")}: ${issue.title}`}>{t("p.plan_issues.edit")}</button>
{/snippet}

<svelte:head><title>{t("p.plan_issues.title_ppm")}</title></svelte:head>

<p class="small">
  <a href={localeHref(`/plans/${pid}`)}>{t("p.plan_issues.back")}</a> ·
  <a href={localeHref("/issues")}>{t("p.plan_issues.register_link")}</a>
</p>
<h1>{t("p.plan_issues.title")}</h1>
<p class="small muted">{t("p.plan_issues.intro")}</p>

{#if error}<p class="banner" role="alert">{error}</p>{/if}
<p class="small" role="status">{notice}</p>

{#if list}
  <p class="small muted" aria-live="polite">
    {t("p.plan_issues.counts", { total: list.total, open: list.open, overdue: list.overdue })}
  </p>
  {#if list.issues.length === 0}
    <p>{t("p.issues.none")}</p>
  {:else}
    <DataTable label={t("p.issues.title_col")} columns={columns} rows={list.issues} rowId={(r) => String(r.pid)} />
  {/if}
{:else if !error}
  <p class="muted">{t("p.issues.loading")}</p>
{/if}

<section aria-labelledby="issue-form-h">
  <h2 id="issue-form-h">{editing ? t("p.plan_issues.edit_heading") : t("p.plan_issues.raise_heading")}</h2>
  <form class="issue-form" onsubmit={save} aria-labelledby="issue-form-h">
    <label>{t("p.plan_issues.f_title")}
      <input bind:value={form.title} maxlength="200" required />
    </label>
    <label>{t("p.plan_issues.f_description")}
      <textarea bind:value={form.description} rows="3"></textarea>
    </label>
    <label>{t("p.issues.kind_col")}
      <select bind:value={form.kind}>
        {#each ISSUE_KINDS as k (k)}<option value={k}>{enumLabel("issue_kind", k)}</option>{/each}
      </select>
    </label>
    <label>{t("p.issues.severity_col")}
      <select bind:value={form.severity}>
        {#each ISSUE_SEVERITIES as s (s)}<option value={s}>{enumLabel("severity", s)}</option>{/each}
      </select>
    </label>
    <label>{t("p.issues.status_col")}
      <select bind:value={form.status}>
        {#each ISSUE_STATUSES as s (s)}<option value={s}>{enumLabel("issue_status", s)}</option>{/each}
      </select>
    </label>
    <label>{t("p.plan_issues.f_owner")}
      <input bind:value={form.ownerRef} maxlength="120" />
    </label>
    <label>{t("p.plan_issues.f_raised_on")}
      <input type="date" bind:value={form.raisedOn} />
    </label>
    <label>{t("p.issues.due_col")}
      <input type="date" bind:value={form.dueDate} />
    </label>
    <label>{t("p.plan_issues.f_resolution")}
      <textarea bind:value={form.resolution} rows="2"></textarea>
    </label>
    <div class="row">
      <button class="button primary" type="submit" disabled={saving}>{editing ? t("p.plan_issues.save") : t("p.plan_issues.raise")}</button>
      {#if editing}<button class="button" type="button" onclick={reset}>{t("p.plan_issues.cancel_edit")}</button>{/if}
    </div>
  </form>
</section>

<section aria-labelledby="from-risk-h">
  <h2 id="from-risk-h">{t("p.plan_issues.from_risk_heading")}</h2>
  <p class="small muted">{t("p.plan_issues.from_risk_intro")}</p>
  <form class="issue-form" onsubmit={fromRisk} aria-labelledby="from-risk-h">
    <label>{t("p.plan_issues.f_risk")}
      <select bind:value={riskPid}>
        <option value="">{t("p.plan_issues.choose_risk")}</option>
        {#each risks as r (r.pid)}<option value={r.pid}>{r.title}</option>{/each}
      </select>
    </label>
    <div class="row">
      <button class="button" type="submit" disabled={saving || !riskPid}>{t("p.plan_issues.raise_from_risk")}</button>
    </div>
  </form>
</section>

<style>
  .issue-form {
    display: grid;
    gap: 0.5rem;
    max-inline-size: 32rem;
    margin-block: 1rem;
  }
  .issue-form label {
    display: grid;
    gap: 0.2rem;
  }
</style>
