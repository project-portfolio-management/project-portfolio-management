<!--
  Milestones (`/milestones`): every milestone across the plans you can read,
  with its status against the calendar, whether it is achieved and how, the
  forecast from the schedule of the tasks it waits for, and how far it has
  slipped. Late first. A milestone with no forecast says so — it is never
  "on track" by default.

  English-first, like the other recent views.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { t } from "#lib/i18n.svelte.js";
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
      error = errorText(err, "p.milestones.could_not_load_the_milestones");
    }
  }
  onMount(refresh);
</script>

<svelte:head><title>{t("p.milestones.milestones_ppm")}</title></svelte:head>

<h1>{t("p.milestones.milestones")}</h1>
<p class="small muted">
  {t("p.milestones.a_milestone_is_a_dated")}
</p>

{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if list}
  <ul class="summary" aria-label={t("p.milestones.milestones_by_status")}>
    {#each statusParts(list.summary.by_status) as part (part.status)}
      <li><span aria-hidden="true">{STATUS_SYMBOL[part.status]}</span> {part.label}: <strong>{part.count}</strong></li>
    {/each}
  </ul>
  {#if Object.keys(list.summary.by_schedule_state).length > 0}
    <p class="small muted">
      {t("p.milestones.forecast")}
      {#each Object.entries(list.summary.by_schedule_state) as [state, count] (state)}
        <!-- the literal space is kept: Svelte trims whitespace before {/each} -->
        <!-- eslint-disable-next-line svelte/no-useless-mustaches -->
        {STATE_TEXT[state as keyof typeof STATE_TEXT]} {count}{" "}
      {/each}
    </p>
  {/if}
{/if}

<form class="filter" onsubmit={(e) => { e.preventDefault(); void refresh(); }}>
  <label>{t("p.milestones.show")}
    <select bind:value={status} onchange={refresh}>
      <option value="">{t("p.milestones.all")}</option>
      <option value="late">{t("p.milestones.late")}</option>
      <option value="due_soon">{t("p.milestones.due_soon")}</option>
      <option value="upcoming">{t("p.milestones.upcoming")}</option>
      <option value="achieved">{t("p.milestones.achieved")}</option>
    </select>
  </label>
</form>

{#if list}
  {#if list.milestones.length === 0}
    <p>{status ? t("p.milestones.none_with_status") : t("p.milestones.none")}</p>
  {:else}
    <p class="small muted" aria-live="polite">{(list.milestones.length === 1 ? t("p.milestones.milestone_by_due_date_one", { length: list.milestones.length }) : t("p.milestones.milestone_by_due_date_other", { length: list.milestones.length }))}</p>
    {#each list.milestones as item (item.pid)}
      <MilestoneCard {item} {ppm} onchange={refresh} onerror={(m) => (error = m)} />
    {/each}
  {/if}
{:else if !error}
  <p class="muted">{t("p.milestones.loading")}</p>
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
