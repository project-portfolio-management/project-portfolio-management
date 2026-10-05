<!--
  My calendar (`/me/calendar`, SC-2): the weekly pattern a person works. A task
  assigned to them is scheduled on the plan's calendar **narrowed by this one**,
  and any leave recorded for them counts as a day off automatically. Setting a
  part-time pattern here is how a three-day week stretches a task on the
  critical path honestly, instead of someone promising dates they cannot keep.
-->
<script lang="ts">
  import { localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient, type AvailabilityRange, type WorkCalendarView } from "#lib/api/ppm.js";
  import CalendarMonth from "#lib/components/CalendarMonth.svelte";
  import { WEEKDAY_NAMES, maskToWeekdays, toggleHoliday, weekdaysToMask, weekdaysValid } from "#lib/cpm-view.js";

  const ppm = PpmClient.withFetch();
  let loaded = $state<WorkCalendarView | null>(null);
  let days = $state<boolean[]>(Array(7).fill(true));
  let holidays = $state<string[]>([]);
  let error = $state<string | null>(null);
  let saved = $state(false);
  let ranges = $state<AvailabilityRange[]>([]);
  let rFrom = $state("");
  let rTo = $state("");
  let rPercent = $state(50);
  let rNote = $state("");
  let rError = $state<string | null>(null);
  const today = new Date();
  let year = $state(today.getFullYear());
  let month = $state(today.getMonth() + 1);

  onMount(async () => {
    try {
      loaded = await ppm.meCalendar();
      days = maskToWeekdays(loaded.weekdays);
      holidays = loaded.holidays;
      ranges = await ppm.availability();
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

  async function addRange(event: SubmitEvent) {
    event.preventDefault();
    rError = null;
    try {
      // No person_ref: the service sets your own.
      await ppm.setAvailability({ from: rFrom, to: rTo, max_percent: rPercent, note: rNote || undefined });
      ranges = await ppm.availability();
      rFrom = rTo = rNote = "";
    } catch (err) {
      rError = err instanceof Error ? err.message : "Could not save";
    }
  }

  async function dropRange(pid: string) {
    rError = null;
    try {
      await ppm.removeAvailability(pid);
      ranges = await ppm.availability();
    } catch (err) {
      rError = err instanceof Error ? err.message : "Could not remove";
    }
  }
</script>

<svelte:head><title>My calendar — PPM</title></svelte:head>

<p class="small"><a href={localeHref("/me")}>← My work</a></p>
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
  <p><button class="button primary" type="button" onclick={() => void save()} disabled={!valid}>Save</button>
    {#if saved}<span role="status"> Saved.</span>{/if}</p>

  <h2>Part-time spells and leave</h2>
  <p class="small muted">
    The most of your time you can commit between two dates: 60 for a part-time spell, 0 for leave. Outside these dates it is 100.
    Capacity is read day by day, so a short spell is never averaged away.
  </p>
  {#if rError}<p class="banner" role="alert">{rError}</p>{/if}
  <ul>
    {#each ranges as r (r.pid)}
      <li>
        {r.from} to {r.to}: {r.max_percent}%{r.note ? ` (${r.note})` : ""}
        <button type="button" onclick={() => void dropRange(r.pid)}>Remove</button>
      </li>
    {/each}
    {#if ranges.length === 0}<li class="muted">None set.</li>{/if}
  </ul>
  <form onsubmit={addRange} class="row">
    <label>From <input type="date" bind:value={rFrom} required /></label>
    <label>To <input type="date" bind:value={rTo} required /></label>
    <label>Most I can commit (%) <input type="number" min="0" max="100" bind:value={rPercent} required /></label>
    <label>Note <input type="text" bind:value={rNote} /></label>
    <button type="submit">Add</button>
  </form>
{:else if !error}
  <p>Loading…</p>
{/if}

<style>
  .row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem 1rem;
    align-items: end;
  }
  .day {
    display: inline-flex;
    gap: 0.3rem;
    margin-inline-end: 1rem;
  }
</style>
