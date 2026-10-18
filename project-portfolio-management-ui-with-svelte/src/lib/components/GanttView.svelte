<!--
  The schedule Gantt (`/gantt`) on Lily's GanttChart: a keyboard- and screen-reader-first grid whose task bars are
  runs of marked cells, with zoom (month / week / day), export (CSV, Excel) and a text summary of each task's
  predecessors. Read-only: schedule edits stay with the schedule endpoints. The critical path is the service's, drawn
  as a colour and also said in words in the row's name; a task with no dates is listed with no bar, never invented
  onto the timeline.
-->
<script lang="ts">
  import GanttChart from "@lilydesignsystem/svelte-gantt-chart";
  import type { GanttTask, GanttTimeUnit } from "@lilydesignsystem/svelte-gantt-chart";
  import { onMount } from "svelte";
  import { t } from "#lib/i18n.svelte.js";
  import { exportLabels, searchAnnouncement } from "#lib/table-labels.js";
  import { chartRange, holidayRanges, nonWorkingWeekdays, type ChartTask } from "#lib/gantt-view.js";

  let {
    tasks,
    label,
    mask = "",
    holidays = [],
  }: {
    tasks: ChartTask[];
    label: string;
    /** The plan's working days, seven `0`/`1`, Monday first. Empty: no weekend shading. */
    mask?: string;
    holidays?: readonly string[];
  } = $props();
  const weekends = $derived(nonWorkingWeekdays(mask));
  const unavailable = $derived(holidayRanges(holidays, t("c.GanttView.holiday")));
  const shaded = $derived(weekends.length > 0 || unavailable.length > 0);

  let today = $state<string | undefined>(undefined);
  let timeUnit = $state<GanttTimeUnit>("week");
  onMount(() => {
    const d = new Date();
    today = `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, "0")}-${String(d.getDate()).padStart(2, "0")}`;
  });

  const range = $derived(chartRange(tasks, today));
  const unitName = (u: GanttTimeUnit) => t(`c.GanttView.unit_${u}` as never);
  const critical = (task: GanttTask) => (task.critical ? t("c.GanttView.critical", { name: task.label }) : task.label);

  const labels = $derived({
    taskColumnHeader: t("c.GanttView.task"),
    columnLabel: (start: string, _end: string, unit: GanttTimeUnit) => (unit === "month" ? start.slice(0, 7) : start.slice(5)),
    dependencySummary: (names: string[]) => t("c.GanttView.depends_on", { names: names.join(", ") }),
    zoomIn: t("c.GanttView.zoom_in"),
    zoomOut: t("c.GanttView.zoom_out"),
    zoomAnnouncement: (u: GanttTimeUnit) => t("c.GanttView.zoomed", { unit: unitName(u) }),
    ...exportLabels(),
    search: t("p.table.find"),
    searchAnnouncement,
    exportHeaders: { task: t("c.GanttView.task"), start: t("c.GanttView.start"), end: t("c.GanttView.end"), dependsOn: t("c.GanttView.predecessors") },
  });
</script>

<div class="gantt-view" data-testid="ppm-gantt">
  {#if range}
    <GanttChart {label} {range} tasks={tasks as GanttTask[]} bind:timeUnit {today} taskLabel={critical} readonly dependencyArrows {weekends} {unavailable} {labels} exportFilename="schedule" />
  {/if}
  {#if shaded}<p class="small muted">{t("c.GanttView.shaded")}</p>{/if}
</div>

<style>
  .gantt-view {
    overflow-x: auto;
  }
  .gantt-view :global(table) {
    border-collapse: collapse;
  }
  .gantt-view :global(th),
  .gantt-view :global(td) {
    border: 1px solid var(--border, #ccc);
    padding: 0.2rem 0.4rem;
    font-size: 0.85em;
  }
  .gantt-view :global([data-in-range]) {
    background: color-mix(in srgb, var(--accent, #1d4ed8) 55%, transparent);
  }
  .gantt-view :global([data-critical] [data-in-range]) {
    background: color-mix(in srgb, #c0392b 70%, transparent);
  }
  .gantt-view :global([data-milestone]) {
    font-weight: 700;
  }
  /* Days off are hatched as well as tinted, so they are not told by colour alone; the note under the chart says it in words. */
  .gantt-view :global([data-weekend]),
  .gantt-view :global([data-unavailable]) {
    background-image: repeating-linear-gradient(135deg, transparent 0 4px, color-mix(in srgb, var(--text, #222) 14%, transparent) 4px 5px);
  }
  .gantt-view :global([data-today]) {
    outline: 2px solid var(--text, #222);
    outline-offset: -2px;
  }
  .gantt-view :global([data-unscheduled]) {
    opacity: 0.7;
    font-style: italic;
  }
  .gantt-view :global(td:focus-visible),
  .gantt-view :global(tr:focus-visible) {
    outline: 3px solid var(--mxi-color-focus, #1d4ed8);
    outline-offset: -3px;
  }
  .gantt-view :global(.gantt-chart-zoom),
  .gantt-view :global(.gantt-chart-export) {
    display: inline-flex;
    gap: 0.25rem;
    margin-block-end: 0.4rem;
    margin-inline-end: 0.75rem;
  }
</style>
