<!--
  Work and assignments (`/plans/[pid]/work`): for each task, how long it takes,
  how much of the assignee's day it asks for, and how much work that is — and
  the rule that decides which of the three moves when you change one. Below, the
  work spread over days, weeks or months by each task's contour, as a table that
  always adds up. Over-capacity is said in words, never hidden; work that cannot
  be placed is listed, never dropped.
-->
<script lang="ts">
  import { rowOf } from "#lib/row-of.js";
  import { errorText } from "#lib/error-text.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import {
    PpmClient,
    type Contour,
    type CpmNode,
    type Grain,
    type SchedulingRule,
    type WorkProfile,
  } from "#lib/api/ppm.js";
  import {
    CONTOUR_TEXT,
    RULE_HELP,
    RULE_TEXT,
    UNPLACED_TEXT,
    assignmentBody,
    capacityText,
    customWeights,
    hoursText,
    resultText,
  } from "#lib/work-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");
  let nodes = $state<CpmNode[]>([]);
  let profile = $state<WorkProfile | null>(null);
  let error = $state<string | null>(null);
  let grain = $state<Grain>("week");
  /** What the last change did, per task. */
  let said = $state<Record<string, string>>({});
  let form = $state<Record<string, { rule: SchedulingRule; changing: "duration" | "units" | "work"; duration: string; units: string; hours: string; contour: Contour; custom: string }>>({});

  async function load() {
    try {
      nodes = (await ppm.planCpm(pid)).nodes;
      for (const n of nodes)
        form[n.pid] ??= {
          rule: n.scheduling_rule ?? "fixed_units",
          changing: n.units_percent == null ? "units" : "work",
          duration: String(n.duration_days ?? ""),
          units: String(n.units_percent ?? ""),
          hours: n.work_minutes ? String(n.work_minutes / 60) : "",
          contour: n.contour ?? "flat",
          custom: "",
        };
      await loadProfile();
      error = null;
    } catch (err) {
      error = errorText(err, "p.plans.pid.work.could_not_load_the_plan");
    }
  }
  async function loadProfile() {
    try {
      profile = await ppm.workProfile(pid, { period: grain });
    } catch (err) {
      profile = null;
      error = errorText(err, "p.plans.pid.work.could_not_load_the_work");
    }
  }
  onMount(load);

  async function save(n: CpmNode, e: SubmitEvent) {
    e.preventDefault();
    const f = form[n.pid];
    if (!f) return;
    const body = assignmentBody({ ...f, first: n.units_percent == null });
    if ("error" in body) {
      said[n.pid] = body.error;
      return;
    }
    try {
      const r = await ppm.setAssignment(pid, n.pid, body);
      said[n.pid] = [resultText(r), capacityText(r.capacity)].filter(Boolean).join(" ");
      await load();
    } catch (err) {
      said[n.pid] = errorText(err, "p.plans.pid.work.could_not_save");
    }
  }
  async function saveContour(n: CpmNode) {
    const f = form[n.pid];
    if (!f) return;
    let custom: number[] | undefined;
    if (f.contour === "custom") {
      const w = customWeights(f.custom);
      if (!Array.isArray(w)) {
        said[n.pid] = w.error;
        return;
      }
      custom = w;
    }
    try {
      await ppm.setContour(pid, n.pid, f.contour, custom);
      said[n.pid] = t("p.plans.pid.work.contour_saved");
      await loadProfile();
    } catch (err) {
      said[n.pid] = errorText(err, "p.plans.pid.work.could_not_save");
    }
  }
  const day = (iso: string) => iso.slice(5);
  const label = (b: { from: string; to: string }) => (b.from === b.to ? b.from : `${day(b.from)}–${day(b.to)}`);

  type WRow = WorkProfile["tasks"][number];
  const workColumns = (profile: WorkProfile): DataGridColumn[] => [
    { id: "title", header: t("p.plans.pid.work.task"), rowHeader: true, cell: workTaskCell },
    ...(profile.totals.buckets ?? []).map((b, i): DataGridColumn => ({
      id: `b${i}`, header: label(b), align: "end",
      accessor: (r) => (rowOf<WRow>(r).buckets[i]?.minutes ?? 0), format: (v) => (v ? hoursText(Number(v)) : "–"),
    })),
    { id: "work_minutes", header: t("p.plans.pid.work.total"), align: "end", cell: workTotalCell, format: (v) => hoursText(Number(v)) },
  ];
</script>

