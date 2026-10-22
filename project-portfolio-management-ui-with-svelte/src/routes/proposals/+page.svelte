<!--
  Work-intake board (`/proposals`, PPM-1): the pipeline (draft →
  submitted → in_review → approved/rejected → promoted) with a
  new-proposal form, per-row pipeline actions, matcher-backed
  duplicate-demand checks, and promote-to-work-item.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { enumLabel } from "#lib/enum-labels.js";
  import { onMount } from "svelte";
  import { PpmClient, type DemandHit, type Proposal, money } from "#lib/api/ppm.js";
  import { COLLECTIONS, type Collection } from "#lib/api/types.js";

  const ppm = PpmClient.withFetch();
  let proposals = $state<Proposal[]>([]);
  let statusFilter = $state<string>("");
  let error = $state<string | null>(null);
  let duplicates = $state<Record<string, DemandHit[]>>({});

  let title = $state("");
  let summary = $state("");
  let kindTarget = $state<Collection>("projects");
  let requested = $state("");
  let currency = $state("GBP");

  async function refresh() {
    try {
      proposals = await ppm.listProposals(statusFilter || undefined);
      error = null;
    } catch (err) {
      error = errorText(err, "ppm.common.loadFailed");
    }
  }

  onMount(refresh);

  async function act(action: () => Promise<unknown>) {
    error = null;
    try {
      await action();
      await refresh();
    } catch (err) {
      error = errorText(err, "ppm.common.actionFailed");
    }
  }

  async function submitNew(event: SubmitEvent) {
    event.preventDefault();
    const minor = requested.trim() ? Math.round(Number(requested) * 100) : undefined;
    await act(() =>
      ppm.createProposal({
        title,
        summary: summary || undefined,
        kind_target: kindTarget,
        ...(minor !== undefined && Number.isFinite(minor)
          ? { requested_minor: minor, currency }
          : {}),
      }),
    );
    title = "";
    summary = "";
    requested = "";
  }

  async function showDuplicates(pid: string) {
    try {
      duplicates = { ...duplicates, [pid]: await ppm.proposalDuplicates(pid) };
    } catch (err) {
      error = errorText(err, "p.proposals.duplicate_check_failed");
    }
  }

  /** The pipeline actions legal from a status. */
  function actionsFor(status: Proposal["status"]): ("submit" | "review" | "approve" | "reject")[] {
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
    { id: "pid", header: t("p.proposals.actions"), sortable: false, cell: proposalActions },
  ]);
</script>

{#snippet titleCell({ row, formatted }: DataGridCellContext)}
  <strong>{formatted}</strong>
  {#if row.summary}<div class="small muted">{row.summary}</div>{/if}
{/snippet}

{#snippet statusChip({ formatted }: DataGridCellContext)}<span class="chip">{formatted}</span>{/snippet}

{#snippet proposalActions({ row }: DataGridCellContext)}
  {@const proposal = row as unknown as Proposal}
  {#each actionsFor(proposal.status) as action (action)}
    <button class="button small" onclick={() => act(() => ppm.proposalAction(proposal.pid, action))}>{action}</button>
  {/each}
  {#if proposal.status === "approved"}
    <button class="button small" onclick={() => act(() => ppm.promoteProposal(proposal.pid))}>{t("p.proposals.promote")}</button>
  {/if}
  {#if proposal.status === "promoted" && proposal.promoted_plan_pid}
    <a href={localeHref(`/plans/${proposal.promoted_plan_pid}`)}>{t("ppm.proposals.workItem")}</a>
  {:else}
    <button class="button small" onclick={() => showDuplicates(proposal.pid)}>{t("ppm.proposals.duplicates")}</button>
  {/if}
{/snippet}


<svelte:head><title>{t("p.proposals.proposals_ppm")}</title></svelte:head>

<h1>{t("ppm.proposals.title")}</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

<form class="row" onsubmit={submitNew}>
  <input placeholder={t("ppm.proposals.formTitle")} bind:value={title} required />
  <input placeholder={t("ppm.proposals.formSummary")} bind:value={summary} />
  <select bind:value={kindTarget} aria-label={t("p.proposals.target_collection")}>
    {#each COLLECTIONS as target (target)}<option value={target}>{enumLabel("collection", target)}</option>{/each}
  </select>
  <input placeholder={t("ppm.proposals.requested")} bind:value={requested} size="12" />
  <input bind:value={currency} size="4" aria-label={t("p.proposals.currency")} />
  <button class="button primary" type="submit">{t("ppm.proposals.open")}</button>
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
{#each proposals.filter((p) => duplicates[p.pid]) as proposal (proposal.pid)}
  <section class="small" aria-label={proposal.title}>
    <strong>{proposal.title}:</strong>
    {#if (duplicates[proposal.pid] ?? []).length === 0}
      {t("ppm.proposals.noDuplicates")}
    {:else}
      {#each duplicates[proposal.pid] ?? [] as hit (hit.pid)}
        <span class="chip">
          {hit.source === "plan" ? t("p.proposals.live") : t("p.proposals.proposal")}
          {hit.name} ({hit.score.toFixed(2)})
        </span>
      {/each}
    {/if}
  </section>
{/each}

<style>
  .row { display: flex; gap: 0.5rem; flex-wrap: wrap; align-items: center; margin: 0.8rem 0; }
  .chip {
    display: inline-block;
    border: 1px solid var(--border, #ccc);
    border-radius: 999px;
    padding: 0 0.55rem;
    margin: 0.1rem;
    font-size: 0.8rem;
  }
</style>
