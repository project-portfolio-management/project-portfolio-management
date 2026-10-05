<!--
  Resource leveling (`/plans/[pid]/leveling`): advice only, then an explicit
  apply. The proposal shows what is over-allocated, each start date it would
  change (before and after, with the finish effect), what it could not fix and
  why. Nothing changes until you tick the moves you want, confirm, and apply; each
  applied move is audited. A critical task never moves unless you allow it, and
  then the finish cost is stated.
-->
<script lang="ts">
  import { localeHref } from "#lib/i18n.svelte.js";
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
      error = err instanceof Error ? err.message : "Could not propose a leveling";
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
      error = err instanceof Error ? err.message : "Could not apply the moves";
    } finally {
      busy = false;
    }
  }
</script>

<svelte:head><title>Resource leveling — PPM</title></svelte:head>

<h1>Balance workloads (resource leveling)</h1>
<p class="lede">Suggests moving tasks so that nobody is booked for more than they can do. Nothing changes until you confirm.</p>
<p class="small"><a href={localeHref(`/plans/${pid}`)}>← Plan</a> · <a href={localeHref(`/plans/${pid}/calendar`)}>Working calendar</a></p>
<p class="banner" role="note">This is advice. Nothing changes until you choose moves below and apply them.</p>

{#if error}<p class="banner" role="alert">{error}</p>{/if}
{#if result}
  <p class="banner" role="status">
    Applied {result.ok} of {result.requested} move{result.requested === 1 ? "" : "s"}{result.refused > 0 ? `; ${result.refused} refused` : ""}.
    {finishEffectText(result.finish_before, result.finish_after, result.finish_delay_days)}
  </p>
  {#each result.results.filter((r) => r.result === "refused") as r (r.pid)}
    <p class="small" role="alert">Refused: {r.note}</p>
  {/each}
{/if}

<form class="filter" onsubmit={(e) => { e.preventDefault(); void propose(); }}>
  <label class="check"><input type="checkbox" bind:checked={allowCritical} /> Allow critical tasks to move (this can delay the finish)</label>
  <label>Load per task (% of a person's day) <input type="number" min="1" max="100" step="1" bind:value={loadPercent} /></label>
  <button class="button small" type="submit">Propose again</button>
</form>

{#if proposal}
  <p class="small muted">
    Calendar: {proposal.calendar} · load {proposal.load_percent}% per task · critical tasks {proposal.allow_critical ? "may" : "may not"} move
    {#if proposal.truncated} · <strong>the search stopped at its limit</strong>{/if}
  </p>

  <h2>Over-allocated now</h2>
  {#if proposal.over_allocated.length === 0}
    <p>Nobody is over-allocated, so there is nothing to level.</p>
  {:else}
    <ul>
      {#each proposal.over_allocated as r (r.person + r.from)}
        <li>{runText(r)} <span class="small muted">({r.tasks.map((t) => t.title ?? t.pid).join(", ")})</span></li>
      {/each}
    </ul>
  {/if}

  <h2>Proposed start dates</h2>
  {#if proposal.moves.length === 0}
    <p>No start date would change.</p>
  {:else}
    <p class="small" aria-live="polite">{finishEffectText(proposal.finish_before, proposal.finish_after, proposal.finish_delay_days)}</p>
    <form onsubmit={apply}>
      <table>
        <caption class="sr-only">Each start date the proposal would change</caption>
        <thead>
          <tr>
            <th scope="col">Apply</th>
            <th scope="col">Task</th>
            <th scope="col">Person</th>
            <th scope="col">Start</th>
            <th scope="col">Finish</th>
            <th scope="col">Later by</th>
            <th scope="col">Float</th>
          </tr>
        </thead>
        <tbody>
          {#each proposal.moves as m (m.pid)}
            <tr class:critical={m.critical}>
              <td><input type="checkbox" bind:checked={picked[m.pid]} aria-label={`Apply: ${moveText(m)}`} /></td>
              <th scope="row">{m.title ?? m.pid}{#if m.critical} <span aria-hidden="true">▲</span> <strong>critical</strong>{/if}</th>
              <td>{m.masked ? m.person : (m.person_ref ?? m.person)}</td>
              <td>{m.from_start} → {m.scheduled_start}</td>
              <td>{m.from_finish} → {m.to_finish}</td>
              <td>{m.delay_days} working day{m.delay_days === 1 ? "" : "s"}</td>
              <td>{m.float_before} → {m.float_after}</td>
            </tr>
          {/each}
        </tbody>
      </table>
      {#if proposal.effects.length > 0}
        <details>
          <summary>{proposal.effects.length} other task{proposal.effects.length === 1 ? "" : "s"} would be pushed as a result</summary>
          <ul>
            {#each proposal.effects as e (e.pid)}
              <li>{e.title ?? e.pid}: start {e.start_days >= 0 ? "+" : ""}{e.start_days}, finish {e.finish_days >= 0 ? "+" : ""}{e.finish_days} working days</li>
            {/each}
          </ul>
        </details>
      {/if}
      <fieldset>
        <legend>Apply the ticked moves</legend>
        <label>Why (kept with each audit entry) <input type="text" bind:value={reason} maxlength="500" /></label>
        <label class="check"><input type="checkbox" bind:checked={confirmed} /> I have read the proposal and want these start dates changed</label>
        <button class="button primary" type="submit" disabled={!confirmed || chosen.length === 0 || busy}>
          Apply {chosen.length} move{chosen.length === 1 ? "" : "s"}
        </button>
      </fieldset>
    </form>
  {/if}

  {#if proposal.unresolved.length > 0}
    <h2>Could not be fixed</h2>
    <ul>
      {#each proposal.unresolved as r (r.person + r.from)}
        <li>{runText(r)} {levelingReason(r.reason)}</li>
      {/each}
    </ul>
  {/if}
  <p class="small muted">{proposal.note}</p>
{:else if !error}
  <p class="muted">Loading…</p>
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
