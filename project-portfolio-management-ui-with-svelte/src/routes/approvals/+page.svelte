<!--
  Approvals (`/approvals`, PL-1; NC-1 slice 12 part n): one person asked to approve a plan, task, risk or other subject,
  outside any approval process. What waits on the reader (`GET /api/approvals?mine=approver&status=pending`) is approved
  or rejected there, a rejection saying why (`POST …/decide`); what the reader asked for (`?mine=requester`) is listed
  with its outcome, and a pending one is withdrawn after a question (`POST …/withdraw`); and the reader asks someone
  (`POST /api/approvals`). Every problem is said before sending; a decision is final, and a request already settled
  (409) or not the reader's to decide or withdraw (403) is said in the page's words. Rules in `src/lib/approvals-view.ts`.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import { errorText } from "#lib/error-text.js";
  import { localeHref, t, tDynamic } from "#lib/i18n.svelte.js";
  import { formatDateTime } from "#lib/locale-format.js";
  import { ApiError } from "#lib/api/client.js";
  import { PpmClient, type ApprovalRequest } from "#lib/api/ppm.js";
  import { MAX_TEXT, SUBJECT_KINDS, decideBody, refusalKey, requestBody } from "#lib/approvals-view.js";

  const ppm = PpmClient.withFetch();

  let waiting = $state<ApprovalRequest[] | null>(null);
  let asked = $state<ApprovalRequest[] | null>(null);
  let loadError = $state<string | null>(null);
  let error = $state<string | null>(null);
  let notice = $state<string | null>(null);
  let busy = $state(false);
  let comments = $state<Record<string, string>>({});
  let form = $state({ subject_kind: "plan", subject_pid: "", approver_ref: "", message: "" });
  let problems = $state<string[]>([]);

  async function load() {
    try {
      [waiting, asked] = await Promise.all([
        ppm.approvalRequests("approver", "pending"),
        ppm.approvalRequests("requester"),
      ]);
      loadError = null;
    } catch (err) {
      loadError = errorText(err, "p.approvals.could_not_load");
    }
  }
  onMount(load);

  /** A refusal in the page's words where it has its own, else the plain sentence with the service's reason. */
  function refused(err: unknown, action: "request" | "decide" | "withdraw") {
    const key = err instanceof ApiError ? refusalKey(err.status, action) : null;
    error = key ? tDynamic(key) : errorText(err, `p.approvals.could_not_${action}` as "p.approvals.could_not_decide");
  }

  async function decide(r: ApprovalRequest, decision: "approved" | "rejected") {
    notice = error = null;
    const d = decideBody(decision, comments[r.pid] ?? "");
    if ("problem" in d) {
      error = tDynamic(d.problem);
      return;
    }
    busy = true;
    try {
      await ppm.decideApproval(r.pid, d.body);
      notice = decision === "approved" ? t("p.approvals.approved_done") : t("p.approvals.rejected_done");
      await load();
    } catch (err) {
      refused(err, "decide");
      await load();
    } finally {
      busy = false;
    }
  }

  async function withdraw(r: ApprovalRequest) {
    if (!(await askConfirm(t("p.approvals.withdraw_ask")))) return;
    notice = error = null;
    busy = true;
    try {
      await ppm.withdrawApproval(r.pid);
      notice = t("p.approvals.withdrawn_done");
    } catch (err) {
      refused(err, "withdraw");
    } finally {
      busy = false;
      await load();
    }
  }

  async function ask(e: SubmitEvent) {
    e.preventDefault();
    notice = error = null;
    const r = requestBody(form);
    if ("problems" in r) {
      problems = r.problems;
      return;
    }
    problems = [];
    busy = true;
    try {
      await ppm.requestApproval(r.body);
      notice = t("p.approvals.asked_done", { approver: r.body.approver_ref });
      form = { ...form, subject_pid: "", approver_ref: "", message: "" };
      await load();
    } catch (err) {
      refused(err, "request");
    } finally {
      busy = false;
    }
  }
</script>

