<!--
  Collaborative review (`/reviews`): delegate an idea, proposal, or plan
  to the right internal or external expert, then track who accepted, who
  declined, and who still owes a verdict.

  Reviewers are named by EntityRef URN (person:/worker:/organization:),
  never by raw email — an outside expert is a record in the person
  registry, so no contact detail is duplicated here. The internal /
  external scope is chosen explicitly, because sharing something with an
  outsider is a disclosure decision, not an inference.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import {
    CapabilityClient,
    type Consensus,
    type Recommendation,
    type Review,
    type ReviewStatus,
    type ReviewSubjectKind,
    type ReviewerScope,
  } from "#lib/api/capabilities.js";

  const api = CapabilityClient.withFetch();
  let reviews = $state<Review[]>([]);
  // Rows matching overall, from `X-Total-Count` — not `reviews.length`,
  // which is only this page (agents/share/restful.md).
  let reviewsTotal = $state(0);
  let error = $state<string | null>(null);
  let notice = $state<string | null>(null);
  let statusFilter = $state<ReviewStatus | "">("");

  // Delegation form.
  let subjectKind = $state<ReviewSubjectKind>("plan");
  let subjectPid = $state("");
  let reviewerRef = $state("");
  let reviewerScope = $state<ReviewerScope>("internal");
  let expertise = $state("");
  let dueOn = $state("");

  // Consensus panel.
  let consensusFor = $state<string | null>(null);
  let consensus = $state<Consensus | null>(null);

  async function refresh() {
    try {
      const page = await api.listReviewsPage({
        status: statusFilter || undefined,
      });
      reviews = page.items;
      reviewsTotal = page.total;
      error = null;
    } catch (err) {
      error = errorText(err, "p.reviews.could_not_load_reviews");
    }
  }

  async function delegate(event: SubmitEvent) {
    event.preventDefault();
    try {
      await api.invite({
        subject_kind: subjectKind,
        subject_pid: subjectPid.trim(),
        reviewer_ref: reviewerRef.trim(),
        reviewer_scope: reviewerScope,
        expertise: expertise.trim() || undefined,
        due_on: dueOn || undefined,
      });
      notice = t("p.reviews.invitation_sent");
      error = null;
      reviewerRef = "";
      expertise = "";
      await refresh();
    } catch (err) {
      notice = null;
      error = errorText(err, "p.reviews.could_not_delegate");
    }
  }

  async function respond(pid: string, response: "accept" | "decline") {
    await act(() => api.respond(pid, response));
  }

  async function submitVerdict(pid: string, recommendation: Recommendation) {
    const raw = window.prompt(t("p.reviews.score_0_100_leave_blank")) ?? "";
    const score = raw.trim() === "" ? undefined : Number(raw);
    if (score !== undefined && (Number.isNaN(score) || score < 0 || score > 100)) {
      error = t("p.reviews.score_must_be_between_0");
      return;
    }
    await act(() => api.submitVerdict(pid, { score, recommendation }));
  }

  async function withdraw(pid: string) {
    await act(() => api.withdraw(pid));
  }

  async function act(run: () => Promise<unknown>) {
    try {
      await run();
      error = null;
      await refresh();
      if (consensusFor) await showConsensus(consensusFor);
    } catch (err) {
      error = errorText(err, "p.reviews.that_action_was_refused");
    }
  }

  async function showConsensus(pid: string) {
    consensusFor = pid;
    try {
      consensus = await api.consensus(subjectKind, pid);
    } catch (err) {
      error = errorText(err, "p.reviews.could_not_load_consensus");
    }
  }

  onMount(refresh);

  const columns: DataGridColumn[] = $derived([
    { id: "subject", header: t("p.reviews.subject_2"), rowHeader: true, accessor: (r) => `${(r as unknown as Review).subject_kind} ${(r as unknown as Review).subject_pid.slice(0, 8)}…` },
    { id: "reviewer_ref", header: t("p.reviews.reviewer_2") },
    { id: "reviewer_scope", header: t("p.reviews.scope_2"), cell: scopeCell },
    { id: "expertise", header: t("p.reviews.expertise_2"), format: (v) => (v ? String(v) : "—") },
    { id: "status", header: t("p.reviews.status_2") },
    { id: "verdict", header: t("p.reviews.verdict"), accessor: (r) => { const x = r as unknown as Review; return x.status === "submitted" ? `${x.recommendation} · ${x.score ?? t("p.reviews.no_score")}` : "—"; } },
    { id: "pid", header: t("p.reviews.actions"), sortable: false, cell: reviewActions },
  ]);
</script>

