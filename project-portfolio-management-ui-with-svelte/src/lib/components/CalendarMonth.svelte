<!--
  A working calendar (SC-2) on the Lily CalendarView: which days work under the
  weekly pattern, which are holidays, and which are the person's recorded
  absences. Browse by week, four weeks or month. Every state has a symbol as
  well as a colour (– off day, ✕ holiday, A absent), and when `editable`, a
  business day is selectable (click, Enter or Space) to toggle a holiday.
-->
<script lang="ts">
  import CalendarView from "@lilydesignsystem/svelte-calendar";
  import type { CalendarView as Mode } from "@lilydesignsystem/svelte-calendar";
  import { onMount } from "svelte";
  import { t } from "#lib/i18n.svelte.js";
  import { formatDateRange } from "#lib/locale-format.js";
  import { lazyArray } from "#lib/lazy.js";
  import { WEEKDAY_NAMES, dayState } from "#lib/cpm-view.js";

  let {
    year = $bindable(),
    month = $bindable(),
    mask,
    holidays,
    absences = [],
    editable = false,
    ontoggle,
  }: {
    year: number;
    month: number;
    mask: string;
    holidays: string[];
    absences?: string[];
    editable?: boolean;
    ontoggle?: (date: string) => void;
  } = $props();

  const pad = (n: number, w = 2) => String(n).padStart(w, "0");
  let view = $state<Mode>("month");
  let anchorDate = $state(`${pad(year, 4)}-${pad(month)}-01`);
  let range = $state<[string, string] | null>(null);
  let today = $state<string | undefined>(undefined);

  onMount(() => {
    const d = new Date();
    today = `${pad(d.getFullYear(), 4)}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}`;
  });

  // The anchor is the source of truth once browsing starts; the bound year and month follow it.
  $effect(() => {
    const y = Number(anchorDate.slice(0, 4));
    const m = Number(anchorDate.slice(5, 7));
    if (y !== year) year = y;
    if (m !== month) month = m;
  });

  const MONTHS = lazyArray([
    "c.CalendarMonth.january", "c.CalendarMonth.february", "c.CalendarMonth.march", "c.CalendarMonth.april",
    "c.CalendarMonth.may", "c.CalendarMonth.june", "c.CalendarMonth.july", "c.CalendarMonth.august",
    "c.CalendarMonth.september", "c.CalendarMonth.october", "c.CalendarMonth.november", "c.CalendarMonth.december",
  ]);
  const heading = $derived(
    view === "month" || !range ? `${MONTHS[month - 1]} ${year}` : formatDateRange(range[0], range[1]),
  );

  const outside = (d: string) => view === "month" && d.slice(0, 7) !== anchorDate.slice(0, 7);
  const sundayFirst = (wd: number) => (wd + 6) % 7;
  const viewName = (v: Mode) =>
    v === "week" ? t("c.CalendarMonth.week") : v === "four-week" ? t("c.CalendarMonth.four_weeks") : t("c.CalendarMonth.month");

  // A day's name starts with the number the cell shows ("1 October 2026"), so what is seen is in what is said (WCAG 2.5.3).
  const spoken = (date: string) => `${Number(date.slice(8, 10))} ${MONTHS[Number(date.slice(5, 7)) - 1] ?? ""} ${date.slice(0, 4)}`;

  function describe(date: string, wd: number): string {
    const c = dayState(date, mask, holidays, absences);
    const state = c.holiday
      ? t("c.CalendarMonth.holiday_state")
      : c.absent
        ? t("c.CalendarMonth.absent_state")
        : c.offDay
          ? t("c.CalendarMonth.not_a_business_day")
          : t("c.CalendarMonth.business_day");
    return `${spoken(date)}, ${WEEKDAY_NAMES[sundayFirst(wd)] ?? ""}: ${state}`;
  }

  const labels = $derived({
    weekView: t("c.CalendarMonth.week"),
    fourWeekView: t("c.CalendarMonth.four_weeks"),
    monthView: t("c.CalendarMonth.month"),
    previous: (v: Mode) => t("c.CalendarMonth.previous", { view: viewName(v) }),
    next: (v: Mode) => t("c.CalendarMonth.next", { view: viewName(v) }),
    today: t("c.CalendarMonth.today"),
    weekday: (wd: number) => WEEKDAY_NAMES[sundayFirst(wd)] ?? "",
    weekdayAbbr: (wd: number) => (WEEKDAY_NAMES[sundayFirst(wd)] ?? "").slice(0, 3),
    day: (date: string, wd: number) => describe(date, wd),
  });

  /** Only a business day of the shown month can be marked; read-only has none. */
  const disabled = (d: string) => !editable || outside(d) || dayState(d, mask, holidays, absences).offDay;
</script>

<div class="cal" data-testid="calendar-month">
  <strong class="heading" aria-live="polite">{heading}</strong>
  <CalendarView
    label={t("c.CalendarMonth.working_calendar")}
    bind:view
    bind:anchorDate
    {today}
    {labels}
    isDisabled={disabled}
    onNavigate={(_v: Mode, start: string, end: string) => (range = [start, end])}
    onDayClick={(d: string) => ontoggle?.(d)}
  >
    {#snippet day({ date, outsidePeriod })}
      {@const c = dayState(date, mask, holidays, absences)}
      <span class="cell" class:out={outsidePeriod} class:off={c.offDay} class:holiday={c.holiday} class:absent={c.absent}>
        <span class="d">{Number(date.slice(8))}</span>
        <span class="m" aria-hidden="true">{c.holiday ? "✕" : c.absent ? "A" : c.offDay ? "–" : ""}</span>
      </span>
    {/snippet}
  </CalendarView>
  <p class="small muted">{t("c.CalendarMonth.legend")}{editable ? t("c.CalendarMonth.legend_editable") : ""}</p>
</div>

<style>
  .cal {
    display: inline-block;
    max-width: 100%;
  }
  .heading {
    display: block;
    margin-block-end: 0.35rem;
  }
  .cal :global(.calendar-view-toolbar) {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    align-items: center;
    gap: 0.5rem;
    margin-block-end: 0.35rem;
  }
  .cal :global(.calendar-view-nav),
  .cal :global(.calendar-view-switcher) {
    display: flex;
    gap: 0.25rem;
  }
  .cal :global(button[aria-pressed="true"]) {
    font-weight: 700;
  }
  .cal :global(table) {
    border-collapse: collapse;
  }
  .cal :global(th),
  .cal :global(td) {
    border: 1px solid var(--border, #ccc);
    width: 3.1rem;
    height: 2.6rem;
    padding: 0;
    text-align: center;
    vertical-align: middle;
    font-size: 0.85em;
  }
  .cal :global(td[tabindex="0"]:focus-visible) {
    outline: 3px solid var(--mxi-color-focus, #1d4ed8);
    outline-offset: -3px;
  }
  .cal :global(td:not([aria-disabled])) {
    cursor: pointer;
  }
  .cell {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
  }
  .cell.out {
    opacity: 0.4;
  }
  .cell.off {
    background: var(--surface-2, #eee);
  }
  .cell.holiday {
    background: color-mix(in srgb, #a4262c 18%, transparent);
    font-weight: 700;
  }
  .cell.absent {
    background: color-mix(in srgb, #b45309 18%, transparent);
  }
  .m {
    font-size: 0.75em;
    line-height: 1;
    min-height: 0.8em;
  }
</style>
