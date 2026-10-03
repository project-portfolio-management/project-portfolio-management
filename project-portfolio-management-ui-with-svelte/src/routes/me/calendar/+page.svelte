<!--
  My calendar (`/me/calendar`, SC-2): the weekly pattern a person works. A task
  assigned to them is scheduled on the plan's calendar **narrowed by this one**,
  and any leave recorded for them counts as a day off automatically. Setting a
  part-time pattern here is how a three-day week stretches a task on the
  critical path honestly, instead of someone promising dates they cannot keep.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { PpmClient, type WorkCalendarView } from "#lib/api/ppm.js";
  import CalendarMonth from "#lib/components/CalendarMonth.svelte";
  import { WEEKDAY_NAMES, maskToWeekdays, toggleHoliday, weekdaysToMask, weekdaysValid } from "#lib/cpm-view.js";

  const ppm = PpmClient.withFetch();
  let loaded = $state<WorkCalendarView | null>(null);
  let days = $state<boolean[]>(Array(7).fill(true));
  let holidays = $state<string[]>([]);
  let error = $state<string | null>(null);
  let saved = $state(false);
  const today = new Date();
  let year = $state(today.getFullYear());
  let month = $state(today.getMonth() + 1);

  onMount(async () => {
    try {
      loaded = await ppm.meCalendar();
      days = maskToWeekdays(loaded.weekdays);
      holidays = loaded.holidays;
    } catch (err) {
      error = err instanceof Error ? err.message : "Could not load your calendar";
    }
  });

  const mask = $derived(weekdaysToMask(days));
  const valid = $derived(weekdaysValid(days));

  async function save() {
    error = null;
    saved = false;
    try {
      loaded = await ppm.setMeCalendar({ weekdays: mask, holidays });
      saved = true;
    } catch (err) {
      error = err instanceof Error ? err.message : "Could not save";
    }
  }
</script>

<svelte:head><title>My calendar — PPM</title></svelte:head>

<p class="small"><a href="/me">← My work</a></p>
<h1>My calendar</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if loaded}
  <p class="small muted">
    {loaded.source === "person" ? "Your own calendar is set." : "No personal calendar: the plan's calendar applies to you."}
    Leave recorded for you already counts as days off.
  </p>
  <fieldset>
    <legend>The days I work</legend>
    {#each WEEKDAY_NAMES as name, i (name)}
      <label class="day"><input type="checkbox" bind:checked={days[i]} /> {name}</label>
    {/each}
  </fieldset>
  {#if !valid}<p class="banner" role="alert">At least one weekday must work.</p>{/if}
  <h2>My days off</h2>
  <CalendarMonth bind:year bind:month {mask} {holidays} editable ontoggle={(d) => (holidays = toggleHoliday(holidays, d))} />
  <p><button type="button" onclick={() => void save()} disabled={!valid}>Save</button>
    {#if saved}<span role="status"> Saved.</span>{/if}</p>
{:else if !error}
  <p>Loading…</p>
{/if}

<style>
  .day {
    display: inline-flex;
    gap: 0.3rem;
    margin-inline-end: 1rem;
  }
</style>
