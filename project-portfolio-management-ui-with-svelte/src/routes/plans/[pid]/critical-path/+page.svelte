<!--
  Critical path (`/plans/[pid]/critical-path`, SC-1 / SC-2 / PJ-12): the plan's
  tasks with their float. Three views of one result — a Gantt with the float
  drawn as a tail, a precedence network, and a table — plus the critical chains,
  what could not be scheduled (and why), and an editor for a task's start,
  duration, constraint and deadline that answers with **what moved**.
  English-first, like the other recent views.
-->
<script lang="ts">
  import { taskEstimateProblems, taskRangeText } from "#lib/simulation-inputs.js";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { ApiError } from "#lib/api/client.js";
  import {
    PpmClient,
    type Cpm,
    type CpmNode,
    type ScheduleBaseline,
    type ScheduleMoved,
    type ScheduleVariance,
  } from "#lib/api/ppm.js";
  import CpmGantt from "#lib/components/CpmGantt.svelte";
  import CpmTable from "#lib/components/CpmTable.svelte";
  import NetworkDiagram from "#lib/components/NetworkDiagram.svelte";
  import VarianceTable from "#lib/components/VarianceTable.svelte";
  import { diagramFits, floatLabel, restrictedText, unscheduledReason } from "#lib/cpm-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");

  let cpm = $state<Cpm | null>(null);
  let error = $state<string | null>(null);
  let view = $state<"gantt" | "network" | "table">("gantt");
  let near = $state(5);
  let selected = $state<string | null>(null);
  let result = $state<ScheduleMoved | null>(null);
  let saving = $state(false);

  // Baselines (SC-3): append-only; variance is against a chosen version.
  let baselines = $state<ScheduleBaseline[]>([]);
  let chosen = $state<number | null>(null);
  let variance = $state<ScheduleVariance | null>(null);
  let baselineReason = $state("");
  let showBaseline = $state(true);

  // The editor's fields (an absent field clears it, so the form is always complete).
  let start = $state("");
  let duration = $state("");
  let optimistic = $state("");
  let pessimistic = $state("");
  let estimateProblem = $state<string[]>([]);
  let estimateSaved = $state(false);
  let ckind = $state("");
  let cdate = $state("");
  let deadline = $state("");
  let reason = $state("");

  async function loadBaselines() {
    try {
      baselines = await ppm.scheduleBaselines(pid);
      variance = await ppm.scheduleVariance(pid, chosen ?? undefined);
      chosen = variance.baseline?.version ?? null;
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.plans.pid.critical_path.could_not_load_the_baselines");
    }
  }

  async function takeBaseline(event: SubmitEvent) {
    event.preventDefault();
    error = null;
    try {
      await ppm.takeScheduleBaseline(pid, baselineReason);
      baselineReason = "";
      chosen = null;
      await loadBaselines();
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.plans.pid.critical_path.could_not_take_the_baseline");
    }
  }

  async function load() {
    try {
      cpm = await ppm.planCpm(pid, near);
      error = null;
      await loadBaselines();
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.plans.pid.critical_path.could_not_load_the_schedule");
    }
  }
  onMount(() => void load());

  // Where each task was baselined to sit, for the outline under its Gantt bar.
  const ghosts = $derived(
    new Map(
      (showBaseline ? (variance?.variance?.rows ?? []) : [])
        .filter((r) => r.baseline_start && r.baseline_finish)
        .map((r) => [r.task_pid, { start: r.baseline_start, finish: r.baseline_finish }] as const),
    ),
  );

  const titleOf = $derived(new Map((cpm?.nodes ?? []).map((n) => [n.pid, n.title ?? "(untitled)"])));
  const node = $derived<CpmNode | null>(cpm?.nodes.find((n) => n.pid === selected) ?? null);
  const unscheduledSelected = $derived(cpm?.unscheduled.find((u) => u.pid === selected) ?? null);

  function select(p: string) {
    selected = p;
    result = null;
    const n = cpm?.nodes.find((x) => x.pid === p);
    start = n?.planned_start ?? "";
    duration = n?.duration_days === null || n?.duration_days === undefined ? "" : String(n.duration_days);
    optimistic = n?.duration_optimistic_days === null || n?.duration_optimistic_days === undefined ? "" : String(n.duration_optimistic_days);
    pessimistic = n?.duration_pessimistic_days === null || n?.duration_pessimistic_days === undefined ? "" : String(n.duration_pessimistic_days);
    estimateProblem = [];
    estimateSaved = false;
    ckind = n?.constraint ?? "";
    cdate = n?.constraint_date ?? "";
    deadline = n?.deadline ?? "";
    reason = "";
  }

  const hard = $derived(ckind === "mso" || ckind === "mfo");

  async function saveEstimate(event: SubmitEvent) {
    event.preventDefault();
    if (!selected) return;
    estimateSaved = false;
    estimateProblem = taskEstimateProblems(optimistic, pessimistic, node?.duration_days ?? null);
    if (estimateProblem.length > 0) return;
    try {
      await ppm.setTaskEstimate(pid, selected, {
        optimistic_days: optimistic === "" ? null : Number(optimistic),
        pessimistic_days: pessimistic === "" ? null : Number(pessimistic),
      });
      estimateSaved = true;
      await load();
    } catch (err) {
      error = err instanceof ApiError || err instanceof Error ? err.message : t("p.plans.pid.critical_path.could_not_save_the_estimate");
    }
  }

  async function save(event: SubmitEvent) {
    event.preventDefault();
    if (!selected) return;
    saving = true;
    error = null;
    try {
      result = await ppm.setTaskSchedule(pid, selected, {
        start_date: start || null,
        duration_days: duration === "" ? null : Number(duration),
        constraint_kind: ckind || null,
        constraint_date: ckind ? cdate || null : null,
        deadline: deadline || null,
        reason: hard ? reason : null,
      });
      await load();
    } catch (err) {
      error = err instanceof ApiError || err instanceof Error ? err.message : t("p.plans.pid.critical_path.could_not_save");
    } finally {
      saving = false;
    }
  }