{#snippet scopeCell({ row, formatted }: DataGridCellContext)}<span class="chip" class:external={row.reviewer_scope === "external"}>{formatted}</span>{/snippet}

{#snippet reviewActions({ row }: DataGridCellContext)}
  {@const review = row as unknown as Review}
  {#if review.status === "invited"}
    <button class="button small" type="button" onclick={() => respond(review.pid, "accept")}>{t("p.reviews.accept")}</button>
    <button class="button small" type="button" onclick={() => respond(review.pid, "decline")}>{t("p.reviews.decline")}</button>
  {:else if review.status === "accepted"}
    <button class="button small" type="button" onclick={() => submitVerdict(review.pid, "advance")}>{t("p.reviews.advance")}</button>
    <button class="button small" type="button" onclick={() => submitVerdict(review.pid, "hold")}>{t("p.reviews.hold")}</button>
    <button class="button small" type="button" onclick={() => submitVerdict(review.pid, "reject")}>{t("p.reviews.reject")}</button>
  {/if}
  {#if review.status === "invited" || review.status === "accepted"}
    <button class="button small" type="button" onclick={() => withdraw(review.pid)}>{t("p.reviews.withdraw")}</button>
  {/if}
{/snippet}


<svelte:head><title>{t("p.reviews.reviews_ppm")}</title></svelte:head>

<h1>{t("p.reviews.collaborative_review")}</h1>
<p class="small muted">
  {t("p.reviews.delegate_a_specific_item_to")}
</p>

{#if error}<p class="banner" role="alert">{error}</p>{/if}
{#if notice}<p class="small">{notice}</p>{/if}

<form class="grid" onsubmit={delegate}>
  <label class="small">
    {t("p.reviews.subject")}
    <select bind:value={subjectKind}>
      <option value="plan">{t("p.reviews.plan")}</option>
      <option value="proposal">{t("p.reviews.proposal")}</option>
      <option value="idea">{t("p.reviews.idea")}</option>
    </select>
  </label>
  <label class="small">
    {t("p.reviews.subject_pid")}
    <input bind:value={subjectPid} placeholder={t("p.reviews.uuid")} required />
  </label>
  <label class="small">
    {t("p.reviews.reviewer")}
    <input bind:value={reviewerRef} placeholder={t("p.reviews.person_uuid")} required />
  </label>
  <label class="small">
    {t("p.reviews.scope")}
    <select bind:value={reviewerScope}>
      <option value="internal">{t("p.reviews.internal")}</option>
      <option value="external">{t("p.reviews.external")}</option>
    </select>
  </label>
  <label class="small">
    {t("p.reviews.expertise")}
    <input bind:value={expertise} placeholder={t("p.reviews.e_g_clinical_safety")} />
  </label>
  <label class="small">{t("p.reviews.due")} <input type="date" bind:value={dueOn} /></label>
  <button class="button small primary" type="submit">{t("p.reviews.delegate")}</button>
</form>

<div class="row">
  <label class="small">
    {t("p.reviews.status")}
    <select bind:value={statusFilter} onchange={refresh}>
      <option value="">{t("p.reviews.all")}</option>
      <option value="invited">{t("p.reviews.invited")}</option>
      <option value="accepted">{t("p.reviews.accepted")}</option>
      <option value="submitted">{t("p.reviews.submitted")}</option>
      <option value="declined">{t("p.reviews.declined")}</option>
      <option value="expired">{t("p.reviews.expired")}</option>
    </select>
  </label>
  {#if subjectPid}
    <button class="button small" type="button" onclick={() => showConsensus(subjectPid.trim())}>
      {t("p.reviews.consensus_for_this_subject")}
    </button>
  {/if}
</div>

{#if consensus}
  <div class="panel">
    <strong>{t("p.reviews.consensus")}</strong>
    <p class="small">
      {t("p.reviews.submitted_outstanding_declined_exter", { submitted: consensus.submitted, outstanding: consensus.outstanding, declined: consensus.declined, external_submitted: consensus.external_submitted })}
    </p>
    <p class="small">
      {t("p.reviews.consensus_line", { mean: consensus.mean_score ?? "—", majority: consensus.majority ?? t("p.reviews.no_majority"), state: consensus.complete ? t("p.reviews.everyone_answered") : t("p.reviews.still_waiting") })}
    </p>
  </div>
{/if}

{#if reviews.length > 0}
  <p class="count">
    {reviews.length === reviewsTotal
      ? `${reviewsTotal}`
      : `${reviews.length} / ${reviewsTotal}`}
  </p>
{/if}
<DataTable label={t("p.reviews.subject_2")} columns={columns} rows={reviews} rowId={(r) => String(r.pid)} searchable />
{#if reviews.length === 0}
  <p class="small muted">{t("p.reviews.no_review_invitations_yet")}</p>
{/if}

<style>
  .grid {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
    align-items: flex-end;
    margin: 0.8rem 0;
  }
  .row { display: flex; gap: 0.5rem; flex-wrap: wrap; align-items: center; margin: 0.8rem 0; }
  .chip {
    display: inline-block;
    border: 1px solid currentColor;
    border-radius: 999px;
    padding: 0 0.5rem;
    font-size: 0.75rem;
  }
  .chip.external { color: #8a6d1d; font-weight: 700; }

  /* The row count: just the total when the page holds everything, and
     "shown / total" when it does not, so a first page of a long list
     cannot be mistaken for a short list (same convention as the
     organization service's front-end /organizations list route). */
  .count {
    margin: 0 0 0.3rem;
    opacity: 0.75;
    font-variant-numeric: tabular-nums;
    font-size: 0.85rem;
  }
</style>
