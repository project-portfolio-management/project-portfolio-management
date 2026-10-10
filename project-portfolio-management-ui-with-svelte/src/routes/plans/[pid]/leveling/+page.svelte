<!--
  Resource leveling (`/plans/[pid]/leveling`): advice only, then an explicit
  apply. The proposal shows what is over-allocated, each start date it would
  change (before and after, with the finish effect), what it could not fix and
  why. Nothing changes until you tick the moves you want, confirm, and apply; each
  applied move is audited. A critical task never moves unless you allow it, and
  then the finish cost is stated.
-->
<script lang="ts">
  import { t, localeHref } from "#lib/i18n.svelte.js";
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
      error = err instanceof Error ? err.message : t("p.plans.pid.leveling.could_not_propose_a_leveling");
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
      error = err instanceof Error ? err.message : t("p.plans.pid.leveling.could_not_apply_the_moves");
    } finally {
      busy = false;
    }
  }
</script>

<svelte:head><title>{t("p.plans.pid.leveling.resource_leveling_ppm")}</title></svelte:head>

<h1>{t("p.plans.pid.leveling.balance_workloads_resource_leveling")}</h1>
<p class="lede">{t("p.plans.pid.leveling.suggests_moving_tasks_so_that")}</p>
<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plans.pid.leveling.plan")}</a> · <a href={localeHref(`/plans/${pid}/calendar`)}>{t("p.plans.pid.leveling.working_calendar")}</a></p>
<p class="banner" role="note">{t("p.plans.pid.leveling.this_is_advice_nothing_changes")}</p>

{#if error}<p class="banner" role="alert">{error}</p>{/if}
{#if result}
  <p class="banner" role="status">
    {t(result.requested === 1 ? "p.plans.pid.leveling.applied_one" : "p.plans.pid.leveling.applied_other", { ok: result.ok, requested: result.requested, refused: result.refused > 0 ? t("p.plans.pid.leveling.applied_refused", { refused: result.refused }) : "" })}
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
      <table>
        <caption class="sr-only">{t("p.plans.pid.leveling.each_start_date_the_proposal")}</caption>
        <thead>
          <tr>
            <th scope="col">{t("p.plans.pid.leveling.apply")}</th>
            <th scope="col">{t("p.plans.pid.leveling.task")}</th>
            <th scope="col">{t("p.plans.pid.leveling.person")}</th>
            <th scope="col">{t("p.plans.pid.leveling.start")}</th>
            <th scope="col">{t("p.plans.pid.leveling.finish")}</th>
            <th scope="col">{t("p.plans.pid.leveling.later_by")}</th>
            <th scope="col">{t("p.plans.pid.leveling.float")}</th>
          </tr>
        </thead>
        <tbody>
          {#each proposal.moves as m (m.pid)}
            <tr class:critical={m.critical}>
              <td><input type="checkbox" bind:checked={picked[m.pid]} aria-label={`Apply: ${moveText(m)}`} /></td>
              <th scope="row">{m.title ?? m.pid}{#if m.critical} <span aria-hidden="true">▲</span> <strong>{t("p.plans.pid.leveling.critical")}</strong>{/if}</th>
              <td>{m.masked ? m.person : (m.person_ref ?? m.person)}</td>
              <td>{m.from_start} → {m.scheduled_start}</td>
              <td>{m.from_finish} → {m.to_finish}</td>
              <td>{(m.delay_days === 1 ? t("p.plans.pid.leveling.business_day_one", { delay_days: m.delay_days }) : t("p.plans.pid.leveling.business_day_other", { delay_days: m.delay_days }))}</td>
              <td>{m.float_before} → {m.float_after}</td>
            </tr>
          {/each}
        </tbody>
      </table>
      {#if proposal.effects.length > 0}
        <details>
          <summary>{(proposal.effects.length === 1 ? t("p.plans.pid.leveling.other_task_would_be_pushed_one", { length: proposal.effects.length }) : t("p.plans.pid.leveling.other_task_would_be_pushed_other", { length: proposal.effects.length }))}</summary>
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
          {(chosen.length === 1 ? t("p.plans.pid.leveling.apply_move_one", { length: chosen.length }) : t("p.plans.pid.leveling.apply_move_other", { length: chosen.length }))}
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
  table {
    width: 100%;
    border-collapse: collapse;
  }
  th,
  td {
    text-align: left;
    padding: 0.4rem 0.5rem;
    border-bottom: 1px solid var(--mxi-color-border, #ddd);
  }
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
