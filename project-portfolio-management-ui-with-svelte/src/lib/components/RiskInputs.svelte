<!--
  A register risk's inputs to the Monte Carlo analyses: its probability, the task
  it delays and by how many days (schedule risk), and the cost it adds (cost
  risk). One risk has ONE probability, shared by both: a risk with both effects
  is a single event that strikes once and drives both. Leave a part empty to
  leave it out; what the form would send is decided by `planRiskCalls`.
-->
<script lang="ts">
  import type { PpmClient, Risk, Task } from "#lib/api/ppm.js";
  import { formFrom, planRiskCalls, type RiskInputsForm } from "#lib/simulation-inputs.js";

  let {
    risk,
    pid,
    ppm,
    onchange,
    onerror,
  }: {
    risk: Risk;
    pid: string;
    ppm: PpmClient;
    onchange: () => void;
    onerror: (message: string) => void;
  } = $props();

  const current = $derived({
    task_pid: risk.task_pid ?? null,
    schedule_impact_days: risk.schedule_impact_days ?? null,
    probability_percent: risk.probability_percent ?? null,
    cost_impact_minor: risk.cost_impact_minor ?? null,
    cost_currency: risk.cost_currency ?? null,
  });
  let form = $state<RiskInputsForm>({ probability: "", taskPid: "", impactDays: "", cost: "", currency: "" });
  let tasks = $state<Task[] | null>(null);
  let problem = $state<string | null>(null);
  let saving = $state(false);

  // The form follows what is stored (and is reset only when that changes, never
  // when the section is opened: opening must not wipe what is already typed).
  $effect(() => {
    form = formFrom(current);
    problem = null;
  });

  async function open(e: Event) {
    if (!(e.currentTarget as HTMLDetailsElement).open || tasks !== null) return;
    try {
      tasks = (await ppm.listTasks(pid)).tasks;
    } catch (err) {
      onerror(err instanceof Error ? err.message : "Could not load the plan's tasks");
    }
  }

  async function save(e: SubmitEvent) {
    e.preventDefault();
    const plan = planRiskCalls(form, current);
    if ("error" in plan) {
      problem = plan.error;
      return;
    }
    problem = null;
    saving = true;
    try {
      // The schedule call goes first: it can clear the shared probability, which
      // the cost call then restores when it is wanted.
      if (plan.schedule) await ppm.setRiskScheduleImpact(pid, risk.pid, plan.schedule);
      if (plan.cost) await ppm.setRiskCostImpact(pid, risk.pid, plan.cost);
      onchange();
    } catch (err) {
      onerror(err instanceof Error ? err.message : "Could not save");
    } finally {
      saving = false;
    }
  }

  const summary = $derived.by(() => {
    const parts: string[] = [];
    if (current.probability_percent !== null) parts.push(`${current.probability_percent}% chance`);
    if (current.task_pid && current.schedule_impact_days) parts.push(`adds ${current.schedule_impact_days} days to a task`);
    if (current.cost_impact_minor !== null) parts.push(`adds ${(current.cost_impact_minor / 100).toFixed(2)} ${current.cost_currency}`);
    return parts.length === 0 ? "Not used in the simulations" : parts.join(" · ");
  });
</script>

<details class="risk-inputs" ontoggle={open}>
  <summary class="small">Simulation inputs <span class="muted">— {summary}</span></summary>
  <form class="inline" onsubmit={save}>
    <label>Probability (%) <input type="number" min="0" max="100" step="1" bind:value={form.probability} /></label>
    <label>Delays task
      <select bind:value={form.taskPid}>
        <option value="">None</option>
        {#each tasks ?? [] as t (t.pid)}<option value={t.pid}>{t.title}</option>{/each}
      </select>
    </label>
    <label>by (working days) <input type="number" min="1" max="365" step="1" bind:value={form.impactDays} /></label>
    <label>Adds cost <input type="number" min="0" step="0.01" bind:value={form.cost} /></label>
    <label>Currency <input type="text" maxlength="3" size="4" bind:value={form.currency} placeholder="GBP" /></label>
    <button class="button small" type="submit" disabled={saving}>{saving ? "Saving…" : "Save"}</button>
  </form>
  {#if problem}<p class="banner" role="alert">{problem}</p>{/if}
  <p class="small muted">One probability covers both effects: a risk that delays a task and adds cost strikes once and does both. Leave a part empty to leave it out.</p>
</details>

<style>
  .inline {
    display: flex;
    flex-wrap: wrap;
    gap: 0.6rem;
    align-items: end;
    margin: 0.4rem 0;
  }
  .inline label {
    display: grid;
    gap: 0.15rem;
    font-size: 0.8rem;
  }
</style>
