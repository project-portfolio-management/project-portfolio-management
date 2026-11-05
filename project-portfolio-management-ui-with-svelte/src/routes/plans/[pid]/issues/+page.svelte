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
  import type { RuleField } from "#lib/issue-settings-view.js";
  import { PpmClient, type Issue, type IssueOptions, type PlanIssueList, type Risk } from "#lib/api/ppm.js";
  import {
    ISSUE_SEVERITIES,
    allowedActions,
    emptyIssueForm,
    formFromIssue,
    initialStatus,
    issueBody,
    issueProblem,
    missingFields,
    needsResolution,
    readOnlyFields,
    requiredFields,
    statusChoices,
    statusLabel,
    typeChoices,
    typeLabel,
    usableOptions,
    workflowFor,
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
  // The organization's types and statuses (OC-1). Without them the form offers the built-ins, and the service decides.
  let options = $state<IssueOptions | null>(null);
  async function loadOptions() {
    try {
      options = usableOptions(await ppm.issueOptions(pid));
    } catch {
      options = null;
    }
  }
  onMount(() => {
    void loaded.reload();
    void loadRisks();
    void loadOptions();
  });

  let form = $state(emptyIssueForm());
  let editing = $state<string | null>(null);
  // The status the issue had when editing began, which is where a move starts from.
  let startedAt = $state<string | null>(null);
  // The type it had then, whose workflow locks its fields.
  let startedKind = $state<string | null>(null);
  const workflow = $derived(workflowFor(options, form.kind));
  const statuses = $derived(statusChoices(workflow, startedAt, options?.caller_roles ?? []));
  const roles = $derived(options?.caller_roles ?? []);
  const allowed = $derived(allowedActions(options));
  // The form leaves out what the service would refuse. The service still decides.
  const required = $derived(requiredFields(workflow, form.status, roles));
  const locked = $derived(
    readOnlyFields(workflowFor(options, startedKind ?? form.kind), startedAt, roles, options?.caller_admin ?? false),
  );
  /** Whether a field can be typed in: the permission allows it, and the status has not locked it. */
  function open(field: string): boolean {
    if (editing && !allowed.edit && !(field === "resolution" && allowed.move)) return false;
    return !locked.includes(field);
  }
  const types = $derived(typeChoices(options, form.kind));
  const typesKnown = $derived(options?.types ?? null);
  // The organization may have retired the default type, so the form starts on one it can actually offer.
  $effect(() => {
    if (!types.some((k) => k.key === form.kind)) form.kind = types[0]?.key ?? form.kind;
  });
  // The type decides the workflow: a status it does not offer gives way to the workflow's start.
  $effect(() => {
    if (!statuses.some((s) => s.key === form.status)) form.status = initialStatus(workflow);
  });
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
    startedAt = issue.status;
    startedKind = issue.kind;
    form = formFromIssue(issue);
    notice = "";
    actionError = null;
  }
  function reset() {
    editing = null;
    startedAt = null;
    startedKind = null;
    form = emptyIssueForm();
  }

  async function save(e: SubmitEvent) {
    e.preventDefault();
    actionError = null;
    notice = "";
    const problem = issueProblem(form, needsResolution(workflow, form.status));
    if (problem) {
      actionError = problemText(problem);
      return;
    }
    const missing = missingFields(form, required);
    if (missing.length > 0) {
      actionError = t("p.plan_issues.problem_fields_required", {
        fields: missing.map((f) => t(`p.issue_settings.field_${f as RuleField}`)).join(", "),
      });
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
    { id: "kind", header: t("p.issues.kind_col"), format: (v) => typeLabel(typesKnown, String(v)) },
    { id: "severity", header: t("p.issues.severity_col"), format: (v) => enumLabel("severity", String(v)) },
    { id: "status", header: t("p.issues.status_col"), format: (v, r) => statusLabel(workflowFor(options, rowOf<Issue>(r).kind), String(v)) },
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
  {#if allowed.edit || allowed.move}
    <button class="button small" type="button" onclick={() => startEdit(issue)} aria-label={`${t("p.plan_issues.edit")}: ${issue.title}`}>{t("p.plan_issues.edit")}</button>
  {/if}
{/snippet}

<svelte:head><title>{t("p.plan_issues.title_ppm")}</title></svelte:head>

<p class="small">
  <a href={localeHref(`/plans/${pid}`)}>{t("p.plan_issues.back")}</a> ·
  <a href={localeHref("/issues")}>{t("p.plan_issues.register_link")}</a> ·
  <a href={localeHref(`/plans/${pid}/issue-settings`)}>{t("p.plan_issues.settings_link")}</a>
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

{#if editing || allowed.add}
<section aria-labelledby="issue-form-h">
  <h2 id="issue-form-h">{editing ? t("p.plan_issues.edit_heading") : t("p.plan_issues.raise_heading")}</h2>
  <form class="issue-form" onsubmit={save} aria-labelledby="issue-form-h">
    <label>{t("p.plan_issues.f_title")}
      <input bind:value={form.title} maxlength="200" required disabled={!open("title")} />
    </label>
    <label>{t("p.plan_issues.f_description")}
      <textarea bind:value={form.description} rows="3" disabled={!open("description")} required={required.includes("description")}></textarea>
    </label>
    <label>{t("p.issues.kind_col")}
      <select bind:value={form.kind} disabled={editing !== null && !allowed.edit}>
        {#each types as k (k.key)}<option value={k.key}>{typeLabel(typesKnown, k.key)}</option>{/each}
      </select>
    </label>
    <label>{t("p.issues.severity_col")}
      <select bind:value={form.severity} disabled={!open("severity")}>
        {#each ISSUE_SEVERITIES as s (s)}<option value={s}>{enumLabel("severity", s)}</option>{/each}
      </select>
    </label>
    <label>{t("p.issues.status_col")}
      <select bind:value={form.status} disabled={editing !== null && !allowed.move}>
        {#each statuses as s (s.key)}<option value={s.key}>{statusLabel(workflow, s.key)}</option>{/each}
      </select>
    </label>
    <label>{t("p.issue_settings.field_priority")}
      <input bind:value={form.priority} maxlength="120" disabled={!open("priority")} required={required.includes("priority")} />
    </label>
    <label>{t("p.plan_issues.f_owner")}
      <input bind:value={form.ownerRef} maxlength="120" disabled={!open("owner_ref")} required={required.includes("owner_ref")} />
    </label>
    <label>{t("p.plan_issues.f_raised_on")}
      <input type="date" bind:value={form.raisedOn} disabled={editing !== null && !allowed.edit} />
    </label>
    <label>{t("p.issues.due_col")}
      <input type="date" bind:value={form.dueDate} disabled={!open("due_date")} required={required.includes("due_date")} />
    </label>
    <label>{t("p.plan_issues.f_resolution")}
      <textarea bind:value={form.resolution} rows="2" disabled={!open("resolution")}></textarea>
    </label>
    <div class="row">
      <button class="button primary" type="submit" disabled={saving}>{editing ? t("p.plan_issues.save") : t("p.plan_issues.raise")}</button>
      {#if editing}<button class="button" type="button" onclick={reset}>{t("p.plan_issues.cancel_edit")}</button>{/if}
    </div>
  </form>
</section>
{/if}

{#if allowed.add}
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
{/if}

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
