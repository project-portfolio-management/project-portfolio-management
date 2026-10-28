<!--
  Costs (`/plans/[pid]/costs`): the planned cost by period (labour from the work
  profile at the rate in force each day, plus fixed and material expenses), the
  recorded labour cost, and the expense forms. Each currency stands alone and is
  never added to another. Work with no rate is listed, never counted as free.
  Rates are finance data: someone without access sees a plain note, not an error.
-->
<script lang="ts">
  import { rowOf } from "#lib/row-of.js";
  import { errorReason, errorText } from "#lib/error-text.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t, localeHref } from "#lib/i18n.svelte.js";
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
      const m = errorReason(err);
      // A 404 is how finance data hides from someone without access.
      if (err instanceof ApiError && err.status === 404) denied = true;
      else error = m || t("p.plans.pid.costs.could_not_load_the_costs");
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
    if (!taskPid) return void (status = t("p.plans.pid.costs.choose_a_task"));
    if (minor === null) return void (status = t("p.plans.pid.costs.the_amount_must_be_above"));
    if (!/^[A-Za-z]{3}$/.test(currency)) return void (status = t("p.plans.pid.costs.the_currency_must_be_a"));
    try {
      await ppm.addTaskCost(pid, taskPid, { kind, description, amount_minor: minor, currency: currency.toUpperCase(), accrual });
      description = amount = "";
      status = t("p.plans.pid.costs.expense_added");
      await load();
    } catch (err) {
      status = errorText(err, "p.plans.pid.costs.could_not_add_the_expense");
    }
  }
  async function record() {
    try {
      const r = await ppm.recordLabourCosts(pid);
      status = t(r.recorded === 1 ? "p.plans.pid.costs.recorded_one" : "p.plans.pid.costs.recorded_other", { recorded: r.recorded, already_recorded: r.already_recorded, length: r.no_rate.length });
      await load();
    } catch (err) {
      status = errorText(err, "p.plans.pid.costs.could_not_record");
    }
  }
  const label = (b: { from: string; to: string }) => (b.from === b.to ? b.from : `${b.from.slice(5)}–${b.to.slice(5)}`);

  type Cur = CostPlan["currencies"][number];
  type Measure = Cur["labour"];
  const costRows = (c: Cur) => [
    { name: "Labour", m: c.labour },
    { name: "Expenses", m: c.fixed },
    { name: "Total", m: c.total },
  ];
  const costColumns = (c: Cur): DataGridColumn[] => [
    { id: "name", header: "", rowHeader: true },
    ...c.total.buckets.map((b, i): DataGridColumn => ({
      id: `b${i}`, header: label(b), align: "end",
      accessor: (r) => (rowOf<{ m: Measure }>(r).m.buckets[i]?.minor ?? 0), format: (v) => (v ? money(Number(v)) : "–"),
    })),
    { id: "total", header: t("p.plans.pid.costs.total"), align: "end", accessor: (r) => rowOf<{ m: Measure }>(r).m.total_minor, format: (v) => money(Number(v)) },
  ];
</script>

<svelte:head><title>{t("p.plans.pid.costs.costs_ppm")}</title></svelte:head>

<h1>{t("p.plans.pid.costs.costs")}</h1>
<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plans.pid.costs.plan")}</a> · <a href={localeHref(`/plans/${pid}/work`)}>{t("p.plans.pid.costs.work")}</a> · <a href={localeHref("/cost-rates")}>{t("p.plans.pid.costs.cost_rates")}</a></p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}
{#if denied}
  <p class="muted" data-testid="denied">{t("p.plans.pid.costs.costs_and_rates_are_finance")}</p>
{:else}
  <h2>{t("p.plans.pid.costs.planned_cost")}</h2>
  {#if plan?.reason === "no_costs"}
    <p class="muted">{t("p.plans.pid.costs.nothing_could_be_costed_yet")}</p>
  {/if}
  {#each plan?.currencies ?? [] as c (c.currency)}
    <DataTable label={t("p.plans.pid.costs.per_period", { currency: c.currency })} columns={costColumns(c)} rows={costRows(c)} rowId={(r) => String(r.name)} sortable={false} testid={`cost-${c.currency}`} />
  {/each}
  {#each [...(plan?.unrated ?? []), ...(plan?.unassigned ?? [])] as u, i (i)}
    <p class="small">{t("p.plans.pid.costs.so_its_labour_is_not", { title: u.title, hoursText: hoursText(u.work_minutes), reason: NO_RATE_TEXT[u.reason] ?? u.reason })}</p>
  {/each}
  {#each plan?.unplaced ?? [] as u, i (i)}
    <p class="small">{t("p.plans.pid.costs.an_expense_of_so_it", { money: money(u.amount_minor, u.currency), reason: UNPLACED_TEXT[u.reason] ?? u.reason })}</p>
  {/each}

  <h2>{t("p.plans.pid.costs.recorded_labour_cost")}</h2>
  {#if actuals}
    {#each actuals.currencies as c (c.currency)}
      <p>{(c.entries === 1 ? t("p.plans.pid.costs.for_across_one", { money: money(c.cost_minor, c.currency), hoursText: hoursText(c.minutes), entries: c.entries }) : t("p.plans.pid.costs.for_across_other", { money: money(c.cost_minor, c.currency), hoursText: hoursText(c.minutes), entries: c.entries }))}</p>
    {/each}
    {#if actuals.currencies.length === 0}<p class="muted">{t("p.plans.pid.costs.no_time_has_been_costed")}</p>{/if}
    {#if actuals.unpriced.entries}
      <p class="small">{t("p.plans.pid.costs.time_entries_are_not_costed", { entries: actuals.unpriced.entries, hoursText: hoursText(actuals.unpriced.minutes) })}</p>
    {/if}
  {/if}
  <button class="button" type="button" onclick={record}>{t("p.plans.pid.costs.record_labour_costs")}</button>
  <p class="small muted">{t("p.plans.pid.costs.a_recorded_cost_keeps_the")}</p>

  <h2>{t("p.plans.pid.costs.add_an_expense")}</h2>
  <form class="surface stack" onsubmit={addExpense}>
    <label>{t("p.plans.pid.costs.task")}
      <select bind:value={taskPid}>
        <option value="">{t("p.plans.pid.costs.choose")}</option>
        {#each nodes as n (n.pid)}<option value={n.pid}>{n.title ?? n.pid}</option>{/each}
      </select>
    </label>
    <label>{t("p.plans.pid.costs.kind")}
      <select bind:value={kind}><option value="material">{t("p.plans.pid.costs.material")}</option><option value="fixed">{t("p.plans.pid.costs.fixed_cost")}</option></select>
    </label>
    <label>{t("p.plans.pid.costs.description")} <input bind:value={description} required maxlength="200" /></label>
    <label>{t("p.plans.pid.costs.amount")} <input type="number" min="0" step="0.01" bind:value={amount} /></label>
    <label>{t("p.plans.pid.costs.currency")} <input bind:value={currency} maxlength="3" size="4" /></label>
    <label>{t("p.plans.pid.costs.falls")}
      <select bind:value={accrual}>
        <option value="start">{t("p.plans.pid.costs.all_at_the_start")}</option>
        <option value="prorated">{t("p.plans.pid.costs.evenly_across_the_task")}</option>
        <option value="end">{t("p.plans.pid.costs.all_at_the_end")}</option>
      </select>
    </label>
    <button class="button primary" type="submit">{t("p.plans.pid.costs.add_expense")}</button>
  </form>
  {#if status}<p role="status" class="small">{status}</p>{/if}
{/if}

<style>
  label {
    display: block;
  }
</style>
