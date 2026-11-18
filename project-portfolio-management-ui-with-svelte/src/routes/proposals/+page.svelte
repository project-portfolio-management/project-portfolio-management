<!--
  Work-intake board (`/proposals`, PPM-1): the pipeline (draft →
  submitted → in_review → approved/rejected → promoted) with a
  new-proposal form, per-row pipeline actions, matcher-backed
  duplicate-demand checks, and promote-to-work-item. A new proposal can
  be tagged as an external ask from the sponsor (EX-13), which must name
  what it displaces (a trade-off of 10 to 2000 characters) before it is
  approved; the trade-off is named or changed until the proposal is
  decided. Rules in `src/lib/tradeoff-view.ts`, said before anything is sent.

  NC-8 (NC-1 slice 12 part e): each proposal shows, on asking, the lessons of plans whose title is like its own
  (`GET /api/proposals/{pid}/lessons`), best match first, each with the match's score and confidence in words and the
  title's similarity; how they are matched is said first. Rules in `src/lib/lesson-view.ts`.
-->
<script lang="ts">
  import { rowOf } from "#lib/row-of.js";
  import { errorText } from "#lib/error-text.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { formatDecimal, formatPercent } from "#lib/locale-format.js";
  import { enumLabel } from "#lib/enum-labels.js";
  import { currencyCode, toMinor } from "#lib/money.js";
  import { onMount } from "svelte";
  import { PpmClient, type DemandHit, type Proposal, type ProposalLessonMatch, money } from "#lib/api/ppm.js";
  import { COLLECTIONS, type Collection } from "#lib/api/types.js";
  import {
    TRADEOFF_MAX,
    TRADEOFF_MIN,
    approveProblem,
    canNameTradeoff,
    hasNamedTradeoff,
    newAskFields,
    newProposalProblem,
    proposalRefusalKey,
    tradeoffBody,
  } from "#lib/tradeoff-view.js";
  import { CATEGORY_MARK, CONFIDENCE_KEY, isGate, lessonRefusalKey, nameScore, scorePercent } from "#lib/lesson-view.js";

  type Step = "submit" | "review" | "approve" | "reject";
  const STEP_LABELS = {
    submit: "p.proposals.act_submit",
    review: "p.proposals.act_review",
    approve: "p.proposals.act_approve",
    reject: "p.proposals.act_reject",
  } as const;
  const STEP_NAMED = {
    submit: "p.proposals.act_submit_named",
    review: "p.proposals.act_review_named",
    approve: "p.proposals.act_approve_named",
    reject: "p.proposals.act_reject_named",
  } as const;

  const ppm = PpmClient.withFetch();
  let proposals = $state<Proposal[]>([]);
  let statusFilter = $state<string>("");
  let error = $state<string | null>(null);
  let duplicates = $state<Record<string, DemandHit[]>>({});
  let lessonsOf = $state<Record<string, ProposalLessonMatch[]>>({});

  let title = $state("");
  let summary = $state("");
  let kindTarget = $state<Collection>("projects");
  let requested = $state("");
  let currency = $state("");
  let externalAsk = $state(false);
  let newTradeoff = $state("");

  // The trade-off form, for one external ask at a time.
  let editing = $state<Proposal | null>(null);
  let tradeoffText = $state("");
  let tradeoffError = $state<string | null>(null);
  let tradeoffNotice = $state("");

  async function refresh() {
    try {
      proposals = await ppm.listProposals(statusFilter || undefined);
      error = null;
      if (editing) editing = proposals.find((p) => p.pid === editing?.pid) ?? null;
    } catch (err) {
      error = errorText(err, "ppm.common.loadFailed");
    }
  }

  onMount(refresh);

  async function act(action: () => Promise<unknown>, refusal?: Parameters<typeof proposalRefusalKey>[0]) {
    error = null;
    try {
      await action();
      await refresh();
      return true;
    } catch (err) {
      const k = refusal ? proposalRefusalKey(refusal, err) : null;
      error = k ? t(k, { min: TRADEOFF_MIN, max: TRADEOFF_MAX }) : errorText(err, "ppm.common.actionFailed");
      return false;
    }
  }

  async function step(proposal: Proposal, action: Step) {
    if (action === "approve") {
      const problem = approveProblem(proposal);
      if (problem) {
        error = t(problem, { title: proposal.title, min: TRADEOFF_MIN });
        return;
      }
    }
    await act(() => ppm.proposalAction(proposal.pid, action), action);
  }

  async function submitNew(event: SubmitEvent) {
    event.preventDefault();
    error = null;
    const problem = newProposalProblem({ title, externalAsk, tradeoff: newTradeoff });
    if (problem) {
      error = t(problem, { min: TRADEOFF_MIN, max: TRADEOFF_MAX });
      return;
    }
    const code = currencyCode(currency);
    let minor: number | undefined;
    if (requested.trim()) {
      const m = code === null ? null : toMinor(requested, code);
      if (code === null || m === null || m < 0) {
        error = t("p.money.amount_or_currency");
        return;
      }
      minor = m;
    }
    const ok = await act(
      () =>
        ppm.createProposal({
          title: title.trim(),
          summary: summary || undefined,
          kind_target: kindTarget,
          ...(minor !== undefined && code !== null ? { requested_minor: minor, currency: code } : {}),
          ...newAskFields(externalAsk, newTradeoff),
        }),
      "create",
    );
    if (!ok) return;
    title = "";
    summary = "";
    requested = "";
    externalAsk = false;
    newTradeoff = "";
  }

  function openTradeoff(proposal: Proposal) {
    editing = proposal;
    tradeoffText = proposal.tradeoff ?? "";
    tradeoffError = null;
    tradeoffNotice = "";
  }

  async function saveTradeoff(event: SubmitEvent) {
    event.preventDefault();
    if (!editing) return;
    tradeoffError = null;
    tradeoffNotice = "";
    const r = tradeoffBody(editing, tradeoffText);
    if (r === null) {
      tradeoffNotice = t("p.proposals.tradeoff_unchanged");
      return;
    }
    if ("problem" in r) {
      tradeoffError = t(r.problem, { min: TRADEOFF_MIN, max: TRADEOFF_MAX });
      return;
    }
    const pid = editing.pid;
    try {
      await ppm.setProposalTradeoff(pid, r.body.tradeoff);
      tradeoffNotice = t("p.proposals.tradeoff_saved", { title: editing.title });
      await refresh();
    } catch (err) {
      const k = proposalRefusalKey("tradeoff", err);
      tradeoffError = k ? t(k, { min: TRADEOFF_MIN, max: TRADEOFF_MAX }) : errorText(err, "ppm.common.actionFailed");
    }
  }

  async function showDuplicates(pid: string) {
    try {
      duplicates = { ...duplicates, [pid]: await ppm.proposalDuplicates(pid) };
    } catch (err) {
      error = errorText(err, "p.proposals.duplicate_check_failed");
    }
  }

  async function showLessons(pid: string) {
    error = null;
    try {
      lessonsOf = { ...lessonsOf, [pid]: await ppm.proposalLessons(pid) };
    } catch (err) {
      const k = lessonRefusalKey("proposal", err);
      error = k ? t(k) : errorText(err, "p.lessons.could_not_read_similar");
    }
  }
  const occasionLabel = (o: string) => (isGate(o) ? enumLabel("pk_gate", o) : o === "closure" ? t("p.lessons.closure") : t("p.lessons.occasion_other"));

  /** The pipeline actions legal from a status. */
  function actionsFor(status: Proposal["status"]): Step[] {
    if (status === "draft") return ["submit"];
    if (status === "submitted") return ["review"];
    if (status === "in_review") return ["approve", "reject"];
    return [];
  }

  const columns: DataGridColumn[] = $derived([
    { id: "title", header: t("p.proposals.title"), rowHeader: true, cell: titleCell },
    { id: "kind_target", header: t("p.proposals.target") },
    { id: "requested_minor", header: t("p.proposals.requested"), align: "end", format: (v, r) => (v !== null && v !== undefined ? money(Number(v), String(r.currency)) : "—") },
    { id: "status", header: t("p.proposals.status"), format: (v) => enumLabel("proposal_status", String(v)), cell: statusChip },
    {
      id: "tradeoff",
      header: t("p.proposals.col_tradeoff"),
      sortable: false,
      accessor: (r) => {
        const p = rowOf<Proposal>(r);
        if (!p.external_ask) return t("p.proposals.not_ask");
        return hasNamedTradeoff(p) ? t("p.proposals.ask_with", { tradeoff: (p.tradeoff ?? "").trim() }) : t("p.proposals.ask_without");
      },
      cell: tradeoffCell,
    },
    { id: "pid", header: t("p.proposals.actions"), sortable: false, cell: proposalActions },
  ]);
