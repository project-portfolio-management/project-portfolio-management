<!--
  My calendar (`/me/calendar`, SC-2): the weekly pattern a person works. A task
  assigned to them is scheduled on the plan's calendar **narrowed by this one**,
  and any leave recorded for them counts as a day off automatically. Setting a
  part-time pattern here is how a three-day week stretches a task on the
  critical path honestly, instead of someone promising dates they cannot keep.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { t, localeHref } from "#lib/i18n.svelte.js";
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
      error = errorText(err, "p.me.calendar.could_not_load_your_calendar");
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
      error = errorText(err, "p.me.calendar.could_not_save");
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
      rError = errorText(err, "p.me.calendar.could_not_save");
    }
  }

  async function dropRange(pid: string) {
    rError = null;
    try {
      await ppm.removeAvailability(pid);
      ranges = await ppm.availability();
    } catch (err) {
      rError = errorText(err, "p.me.calendar.could_not_remove");
    }
  }
</script>

<svelte:head><title>{t("p.me.calendar.my_calendar_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref("/me")}>{t("p.me.calendar.my_work")}</a></p>
<h1>{t("p.me.calendar.my_calendar")}</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if loaded}
  <p class="small muted">
    {loaded.source === "person" ? t("p.me.calendar.own_set") : t("p.me.calendar.plan_applies")}
    {t("p.me.calendar.leave_counts")}
  </p>
  <fieldset>
    <legend>{t("p.me.calendar.the_days_i_work")}</legend>
    {#each WEEKDAY_NAMES as name, i (name)}
      <label class="day"><input type="checkbox" bind:checked={days[i]} /> {name}</label>
    {/each}
  </fieldset>
  {#if !valid}<p class="banner" role="alert">{t("p.me.calendar.at_least_one_weekday_must")}</p>{/if}
  <h2>{t("p.me.calendar.my_days_off")}</h2>
  <CalendarMonth bind:year bind:month {mask} {holidays} editable ontoggle={(d) => (holidays = toggleHoliday(holidays, d))} />
  <p><button class="button primary" type="button" onclick={() => void save()} disabled={!valid}>{t("p.me.calendar.save")}</button>
    {#if saved}<span role="status"> {t("p.me.calendar.saved")}</span>{/if}</p>

  <h2>{t("p.me.calendar.part_time_spells_and_leave")}</h2>
  <p class="small muted">
    {t("p.me.calendar.the_most_of_your_time")}
  </p>
  {#if rError}<p class="banner" role="alert">{rError}</p>{/if}
  <ul>
    {#each ranges as r (r.pid)}
      <li>
        {t("p.me.calendar.to", { from: r.from, to: r.to, max_percent: r.max_percent, v: r.note ? ` (${r.note})` : "" })}
        <button type="button" onclick={() => void dropRange(r.pid)}>{t("p.me.calendar.remove")}</button>
      </li>
    {/each}
    {#if ranges.length === 0}<li class="muted">{t("p.me.calendar.none_set")}</li>{/if}
  </ul>
  <form onsubmit={addRange} class="row">
    <label>{t("p.me.calendar.from")} <input type="date" bind:value={rFrom} required /></label>
    <label>{t("p.me.calendar.to_2")} <input type="date" bind:value={rTo} required /></label>
    <label>{t("p.me.calendar.most_i_can_commit")} <input type="number" min="0" max="100" bind:value={rPercent} required /></label>
    <label>{t("p.me.calendar.note")} <input type="text" bind:value={rNote} /></label>
    <button type="submit">{t("p.me.calendar.add")}</button>
  </form>
{:else if !error}
  <p>{t("p.me.calendar.loading")}</p>
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
