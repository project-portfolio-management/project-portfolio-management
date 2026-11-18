<!--
  A plan's change requests (`/plans/[pid]/change-requests`, NC-1 slice 2, IR-4): the plan's change requests newest
  first, draft one, edit a draft (the service replaces all its fields), and take the steps its status allows: submit
  (naming the approver), start the review, record the decision, withdraw. The page offers steps by status only; the
  service decides who may take them. NC-1 slice 12 part (c): the steps read the one request as stored now
  (`GET …/{cr}`, with the issues and risks it answers); an approved change is marked implemented; the issues and risks
  it answers are chosen while it is open (`PUT …/links`; an issue answers one request at most; nothing sent for no
  change); the forecast of its what-if edits is shown, labelled a forecast (`GET …/preview`); and a baseline is taken
  because of it, once, on a person's choice after a question (`POST …/baseline`). Every rule is said first. Refusals
  say the catalogue's sentence, never the service's own text.
-->
<script lang="ts">
  import { rowOf } from "#lib/row-of.js";
  import { errorText } from "#lib/error-text.js";
  import { resource } from "#lib/resource.svelte.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t, tn, localeHref, type StringKey } from "#lib/i18n.svelte.js";
  import { enumLabel } from "#lib/enum-labels.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { ApiError } from "#lib/api/client.js";
  import { formatDate } from "#lib/locale-format.js";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import { PpmClient, money, type ChangeRequest, type Issue, type PlanChangeRequestList, type Risk } from "#lib/api/ppm.js";
  import {
    CR_KINDS,
    LINKS_MAX,
    answeredElsewhere,
    approverProblem,
    canBaseline,
    canRelink,
    forecastView,
    linksBody,
    linksProblem,
    type ChangeRequestAction,
    type ForecastView,
    changeRequestBody,
    changeRequestProblem,
    decisionReasonShort,
    emptyChangeRequestForm,
    formFromChangeRequest,
    refusalKey,
    stepsFor,
    type ChangeRequestProblem,
  } from "#lib/change-request-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");

  const loaded = resource<PlanChangeRequestList>(() => ppm.planChangeRequests(pid), "p.plan_change_requests.could_not_load");
  const list = $derived(loaded.data);
  const loadError = $derived(loaded.error);
  onMount(() => {
    void loaded.reload();
  });

  let form = $state(emptyChangeRequestForm());
  // The draft being edited, kept whole so its what-if edits go back unchanged.
  let editing = $state<ChangeRequest | null>(null);
  // The request whose steps are open, as `GET …/{cr}` returned it (with the issues and risks it answers).
  let chosenPid = $state<string | null>(null);
  let chosen = $state<ChangeRequest | null>(null);
  const steps = $derived(chosen ? stepsFor(chosen.status) : []);
  // The plan's issues and risks a request may answer, read when steps are first opened.
  let planIssueRows = $state<Issue[]>([]);
  let planRisks = $state<Risk[]>([]);
  let linkablesLoaded = $state(false);
  let chosenIssues = $state<string[]>([]);
  let chosenRisks = $state<string[]>([]);
  let forecast = $state<ForecastView | null>(null);
  let approver = $state("");
  let decision = $state<"approve" | "reject">("approve");
  let decisionReason = $state("");
  let notice = $state("");
  let actionError = $state<string | null>(null);
  let saving = $state(false);
  const error = $derived(actionError ?? loadError);

  const PROBLEM_KEY: Record<NonNullable<ChangeRequestProblem>, StringKey> = {
    title_required: "p.plan_change_requests.problem_title_required",
    description_required: "p.plan_change_requests.problem_description_required",
    reason_required: "p.plan_change_requests.problem_reason_required",
    days_not_whole: "p.plan_change_requests.problem_days_not_whole",
    days_out_of_range: "p.plan_change_requests.problem_days_out_of_range",
    cost_not_amount: "p.plan_change_requests.problem_cost_not_amount",
    cost_needs_currency: "p.plan_change_requests.problem_cost_needs_currency",
    currency_needs_cost: "p.plan_change_requests.problem_currency_needs_cost",
    currency_invalid: "p.plan_change_requests.problem_currency_invalid",
  };

  /** The sentence for a failed action: a refusal in the catalogue's words; what changed is read again. */
  async function refused(err: unknown, action: ChangeRequestAction) {
    const key = err instanceof ApiError ? refusalKey(action, err.status) : null;
    actionError = key ? t(key) : errorText(err, "p.plan_change_requests.could_not_save");
    if (!(err instanceof ApiError)) return;
    if (err.status === 404) {
      chosenPid = null;
      chosen = null;
      await loaded.reload();
      return;
    }
    if (err.status === 409) {
      await loaded.reload();
      await readChosen();
    }
    if (action === "links" && (err.status === 409 || err.status === 422)) await loadLinkables();
  }

  /** Read the open request again, as stored now, and its links into the form. */
  async function readChosen() {
    if (!chosenPid) return;
    const cr = await ppm.changeRequest(pid, chosenPid);
    chosen = cr;
    chosenIssues = [...(cr.issue_pids ?? [])];
    chosenRisks = [...(cr.risk_pids ?? [])];
  }

  /** The plan's issues and risks, for the links. A failure leaves the links' form disabled. */
  async function loadLinkables() {
    try {
      const [issues, risks] = await Promise.all([ppm.planIssues(pid), ppm.listRisks(pid)]);
      planIssueRows = issues.issues;
      planRisks = risks;
      linkablesLoaded = true;
    } catch {
      linkablesLoaded = false;
    }
  }

  function startEdit(cr: ChangeRequest) {
    editing = cr;
    form = formFromChangeRequest(cr);
    notice = "";
    actionError = null;
  }
  function reset() {
    editing = null;
    form = emptyChangeRequestForm();
  }
  async function openSteps(cr: ChangeRequest) {
    chosenPid = cr.pid;
    chosen = null;
    forecast = null;
    approver = cr.approver_ref ?? "";
    decision = "approve";
    decisionReason = "";
    notice = "";
    actionError = null;
    if (!linkablesLoaded) void loadLinkables();
    try {
      await readChosen();
      approver = (chosen as ChangeRequest | null)?.approver_ref ?? approver;
    } catch (err) {
      await refused(err, "read");
    }
  }
  function closeSteps() {
    chosenPid = null;
    chosen = null;
    forecast = null;
  }

  async function save(e: SubmitEvent) {
    e.preventDefault();
    actionError = null;
    notice = "";
    const problem = changeRequestProblem(form);
    if (problem) {
      actionError = t(PROBLEM_KEY[problem]);
      return;
    }
    saving = true;
    try {
      if (editing) await ppm.updateChangeRequest(pid, editing.pid, changeRequestBody(form, editing.edits ?? null));
      else await ppm.draftChangeRequest(pid, changeRequestBody(form));
      notice = editing ? t("p.plan_change_requests.saved") : t("p.plan_change_requests.drafted");
      reset();
      await loaded.reload();
    } catch (err) {
      await refused(err, editing ? "edit" : "draft");
    } finally {
      saving = false;
    }
  }

  /** Take an action on the chosen request, then read the list and the request again. */
  async function take(action: ChangeRequestAction, call: () => Promise<unknown>, done: StringKey) {
    actionError = null;
    notice = "";
    saving = true;
    try {
      await call();
      notice = t(done);
      await loaded.reload();
      await readChosen();
    } catch (err) {
      await refused(err, action);
    } finally {
      saving = false;
    }
  }

  function saveLinks(e: SubmitEvent) {
    e.preventDefault();
    if (!chosen) return;
    actionError = null;
    notice = "";
    const problem = linksProblem(chosenIssues, chosenRisks, planIssueRows, chosen.pid);
    if (problem) {
      actionError =
        problem.key === "p.plan_change_requests.problem_links_elsewhere"
          ? t(problem.key, { title: problem.title })
          : t(problem.key, { max: LINKS_MAX });
      return;
    }
    const body = linksBody(chosen, chosenIssues, chosenRisks);
    if (body === null) {
      notice = t("p.plan_change_requests.links_unchanged");
      return;
    }
    const cr = chosen.pid;
    void take("links", async () => {
      await ppm.setChangeRequestLinks(pid, cr, body);
      await loadLinkables();
    }, "p.plan_change_requests.links_saved");
  }

  async function showForecast() {
    if (!chosen) return;
    actionError = null;
    notice = "";
    saving = true;
    try {
      forecast = forecastView(await ppm.changeRequestPreview(pid, chosen.pid));
    } catch (err) {
      forecast = null;
      await refused(err, "preview");
    } finally {
      saving = false;
    }
  }

  async function takeBaseline() {
    if (!chosen) return;
    if (!(await askConfirm(t("p.plan_change_requests.confirm_baseline", { title: chosen.title })))) return;
    const cr = chosen.pid;
    actionError = null;
    notice = "";
    saving = true;
    try {
      const taken = await ppm.baselineChangeRequest(pid, cr);
      notice = t("p.plan_change_requests.baseline_taken", { version: taken.baseline_version });
      await loaded.reload();
      await readChosen();
    } catch (err) {
      await refused(err, "baseline");
    } finally {
      saving = false;
    }
  }

  const issueTitle = (p: string) => planIssueRows.find((i) => i.pid === p)?.title ?? null;
  const riskTitle = (p: string) => planRisks.find((r) => r.pid === p)?.title ?? null;

  function submit(e: SubmitEvent) {
    e.preventDefault();
    if (!chosen) return;
    const problem = approverProblem(approver, chosen.requested_by);
    if (problem) {
      actionError = t(
        problem === "approver_is_requester"
          ? "p.plan_change_requests.problem_approver_is_requester"
          : "p.plan_change_requests.problem_approver_not_reference",
      );
      notice = "";
      return;
    }
    const cr = chosen.pid;
    void take("submit", () => ppm.submitChangeRequest(pid, cr, { approver_ref: approver.trim() }), "p.plan_change_requests.submitted");
  }

  function decide(e: SubmitEvent) {
    e.preventDefault();
    if (!chosen) return;
    if (decisionReasonShort(decisionReason)) {
      actionError = t("p.plan_change_requests.problem_decision_reason");
      notice = "";
      return;
    }
    const cr = chosen.pid;
    void take(
      "decide",
      () => ppm.decideChangeRequest(pid, cr, { decision, reason: decisionReason.trim() }),
      "p.plan_change_requests.decision_recorded",
    );
  }

  type CR = ChangeRequest;
  const impactText = (cr: CR): string => {
    if (cr.impact.schedule_business_days === null && cr.impact.cost_minor === null) return t("p.change_requests.no_impact");
    const days = cr.impact.schedule_business_days !== null ? t("p.change_requests.impact_days", { days: cr.impact.schedule_business_days }) : "";
    const cost = cr.impact.cost_minor !== null ? `${cr.impact.schedule_business_days !== null ? " · " : ""}${money(cr.impact.cost_minor, cr.impact.cost_currency ?? "")}` : "";
    return `${days}${cost}`;
  };
  const columns: DataGridColumn[] = $derived([
    { id: "title", header: t("p.change_requests.title_col"), rowHeader: true },
    { id: "kind", header: t("p.change_requests.kind_col"), format: (v) => enumLabel("cr_kind", String(v)) },
    { id: "status", header: t("p.change_requests.status_col"), format: (v) => enumLabel("cr_status", String(v)) },
    { id: "impact", header: t("p.change_requests.impact_col"), sortable: false, accessor: (r) => impactText(rowOf<CR>(r)) },
    { id: "approver_ref", header: t("p.change_requests.approver_col"), format: (v) => (v ? String(v) : t("p.change_requests.no_approver")) },
    { id: "age_calendar_days", header: t("p.change_requests.age_col"), align: "end" },
    { id: "pid", header: t("p.plan_issues.actions_col"), sortable: false, cell: actionsCell },
  ]);