</script>

{#snippet titleCell({ row, formatted }: DataGridCellContext)}
  <strong>{formatted}</strong>
  {#if row.summary}<div class="small muted">{row.summary}</div>{/if}
{/snippet}

{#snippet statusChip({ formatted }: DataGridCellContext)}<span class="chip">{formatted}</span>{/snippet}

{#snippet tradeoffCell({ row, formatted }: DataGridCellContext)}
  {@const p = rowOf<Proposal>(row)}
  {#if p.external_ask && !hasNamedTradeoff(p)}<span aria-hidden="true">⚠&nbsp;</span>{/if}<span class="tradeoff">{formatted}</span>
{/snippet}

{#snippet proposalActions({ row }: DataGridCellContext)}
  {@const proposal = rowOf<Proposal>(row)}
  {#each actionsFor(proposal.status) as action (action)}
    <button class="button small" aria-label={t(STEP_NAMED[action], { title: proposal.title })} onclick={() => step(proposal, action)}>{t(STEP_LABELS[action])}</button>
  {/each}
  {#if canNameTradeoff(proposal)}
    <button class="button small" aria-label={proposal.tradeoff ? t("p.proposals.tradeoff_change_named", { title: proposal.title }) : t("p.proposals.tradeoff_name_named", { title: proposal.title })} onclick={() => openTradeoff(proposal)}>{proposal.tradeoff ? t("p.proposals.tradeoff_change") : t("p.proposals.tradeoff_name")}</button>
  {/if}
  {#if proposal.status === "approved"}
    <button class="button small" onclick={() => act(() => ppm.promoteProposal(proposal.pid))}>{t("p.proposals.promote")}</button>
  {/if}
  {#if proposal.status === "promoted" && proposal.promoted_plan_pid}
    <a href={localeHref(`/plans/${proposal.promoted_plan_pid}`)}>{t("ppm.proposals.workItem")}</a>
  {:else}
    <button class="button small" onclick={() => showDuplicates(proposal.pid)}>{t("ppm.proposals.duplicates")}</button>
  {/if}
  <button class="button small" aria-label={t("p.lessons.similar_named", { title: proposal.title })} onclick={() => showLessons(proposal.pid)}>{t("p.lessons.similar")}</button>
{/snippet}


<svelte:head><title>{t("p.proposals.proposals_ppm")}</title></svelte:head>

<h1>{t("ppm.proposals.title")}</h1>
<p class="small"><a href={localeHref("/roadmap")}>{t("p.roadmap.see_ideas")}</a></p>
<p class="small muted">{t("p.proposals.ask_rule", { min: TRADEOFF_MIN })}</p>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

<form class="stack" novalidate onsubmit={submitNew} aria-label={t("p.proposals.new_form")}>
  <div class="row">
    <input placeholder={t("ppm.proposals.formTitle")} aria-label={t("ppm.proposals.formTitle")} bind:value={title} />
    <input placeholder={t("ppm.proposals.formSummary")} aria-label={t("ppm.proposals.formSummary")} bind:value={summary} />
    <select bind:value={kindTarget} aria-label={t("p.proposals.target_collection")}>
      {#each COLLECTIONS as target (target)}<option value={target}>{enumLabel("collection", target)}</option>{/each}
    </select>
    <input placeholder={t("ppm.proposals.requested")} aria-label={t("ppm.proposals.requested")} bind:value={requested} size="12" />
    <input bind:value={currency} size="4" maxlength="3" aria-label={t("p.proposals.currency")} placeholder={t("p.money.currency_hint")} />
  </div>
  <label class="check"><input type="checkbox" bind:checked={externalAsk} /> {t("p.proposals.f_external_ask")}</label>
  {#if externalAsk}
    <label class="wide">{t("p.proposals.f_new_tradeoff", { min: TRADEOFF_MIN, max: TRADEOFF_MAX })}<textarea bind:value={newTradeoff} rows="2"></textarea></label>
  {/if}
  <div class="row"><button class="button primary" type="submit">{t("ppm.proposals.open")}</button></div>
</form>

<p class="row small">
  {t("ppm.proposals.filter")}
  <select bind:value={statusFilter} onchange={refresh} aria-label={t("p.proposals.status_filter")}>
    <option value="">{t("ppm.proposals.allOpen")}</option>
    {#each ["draft", "submitted", "in_review", "approved", "rejected", "promoted"] as status (status)}
      <option value={status}>{enumLabel("proposal_status", status)}</option>
    {/each}
  </select>
</p>

<DataTable label={t("p.proposals.title")} columns={columns} rows={proposals} rowId={(r) => String(r.pid)} searchable />

{#if editing}
  <section class="tradeoff-form" aria-labelledby="tradeoff-h">
    <h2 id="tradeoff-h">{t("p.proposals.tradeoff_heading", { title: editing.title })}</h2>
    <p class="small muted">{t("p.proposals.tradeoff_says", { min: TRADEOFF_MIN, max: TRADEOFF_MAX })}</p>
    {#if tradeoffError}<p class="banner error" role="alert">{tradeoffError}</p>{/if}
    <p class="small" role="status">{tradeoffNotice}</p>
    <form class="stack" novalidate onsubmit={saveTradeoff}>
      <label class="wide">{t("p.proposals.f_tradeoff", { min: TRADEOFF_MIN, max: TRADEOFF_MAX })}<textarea bind:value={tradeoffText} rows="3"></textarea></label>
      <div class="row">
        <button class="button" type="submit">{t("p.proposals.tradeoff_save")}</button>
        <button class="button" type="button" onclick={() => (editing = null)}>{t("p.proposals.tradeoff_close")}</button>
      </div>
    </form>
  </section>
{/if}

{#each proposals.filter((p) => duplicates[p.pid]) as proposal (proposal.pid)}
  <section class="small" aria-label={proposal.title}>
    <strong>{proposal.title}:</strong>
    {#if (duplicates[proposal.pid] ?? []).length === 0}
      {t("ppm.proposals.noDuplicates")}
    {:else}
      {#each duplicates[proposal.pid] ?? [] as hit (hit.pid)}
        <span class="chip">
          {hit.source === "plan" ? t("p.proposals.live") : t("p.proposals.proposal")}
          {hit.name} ({formatDecimal(hit.score, 2)})
        </span>
      {/each}
    {/if}
  </section>
{/each}

{#each proposals.filter((p) => lessonsOf[p.pid]) as proposal (proposal.pid)}
  {@const matches = lessonsOf[proposal.pid] ?? []}
  <section class="lessons" aria-label={t("p.lessons.similar_heading", { title: proposal.title })}>
    <h2>{t("p.lessons.similar_heading", { title: proposal.title })}</h2>
    <p class="small muted">{t("p.lessons.similar_says")}</p>
    {#if matches.length === 0}
      <p class="small">{t("p.lessons.similar_none")}</p>
    {:else}
      <ol>
        {#each matches as hit (hit.plan_pid)}
          {@const named = nameScore(hit.match)}
          <li>
            <a href={localeHref(`/plans/${hit.plan_pid}`)}>{hit.plan_name}</a>:
            {t("p.lessons.match", { score: formatPercent(scorePercent(hit.match.score) / 100), confidence: t(CONFIDENCE_KEY[hit.match.confidence] ?? "p.lessons.confidence_low") })}{named === null ? "" : ` ${t("p.lessons.match_title", { score: formatPercent(named / 100) })}`}
            <ul>
              {#each hit.lessons as l (l.pid)}
                <li><span aria-hidden="true">{CATEGORY_MARK[l.category]?.symbol ?? "·"}</span> {CATEGORY_MARK[l.category] ? t(CATEGORY_MARK[l.category].key) : t("p.lessons.category_other")} · {occasionLabel(l.occasion)}: {l.summary}</li>
              {/each}
            </ul>
          </li>
        {/each}
      </ol>
    {/if}
  </section>
{/each}

<style>
  .lessons { margin-block: 1.5rem; }
  .row { display: flex; gap: 0.5rem; flex-wrap: wrap; align-items: center; margin: 0.8rem 0; }
  .check { display: flex; align-items: center; gap: 0.35rem; }
  .wide { display: grid; gap: 0.25rem; max-width: 40rem; }
  .tradeoff { white-space: pre-wrap; overflow-wrap: anywhere; }
  .tradeoff-form { margin-block: 1.5rem; }
  .chip {
    display: inline-block;
    border: 1px solid var(--border, #ccc);
    border-radius: 999px;
    padding: 0 0.55rem;
    margin: 0.1rem;
    font-size: 0.8rem;
  }
</style>
