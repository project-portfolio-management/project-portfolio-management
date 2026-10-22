<!--
  What-if (`/plans/[pid]/what-if`): stage schedule edits, preview what they would
  do on a copy of the plan (nothing is saved), then commit exactly that or
  discard. A preview is tied to the plan as it was; if the plan changes first,
  the commit is refused and you preview again — it is never merged. Cost is shown
  only to someone allowed to see it, and who does the work only to someone who
  may manage people.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { ApiError } from "#lib/api/client.js";
  import {
    PpmClient,
    type CpmEdge,
    type CpmNode,
    type SandboxEdit,
    type SandboxPreview,
  } from "#lib/api/ppm.js";
  import {
    CONSTRAINTS,
    EDIT_TEXT,
    MAX_EDITS,
    buildEdit,
    costReasonText,
    deltaText,
    describeEdit,
    finishText,
    isStale,
    type EditForm,
    type EditKind,
  } from "#lib/whatif-view.js";
  import { hoursText } from "#lib/work-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");
  let nodes = $state<CpmNode[]>([]);
  let edges = $state<CpmEdge[]>([]);
  let edits = $state<SandboxEdit[]>([]);
  let preview = $state<SandboxPreview | null>(null);
  let error = $state<string | null>(null);
  let status = $state("");
  let reason = $state("");
  let busy = $state(false);
  let form = $state<EditForm>({
    kind: "duration" as EditKind, task: "", start: "", days: "", constraint: "", constraintDate: "",
    reason: "", from: "", to: "", linkKind: "fs", lag: "", link: "",
  });

  const name = (p: string) => nodes.find((n) => n.pid === p)?.title ?? p.slice(0, 8);

  async function load() {
    try {
      const c = await ppm.planCpm(pid);
      nodes = c.nodes;
      edges = c.edges;
      error = null;
    } catch (err) {
      error = errorText(err, "p.plans.pid.what_if.could_not_load_the_plan");
    }
  }
  onMount(load);

  function stage(e: SubmitEvent) {
    e.preventDefault();
    const edit = buildEdit(form);
    if ("error" in edit) {
      status = edit.error;
      return;
    }
    if (edits.length >= MAX_EDITS) {
      status = t("p.plans.pid.what_if.at_most_edits_at_a", { MAX_EDITS });
      return;
    }
    edits = [...edits, edit];
    preview = null; // a preview describes the edits it was made from
    status = t("p.plans.pid.what_if.edit_staged_preview_to_see");
  }
  function unstage(i: number) {
    edits = edits.filter((_, j) => j !== i);
    preview = null;
  }
  function discard() {
    edits = [];
    preview = null;
    status = t("p.plans.pid.what_if.discarded_nothing_was_changed");
  }
  async function run() {
    busy = true;
    try {
      preview = await ppm.sandboxPreview(pid, edits);
      status = "";
    } catch (err) {
      preview = null;
      status = errorText(err, "p.plans.pid.what_if.could_not_preview");
    } finally {
      busy = false;
    }
  }
  async function commit() {
    if (!preview) return;
    busy = true;
    try {
      const r = await ppm.sandboxCommit(pid, edits, preview.plan_etag, reason);
      status = t(r.audit_entries === 1 ? "p.plans.pid.what_if.committed_one" : "p.plans.pid.what_if.committed_other", { count: r.audit_entries });
      edits = [];
      preview = null;
      reason = "";
      await load();
    } catch (err) {
      if (err instanceof ApiError && isStale(err.status)) {
        preview = null;
        status = t("p.plans.pid.what_if.the_plan_changed_since_this");
        await load();
      } else status = errorText(err, "p.plans.pid.what_if.could_not_commit");
    } finally {
      busy = false;
    }
  }
  const dates = (d: { early_start: string; early_finish: string; total_float: number; critical: boolean } | null) =>
    d ? t("p.plans.pid.what_if.to", { early_start: d.early_start, early_finish: d.early_finish }) : t("p.plans.pid.what_if.not_scheduled");

  type PRow = SandboxPreview["tasks"][number];
  const columns: DataGridColumn[] = $derived([
    { id: "title", header: t("p.plans.pid.what_if.task"), rowHeader: true, cell: whatTaskCell },
    { id: "before", header: t("p.plans.pid.what_if.before"), accessor: (r) => dates((r as unknown as PRow).before) },
    { id: "after", header: t("p.plans.pid.what_if.after"), accessor: (r) => dates((r as unknown as PRow).after) },
    { id: "float", header: t("p.plans.pid.what_if.float"), sortable: false, accessor: (r) => { const x = r as unknown as PRow; return `${x.before?.total_float ?? "–"} → ${x.after?.total_float ?? "–"}`; }, cell: floatCell },
  ]);
