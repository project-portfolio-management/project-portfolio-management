<!--
  Resource leveling (`/plans/[pid]/leveling`): advice only, then an explicit
  apply. The proposal shows what is over-allocated, each start date it would
  change (before and after, with the finish effect), what it could not fix and
  why. Nothing changes until you tick the moves you want, confirm, and apply; each
  applied move is audited. A critical task never moves unless you allow it, and
  then the finish cost is stated.
-->
<script lang="ts">
  import { rowOf } from "#lib/row-of.js";
  import { errorText } from "#lib/error-text.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t, localeHref, tn } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { PpmClient, type LevelingApplyResult, type LevelingProposal } from "#lib/api/ppm.js";
  import { finishEffectText, levelingReason, moveText, runText } from "#lib/scheduling-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");
  let proposal = $state<LevelingProposal | null>(null);
  let error = $state<string | null>(null);
  let result = $state<LevelingApplyResult | null>(null);
  let allowCritical = $state(false);
  let loadPercent = $state(100);
  let picked = $state<Record<string, boolean>>({});
  let confirmed = $state(false);
  let reason = $state("");
  let busy = $state(false);

  async function propose() {
    try {
      proposal = await ppm.levelingProposal(pid, { allowCritical, loadPercent });
      picked = Object.fromEntries(proposal.moves.map((m) => [m.pid, true]));
      confirmed = false;
      error = null;
    } catch (err) {
      error = errorText(err, "p.plans.pid.leveling.could_not_propose_a_leveling");
    }
  }
  onMount(() => void propose());

  const chosen = $derived((proposal?.moves ?? []).filter((m) => picked[m.pid]));

  async function apply(e: SubmitEvent) {
    e.preventDefault();
    if (!proposal || !confirmed || chosen.length === 0) return;
    busy = true;
    try {
      result = await ppm.applyLeveling(pid, {
        moves: chosen.map((m) => ({ task_pid: m.pid, start_date: m.to_start })),
        confirm: true,
        allow_critical: allowCritical,
        load_percent: loadPercent,
        reason: reason.trim() || undefined,
      });
      await propose();
    } catch (err) {
      error = errorText(err, "p.plans.pid.leveling.could_not_apply_the_moves");
    } finally {
      busy = false;
    }
  }

  type Move = LevelingProposal["moves"][number];
  const columns: DataGridColumn[] = $derived([
    { id: "apply", header: t("p.plans.pid.leveling.apply"), sortable: false, accessor: (r) => picked[rowOf<Move>(r).pid] ?? false, cell: applyCell },
    { id: "title", header: t("p.plans.pid.leveling.task"), rowHeader: true, cell: moveTaskCell, accessor: (r) => rowOf<Move>(r).title ?? rowOf<Move>(r).pid },
    { id: "person", header: t("p.plans.pid.leveling.person"), accessor: (r) => { const m = rowOf<Move>(r); return m.masked ? m.person : (m.person_ref ?? m.person); } },
    { id: "start", header: t("p.plans.pid.leveling.start"), accessor: (r) => `${rowOf<Move>(r).from_start} → ${rowOf<Move>(r).scheduled_start}` },
    { id: "finish", header: t("p.plans.pid.leveling.finish"), accessor: (r) => `${rowOf<Move>(r).from_finish} → ${rowOf<Move>(r).to_finish}` },
    { id: "delay_days", header: t("p.plans.pid.leveling.later_by"), align: "end", format: (v) => tn("p.plans.pid.leveling.business_day", Number(v), { delay_days: Number(v) }) },
    { id: "float", header: t("p.plans.pid.leveling.float"), accessor: (r) => `${rowOf<Move>(r).float_before} → ${rowOf<Move>(r).float_after}` },
  ]);
</script>

