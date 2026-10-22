<!--
  Resources (`/resources`): demand against supply by role (or skill) and period,
  in FTE. The heatmap is a table: each cell says "Short 0.5 FTE", "Spare 1 FTE" or
  "Balanced" in words with a symbol, and only the shading's strength follows the
  size of the gap — colour is never the only signal. Skill rows are demand only
  (supply is not known for skills) and overlap. A caller who may not manage people
  sees roles and totals only. The whole of it is also available as a long table.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient, type ResourcePlan } from "#lib/api/ppm.js";
  import { gapCell, maxAbsGap, periodLabel } from "#lib/scheduling-view.js";

  const ppm = PpmClient.withFetch();
  let plan = $state<ResourcePlan | null>(null);
  let error = $state<string | null>(null);
  let from = $state("");
  let to = $state("");
  let period = $state<"week" | "month">("month");
  let groupBy = $state<"role" | "skill">("role");

  async function load() {
    try {
      plan = await ppm.resourcePlan({ from: from || undefined, to: to || undefined, period, group_by: groupBy });
      error = null;
    } catch (err) {
      error = errorText(err, "p.resources.could_not_load_the_resource");
    }
  }
  onMount(() => void load());

  const keys = $derived(
    [...new Set((plan?.periods ?? []).flatMap((p) => p.rows.map((r) => r.key)))].sort(),
  );
  const maxAbs = $derived(maxAbsGap((plan?.periods ?? []).flatMap((p) => p.rows)));

  type PRow = ResourcePlan["periods"][number]["rows"][number] & { period: string };
  const planRows = (plan: ResourcePlan): PRow[] => plan.periods.flatMap((p) => p.rows.map((r) => ({ ...r, period: periodLabel(p.from, p.to) })));
  const planColumns = (plan: ResourcePlan): DataGridColumn[] => [
    { id: "period", header: t("p.resources.period"), rowHeader: true },
    { id: "key", header: plan.group_by === "role" ? t("p.resources.role") : t("p.resources.skill") },
    { id: "demand_fte", header: t("p.resources.demand_fte"), align: "end" },
    { id: "placeholder_fte", header: t("p.resources.of_which_unassigned"), align: "end" },
    { id: "proposed_fte", header: t("p.resources.proposed_fte"), align: "end" },
    { id: "supply_fte", header: t("p.resources.supply_fte"), align: "end", format: (v) => (v ?? t("p.resources.not_known")) as string },
    { id: "gap_fte", header: t("p.resources.gap_fte"), align: "end", format: (v) => (v ?? t("p.resources.not_known")) as string },
    { id: "headcount", header: t("p.resources.people"), align: "end", format: (v) => (plan.group_by === "role" ? String(v) : "—") },
  ];
</script>

<svelte:head><title>{t("p.resources.resources_ppm")}</title></svelte:head>

<h1>{t("p.resources.resources")}</h1>
<p class="small muted">
  {t("p.resources.what_the_plans_ask_for")}
</p>

{#if error}<p class="banner" role="alert">{error}</p>{/if}

<form class="filter" onsubmit={(e) => { e.preventDefault(); void load(); }}>
  <label>{t("p.resources.from")} <input type="date" bind:value={from} /></label>
  <label>{t("p.resources.to")} <input type="date" bind:value={to} /></label>
  <label>{t("p.resources.per")}
    <select bind:value={period}>
      <option value="month">{t("p.resources.month")}</option>
      <option value="week">{t("p.resources.week")}</option>
    </select>
  </label>
  <label>{t("p.resources.by")}
    <select bind:value={groupBy}>
      <option value="role">{t("p.resources.role")}</option>
      <option value="skill">{t("p.resources.skill_demand_only")}</option>
    </select>
  </label>
  <button class="button small" type="submit">{t("p.resources.show")}</button>
</form>

{#if plan}
  <p class="small muted">
    {t("p.resources.per_by_scope", { periodLabel: periodLabel(plan.from, plan.to), period: plan.period, group_by: plan.group_by, kind: plan.scope.kind })}
    {#if plan.masked} · <strong>{t("p.resources.roles_and_totals_only")}</strong>{/if}
  </p>
  {#if plan.overlapping}
    <p class="banner" role="note">{t("p.resources.a_multi_skill_booking_counts")}</p>
  {/if}

  {#if keys.length === 0}
    <p>{t("p.resources.no_demand_or_supply_in")}</p>
  {:else}
    <table class="heat">
      <caption>{t("p.resources.gap_per_in_fte_supply", { period: plan.period })}</caption>
      <thead>
        <tr>
          <th scope="col">{plan.group_by === "role" ? t("p.resources.role") : t("p.resources.skill")}</th>
          {#each plan.periods as p (p.from)}<th scope="col">{periodLabel(p.from, p.to)}</th>{/each}
        </tr>
      </thead>
      <tbody>
        {#each keys as key (key)}
          <tr>
            <th scope="row">{key}</th>
            {#each plan.periods as p (p.from)}
              {@const cell = gapCell(p.rows.find((r) => r.key === key), maxAbs)}
              <td class={`cell ${cell.kind}`} style:--level={cell.level}>
                <span aria-hidden="true">{cell.symbol}</span> {cell.text}
              </td>
            {/each}
          </tr>
        {/each}
      </tbody>
    </table>
    <p class="small muted">{t("p.resources.short_spare_balanced_supply_not")}</p>

    <details>
      <summary>{t("p.resources.every_figure_as_a_table")}</summary>
      <DataTable label={t("p.resources.demand_supply_and_gap_by", { group_by: plan.group_by })} columns={planColumns(plan)} rows={planRows(plan)} rowId={(_r, i) => String(i)} sortable={false} pageSize={50} searchable />
    </details>
  {/if}

  {#if plan.placeholders.length > 0}
    <h2>{t("p.resources.roles_asked_for_with_no")}</h2>
    <ul>
      {#each plan.placeholders as ph (ph.allocation_pid)}
        <li>{ph.role ?? t("p.resources.unclassified")}: {ph.percent}% {ph.start_date ? t("p.resources.from_date", { date: ph.start_date }) : ""} {ph.end_date ? t("p.resources.to_date", { date: ph.end_date }) : ""} ({ph.booking})</li>
      {/each}
    </ul>
  {/if}
  <p class="small muted">{plan.note}</p>
{:else if !error}
  <p class="muted">{t("p.resources.loading")}</p>
{/if}

<style>
  table {
    border-collapse: collapse;
    width: 100%;
  }
  th,
  td {
    text-align: left;
    padding: 0.4rem 0.5rem;
    border-bottom: 1px solid var(--mxi-color-border, #ddd);
  }
  .cell {
    white-space: nowrap;
  }
  .cell.short {
    background: color-mix(in srgb, var(--danger, #b00020) calc(var(--level) * 35%), transparent);
    font-weight: 600;
  }
  .cell.spare {
    background: color-mix(in srgb, var(--info, #1f6feb) calc(var(--level) * 30%), transparent);
  }
  .filter {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: end;
    margin: 0.75rem 0;
  }
  .filter label {
    display: grid;
    gap: 0.2rem;
    font-size: 0.85rem;
  }
</style>
