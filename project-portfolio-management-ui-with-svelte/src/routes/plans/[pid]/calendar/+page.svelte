<!--
  A plan's working calendar (`/plans/[pid]/calendar`, SC-2): which weekdays
  work and which dates are holidays. Float and finish dates on the critical path
  are counted in these business days, so a Friday task of two business days ends on Monday
  and a Monday holiday is skipped. With no calendar every day works, which is
  what every plan did before calendars existed.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { t, localeHref } from "#lib/i18n.svelte.js";
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
      error = errorText(err, "p.plans.pid.calendar.could_not_load_the_calendar");
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
      error = errorText(err, "p.plans.pid.calendar.could_not_save");
    }
  }
</script>

<svelte:head><title>{t("p.plans.pid.calendar.working_calendar_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref(`/plans/${pid}/critical-path`)}>{t("p.plans.pid.calendar.critical_path")}</a></p>
<h1>{t("p.plans.pid.calendar.working_calendar")}</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if loaded}
  <p class="small muted">
    {loaded.source === "plan" ? t("p.plans.pid.calendar.own_calendar") : t("p.plans.pid.calendar.no_calendar")}
    {t("p.plans.pid.calendar.each_person_can_set")}<a href={localeHref("/me/calendar")}>{t("p.plans.pid.calendar.my_calendar")}</a>{t("p.plans.pid.calendar.their_work_then_happens_only")}
  </p>
  <fieldset>
    <legend>{t("p.plans.pid.calendar.working_weekdays")}</legend>
    {#each WEEKDAY_NAMES as name, i (name)}
      <label class="day"><input type="checkbox" bind:checked={days[i]} /> {name}</label>
    {/each}
  </fieldset>
  {#if !valid}<p class="banner" role="alert">{t("p.plans.pid.calendar.at_least_one_weekday_must")}</p>{/if}

  <h2>{t("p.plans.pid.calendar.holidays")}</h2>
  <CalendarMonth bind:year bind:month {mask} {holidays} editable ontoggle={(d) => (holidays = toggleHoliday(holidays, d))} />
  {#if holidays.length > 0}
    <p class="small">{(holidays.length === 1 ? t("p.plans.pid.calendar.holiday_one", { length: holidays.length, join: holidays.join(", ") }) : t("p.plans.pid.calendar.holiday_other", { length: holidays.length, join: holidays.join(", ") }))}</p>
  {/if}
  <p><button class="button primary" type="button" onclick={() => void save()} disabled={!valid}>{t("p.plans.pid.calendar.save_calendar")}</button>
    {#if saved}<span role="status"> {t("p.plans.pid.calendar.saved")}</span>{/if}</p>
{:else if !error}
  <p>{t("p.plans.pid.calendar.loading")}</p>
{/if}

<style>
  .day {
    display: inline-flex;
    gap: 0.3rem;
    margin-inline-end: 1rem;
  }
</style>
