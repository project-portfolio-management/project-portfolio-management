<!--
  Governance panel (`/[collection]/[pid]/governance`): the PPM view of
  one work item — the summary strip (stage / risk posture / budget
  variance), the gate journey, risks, budget lines, benefits + ROI,
  OKR mappings, milestones, and allocations, each with its minimal
  add-form. Every action maps 1:1 to a service endpoint.
-->
<script lang="ts">
  import { rowOf } from "#lib/row-of.js";
  import { errorReason, errorText } from "#lib/error-text.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import RiskInputs from "#lib/components/RiskInputs.svelte";
  import LineRange from "#lib/components/LineRange.svelte";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { formatDate, formatDecimal, formatPercentValue } from "#lib/locale-format.js";
  import { currencyCode, minorToInput, toMinor } from "#lib/money.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { ApiError } from "#lib/api/client.js";
  import { isOverCapacityConflict } from "#lib/capacity-view.js";
  import {
    PpmClient,
    money,
    type Allocation,
    type BenefitBoard,
    type BudgetBoard,
    type GateJourney,
    type GovernanceSummary,
    type Milestone,
    type Objective,
    type Risk,
  } from "#lib/api/ppm.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");

  let summary = $state<GovernanceSummary | null>(null);
  let journey = $state<GateJourney | null>(null);
  let risks = $state<Risk[]>([]);
  let budget = $state<BudgetBoard | null>(null);
  let benefits = $state<BenefitBoard | null>(null);
  let objectives = $state<Objective[]>([]);
  let mappings = $state<{ objective_pid: string; title: string; weight: number }[]>([]);
  let milestones = $state<Milestone[]>([]);
  let allocations = $state<Allocation[]>([]);
  let error = $state<string | null>(null);

  // Add-form state (one small form per section).
  let gateDecision = $state("approved");
  let riskTitle = $state("");
  let riskProbability = $state(3);
  let riskImpact = $state(3);
  let lineDescription = $state("");
  let lineCategory = $state("capex");
  let lineCurrency = $state("");
  let benefitCurrency = $state("");
  let linePlanned = $state("");
  let benefitTitle = $state("");
  let benefitCategory = $state("cost_saving");
  let benefitTarget = $state("");
  let mapObjective = $state("");
  let mapWeight = $state(3);
  let milestoneName = $state("");
  let milestoneDue = $state("");
  let allocPerson = $state("");
  let allocPercent = $state(50);
  let allocProposed = $state(false);

  async function refresh() {
    try {
      [summary, journey, risks, budget, benefits, mappings, milestones, allocations, objectives] =
        await Promise.all([
          ppm.governance(pid),
          ppm.gateJourney(pid),
          ppm.listRisks(pid),
          ppm.budget(pid),
          ppm.benefits(pid),
          ppm.itemObjectives(pid),
          ppm.milestones(pid),
          ppm.allocations(pid),
          ppm.listObjectives(),
        ]);
      error = null;
    } catch (err) {
      error = errorText(err, "ppm.common.loadFailed");
    }
  }
  onMount(refresh);

  async function act(action: () => Promise<unknown>) {
    error = null;
    try {
      await action();
      await refresh();
    } catch (err) {
      error = errorText(err, "ppm.common.actionFailed");
    }
  }

  // PJ-4: committing a proposed booking; a 409 over_capacity offers "Commit anyway".
  let overCapacity = $state<string | null>(null);
  async function commit(allocationPid: string, confirm: boolean) {
    error = null;
    try {
      await ppm.commitAllocation(pid, allocationPid, confirm);
      overCapacity = null;
      await refresh();
    } catch (err) {
      if (err instanceof ApiError && isOverCapacityConflict(err.status, err.body)) {
        overCapacity = allocationPid;
        error = errorReason(err);
      } else {
        error = errorText(err, "ppm.common.actionFailed");
      }
    }
  }

  type BudgetLine = BudgetBoard["lines"][number];
  type BenefitRow = BenefitBoard["benefits"][number];
  const riskColumns: DataGridColumn[] = $derived([
    { id: "title", header: t("p.plans.pid.governance.risk"), rowHeader: true, cell: riskCell },
    { id: "exposure", header: "P×I", align: "end", cell: exposureCell },
    { id: "status", header: t("p.plans.pid.governance.status"), cell: riskStatus },
    { id: "pid", header: "", sortable: false, cell: riskActions },
  ]);
  const lineColumns: DataGridColumn[] = $derived([
    { id: "description", header: t("p.plans.pid.governance.line"), rowHeader: true, cell: lineCell },
    { id: "planned_minor", header: t("p.plans.pid.governance.planned"), align: "end", format: (v, r) => money(Number(v), String(r.currency)) },
    { id: "range", header: t("p.plans.pid.governance.range_for_cost_risk"), sortable: false, accessor: () => "", cell: rangeCell },
    { id: "actual_minor", header: t("p.plans.pid.governance.actual"), align: "end", format: (v, r) => money(Number(v), String(r.currency)) },
    { id: "record", header: t("p.plans.pid.governance.record_actual"), sortable: false, accessor: () => "", cell: recordCell },
  ]);
  const benefitColumns: DataGridColumn[] = $derived([
    { id: "title", header: t("p.plans.pid.governance.benefit"), rowHeader: true, cell: benefitCell },
    { id: "target", header: t("p.plans.pid.governance.target"), align: "end", accessor: (r) => { const b = rowOf<BenefitRow>(r); return b.target_minor !== null ? money(b.target_minor, b.currency ?? "") : (b.target_note ?? "—"); } },
    { id: "realized_minor", header: t("p.plans.pid.governance.realized"), align: "end", format: (v, r) => money(Number(v), String(r.currency)) },
    { id: "status", header: t("p.plans.pid.governance.status"), cell: chip },
    { id: "realize", header: "", sortable: false, accessor: () => "", cell: realizeCell },
  ]);
