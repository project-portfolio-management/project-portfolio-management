<!--
  Approval processes (`/approval-processes`, PL-2; `/processes` is the plan-kind collection): define a process (an ordered list of approve, to-do and notify
  steps), start a run of one on a plan, task or other subject, and follow your own runs: which step each is
  on, what happened, and the buttons for the step that is yours (mark a to-do done), cancelling, and checking
  an approval again. The service decides who may do what; a refusal is shown as a message.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { t } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient, type ProcessDefinition, type ProcessRun } from "#lib/api/ppm.js";
  import {
    MAX_STEPS,
    STEP_KINDS,
    blankStep,
    definitionProblems,
    isRunning,
    kindLabel,
    runStatusLabel,
    stepStateLabel,
    type StepDraft,
  } from "#lib/processes-view.js";

  const ppm = PpmClient.withFetch();
  /** What a run can be about (the service's subject kinds that a person can name by id). */
  const SUBJECT_KINDS = ["plan", "task", "risk", "proposal", "idea", "canvas_item", "budget_line", "scenario", "document_version"] as const;

  let processes = $state<ProcessDefinition[] | null>(null);
  let runs = $state<ProcessRun[] | null>(null);
  let error = $state<string | null>(null);
  let notice = $state("");

  let name = $state("");
  let steps = $state<StepDraft[]>([blankStep()]);
  let problems = $state<string[]>([]);

  let startFor = $state<ProcessDefinition | null>(null);
  let subjectKind = $state<string>("plan");
  let subjectPid = $state("");

  let shown = $state<Record<string, ProcessRun>>({});

  async function load() {
    try {
      [processes, runs] = await Promise.all([ppm.listProcesses(), ppm.listProcessRuns()]);
      error = null;
    } catch (err) {
      error = errorText(err, "p.processes.could_not_load");
    }
  }
  onMount(load);

  /** Run a request, then reload; any refusal is shown, not thrown. */
  async function attempt(work: () => Promise<unknown>, done: string) {
    try {
      await work();
      notice = done;
    } catch (err) {
      notice = t("p.processes.could_not", { reason: err instanceof Error ? err.message : "" });
    }
    await load();
  }

  function create(e: SubmitEvent) {
    e.preventDefault();
    problems = definitionProblems(name, steps);
    if (problems.length > 0) return;
    const body = { name: name.trim(), steps: steps.map((s) => ({ ...s, person_ref: s.person_ref.trim(), title: s.title.trim() })) };
    void attempt(async () => {
      await ppm.createProcess(body);
      name = "";
      steps = [blankStep()];
    }, t("p.processes.defined", { name: body.name }));
  }

  function start(e: SubmitEvent) {
    e.preventDefault();
    const process = startFor;
    if (!process) return;
    void attempt(async () => {
      await ppm.startProcess(process.pid, { subject_kind: subjectKind, subject_pid: subjectPid.trim() });
      startFor = null;
      subjectPid = "";
    }, t("p.processes.started", { name: process.name }));
  }

  async function toggle(run: ProcessRun) {
    if (shown[run.pid]) {
      const { [run.pid]: _gone, ...rest } = shown;
      shown = rest;
      return;
    }
    try {
      shown = { ...shown, [run.pid]: await ppm.processRun(run.pid) };
    } catch (err) {
      notice = t("p.processes.could_not", { reason: err instanceof Error ? err.message : "" });
    }
  }

  /** Do something to a run, then show its fresh detail. */
  async function act(run: ProcessRun, work: () => Promise<unknown>) {
    await attempt(work, t("p.processes.done"));
    if (shown[run.pid]) shown = { ...shown, [run.pid]: await ppm.processRun(run.pid).catch(() => shown[run.pid]!) };
  }
</script>

<svelte:head><title>{t("p.processes.processes_ppm")}</title></svelte:head>

