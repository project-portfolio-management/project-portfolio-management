<!--
  Delivery calendar (`/calendar`) — the estate's milestones on Lily's CalendarView, filterable by kind
  (milestone / demo / release / checkpoint). Browse by day, week, four weeks, month or as an agenda list; each
  milestone carries its kind, work item and status as typed fields, a done milestone is marked with a tick and
  says so in words, and the shown milestones export as CSV or Excel. Read-only: milestones are created and
  completed on their work item. The list below the calendar is the same data as plain text.
-->
<script lang="ts">
  import CalendarView from "@lilydesignsystem/svelte-calendar";
  import type { CalendarEvent, CalendarEventField, CalendarView as Mode } from "@lilydesignsystem/svelte-calendar";
  import { onMount } from "svelte";
  import { PpmClient, type MilestoneCalendar } from "#lib/api/ppm.js";
  import { t } from "#lib/i18n.svelte.js";
  import { exportLabels, searchAnnouncement } from "#lib/table-labels.js";
  import { lazyArray } from "#lib/lazy.js";
  import { WEEKDAY_NAMES } from "#lib/cpm-view.js";

  const ppm = PpmClient.withFetch();
  let calendar = $state<MilestoneCalendar | null>(null);
  let kind = $state("");
  let error = $state<string | null>(null);

  const pad = (n: number) => String(n).padStart(2, "0");
  let view = $state<Mode>("month");
  let anchorDate = $state("2000-01-01");
  let search = $state("");
  let today = $state<string | undefined>(undefined);
  onMount(() => {
    const d = new Date();
    today = anchorDate = `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}`;
  });

  // The effect re-runs when `kind` changes, so the filter needs no load of its own (CQ-17); a
  // late answer for an older filter is dropped rather than shown over a newer one.
  let latest = 0;
  async function load(filter: string) {
    const mine = ++latest;
    try {
      const result = await ppm.milestoneCalendar(filter || undefined);
      if (mine !== latest) return;
      calendar = result;
      error = null;
    } catch (err) {
      if (mine === latest) error = err instanceof Error ? err.message : t("ppm.common.loadFailed");
    }
  }
  $effect(() => {
    void load(kind);
  });

  const events = $derived<CalendarEvent[]>(
    (calendar?.milestones ?? []).map((m) => ({
      id: m.pid,
      date: m.due,
      title: `${m.kind}: ${m.name}`,
      done: m.done,
      values: { kind: m.kind, item: m.item?.name ?? "—", status: m.done ? t("p.calendar.status_done") : t("p.calendar.status_open") },
    })),
  );
  const eventFields = $derived<CalendarEventField[]>([
    { id: "kind", header: t("p.calendar.kind"), type: "text" },
    { id: "item", header: t("p.calendar.item"), type: "text" },
    { id: "status", header: t("p.calendar.status"), type: "text" },
  ]);

  const MONTHS = lazyArray([
    "c.CalendarMonth.january", "c.CalendarMonth.february", "c.CalendarMonth.march", "c.CalendarMonth.april",
    "c.CalendarMonth.may", "c.CalendarMonth.june", "c.CalendarMonth.july", "c.CalendarMonth.august",
    "c.CalendarMonth.september", "c.CalendarMonth.october", "c.CalendarMonth.november", "c.CalendarMonth.december",
  ]);
  const sundayFirst = (wd: number) => (wd + 6) % 7;
  const viewName = (v: Mode) =>
    t(({ day: "p.calendar.day", week: "c.CalendarMonth.week", "four-week": "c.CalendarMonth.four_weeks", month: "c.CalendarMonth.month", agenda: "p.calendar.agenda" } as const)[v]);

  const MAX_SHOWN = 3;
  /** The text the day cell shows: its number, then up to three milestones, then "+N more". Keep in step with the `day` snippet below. */
  function shownOn(date: string): string {
    const here = events.filter((e) => e.date === date);
    const shown = here.slice(0, MAX_SHOWN).map((e) => `${e.title}${e["done"] ? ` ✓ ${t("p.calendar.status_done")}` : ""}`);
    const more = here.length > MAX_SHOWN ? [t("p.calendar.more", { n: here.length - MAX_SHOWN })] : [];
    return [String(Number(date.slice(8, 10))), ...shown, ...more].join(" ");
  }

  const labels = $derived({
    dayView: t("p.calendar.day"),
    weekView: t("c.CalendarMonth.week"),
    fourWeekView: t("c.CalendarMonth.four_weeks"),
    monthView: t("c.CalendarMonth.month"),
    agendaView: t("p.calendar.agenda"),
    agendaEmpty: t("p.calendar.no_milestones_match"),
    previous: (v: Mode) => t("c.CalendarMonth.previous", { view: viewName(v) }),
    next: (v: Mode) => t("c.CalendarMonth.next", { view: viewName(v) }),
    today: t("c.CalendarMonth.today"),
    periodTitle: (v: Mode, start: string, end: string) =>
      v === "month" || v === "agenda" ? `${MONTHS[Number(anchorDate.slice(5, 7)) - 1]} ${anchorDate.slice(0, 4)}` : v === "day" ? start : `${start} – ${end}`,
    periodAnnouncement: (v: Mode, start: string, end: string) => t("p.calendar.showing", { view: viewName(v), start, end }),
    weekday: (wd: number) => WEEKDAY_NAMES[sundayFirst(wd)] ?? "",
    weekdayAbbr: (wd: number) => (WEEKDAY_NAMES[sundayFirst(wd)] ?? "").slice(0, 3),
    // A cell's name starts with exactly the text it shows (the day number and its milestones), then says the full date (WCAG 2.5.3).
    day: (date: string, wd: number) =>
      `${shownOn(date)}. ${Number(date.slice(8, 10))} ${MONTHS[Number(date.slice(5, 7)) - 1] ?? ""} ${date.slice(0, 4)}, ${WEEKDAY_NAMES[sundayFirst(wd)] ?? ""}`,
    ...exportLabels(),
    search: t("p.table.find"),
    searchAnnouncement,
    exportHeaders: { title: t("p.calendar.milestone"), start: t("p.calendar.due"), end: t("p.calendar.due") },
  });
