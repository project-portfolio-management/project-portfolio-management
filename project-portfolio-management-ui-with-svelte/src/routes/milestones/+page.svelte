<!--
  Milestones (`/milestones`): every milestone across the plans you can read,
  with its status against the calendar, whether it is achieved and how, the
  forecast from the schedule of the tasks it waits for, and how far it has
  slipped. Late first. A milestone with no forecast says so — it is never
  "on track" by default.

  English-first, like the other recent views.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { PpmClient, type MilestoneList, type MilestoneStatus } from "#lib/api/ppm.js";
  import MilestoneCard from "#lib/components/MilestoneCard.svelte";
  import { STATE_TEXT, STATUS_SYMBOL, statusParts } from "#lib/milestone-view.js";

  const ppm = PpmClient.withFetch();
  let list = $state<MilestoneList | null>(null);
  let error = $state<string | null>(null);
  let status = $state<MilestoneStatus | "">("");

  async function refresh() {
    try {
      list = await ppm.portfolioMilestones(status ? { status } : {});
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : "Could not load the milestones";
    }
  }
  onMount(refresh);
</script>

<svelte:head><title>Milestones — PPM</title></svelte:head>

<h1>Milestones</h1>
<p class="small muted">
  A milestone is a dated promise. Tie it to the tasks it waits for and it is achieved when they
  are done, with a forecast from the schedule; leave it unlinked and a person marks it done.
</p>

{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if list}
  <ul class="summary" aria-label="Milestones by status">
    {#each statusParts(list.summary.by_status) as part (part.status)}
      <li><span aria-hidden="true">{STATUS_SYMBOL[part.status]}</span> {part.label}: <strong>{part.count}</strong></li>
    {/each}
  </ul>
  {#if Object.keys(list.summary.by_schedule_state).length > 0}
    <p class="small muted">
      Forecast:
      {#each Object.entries(list.summary.by_schedule_state) as [state, count] (state)}
        {STATE_TEXT[state as keyof typeof STATE_TEXT]} {count}{" "}
      {/each}
    </p>
  {/if}
{/if}

<form class="filter" onsubmit={(e) => { e.preventDefault(); void refresh(); }}>
  <label>Show
    <select bind:value={status} onchange={refresh}>
      <option value="">All</option>
      <option value="late">Late</option>
      <option value="due_soon">Due soon</option>
      <option value="upcoming">Upcoming</option>
      <option value="achieved">Achieved</option>
    </select>
  </label>
</form>

{#if list}
  {#if list.milestones.length === 0}
    <p>No milestones{status ? " with that status" : ""} yet. Add them from a plan's governance page.</p>
  {:else}
    <p class="small muted" aria-live="polite">{list.milestones.length} milestone{list.milestones.length === 1 ? "" : "s"}, by due date</p>
    {#each list.milestones as item (item.pid)}
      <MilestoneCard {item} {ppm} onchange={refresh} onerror={(m) => (error = m)} />
    {/each}
  {/if}
{:else if !error}
  <p class="muted">Loading…</p>
{/if}

<style>
  .summary {
    display: flex;
    flex-wrap: wrap;
    gap: 1.25rem;
    list-style: none;
    padding: 0;
  }
  .filter {
    display: flex;
    gap: 0.75rem;
    align-items: end;
    margin: 0.75rem 0;
  }
  .filter label {
    display: grid;
    gap: 0.2rem;
    font-size: 0.85rem;
  }
</style>