</script>

{#snippet whatTaskCell({ row }: DataGridCellContext)}
  {@const r = row as unknown as PRow}
  {r.title}{#if r.assignee} <span class="muted small">({r.assignee})</span>{/if}
{/snippet}

{#snippet floatCell({ row, formatted }: DataGridCellContext)}
  {formatted}{#if (row as unknown as PRow).after?.critical}<span class="small"> {t("p.plans.pid.what_if.critical")}</span>{/if}
{/snippet}


<svelte:head><title>{t("p.plans.pid.what_if.what_if_ppm")}</title></svelte:head>

<h1>{t("p.plans.pid.what_if.what_if")}</h1>
<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plans.pid.what_if.plan")}</a> · <a href={localeHref(`/plans/${pid}/critical-path`)}>{t("p.plans.pid.what_if.critical_path")}</a></p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}
<p class="muted small">{t("p.plans.pid.what_if.try_changes_on_a_copy")}</p>

<form class="surface stack" onsubmit={stage} aria-labelledby="stage-h">
  <h2 id="stage-h">{t("p.plans.pid.what_if.stage_an_edit")}</h2>
  <label>
    {t("p.plans.pid.what_if.edit")}
    <select bind:value={form.kind}>
      {#each Object.entries(EDIT_TEXT) as [k, v]}<option value={k}>{v}</option>{/each}
    </select>
  </label>
  {#if form.kind === "add_link"}
    <label>{t("p.plans.pid.what_if.predecessor")}
      <select bind:value={form.from}><option value="">{t("p.plans.pid.what_if.choose")}</option>{#each nodes as n (n.pid)}<option value={n.pid}>{n.title ?? n.pid}</option>{/each}</select>
    </label>
    <label>{t("p.plans.pid.what_if.successor")}
      <select bind:value={form.to}><option value="">{t("p.plans.pid.what_if.choose")}</option>{#each nodes as n (n.pid)}<option value={n.pid}>{n.title ?? n.pid}</option>{/each}</select>
    </label>
    <label>{t("p.plans.pid.what_if.type")}
      <select bind:value={form.linkKind}>
        <option value="fs">{t("p.plans.pid.what_if.finish_to_start")}</option><option value="ss">{t("p.plans.pid.what_if.start_to_start")}</option>
        <option value="ff">{t("p.plans.pid.what_if.finish_to_finish")}</option><option value="sf">{t("p.plans.pid.what_if.start_to_finish")}</option>
      </select>
    </label>
    <label>{t("p.plans.pid.what_if.lag_business_days")} <input type="number" step="1" bind:value={form.lag} /></label>
  {:else if form.kind === "remove_link"}
    <label>{t("p.plans.pid.what_if.link")}
      <select bind:value={form.link}>
        <option value="">{t("p.plans.pid.what_if.choose")}</option>
        {#each edges as l (l.pid)}<option value={l.pid}>{t("p.plans.pid.what_if.follows", { name: name(l.successor_pid), name2: name(l.predecessor_pid), toUpperCase: l.kind.toUpperCase() })}</option>{/each}
      </select>
    </label>
  {:else}
    <label>{t("p.plans.pid.what_if.task")}
      <select bind:value={form.task}><option value="">{t("p.plans.pid.what_if.choose")}</option>{#each nodes as n (n.pid)}<option value={n.pid}>{n.title ?? n.pid}</option>{/each}</select>
    </label>
    {#if form.kind === "move"}
      <label>{t("p.plans.pid.what_if.new_start")} <input type="date" bind:value={form.start} /></label>
    {:else if form.kind === "duration"}
      <label>{t("p.plans.pid.what_if.business_days")} <input type="number" min="0" max="3650" step="1" bind:value={form.days} /></label>
    {:else}
      <label>{t("p.plans.pid.what_if.constraint")}
        <select bind:value={form.constraint}>
          <option value="">{t("p.plans.pid.what_if.none_clear_it")}</option>
          {#each Object.entries(CONSTRAINTS) as [k, v]}<option value={k}>{v}</option>{/each}
        </select>
      </label>
      {#if form.constraint}
        <label>{t("p.plans.pid.what_if.date")} <input type="date" bind:value={form.constraintDate} /></label>
        {#if form.constraint === "mso" || form.constraint === "mfo"}
          <label>{t("p.plans.pid.what_if.reason")} <input bind:value={form.reason} /></label>
        {/if}
      {/if}
    {/if}
  {/if}
  <button class="button" type="submit">{t("p.plans.pid.what_if.stage_edit")}</button>
</form>
{#if status}<p role="status" class="small">{status}</p>{/if}

<h2>{t("p.plans.pid.what_if.staged_edits")}</h2>
{#if edits.length === 0}
  <p class="muted">{t("p.plans.pid.what_if.none_yet")}</p>
{:else}
  <ol data-testid="staged">
    {#each edits as e, i}
      <li>
        {describeEdit(e, name)}
        <button class="button" type="button" onclick={() => unstage(i)} aria-label={t("p.plans.pid.what_if.remove_edit", { n: i + 1 })}>{t("p.plans.pid.what_if.remove")}</button>
      </li>
    {/each}
  </ol>
  <div class="row">
    <button class="button" class:primary={!preview} type="button" onclick={run} disabled={busy}>{t("p.plans.pid.what_if.preview")}</button>
    <button class="button" type="button" onclick={discard} disabled={busy}>{t("p.plans.pid.what_if.discard")}</button>
  </div>
{/if}

{#if preview}
  <section class="surface stack" aria-labelledby="pv-h" data-testid="preview">
    <h2 id="pv-h">{t("p.plans.pid.what_if.preview")}</h2>
    <p>{finishText(preview)}</p>
    {#if preview.tasks.length === 0}
      <p class="muted">{t("p.plans.pid.what_if.no_task_s_dates_or")}</p>
    {:else}
      <DataTable label={t("p.plans.pid.what_if.tasks_whose_dates_float_or")} {columns} rows={preview.tasks} rowId={(r) => String(r.pid)} />
    {/if}
    {#if preview.critical_path.added.length || preview.critical_path.removed.length}
      <p class="small">
        {#if preview.critical_path.added.length}{t("p.plans.pid.what_if.joins_the_critical_path", { join: preview.critical_path.added.map(name).join(", ") })} {/if}
        {#if preview.critical_path.removed.length}{t("p.plans.pid.what_if.leaves_it", { join: preview.critical_path.removed.map(name).join(", ") })}{/if}
      </p>
    {/if}
    {#each preview.assignments_recomputed as r}
      <p class="small">{t("p.plans.pid.what_if.now_takes_of_the_day", { name: name(r.task_pid), units_percent: r.units_percent, hoursText: hoursText(r.work_minutes) })}</p>
    {/each}
    {#if preview.cost_impact.available}
      {#each preview.cost_impact.labour as c}
        <p>{t("p.plans.pid.what_if.planned_labour_cost", { deltaText: deltaText(c.delta_minor, c.currency), deltaText2: deltaText(c.before_minor, c.currency), deltaText3: deltaText(c.after_minor, c.currency) })}</p>
      {/each}
      {#if preview.cost_impact.unrated_minutes_after}
        <p class="small">{t("p.plans.pid.what_if.of_work_has_no_rate", { hoursText: hoursText(preview.cost_impact.unrated_minutes_after) })}</p>
      {/if}
    {:else}
      <p class="small muted">{costReasonText(preview.cost_impact)}</p>
    {/if}
    <label>{t("p.plans.pid.what_if.reason_recorded_with_each_change")} <input bind:value={reason} maxlength="500" /></label>
    <div class="row">
      <button class="button primary" type="button" onclick={commit} disabled={busy}>{t("p.plans.pid.what_if.commit_these_changes")}</button>
      <button class="button" type="button" onclick={discard} disabled={busy}>{t("p.plans.pid.what_if.discard")}</button>
    </div>
  </section>
{/if}

<style>
  label {
    display: block;
  }
  li {
    margin-bottom: 0.25rem;
  }
</style>
