<!--
  Costs (`/plans/[pid]/costs`): the planned cost by period (labour from the work
  profile at the rate in force each day, plus fixed and material expenses), the
  recorded labour cost, and the expense forms. Each currency stands alone and is
  never added to another. Work with no rate is listed, never counted as free.
  Rates are finance data: someone without access sees a plain note, not an error.
-->
<script lang="ts">
  import { localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { ApiError } from "#lib/api/client.js";
  import { PpmClient, money, type CostPlan, type CpmNode, type LabourCosts } from "#lib/api/ppm.js";
  import { NO_RATE_TEXT, UNPLACED_TEXT, hoursText, minorFromMajor } from "#lib/work-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");
  let plan = $state<CostPlan | null>(null);
  let actuals = $state<LabourCosts | null>(null);
  let nodes = $state<CpmNode[]>([]);
  let denied = $state(false);
  let error = $state<string | null>(null);
  let status = $state("");

  async function load() {
    try {
      nodes = (await ppm.planCpm(pid)).nodes;
      plan = await ppm.costPlan(pid);
      actuals = await ppm.labourCosts(pid);
      error = null;
    } catch (err) {
      const m = err instanceof Error ? err.message : "";
      // A 404 is how finance data hides from someone without access.
      if (err instanceof ApiError && err.status === 404) denied = true;
      else error = m || "Could not load the costs";
    }
  }
  onMount(load);

  let taskPid = $state("");
  let kind = $state<"material" | "fixed">("material");
  let description = $state("");
  let amount = $state("");
  let currency = $state("GBP");
  let accrual = $state<"start" | "prorated" | "end">("prorated");
  async function addExpense(e: SubmitEvent) {
    e.preventDefault();
    const minor = minorFromMajor(amount);
    if (!taskPid) return void (status = "Choose a task.");
    if (minor === null) return void (status = "The amount must be above zero.");
    if (!/^[A-Za-z]{3}$/.test(currency)) return void (status = "The currency must be a 3-letter code such as GBP.");
    try {
      await ppm.addTaskCost(pid, taskPid, { kind, description, amount_minor: minor, currency: currency.toUpperCase(), accrual });
      description = amount = "";
      status = "Expense added.";
      await load();
    } catch (err) {
      status = err instanceof Error ? err.message : "Could not add the expense";
    }
  }
  async function record() {
    try {
      const r = await ppm.recordLabourCosts(pid);
      status = `Recorded ${r.recorded} time ${r.recorded === 1 ? "entry" : "entries"}; ${r.already_recorded} were already recorded; ${r.no_rate.length} have no rate.`;
      await load();
    } catch (err) {
      status = err instanceof Error ? err.message : "Could not record";
    }
  }
  const label = (b: { from: string; to: string }) => (b.from === b.to ? b.from : `${b.from.slice(5)}–${b.to.slice(5)}`);
</script>

<svelte:head><title>Costs — PPM</title></svelte:head>

<h1>Costs</h1>
<p class="small"><a href={localeHref(`/plans/${pid}`)}>← Plan</a> · <a href={localeHref(`/plans/${pid}/work`)}>Work</a> · <a href={localeHref("/cost-rates")}>Cost rates</a></p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}
{#if denied}
  <p class="muted" data-testid="denied">Costs and rates are finance data. You do not have access to them.</p>
{:else}
  <h2>Planned cost</h2>
  {#if plan?.reason === "no_costs"}
    <p class="muted">Nothing could be costed yet: add rates for the people doing the work, or add an expense.</p>
  {/if}
  {#each plan?.currencies ?? [] as c (c.currency)}
    <table data-testid="cost-{c.currency}">
      <caption class="small muted">{c.currency}, per period</caption>
      <thead><tr><th scope="col"></th>{#each c.total.buckets as b}<th scope="col">{label(b)}</th>{/each}<th scope="col">Total</th></tr></thead>
      <tbody>
        {#each [["Labour", c.labour], ["Expenses", c.fixed], ["Total", c.total]] as [name, m]}
          <tr>
            <th scope="row">{name}</th>
            {#each (m as typeof c.labour).buckets as b}<td>{b.minor ? money(b.minor) : "–"}</td>{/each}
            <td>{money((m as typeof c.labour).total_minor)}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/each}
  {#each [...(plan?.unrated ?? []), ...(plan?.unassigned ?? [])] as u}
    <p class="small">▲ {u.title} ({hoursText(u.work_minutes)}): {NO_RATE_TEXT[u.reason] ?? u.reason}, so its labour is not costed.</p>
  {/each}
  {#each plan?.unplaced ?? [] as u}
    <p class="small">▲ An expense of {money(u.amount_minor, u.currency)} {UNPLACED_TEXT[u.reason] ?? u.reason}, so it is not in the periods.</p>
  {/each}

  <h2>Recorded labour cost</h2>
  {#if actuals}
    {#each actuals.currencies as c (c.currency)}
      <p>{money(c.cost_minor, c.currency)} for {hoursText(c.minutes)} across {c.entries} {c.entries === 1 ? "entry" : "entries"}.</p>
    {/each}
    {#if actuals.currencies.length === 0}<p class="muted">No time has been costed yet.</p>{/if}
    {#if actuals.unpriced.entries}
      <p class="small">▲ {actuals.unpriced.entries} time entries ({hoursText(actuals.unpriced.minutes)}) are not costed: not recorded yet, or no rate. Their cost is unknown, not zero.</p>
    {/if}
  {/if}
  <button class="button" type="button" onclick={record}>Record labour costs</button>
  <p class="small muted">A recorded cost keeps the rate it was priced at; a later rate change never rewrites it.</p>

  <h2>Add an expense</h2>
  <form class="surface stack" onsubmit={addExpense}>
    <label>Task
      <select bind:value={taskPid}>
        <option value="">Choose…</option>
        {#each nodes as n (n.pid)}<option value={n.pid}>{n.title ?? n.pid}</option>{/each}
      </select>
    </label>
    <label>Kind
      <select bind:value={kind}><option value="material">Material</option><option value="fixed">Fixed cost</option></select>
    </label>
    <label>Description <input bind:value={description} required maxlength="200" /></label>
    <label>Amount <input type="number" min="0" step="0.01" bind:value={amount} /></label>
    <label>Currency <input bind:value={currency} maxlength="3" size="4" /></label>
    <label>Falls
      <select bind:value={accrual}>
        <option value="start">All at the start</option>
        <option value="prorated">Evenly across the task</option>
        <option value="end">All at the end</option>
      </select>
    </label>
    <button class="button primary" type="submit">Add expense</button>
  </form>
  {#if status}<p role="status" class="small">{status}</p>{/if}
{/if}

<style>
  label {
    display: block;
  }
</style>