</script>

<svelte:head><title>{t("p.calendar.ppm", { t: t("ppm.nav.calendar") })}</title></svelte:head>

<h1>{t("ppm.nav.calendar")}</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

<p>
  <label>
    {t("p.calendar.kind")}
    <select
      value={kind}
      onchange={(event) => {
        kind = event.currentTarget.value;
      }}
    >
      <option value="">{t("p.calendar.all")}</option>
      {#each calendar?.kinds ?? [] as option (option)}
        <option value={option}>{option}</option>
      {/each}
    </select>
  </label>
</p>

{#if calendar}
  <div class="calendar-wrap" data-testid="milestone-calendar">
    <CalendarView
      label={t("ppm.nav.calendar")}
      bind:view
      bind:anchorDate
      {today}
      {events}
      {eventFields}
      {labels}
      maxEvents={MAX_SHOWN}
      agendaBelow={640}
      bind:search
      exportFilename="milestones"
    >
      {#snippet day({ date, visibleEvents, hiddenCount, fieldText })}
        <span class="d">{Number(date.slice(8))}</span>
        {#each visibleEvents as event (event.id)}
          <span class="ev" class:done={event["done"]}>{event["title"]}{event["done"] ? ` ✓ ${t("p.calendar.status_done")}` : ""}</span>
          {#if view === "agenda"}
            <span class="fields">{eventFields.map((f) => `${f.header}: ${fieldText(event, f.id)}`).join(" · ")}</span>
          {/if}
        {/each}
        {#if hiddenCount > 0}<span class="more">{t("p.calendar.more", { n: hiddenCount })}</span>{/if}
      {/snippet}
    </CalendarView>
  </div>
  <ul data-testid="milestone-list">
    {#each calendar.milestones as milestone (milestone.pid)}
      <li>
        <strong>{milestone.due}</strong> — {milestone.kind}: {milestone.name}
        <span class="muted">({milestone.item?.name ?? "—"}{milestone.done ? t("p.calendar.done") : ""})</span>
      </li>
    {:else}
      <li class="muted">{t("p.calendar.no_milestones_match")}</li>
    {/each}
  </ul>
{/if}

<style>
  .calendar-wrap { overflow-x: auto; margin-bottom: 1rem; }
  .calendar-wrap :global(table) { border-collapse: collapse; width: 100%; }
  .calendar-wrap :global(th),
  .calendar-wrap :global(td) {
    border: 1px solid var(--border, #ccc);
    padding: 0.2rem 0.3rem;
    vertical-align: top;
    font-size: 0.85em;
  }
  .calendar-wrap :global(td) { min-width: 5.5rem; height: 5rem; }
  .calendar-wrap :global([data-outside-period]) { background: var(--surface-2, #f1f1f1); color: var(--text-2, #595959); }
  .calendar-wrap :global([data-today]) { outline: 2px solid var(--text, #222); outline-offset: -2px; }
  .calendar-wrap :global(td:focus-visible) { outline: 3px solid var(--mxi-color-focus, #1d4ed8); outline-offset: -3px; }
  .calendar-wrap :global(.calendar-view-toolbar) {
    display: flex; flex-wrap: wrap; justify-content: space-between; align-items: center; gap: 0.5rem; margin-block-end: 0.4rem;
  }
  .calendar-wrap :global(.calendar-view-nav),
  .calendar-wrap :global(.calendar-view-switcher),
  .calendar-wrap :global(.calendar-view-export) { display: flex; gap: 0.25rem; }
  .calendar-wrap :global(button[aria-pressed="true"]) { font-weight: 700; }
  .d { display: block; font-weight: 600; }
  .ev { display: block; margin-block-start: 0.15rem; padding-inline-start: 0.25rem; border-inline-start: 3px solid var(--accent, #1d4ed8); overflow-wrap: anywhere; }
  .ev.done { text-decoration: line-through; border-inline-start-style: dashed; }
  .fields { display: block; margin-inline-start: 0.5rem; color: var(--text-2, #595959); }
  .more { display: block; font-style: italic; }
</style>