</script>

{#snippet riskCell({ row }: DataGridCellContext)}
  {@const risk = rowOf<Risk>(row)}
  {risk.title}{#if risk.mitigation}<div class="small muted">{risk.mitigation}</div>{/if}
  <RiskInputs {risk} {pid} {ppm} onchange={refresh} onerror={(m) => (error = m)} />
{/snippet}

{#snippet exposureCell({ row }: DataGridCellContext)}
  {@const risk = rowOf<Risk>(row)}
  {risk.probability}×{risk.impact} = <strong>{risk.exposure}</strong>
{/snippet}

{#snippet riskStatus({ row, formatted }: DataGridCellContext)}<span class="chip" class:red={row.status === "materialized"}>{formatted}</span>{/snippet}

{#snippet chip({ formatted }: DataGridCellContext)}<span class="chip">{formatted}</span>{/snippet}

{#snippet riskActions({ row }: DataGridCellContext)}
  {@const risk = rowOf<Risk>(row)}
  {#if risk.status === "open" || risk.status === "mitigating"}
    <button class="button danger small" onclick={() => act(() => ppm.escalateRisk(pid, risk.pid))}>{t("ppm.gov.escalate")}</button>
  {/if}
{/snippet}

{#snippet lineCell({ row, formatted }: DataGridCellContext)}{formatted} <span class="chip">{row.category}</span>{/snippet}

{#snippet rangeCell({ row }: DataGridCellContext)}
  <LineRange line={rowOf<BudgetLine>(row)} {pid} {ppm} onchange={refresh} onerror={(m) => (error = m)} />
{/snippet}

{#snippet recordCell({ row }: DataGridCellContext)}
  {@const line = rowOf<BudgetLine>(row)}
  <form
    class="row"
    onsubmit={(e) => {
      e.preventDefault();
      const input = (e.currentTarget as HTMLFormElement).elements.namedItem("amount") as HTMLInputElement;
      const minor = toMinor(input.value, line.currency);
      if (minor !== null) {
        act(() => ppm.recordActual(pid, line.pid, minor));
        input.value = "";
      }
    }}
  >
    <input name="amount" size="8" inputmode="decimal" placeholder={minorToInput(0, line.currency)} />
    <button class="button small" type="submit">+</button>
  </form>
{/snippet}

{#snippet benefitCell({ row, formatted }: DataGridCellContext)}{formatted} <span class="chip">{row.category}</span>{/snippet}

{#snippet realizeCell({ row }: DataGridCellContext)}
  {@const benefit = rowOf<BenefitRow>(row)}
  <form
    class="row"
    onsubmit={(e) => {
      e.preventDefault();
      const input = (e.currentTarget as HTMLFormElement).elements.namedItem("realize") as HTMLInputElement;
      const minor = toMinor(input.value, benefit.currency ?? "");
      if (minor !== null) {
        act(() => ppm.realizeBenefit(pid, benefit.pid, { amount_minor: minor, status: "on_track" }));
        input.value = "";
      }
    }}
  >
    <input name="realize" size="8" inputmode="decimal" placeholder={minorToInput(0, benefit.currency ?? "")} />
    <button class="button small" type="submit">{t("ppm.gov.realize")}</button>
  </form>
{/snippet}


<svelte:head><title>{t("p.plans.pid.governance.governance_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref(`/plans/`)}>{t("ppm.gov.backToItem")}</a></p>
<h1>{t("ppm.gov.title")}{#if summary} — {summary.name}{/if}</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if summary}
  <section class="strip" data-testid="governance-strip">
    <span class="chip">{t("p.plans.pid.governance.stage", { v: summary.stage ?? t("p.common.pre_gate") })}</span>
    {#if summary.next_gate}<span class="chip">{t("p.plans.pid.governance.next", { next_gate: summary.next_gate })}</span>{/if}
    <span class="chip">{t("p.plans.pid.governance.open_risks", { open: summary.risks.open })}</span>
    {#if summary.risks.materialized > 0}
      <span class="chip red">{t("p.plans.pid.governance.materialized", { materialized: summary.risks.materialized })}</span>
    {/if}
    {#if summary.risks.max_exposure !== null}
      <span class="chip">{t("p.plans.pid.governance.max_exposure", { max_exposure: summary.risks.max_exposure })}</span>
    {/if}
    {#each summary.budget as total (total.currency)}
      <span class="chip" class:red={total.variance_minor < 0}>
        {total.currency}: {money(total.actual_minor, total.currency)} / {money(total.planned_minor, total.currency)}
      </span>
    {/each}
    <a class="button small" href={localeHref(`/plans/${pid}/schedule`)}>{t("ppm.common.schedule")}</a>
  </section>
{/if}

<details open>
  <summary><h2>{t("ppm.gov.gates")}</h2></summary>
  {#if journey}
    <p class="small">
      {#each journey.reviews as review (review.pid)}
        <span class="chip" class:green={review.decision.startsWith("approved")}>
          {review.gate}: {review.decision}
        </span>
      {/each}
    </p>
    {#if journey.next_gate}
      <form
        class="row"
        onsubmit={(e) => {
          e.preventDefault();
          act(() =>
            ppm.reviewGate(pid, {
              gate: journey?.next_gate ?? "",
              decision: gateDecision,
            }),
          );
        }}
      >
        <span class="small">{t("p.plans.pid.governance.review")} <strong>{journey.next_gate}</strong>:</span>
        <select bind:value={gateDecision} aria-label={t("p.plans.pid.governance.decision")}>
          <option>{t("p.plans.pid.governance.approved")}</option>
          <option>{t("p.plans.pid.governance.approved_with_conditions")}</option>
          <option>{t("p.plans.pid.governance.hold")}</option>
          <option>{t("p.plans.pid.governance.rejected")}</option>
        </select>
        <button class="button primary small" type="submit">{t("ppm.gov.recordDecision")}</button>
      </form>
    {:else}
      <p class="small muted">{t("ppm.gov.journeyComplete")}</p>
    {/if}
  {/if}
</details>

<details open>
  <summary><h2>{t("ppm.gov.risks")}</h2></summary>
    <DataTable label={t("p.plans.pid.governance.risk")} columns={riskColumns} rows={risks} rowId={(r) => String(r.pid)} />
  <form
    class="row"
    onsubmit={(e) => {
      e.preventDefault();
      act(() =>
        ppm.createRisk(pid, {
          title: riskTitle,
          probability: riskProbability,
          impact: riskImpact,
        }),
      ).then(() => (riskTitle = ""));
    }}
  >
    <input placeholder={t("p.plans.pid.governance.new_risk")} bind:value={riskTitle} required />
    <label class="small">P <input type="number" min="1" max="5" bind:value={riskProbability} /></label>
    <label class="small">I <input type="number" min="1" max="5" bind:value={riskImpact} /></label>
    <button class="button small" type="submit">{t("ppm.gov.raise")}</button>
  </form>
</details>

<details open>
  <summary><h2>{t("ppm.gov.budget")}</h2></summary>
  {#if budget}
    <DataTable label={t("p.plans.pid.governance.line")} columns={lineColumns} rows={budget.lines} rowId={(r) => String(r.pid)} />
    {#each budget.totals as total (total.currency)}
      <p class="small">
        <strong>{total.currency}</strong>{t("p.plans.pid.governance.planned_actual_variance", { money: money(total.planned_minor, total.currency), money2: money(total.actual_minor, total.currency), money3: money(total.variance_minor, total.currency) })}
      </p>
    {/each}
  {/if}
  <form
    class="row"
    onsubmit={(e) => {
      e.preventDefault();
      const code = currencyCode(lineCurrency);
      const minor = code === null ? null : toMinor(linePlanned, code);
      if (code === null || minor === null) {
        error = t("p.money.amount_or_currency");
        return;
      }
      act(() =>
        ppm.createBudgetLine(pid, {
          category: lineCategory,
          description: lineDescription,
          currency: code,
          planned_minor: minor,
        }),
      ).then(() => {
        lineDescription = "";
        linePlanned = "";
      });
    }}
  >
    <input placeholder={t("p.plans.pid.governance.new_budget_line")} bind:value={lineDescription} required />
    <select bind:value={lineCategory} aria-label={t("p.plans.pid.governance.category")}><option>{t("p.plans.pid.governance.capex")}</option><option>{t("p.plans.pid.governance.opex")}</option></select>
    <input bind:value={lineCurrency} required maxlength="3" size="4" aria-label={t("p.plans.pid.governance.currency")} placeholder={t("p.money.currency_hint")} />
    <input placeholder={t("p.plans.pid.governance.planned_major")} bind:value={linePlanned} inputmode="decimal" size="10" required />
    <button class="button small" type="submit">{t("ppm.gov.addLine")}</button>
  </form>
</details>

<details open>
  <summary><h2>{t("ppm.gov.benefits")}</h2></summary>
  {#if benefits}
    <DataTable label={t("p.plans.pid.governance.benefit")} columns={benefitColumns} rows={benefits.benefits} rowId={(r) => String(r.pid)} />
    {#each benefits.totals as total (total.currency)}
      <p class="small">
        <strong>{total.currency}</strong>{t("p.plans.pid.governance.target_realized_spend", { money: money(total.target_minor, total.currency), money2: money(total.realized_minor, total.currency), money3: money(total.spend_minor, total.currency) })}
        {#if total.roi_basis_points !== null}
          {t("p.plans.pid.governance.roi", { toFixed: formatDecimal(total.roi_basis_points / 100, 1) })}
        {/if}
      </p>
    {/each}
  {/if}
  <form
    class="row"
    onsubmit={(e) => {
      e.preventDefault();
      const code = currencyCode(benefitCurrency);
      let minor: number | undefined;
      if (benefitTarget.trim()) {
        const m = code === null ? null : toMinor(benefitTarget, code);
        if (code === null || m === null) {
          error = t("p.money.amount_or_currency");
          return;
        }
        minor = m;
      }
      act(() =>
        ppm.createBenefit(pid, {
          title: benefitTitle,
          category: benefitCategory,
          ...(minor !== undefined && code !== null
            ? { currency: code, target_minor: minor }
            : { target_note: "qualitative" }),
        }),
      ).then(() => {
        benefitTitle = "";
        benefitTarget = "";
      });
    }}
  >
    <input placeholder={t("p.plans.pid.governance.new_benefit")} bind:value={benefitTitle} required />
    <select bind:value={benefitCategory} aria-label={t("p.plans.pid.governance.benefit_category")}>
      <option>{t("p.plans.pid.governance.cost_saving")}</option><option>{t("p.plans.pid.governance.revenue")}</option><option>{t("p.plans.pid.governance.risk_reduction")}</option>
      <option>{t("p.plans.pid.governance.quality")}</option><option>{t("p.plans.pid.governance.compliance")}</option><option>{t("p.plans.pid.governance.other")}</option>
    </select>
    <input placeholder={t("p.plans.pid.governance.target_major")} bind:value={benefitTarget} inputmode="decimal" size="10" />
    <input bind:value={benefitCurrency} maxlength="3" size="4" aria-label={t("p.plans.pid.governance.currency")} placeholder={t("p.money.currency_hint")} />
    <button class="button small" type="submit">{t("ppm.gov.declare")}</button>
  </form>
</details>

<details>
  <summary><h2>{t("ppm.gov.objectives")}</h2></summary>
  <p class="small">
    {#each mappings as mapping (mapping.objective_pid)}
      <span class="chip">{mapping.title} (w{mapping.weight})</span>
    {/each}
    {#if mappings.length === 0}<span class="muted">{t("ppm.gov.noMappings")}</span>{/if}
  </p>
  <form
    class="row"
    onsubmit={(e) => {
      e.preventDefault();
      if (mapObjective) act(() => ppm.linkObjective(pid, mapObjective, mapWeight));
    }}
  >
    <select bind:value={mapObjective} aria-label={t("p.plans.pid.governance.objective")}>
      <option value="">{t("p.plans.pid.governance.choose_objective")}</option>
      {#each objectives as objective (objective.pid)}
        <option value={objective.pid}>{objective.title}</option>
      {/each}
    </select>
    <label class="small">{t("p.plans.pid.governance.weight")} <input type="number" min="1" max="5" bind:value={mapWeight} /></label>
    <button class="button small" type="submit">{t("ppm.gov.map")}</button>
  </form>
</details>

<details>
  <summary><h2>{t("ppm.gov.milestones")}</h2></summary>
  <ul>
    {#each milestones as milestone (milestone.pid)}
      <li class="small">
        {milestone.name} — {formatDate(milestone.due)}
        {#if milestone.done}<span class="chip green">{t("ppm.gov.done")}</span>
        {:else if milestone.overdue}<span class="chip red">{t("ppm.gov.overdue")}</span>
        {:else}
          <button class="button small" onclick={() => act(() => ppm.completeMilestone(pid, milestone.pid))}>
            {t("ppm.gov.complete")}
          </button>
        {/if}
      </li>
    {/each}
  </ul>
  <form
    class="row"
    onsubmit={(e) => {
      e.preventDefault();
      act(() => ppm.createMilestone(pid, { name: milestoneName, due: milestoneDue })).then(
        () => (milestoneName = ""),
      );
    }}
  >
    <input placeholder={t("p.plans.pid.governance.new_milestone")} bind:value={milestoneName} required />
    <input type="date" bind:value={milestoneDue} required />
    <button class="button small" type="submit">{t("ppm.gov.add")}</button>
  </form>
</details>

<details>
  <summary><h2>{t("ppm.gov.allocations")}</h2></summary>
  <ul>
    {#each allocations as allocation (allocation.pid)}
      <li class="small">
        {allocation.person_ref} — {formatPercentValue(allocation.percent)}
        {#if allocation.role}({allocation.role}){/if}
        {#if allocation.booking === "proposed"}
          <span class="chip">{t("p.plans.pid.governance.proposed_not_counted", { requested: allocation.requested ? t("p.plans.pid.governance.requested") : "" })}</span>
          <button class="button small" type="button" onclick={() => void commit(allocation.pid, false)}>{t("p.plans.pid.governance.commit")}</button>
          {#if overCapacity === allocation.pid}
            <span role="alert">
              {t("p.plans.pid.governance.committing_this_puts_them_over")}
              <button class="button small" type="button" onclick={() => void commit(allocation.pid, true)}>{t("p.plans.pid.governance.commit_anyway")}</button>
            </span>
          {/if}
        {/if}
      </li>
    {/each}
  </ul>
  <form
    class="row"
    onsubmit={(e) => {
      e.preventDefault();
      act(() =>
        ppm.createAllocation(pid, {
          person_ref: allocPerson,
          percent: allocPercent,
          booking: allocProposed ? "proposed" : "committed",
        }),
      ).then(() => (allocPerson = ""));
    }}
  >
    <input placeholder={t("p.plans.pid.governance.worker_uuid")} bind:value={allocPerson} size="42" required />
    <label class="small">% <input type="number" min="1" max="100" bind:value={allocPercent} /></label>
    <label class="small"><input type="checkbox" bind:checked={allocProposed} /> {t("p.plans.pid.governance.proposed_does_not_count_until")}</label>
    <button class="button small" type="submit">{t("ppm.gov.allocate")}</button>
  </form>
</details>

<style>
  .strip { display: flex; gap: 0.4rem; flex-wrap: wrap; align-items: center; margin: 0.6rem 0 1.2rem; }
  .row { display: flex; gap: 0.5rem; flex-wrap: wrap; align-items: center; margin: 0.5rem 0; }
  .chip {
    display: inline-block;
    border: 1px solid var(--border, #ccc);
    border-radius: 999px;
    padding: 0 0.55rem;
    font-size: 0.8rem;
  }
  .chip.red { color: #a4262c; border-color: #a4262c; }
  .chip.green { color: #1d8a4e; border-color: #1d8a4e; }
  details > summary { cursor: pointer; }
  details > summary h2 { display: inline; font-size: 1.05rem; }
  input[type="number"] { width: 4rem; }
</style>