{#snippet applyCell({ row }: DataGridCellContext)}
  {@const m = rowOf<Move>(row)}
  <input type="checkbox" bind:checked={picked[m.pid]} aria-label={`Apply: ${moveText(m)}`} />
{/snippet}

{#snippet moveTaskCell({ row }: DataGridCellContext)}
  {@const m = rowOf<Move>(row)}
  {m.title ?? m.pid}{#if m.critical} <span aria-hidden="true">▲</span> <strong>{t("p.plans.pid.leveling.critical")}</strong>{/if}
{/snippet}


<svelte:head><title>{t("p.plans.pid.leveling.resource_leveling_ppm")}</title></svelte:head>

<h1>{t("p.plans.pid.leveling.balance_workloads_resource_leveling")}</h1>
<p class="lede">{t("p.plans.pid.leveling.suggests_moving_tasks_so_that")}</p>
<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plans.pid.leveling.plan")}</a> · <a href={localeHref(`/plans/${pid}/calendar`)}>{t("p.plans.pid.leveling.working_calendar")}</a></p>
<p class="banner" role="note">{t("p.plans.pid.leveling.this_is_advice_nothing_changes")}</p>

{#if error}<p class="banner" role="alert">{error}</p>{/if}
{#if result}
  <p class="banner" role="status">
    {tn("p.plans.pid.leveling.applied", result.requested, { ok: result.ok, requested: result.requested, refused: result.refused > 0 ? t("p.plans.pid.leveling.applied_refused", { refused: result.refused }) : "" })}
    {finishEffectText(result.finish_before, result.finish_after, result.finish_delay_days)}
  </p>
  {#each result.results.filter((r) => r.result === "refused") as r (r.pid)}
    <p class="small" role="alert">{t("p.plans.pid.leveling.refused", { note: r.note })}</p>
  {/each}
{/if}

<form class="filter" onsubmit={(e) => { e.preventDefault(); void propose(); }}>
  <label class="check"><input type="checkbox" bind:checked={allowCritical} /> {t("p.plans.pid.leveling.allow_critical_tasks_to_move")}</label>
  <label>{t("p.plans.pid.leveling.load_per_task_of_a")} <input type="number" min="1" max="100" step="1" bind:value={loadPercent} /></label>
  <button class="button small" type="submit">{t("p.plans.pid.leveling.propose_again")}</button>
</form>

{#if proposal}
  <p class="small muted">
    {t("p.plans.pid.leveling.proposal_summary", { calendar: proposal.calendar, load: proposal.load_percent, critical: proposal.allow_critical ? t("p.plans.pid.leveling.may") : t("p.plans.pid.leveling.may_not") })}
    {#if proposal.truncated} · <strong>{t("p.plans.pid.leveling.the_search_stopped_at_its")}</strong>{/if}
  </p>

  <h2>{t("p.plans.pid.leveling.over_allocated_now")}</h2>
  {#if proposal.over_allocated.length === 0}
    <p>{t("p.plans.pid.leveling.nobody_is_over_allocated_so")}</p>
  {:else}
    <ul>
      {#each proposal.over_allocated as r (r.person + r.from)}
        <li>{runText(r)} <span class="small muted">({r.tasks.map((t) => t.title ?? t.pid).join(", ")})</span></li>
      {/each}
    </ul>
  {/if}

  <h2>{t("p.plans.pid.leveling.proposed_start_dates")}</h2>
  {#if proposal.moves.length === 0}
    <p>{t("p.plans.pid.leveling.no_start_date_would_change")}</p>
  {:else}
    <p class="small" aria-live="polite">{finishEffectText(proposal.finish_before, proposal.finish_after, proposal.finish_delay_days)}</p>
    <form onsubmit={apply}>
        <DataTable label={t("p.plans.pid.leveling.each_start_date_the_proposal")} {columns} rows={proposal.moves} rowId={(r) => String(r.pid)} sortable={false} />
      {#if proposal.effects.length > 0}
        <details>
          <summary>{tn("p.plans.pid.leveling.other_task_would_be_pushed", proposal.effects.length, { length: proposal.effects.length })}</summary>
          <ul>
            {#each proposal.effects as e (e.pid)}
              <li>{t("p.plans.pid.leveling.start_finish_business_days", { pid: e.title ?? e.pid, v: e.start_days >= 0 ? "+" : "", start_days: e.start_days, v2: e.finish_days >= 0 ? "+" : "", finish_days: e.finish_days })}</li>
            {/each}
          </ul>
        </details>
      {/if}
      <fieldset>
        <legend>{t("p.plans.pid.leveling.apply_the_ticked_moves")}</legend>
        <label>{t("p.plans.pid.leveling.why_kept_with_each_audit")} <input type="text" bind:value={reason} maxlength="500" /></label>
        <label class="check"><input type="checkbox" bind:checked={confirmed} /> {t("p.plans.pid.leveling.i_have_read_the_proposal")}</label>
        <button class="button primary" type="submit" disabled={!confirmed || chosen.length === 0 || busy}>
          {tn("p.plans.pid.leveling.apply_move", chosen.length, { length: chosen.length })}
        </button>
      </fieldset>
    </form>
  {/if}

  {#if proposal.unresolved.length > 0}
    <h2>{t("p.plans.pid.leveling.could_not_be_fixed")}</h2>
    <ul>
      {#each proposal.unresolved as r (r.person + r.from)}
        <li>{runText(r)} {levelingReason(r.reason)}</li>
      {/each}
    </ul>
  {/if}
  <p class="small muted">{proposal.note}</p>
{:else if !error}
  <p class="muted">{t("p.plans.pid.leveling.loading")}</p>
{/if}

<style>
  .filter {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: end;
    margin: 0.75rem 0;
  }
  .filter label,
  fieldset label {
    display: grid;
    gap: 0.2rem;
    font-size: 0.85rem;
  }
  .check {
    display: flex !important;
    align-items: center;
    gap: 0.4rem;
  }
  fieldset {
    margin-top: 1rem;
    display: grid;
    gap: 0.6rem;
  }
</style>