</script>

{#snippet actionsCell({ row }: DataGridCellContext)}
  {@const cr = rowOf<CR>(row)}
  <span class="row-actions">
    {#if cr.status === "draft"}
      <button class="button small" type="button" onclick={() => startEdit(cr)} aria-label={`${t("p.plan_issues.edit")}: ${cr.title}`}>{t("p.plan_issues.edit")}</button>
    {/if}
    <button class="button small" type="button" onclick={() => void openSteps(cr)} aria-label={`${t("p.plan_change_requests.steps")}: ${cr.title}`}>{t("p.plan_change_requests.steps")}</button>
  </span>
{/snippet}

<svelte:head><title>{t("p.plan_change_requests.title_ppm")}</title></svelte:head>

<p class="small">
  <a href={localeHref(`/plans/${pid}`)}>{t("p.plan_issues.back")}</a> ·
  <a href={localeHref("/change-requests")}>{t("p.plan_change_requests.register_link")}</a>
</p>
<h1>{t("p.plan_change_requests.title")}</h1>
<p class="small muted">{t("p.plan_change_requests.intro")}</p>

{#if error}<p class="banner" role="alert">{error}</p>{/if}
<p class="small" role="status">{notice}</p>

{#if list}
  {#if list.change_requests.length === 0}
    <p>{t("p.change_requests.none")}</p>
  {:else}
    <p class="small muted" aria-live="polite">
      {tn("p.change_requests.count", list.change_requests.length, { n: list.change_requests.length })}
    </p>
    <DataTable label={t("p.change_requests.title_col")} columns={columns} rows={list.change_requests} rowId={(r) => String(r.pid)} />
  {/if}
{:else if !error}
  <p class="muted">{t("p.change_requests.loading")}</p>
{/if}

{#if chosen}
<section aria-labelledby="cr-steps-h">
  <h2 id="cr-steps-h">{t("p.plan_change_requests.steps_heading", { title: chosen.title })}</h2>
  <p>{t("p.plan_change_requests.status_now", { status: enumLabel("cr_status", chosen.status) })}</p>
  <p class="small">{t("p.plan_change_requests.stated_impact", { impact: impactText(chosen) })}</p>
  {#if chosen.implemented_on}
    <p class="small">{t("p.plan_change_requests.implemented_on", { date: formatDate(chosen.implemented_on) })}</p>
  {/if}
  {#if chosen.decided_on && chosen.decision_reason}
    <p class="small">{t("p.plan_change_requests.decided", { who: chosen.decided_by ?? "", on: formatDate(chosen.decided_on), reason: chosen.decision_reason })}</p>
  {/if}
  {#if steps.length === 0}
    <p class="small muted">{t("p.plan_change_requests.no_steps")}</p>
  {:else}
    <p class="small muted">{t("p.plan_change_requests.who_note")}</p>
  {/if}

  {#if steps.includes("submit")}
    <form class="cr-form" onsubmit={submit} aria-labelledby="cr-submit-h">
      <h3 id="cr-submit-h">{t("p.plan_change_requests.submit_heading")}</h3>
      <label>{t("p.plan_change_requests.f_approver")}
        <input bind:value={approver} maxlength="120" required />
      </label>
      <div class="row">
        <button class="button" type="submit" disabled={saving}>{t("p.plan_change_requests.submit")}</button>
      </div>
    </form>
  {/if}

  {#if steps.includes("review")}
    <div class="row">
      <button class="button" type="button" disabled={saving}
        onclick={() => { const cr = chosen?.pid ?? ""; void take("review", () => ppm.reviewChangeRequest(pid, cr), "p.plan_change_requests.review_started"); }}
      >{t("p.plan_change_requests.review")}</button>
    </div>
  {/if}

  {#if steps.includes("decide")}
    <form class="cr-form" onsubmit={decide} aria-labelledby="cr-decide-h">
      <h3 id="cr-decide-h">{t("p.plan_change_requests.decide_heading")}</h3>
      <label>{t("p.plan_change_requests.f_decision")}
        <select bind:value={decision}>
          <option value="approve">{t("p.plan_change_requests.approve")}</option>
          <option value="reject">{t("p.plan_change_requests.reject")}</option>
        </select>
      </label>
      <label>{t("p.plan_change_requests.f_decision_reason")}
        <textarea bind:value={decisionReason} rows="3" maxlength="10000" required></textarea>
      </label>
      <div class="row">
        <button class="button" type="submit" disabled={saving}>{t("p.plan_change_requests.decide")}</button>
      </div>
    </form>
  {/if}

  {#if steps.includes("implement")}
    <div class="row">
      <button class="button" type="button" disabled={saving}
        onclick={() => { const cr = chosen?.pid ?? ""; void take("implement", () => ppm.implementChangeRequest(pid, cr), "p.plan_change_requests.implemented"); }}
      >{t("p.plan_change_requests.implement")}</button>
    </div>
  {/if}

  <section aria-labelledby="cr-links-h">
    <h3 id="cr-links-h">{t("p.plan_change_requests.links_heading")}</h3>
    {#if (chosen.issue_pids ?? []).length + (chosen.risk_pids ?? []).length === 0}
      <p class="small" data-testid="cr-answers">{t("p.plan_change_requests.links_none")}</p>
    {:else}
      <ul class="small" data-testid="cr-answers">
        {#each chosen.issue_pids ?? [] as p (p)}<li>{t("p.plan_change_requests.answers_issue", { title: issueTitle(p) ?? t("p.plan_change_requests.an_issue") })}</li>{/each}
        {#each chosen.risk_pids ?? [] as p (p)}<li>{t("p.plan_change_requests.answers_risk", { title: riskTitle(p) ?? t("p.plan_change_requests.a_risk") })}</li>{/each}
      </ul>
    {/if}
    {#if canRelink(chosen.status)}
      <p class="small muted">{t("p.plan_change_requests.links_says", { max: LINKS_MAX })}</p>
      <form class="cr-form" novalidate onsubmit={saveLinks} aria-labelledby="cr-links-h">
        <fieldset>
          <legend>{t("p.plan_change_requests.links_issues")}</legend>
          {#each planIssueRows as i (i.pid)}
            {@const elsewhere = answeredElsewhere(i, chosen.pid)}
            <label class="check">
              <input type="checkbox" value={i.pid} bind:group={chosenIssues} disabled={!linkablesLoaded || (elsewhere && !chosenIssues.includes(i.pid))} />
              {elsewhere ? t("p.plan_change_requests.issue_elsewhere", { title: i.title }) : i.title}
            </label>
          {/each}
          {#if linkablesLoaded && planIssueRows.length === 0}<p class="small muted">{t("p.plan_change_requests.links_no_issues")}</p>{/if}
        </fieldset>
        <fieldset>
          <legend>{t("p.plan_change_requests.links_risks")}</legend>
          {#each planRisks as r (r.pid)}
            <label class="check">
              <input type="checkbox" value={r.pid} bind:group={chosenRisks} disabled={!linkablesLoaded} />
              {r.title}
            </label>
          {/each}
          {#if linkablesLoaded && planRisks.length === 0}<p class="small muted">{t("p.plan_change_requests.links_no_risks")}</p>{/if}
        </fieldset>
        <div class="row">
          <button class="button" type="submit" disabled={saving || !linkablesLoaded}>{t("p.plan_change_requests.links_save")}</button>
        </div>
      </form>
    {:else}
      <p class="small muted">{t("p.plan_change_requests.links_closed")}</p>
    {/if}
  </section>

  <section aria-labelledby="cr-forecast-h">
    <h3 id="cr-forecast-h">{t("p.plan_change_requests.forecast_heading")}</h3>
    {#if chosen.impact.has_forecast_edits}
      <p class="small muted">{t("p.plan_change_requests.forecast_says")}</p>
      <div class="row">
        <button class="button" type="button" disabled={saving} onclick={() => void showForecast()}>{t("p.plan_change_requests.forecast_show")}</button>
      </div>
      {#if forecast}
        <div data-testid="cr-forecast">
          <p><strong>{t("p.plan_change_requests.forecast_label")}</strong></p>
          <p>{forecast.finishBefore && forecast.finishAfter
              ? t("p.plan_change_requests.forecast_finish", { before: formatDate(forecast.finishBefore), after: formatDate(forecast.finishAfter) })
              : t("p.plan_change_requests.forecast_finish_unknown")}</p>
          <p>{tn("p.plan_change_requests.forecast_moved", forecast.moved.length, { n: forecast.moved.length })}</p>
          {#if forecast.moved.length > 0}
            <ul class="small">
              {#each forecast.moved as m (m.pid)}
                <li>{m.finishDays > 0
                    ? tn("p.plan_change_requests.forecast_later", m.finishDays, { title: m.title ?? t("p.plan_change_requests.a_task"), n: m.finishDays })
                    : tn("p.plan_change_requests.forecast_earlier", -m.finishDays, { title: m.title ?? t("p.plan_change_requests.a_task"), n: -m.finishDays })}</li>
              {/each}
            </ul>
          {/if}
          {#if forecast.labour.shown}
            {#each forecast.labour.deltas as d (d.currency)}
              <p class="small">{t("p.plan_change_requests.forecast_labour", { amount: money(d.deltaMinor, d.currency) })}</p>
            {/each}
          {:else if forecast.labour.why === "no_finance_access"}
            <p class="small muted">{t("p.plan_change_requests.forecast_labour_hidden")}</p>
          {:else if forecast.labour.why === "no_costed_work"}
            <p class="small muted">{t("p.plan_change_requests.forecast_labour_none")}</p>
          {:else}
            <p class="small muted">{t("p.plan_change_requests.forecast_labour_unknown")}</p>
          {/if}
        </div>
      {/if}
    {:else}
      <p class="small muted">{t("p.plan_change_requests.forecast_no_edits")}</p>
    {/if}
  </section>

  <section aria-labelledby="cr-baseline-h">
    <h3 id="cr-baseline-h">{t("p.plan_change_requests.baseline_heading")}</h3>
    {#if chosen.baseline_pid}
      <p class="small" data-testid="cr-baseline">{t("p.plan_change_requests.baseline_done")}</p>
    {:else if canBaseline(chosen)}
      <p class="small muted">{t("p.plan_change_requests.baseline_says")}</p>
      <div class="row">
        <button class="button" type="button" disabled={saving} onclick={() => void takeBaseline()}>{t("p.plan_change_requests.baseline_take")}</button>
      </div>
    {:else}
      <p class="small muted" data-testid="cr-baseline">{t("p.plan_change_requests.baseline_not_yet")}</p>
    {/if}
  </section>

  <div class="row">
    {#if steps.includes("withdraw")}
      <button class="button" type="button" disabled={saving}
        onclick={() => { const cr = chosen?.pid ?? ""; void take("withdraw", () => ppm.withdrawChangeRequest(pid, cr), "p.plan_change_requests.withdrawn"); }}
      >{t("p.plan_change_requests.withdraw")}</button>
    {/if}
    <button class="button" type="button" onclick={closeSteps}>{t("p.plan_change_requests.close_steps")}</button>
  </div>
</section>
{/if}

<section aria-labelledby="cr-form-h">
  <h2 id="cr-form-h">{editing ? t("p.plan_change_requests.edit_heading") : t("p.plan_change_requests.draft_heading")}</h2>
  <form class="cr-form" onsubmit={save} aria-labelledby="cr-form-h">
    <label>{t("p.plan_issues.f_title")}
      <input bind:value={form.title} maxlength="200" required />
    </label>
    <label>{t("p.plan_change_requests.f_description")}
      <textarea bind:value={form.description} rows="3" maxlength="10000" required></textarea>
    </label>
    <label>{t("p.plan_change_requests.f_reason")}
      <textarea bind:value={form.reason} rows="3" maxlength="10000" required></textarea>
    </label>
    <label>{t("p.change_requests.kind_col")}
      <select bind:value={form.kind}>
        {#each CR_KINDS as k (k)}<option value={k}>{enumLabel("cr_kind", k)}</option>{/each}
      </select>
    </label>
    <label>{t("p.plan_change_requests.f_days")}
      <input bind:value={form.scheduleDays} inputmode="numeric" />
    </label>
    <label>{t("p.plan_change_requests.f_cost")}
      <input bind:value={form.costAmount} inputmode="decimal" />
    </label>
    <label>{t("p.plan_change_requests.f_currency")}
      <input bind:value={form.currency} maxlength="3" autocomplete="off" />
    </label>
    <label>{t("p.plan_change_requests.f_summary")}
      <textarea bind:value={form.summary} rows="2" maxlength="10000"></textarea>
    </label>
    <div class="row">
      <button class="button primary" type="submit" disabled={saving}>{editing ? t("p.plan_issues.save") : t("p.plan_change_requests.save_draft")}</button>
      {#if editing}<button class="button" type="button" onclick={reset}>{t("p.plan_issues.cancel_edit")}</button>{/if}
    </div>
  </form>
</section>

<style>
  .cr-form {
    display: grid;
    gap: 0.5rem;
    max-inline-size: 32rem;
    margin-block: 1rem;
  }
  .cr-form label {
    display: grid;
    gap: 0.2rem;
  }
  .row-actions {
    display: inline-flex;
    flex-wrap: wrap;
    gap: 0.25rem;
  }
  .check {
    display: flex;
    gap: 0.4rem;
    align-items: center;
    min-block-size: 24px;
  }
  fieldset {
    display: grid;
    gap: 0.25rem;
  }
</style>