<h1>{t("p.processes.processes")}</h1>
<p class="small muted">{t("p.processes.intro")}</p>
{#if error}<p class="banner" role="alert">{error}</p>{/if}
<p class="small" role="status">{notice}</p>

<section aria-labelledby="defs-h">
  <h2 id="defs-h">{t("p.processes.definitions")}</h2>
  {#if processes === null}
    <p class="muted">…</p>
  {:else if processes.length === 0}
    <p>{t("p.processes.none_defined")}</p>
  {:else}
    <ul>
      {#each processes as p (p.pid)}
        <li>
          <strong>{p.name}</strong>
          <span class="small muted">({t("p.processes.version", { v: p.current_version })})</span>
          <button type="button" onclick={() => (startFor = p)}>{t("p.processes.start_heading", { name: p.name })}</button>
        </li>
      {/each}
    </ul>
  {/if}

  {#if startFor}
    <form onsubmit={start} aria-label={t("p.processes.start_heading", { name: startFor.name })}>
      <label>{t("p.processes.subject_kind")}
        <select bind:value={subjectKind}>{#each SUBJECT_KINDS as k (k)}<option value={k}>{k}</option>{/each}</select>
      </label>
      <label>{t("p.processes.subject_pid")} <input bind:value={subjectPid} required /></label>
      <button type="submit">{t("p.processes.start")}</button>
    </form>
  {/if}
</section>

<section aria-labelledby="new-h">
  <h2 id="new-h">{t("p.processes.new_process")}</h2>
  <form onsubmit={create} aria-labelledby="new-h">
    <label>{t("p.processes.name")} <input bind:value={name} maxlength="120" /></label>
    {#each steps as s, i (i)}
      <fieldset>
        <legend>{t("p.processes.step_n", { n: i + 1 })}</legend>
        <label>{t("p.processes.kind")}
          <select bind:value={s.kind}>{#each STEP_KINDS as k (k)}<option value={k}>{kindLabel(k)}</option>{/each}</select>
        </label>
        <label>{t("p.processes.person")} <input bind:value={s.person_ref} /></label>
        <label>{t("p.processes.step_title")} <input bind:value={s.title} maxlength="200" /></label>
        {#if steps.length > 1}
          <button type="button" onclick={() => (steps = steps.filter((_, j) => j !== i))}>{t("p.processes.remove_step", { n: i + 1 })}</button>
        {/if}
      </fieldset>
    {/each}
    {#if steps.length < MAX_STEPS}
      <button type="button" onclick={() => (steps = [...steps, blankStep()])}>{t("p.processes.add_step")}</button>
    {/if}
    {#if problems.length > 0}
      <ul class="banner" role="alert">{#each problems as m (m)}<li>{m}</li>{/each}</ul>
    {/if}
    <button type="submit">{t("p.processes.create")}</button>
  </form>
</section>

<section aria-labelledby="runs-h">
  <h2 id="runs-h">{t("p.processes.runs")}</h2>
  {#if runs === null}
    <p class="muted">…</p>
  {:else if runs.length === 0}
    <p>{t("p.processes.none_runs")}</p>
  {:else}
    <ul>
      {#each runs as run (run.pid)}
        {@const label = processes?.find((p) => p.pid === run.process_pid)?.name ?? run.process_pid}
        {@const detail = shown[run.pid]}
        <li>
          {t("p.processes.run_of", { name: label, kind: run.subject_kind, pid: run.subject_pid })}
          — {t("p.processes.status")}: <strong>{runStatusLabel(run.status)}</strong>
          <button type="button" aria-expanded={detail !== undefined} onclick={() => toggle(run)}>{detail ? t("p.processes.hide_run", { name: label }) : t("p.processes.show_run", { name: label })}</button>
          {#if detail}
            <div role="region" aria-label={t("p.processes.show_run", { name: label })}>
              <h3>{t("p.processes.steps")}</h3>
              <ol>
                {#each detail.steps ?? [] as step (step.index)}
                  <li>
                    {kindLabel(step.kind)}: {step.title} ({step.person_ref}) — {stepStateLabel(step.state)}
                    {#if step.kind === "todo" && step.state === "waiting"}
                      <button type="button" onclick={() => act(run, () => ppm.completeProcessStep(run.pid, step.index))}>{t("p.processes.complete_step", { n: step.index + 1 })}</button>
                    {/if}
                  </li>
                {/each}
              </ol>
              {#if isRunning(detail.status)}
                <button type="button" onclick={() => act(run, () => ppm.cancelProcessRun(run.pid))}>{t("p.processes.cancel")}</button>
                <button type="button" onclick={() => act(run, () => ppm.resumeProcessRun(run.pid))}>{t("p.processes.resume")}</button>
              {/if}
              <h3>{t("p.processes.log")}</h3>
              <ul>{#each detail.log ?? [] as entry, i (i)}<li class="small">{entry.at} — {entry.event}{entry.actor ? ` (${entry.actor})` : ""}{entry.note ? `: ${entry.note}` : ""}</li>{/each}</ul>
            </div>
          {/if}
        </li>
      {/each}
    </ul>
  {/if}
</section>

<style>
  form { display: grid; gap: 0.5rem; max-inline-size: 32rem; margin-block: 1rem; }
  fieldset { display: grid; gap: 0.5rem; }
  label { display: grid; gap: 0.25rem; }
</style>