{#snippet subject(r: ApprovalRequest)}
  {tDynamic(`p.approvals.kind.${r.subject_kind}`)}
  {#if r.subject_kind === "plan"}<a href={localeHref(`/plans/${r.subject_pid}`)}>{r.subject_pid}</a>{:else}<code>{r.subject_pid}</code>{/if}
{/snippet}

<svelte:head><title>{t("p.approvals.title_ppm")}</title></svelte:head>

<h1>{t("p.approvals.title")}</h1>
<p class="lede">{t("p.approvals.intro")}</p>
{#if loadError}<p class="banner" role="alert">{loadError}</p>{/if}
{#if error}<p class="banner error" role="alert" data-testid="approvals-error">{error}</p>{/if}
{#if notice}<p class="banner" role="status" data-testid="approvals-notice">{notice}</p>{/if}

{#if !waiting && !loadError}
  <p class="muted">{t("p.approvals.loading")}</p>
{:else if waiting && asked}
  <section class="surface stack" aria-labelledby="waiting-h">
    <h2 id="waiting-h">{t("p.approvals.waiting_h")}</h2>
    {#if waiting.length === 0}
      <p class="muted" data-testid="waiting-none">{t("p.approvals.waiting_none")}</p>
    {:else}
      <ul class="items" data-testid="waiting">
        {#each waiting as r (r.pid)}
          <li>
            <p id={`req-${r.pid}`}>{@render subject(r)}</p>
            <p class="small muted">{t("p.approvals.asked_by", { who: r.requested_by, at: formatDateTime(r.created_at) })}</p>
            {#if r.message}<p>{r.message}</p>{/if}
            <label
              >{t("p.approvals.comment_field")}
              <textarea rows="2" maxlength={MAX_TEXT} bind:value={comments[r.pid]} aria-describedby={`req-${r.pid}`}></textarea></label
            >
            <div class="row">
              <button class="button" type="button" disabled={busy} onclick={() => decide(r, "approved")} aria-describedby={`req-${r.pid}`}
                >{t("p.approvals.approve")}</button
              >
              <button class="button danger" type="button" disabled={busy} onclick={() => decide(r, "rejected")} aria-describedby={`req-${r.pid}`}
                >{t("p.approvals.reject")}</button
              >
            </div>
          </li>
        {/each}
      </ul>
    {/if}
  </section>

  <section class="surface stack" aria-labelledby="asked-h">
    <h2 id="asked-h">{t("p.approvals.asked_h")}</h2>
    {#if asked.length === 0}
      <p class="muted" data-testid="asked-none">{t("p.approvals.asked_none")}</p>
    {:else}
      <ul class="items" data-testid="asked">
        {#each asked as r (r.pid)}
          <li>
            <p id={`mine-${r.pid}`}>{@render subject(r)}</p>
            <p class="small">
              {t("p.approvals.asked_of", { approver: r.approver_ref })} ·
              <strong>{tDynamic(`p.approvals.status.${r.status}`)}</strong>
              {#if r.decided_at}· {formatDateTime(r.decided_at)}{/if}
            </p>
            {#if r.decision_comment}<p class="small">{t("p.approvals.their_comment", { comment: r.decision_comment })}</p>{/if}
            {#if r.process_execution_pid}<p class="small muted">{t("p.approvals.from_process")}</p>{/if}
            {#if r.status === "pending" && !r.process_execution_pid}
              <button class="button small" type="button" disabled={busy} onclick={() => withdraw(r)} aria-describedby={`mine-${r.pid}`}
                >{t("p.approvals.withdraw")}</button
              >
            {/if}
          </li>
        {/each}
      </ul>
    {/if}
  </section>

  <form class="surface stack" onsubmit={ask} aria-labelledby="ask-h" novalidate>
    <h2 id="ask-h">{t("p.approvals.ask_h")}</h2>
    {#if problems.length > 0}
      <ul class="banner error" role="alert" data-testid="ask-problems">
        {#each problems as p (p)}<li>{tDynamic(p)}</li>{/each}
      </ul>
    {/if}
    <label
      >{t("p.approvals.subject_kind_field")}
      <select bind:value={form.subject_kind}>
        {#each SUBJECT_KINDS as k (k)}<option value={k}>{tDynamic(`p.approvals.kind.${k}`)}</option>{/each}
      </select></label
    >
    <label>{t("p.approvals.subject_field")} <input bind:value={form.subject_pid} autocomplete="off" /></label>
    <label>{t("p.approvals.approver_field")} <input bind:value={form.approver_ref} autocomplete="off" /></label>
    <label>{t("p.approvals.message_field")} <textarea rows="2" maxlength={MAX_TEXT} bind:value={form.message}></textarea></label>
    <button class="button primary" type="submit" disabled={busy}>{t("p.approvals.ask")}</button>
  </form>
{/if}

<style>
  .items {
    list-style: none;
    padding: 0;
    display: grid;
    gap: 1rem;
  }
  h2 {
    font-size: 1.05rem;
  }
  label {
    display: block;
  }
  .row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }
</style>