{#snippet workTaskCell({ row }: DataGridCellContext)}
  {@const r = rowOf<WRow>(row)}
  {r.title} <span class="muted small">({r.contour.replace("_", " ")})</span>
{/snippet}

{#snippet workTotalCell({ row, formatted }: DataGridCellContext)}
  {formatted}{#if !rowOf<WRow>(row).placed} <span class="small">{t("p.plans.pid.work.not_placed")}</span>{/if}
{/snippet}


<svelte:head><title>{t("p.plans.pid.work.work_and_assignments_ppm")}</title></svelte:head>

<h1>{t("p.plans.pid.work.work_and_assignments")}</h1>
<p class="lede">{t("p.plans.pid.work.how_long_each_task_takes")}</p>
<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plans.pid.work.plan")}</a> · <a href={localeHref(`/plans/${pid}/costs`)}>{t("p.plans.pid.work.costs")}</a></p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}

<h2>{t("p.plans.pid.work.tasks")}</h2>
{#if nodes.length === 0 && !error}
  <p class="muted">{t("p.plans.pid.work.this_plan_has_no_scheduled")}</p>
{/if}
<ul class="stack tasks">
  {#each nodes as n (n.pid)}
    {@const f = form[n.pid]}
    <li class="surface">
      <h3>{n.title ?? n.pid}</h3>
      <p class="small">
        {#if n.units_percent != null && n.work_minutes != null}
          {t("p.plans.pid.work.business_days_of_a_day", { duration_days: n.duration_days, units_percent: n.units_percent, hoursText: hoursText(n.work_minutes), v: RULE_TEXT[n.scheduling_rule ?? "fixed_units"] })}
        {:else}
          <span class="muted">{t("p.plans.pid.work.no_assignment_yet_give_a")}</span>
        {/if}
      </p>
      {#if f}
        <details>
          <summary>{t("p.plans.pid.work.edit_assignment")}</summary>
          <form class="stack" onsubmit={(e) => save(n, e)}>
            <label>
              {t("p.plans.pid.work.rule")}
              <select bind:value={f.rule}>
                {#each Object.entries(RULE_TEXT) as [k, v] (k)}<option value={k}>{v}</option>{/each}
              </select>
            </label>
            <p class="small muted">{RULE_HELP[f.rule]}</p>
            {#if n.units_percent == null}
              <label>{t("p.plans.pid.work.duration_business_days")} <input type="number" min="1" max="3650" step="1" bind:value={f.duration} /></label>
              <label>{t("p.plans.pid.work.units_of_the_day")} <input type="number" min="1" max="1000" step="1" bind:value={f.units} /></label>
            {:else}
              <label>
                {t("p.plans.pid.work.change")}
                <select bind:value={f.changing}>
                  <option value="work">{t("p.plans.pid.work.work_hours")}</option>
                  <option value="duration">{t("p.plans.pid.work.duration_business_days")}</option>
                  <option value="units">{t("p.plans.pid.work.units")}</option>
                </select>
              </label>
              {#if f.changing === "work"}
                <label>{t("p.plans.pid.work.work_hours")} <input type="number" min="0" step="0.25" bind:value={f.hours} /></label>
              {:else if f.changing === "duration"}
                <label>{t("p.plans.pid.work.duration_business_days")} <input type="number" min="1" max="3650" step="1" bind:value={f.duration} /></label>
              {:else}
                <label>{t("p.plans.pid.work.units_of_the_day")} <input type="number" min="1" max="1000" step="1" bind:value={f.units} /></label>
              {/if}
            {/if}
            <button class="button" type="submit">{t("p.plans.pid.work.save_assignment")}</button>
          </form>
          {#if n.units_percent != null}
            <div class="row">
              <label>
                {t("p.plans.pid.work.spread")}
                <select bind:value={f.contour}>
                  {#each Object.entries(CONTOUR_TEXT) as [k, v] (k)}<option value={k}>{v}</option>{/each}
                </select>
              </label>
              {#if f.contour === "custom"}
                <label>{t("p.plans.pid.work.percentages")} <input bind:value={f.custom} placeholder="10, 20, 70" /></label>
              {/if}
              <button class="button" type="button" onclick={() => saveContour(n)}>{t("p.plans.pid.work.save_contour")}</button>
            </div>
          {/if}
        </details>
        {#if said[n.pid]}<p class="small" role="status">{said[n.pid]}</p>{/if}
      {/if}
    </li>
  {/each}
</ul>

<h2>{t("p.plans.pid.work.work_over_time")}</h2>
<label>{t("p.plans.pid.work.period")}
  <select bind:value={grain} onchange={loadProfile}>
    <option value="day">{t("p.plans.pid.work.day")}</option>
    <option value="week">{t("p.plans.pid.work.week")}</option>
    <option value="month">{t("p.plans.pid.work.month")}</option>
  </select>
</label>
{#if profile && profile.reason === "no_assignments"}
  <p class="muted">{t("p.plans.pid.work.no_task_carries_work_yet")}</p>
{:else if profile && profile.totals.buckets}
  {@const p = profile}
  {@const buckets = profile.totals.buckets}
  <DataTable label={t("p.plans.pid.work.hours_per_each_row_adds", { grain })} columns={workColumns(p)} rows={p.tasks} rowId={(r) => String(r.pid)} sortable={false} testid="work-profile">
    {#snippet footer()}
      <tr>
        <th scope="row">{t("p.plans.pid.work.plan_2")}</th>
        {#each buckets as b, i (i)}<td>{b.minutes ? hoursText(b.minutes) : "–"}</td>{/each}
        <td>{hoursText(p.totals.work_minutes)}</td>
      </tr>
    {/snippet}
  </DataTable>
  {#if profile.unplaced.length}
    <ul>
      {#each profile.unplaced as u, i (i)}
        <li>{t("p.plans.pid.work.so_its_work_is_not", { title: u.title, hoursText: hoursText(u.work_minutes), reason: UNPLACED_TEXT[u.reason] ?? u.reason })}</li>
      {/each}
    </ul>
  {/if}
  {#if profile.totals.outside_window_minutes}
    <p class="small muted">{t("p.plans.pid.work.falls_outside_this_window", { hoursText: hoursText(profile.totals.outside_window_minutes) })}</p>
  {/if}
{/if}

<style>
  .tasks {
    list-style: none;
    padding: 0;
  }
  h3 {
    margin: 0 0 0.25rem;
    font-size: 1rem;
  }
  label {
    display: block;
  }
</style>
