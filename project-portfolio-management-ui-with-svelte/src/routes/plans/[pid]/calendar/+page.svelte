<!--
  A plan's working calendar (`/plans/[pid]/calendar`, SC-2): which weekdays
  work and which dates are holidays. Float and finish dates on the critical path
  are counted in these working days, so a Friday task of two days ends on Monday
  and a Monday holiday is skipped. With no calendar every day works, which is
  what every plan did before calendars existed.
-->
<script lang="ts">
  import { localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { PpmClient, type WorkCalendarView } from "#lib/api/ppm.js";
  import CalendarMonth from "#lib/components/CalendarMonth.svelte";
  import { WEEKDAY_NAMES, maskToWeekdays, toggleHoliday, weekdaysToMask, weekdaysValid } from "#lib/cpm-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");

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
      loaded = await ppm.planCalendar(pid);
      days = maskToWeekdays(loaded.weekdays);
      holidays = loaded.holidays;
    } catch (err) {
      error = err instanceof Error ? err.message : "Could not load the calendar";
    }
  });

  const mask = $derived(weekdaysToMask(days));
  const valid = $derived(weekdaysValid(days));

  async function save() {
    error = null;
    saved = false;
    try {
      loaded = await ppm.setPlanCalendar(pid, { weekdays: mask, holidays });
      saved = true;
    } catch (err) {
      error = err instanceof Error ? err.message : "Could not save";
    }
  }
</script>

<svelte:head><title>Working calendar — PPM</title></svelte:head>

<p class="small"><a href={localeHref(`/plans/${pid}/critical-path`)}>← Critical path</a></p>
<h1>Working calendar</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if loaded}
  <p class="small muted">
    {loaded.source === "plan" ? "This plan has its own calendar." : "No calendar set: every day works."}
    Each person can also set their own (<a href={localeHref("/me/calendar")}>My calendar</a>); their work then happens only on days both calendars work.
  </p>
  <fieldset>
    <legend>Working weekdays</legend>
    {#each WEEKDAY_NAMES as name, i (name)}
      <label class="day"><input type="checkbox" bind:checked={days[i]} /> {name}</label>
    {/each}
  </fieldset>
  {#if !valid}<p class="banner" role="alert">At least one weekday must work.</p>{/if}

  <h2>Holidays</h2>
  <CalendarMonth bind:year bind:month {mask} {holidays} editable ontoggle={(d) => (holidays = toggleHoliday(holidays, d))} />
  {#if holidays.length > 0}
    <p class="small">{holidays.length} holiday{holidays.length === 1 ? "" : "s"}: {holidays.join(", ")}</p>
  {/if}
  <p><button type="button" onclick={() => void save()} disabled={!valid}>Save calendar</button>
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