</script>

<svelte:head><title>{t("p.plans.pid.critical_path.critical_path_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plans.pid.critical_path.plan")}</a> · <a href={localeHref(`/plans/${pid}/calendar`)}>{t("p.plans.pid.critical_path.working_calendar")}</a></p>
<h1>{t("p.plans.pid.critical_path.critical_path")}</h1>
<p class="lede">{t("p.plans.pid.critical_path.the_longest_chain_of_tasks")}</p>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if cpm}
  <p class="small muted">
    {cpm.project_finish ? t("p.plans.pid.critical_path.finishes", { date: cpm.project_finish }) : t("p.plans.pid.critical_path.nothing_scheduled")}
    {t("p.plans.pid.critical_path.calendar_label", { calendar: cpm.calendar === "plan" ? t("p.plans.pid.critical_path.calendar_plan") : t("p.plans.pid.critical_path.calendar_default") })}
    {#if cpm.critical_chains.length > 1}
      {t("p.plans.pid.critical_path.critical_chains", { count: cpm.critical_chains.length, more: cpm.chains_truncated ? t("p.plans.pid.critical_path.more_not_shown") : "" })}
    {/if}
  </p>
  {#if (cpm.restricted_edges ?? []).length > 0}
    <p class="banner" role="note">{restrictedText(cpm.restricted_edges?.length ?? 0)}</p>
  {/if}
  {#if cpm.calendar_conflicts.length > 0}
    <p class="banner" role="alert">
      {t("p.plans.pid.critical_path.these_assignees_share_no_business", { join: cpm.calendar_conflicts.join(", ") })}
    </p>
  {/if}

  <div class="toolbar">
    <div role="group" aria-label={t("p.plans.pid.critical_path.view")}>
      <button type="button" aria-pressed={view === "gantt"} onclick={() => (view = "gantt")}>{t("p.plans.pid.critical_path.gantt")}</button>
      <button type="button" aria-pressed={view === "network"} onclick={() => (view = "network")}>{t("p.plans.pid.critical_path.network")}</button>
      <button type="button" aria-pressed={view === "table"} onclick={() => (view = "table")}>{t("p.plans.pid.critical_path.table")}</button>
    </div>
    <label>
      {t("p.plans.pid.critical_path.near_critical_within")}
      <input type="number" min="0" max="3650" bind:value={near} onchange={() => void load()} class="short" />
      {t("p.plans.pid.critical_path.business_days")}
    </label>
  </div>

  {#if view === "gantt"}
    <CpmGantt nodes={cpm.nodes} baseline={ghosts} {selected} onselect={select} />
  {:else if view === "network" && !diagramFits(cpm.nodes.length)}
    <p class="banner" role="note">{t("p.plans.pid.critical_path.this_network_has_tasks_too", { length: cpm.nodes.length })}</p>
    <CpmTable nodes={cpm.nodes} />
  {:else if view === "network"}
    <NetworkDiagram nodes={cpm.nodes} edges={cpm.edges} chains={cpm.critical_chains} {selected} onselect={select} />
  {:else}
    <CpmTable nodes={cpm.nodes} />
  {/if}

  <section aria-labelledby="bl-h" class="stack">
    <h2 id="bl-h">{t("p.plans.pid.critical_path.baselines")}</h2>
    <p class="small muted">
      {t("p.plans.pid.critical_path.a_baseline_is_a_numbered")}
    </p>
    <form onsubmit={takeBaseline} class="row">
      <label>{t("p.plans.pid.critical_path.why_a_baseline_now")} <input type="text" bind:value={baselineReason} required maxlength="500" /></label>
      <button type="submit">{t("p.plans.pid.critical_path.take_baseline")}</button>
    </form>
    {#if baselines.length > 0}
      <label>
        {t("p.plans.pid.critical_path.compare_with")}
        <select bind:value={chosen} onchange={() => void loadBaselines()}>
          {#each baselines as b (b.pid)}
            <option value={b.version}>{t("p.plans.pid.critical_path.version", { version: b.version, reason: b.reason, slice: b.created_at.slice(0, 10) })}</option>
          {/each}
        </select>
      </label>
      <label><input type="checkbox" bind:checked={showBaseline} /> {t("p.plans.pid.critical_path.show_the_baseline_under_each")}</label>
    {/if}
    {#if variance?.variance}
      <VarianceTable variance={variance.variance} />
    {:else if variance}
      <p class="muted">{t("p.plans.pid.critical_path.no_baseline_yet_so_nothing")}</p>
    {/if}
  </section>

  {#if cpm.critical_chains.length > 0}
    <h2>{t("p.plans.pid.critical_path.critical_chains")}</h2>
    <ol class="small">
      {#each cpm.critical_chains as chain, i (i)}
        <li>{chain.map((p) => titleOf.get(p) ?? "?").join(" → ")}</li>
      {/each}
    </ol>
  {/if}

  {#if cpm.unscheduled.length > 0}
    <h2>{t("p.plans.pid.critical_path.not_scheduled")}</h2>
    <ul class="small">
      {#each cpm.unscheduled as u (u.pid)}
        <li>
          <button type="button" class="link" onclick={() => select(u.pid)}>{u.title ?? t("p.plans.pid.critical_path.untitled")}</button>
          {unscheduledReason(u.reason)}
        </li>
      {/each}
    </ul>
  {/if}

  {#if selected}
    <section class="surface stack" aria-labelledby="edit-h">
      <h2 id="edit-h">{node?.title ?? unscheduledSelected?.title ?? t("p.plans.pid.critical_path.task")}</h2>
      {#if node}<p>{floatLabel(node)}.{node.pushed_days > 0 ? " " + t("p.plans.pid.critical_path.pushed_later", { days: node.pushed_days }) : ""}</p>{/if}
      <form onsubmit={save} class="stack">
        <label>{t("p.plans.pid.critical_path.planned_start")} <input type="date" bind:value={start} /></label>
        <label>{t("p.plans.pid.critical_path.duration_business_days_0_is")} <input type="number" min="0" max="3650" bind:value={duration} /></label>
        <label>
          {t("p.plans.pid.critical_path.constraint")}
          <select bind:value={ckind}>
            <option value="">{t("p.plans.pid.critical_path.none_as_soon_as_possible")}</option>
            <option value="snet">{t("p.plans.pid.critical_path.start_no_earlier_than")}</option>
            <option value="snlt">{t("p.plans.pid.critical_path.start_no_later_than")}</option>
            <option value="fnet">{t("p.plans.pid.critical_path.finish_no_earlier_than")}</option>
            <option value="fnlt">{t("p.plans.pid.critical_path.finish_no_later_than")}</option>
            <option value="mso">{t("p.plans.pid.critical_path.must_start_on_overrides_logic")}</option>
            <option value="mfo">{t("p.plans.pid.critical_path.must_finish_on_overrides_logic")}</option>
          </select>
        </label>
        {#if ckind}<label>{t("p.plans.pid.critical_path.constraint_date")} <input type="date" bind:value={cdate} required /></label>{/if}
        {#if hard}
          <label>
            {t("p.plans.pid.critical_path.reason_required_this_overrides_the")}
            <input type="text" bind:value={reason} required />
          </label>
        {/if}
        <label>{t("p.plans.pid.critical_path.deadline_a_goal_it_shows")} <input type="date" bind:value={deadline} /></label>
        <button class="button primary" type="submit" disabled={saving}>{saving ? t("p.plans.pid.critical_path.saving") : t("p.plans.pid.critical_path.save_schedule")}</button>
      </form>
      <form onsubmit={saveEstimate} class="stack" aria-labelledby="est-h">
        <h3 id="est-h">{t("p.plans.pid.critical_path.range_for_schedule_risk")}</h3>
        <p class="small muted">{t("p.plans.pid.critical_path.the_duration_above_is_the", { taskRangeText: taskRangeText(node?.duration_optimistic_days, node?.duration_days ?? null, node?.duration_pessimistic_days) })}</p>
        <label>{t("p.plans.pid.critical_path.optimistic_business_days")} <input type="number" min="0" max="3650" step="1" bind:value={optimistic} /></label>
        <label>{t("p.plans.pid.critical_path.pessimistic_business_days")} <input type="number" min="0" max="3650" step="1" bind:value={pessimistic} /></label>
        <button type="submit">{t("p.plans.pid.critical_path.save_range")}</button>
        {#each estimateProblem as p (p)}<p class="banner" role="alert">{p}</p>{/each}
        {#if estimateSaved}<p role="status" class="small">{t("p.plans.pid.critical_path.range_saved")}</p>{/if}
      </form>
      {#if result}
        <div role="status" data-testid="moved">
          {#if result.moved === null}
            <p>{result.note}</p>
          {:else if result.moved.length === 0}
            <p>{t("p.plans.pid.critical_path.saved_nothing_else_moved")}</p>
          {:else}
            <p>{t("p.plans.pid.critical_path.saved_moved")}</p>
            <ul>
              {#each result.moved as m (m.pid)}
                <li>
                  {t("p.plans.pid.critical_path.moved_line", {
                    title: titleOf.get(m.pid) ?? "?",
                    moved: m.moved === "both" ? t("p.plans.pid.critical_path.moved_both") : m.moved === "start" ? t("p.plans.pid.critical_path.moved_start") : t("p.plans.pid.critical_path.moved_finish"),
                    finish: m.finish_days !== 0 ? t("p.plans.pid.critical_path.moved_finish_by", { days: (m.finish_days > 0 ? "+" : "") + m.finish_days }) : "",
                    start: m.start_days !== 0 ? t("p.plans.pid.critical_path.moved_start_by", { days: (m.start_days > 0 ? "+" : "") + m.start_days }) : "",
                  })}
                </li>
              {/each}
            </ul>
          {/if}
        </div>
      {/if}
    </section>
  {/if}
{:else if !error}
  <p>{t("p.plans.pid.critical_path.loading")}</p>
{/if}

<style>
  .row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem 1rem;
    align-items: end;
  }
  .toolbar {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem 1.5rem;
    align-items: center;
    margin-block: 0.75rem;
  }
  .toolbar button[aria-pressed="true"] {
    font-weight: 700;
    text-decoration: underline;
  }
  .short {
    width: 4.5rem;
  }
  .link {
    background: none;
    border: 0;
    padding: 0;
    font: inherit;
    text-decoration: underline;
    cursor: pointer;
    color: inherit;
  }
  form label {
    display: flex;
    flex-direction: column;
    gap: 0.2rem;
    max-width: 28rem;
  }
</style>
