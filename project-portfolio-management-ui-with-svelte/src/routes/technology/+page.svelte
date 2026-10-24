<!--
  CTO technology area (`/technology`): the dependency-risk lens (top
  fan-out items, cross-portfolio edges, edges with a RAG-red
  predecessor) and the tag-encoded technology radar
  (`tech:<name>[:<ring>]`; the server's convention string is displayed
  verbatim). All derivations are server-side. English-first, like the
  other PPM views.
-->
<script lang="ts">
  import { rowOf } from "#lib/row-of.js";
  import { errorText } from "#lib/error-text.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import Term from "#lib/components/Term.svelte";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import {
    PpmClient,
    type DependencyRisk,
    type TechnologyRadar,
    type TechDebtRegister,
    type FlowMetrics,
  } from "#lib/api/ppm.js";

  const ppm = PpmClient.withFetch();
  let risk = $state<DependencyRisk | null>(null);
  let radar = $state<TechnologyRadar | null>(null);
  let debt = $state<TechDebtRegister | null>(null);
  let flow = $state<FlowMetrics | null>(null);
  let error = $state<string | null>(null);

  onMount(async () => {
    try {
      risk = await ppm.technologyDependencyRisk();
      radar = await ppm.technologyRadar();
      debt = await ppm.technologyDebt();
      flow = await ppm.technologyFlow();
    } catch (err) {
      error = errorText(err, "ppm.common.loadFailed");
    }
  });

  const RINGS = ["adopt", "trial", "assess", "hold", "unclassified"] as const;

  type Fan = DependencyRisk["top_fan_out"][number];
  type Edge = DependencyRisk["red_predecessor_edges"][number];
  type DebtRow = TechDebtRegister["register"][number];
  const fanColumns: DataGridColumn[] = $derived([
    { id: "item", header: t("p.technology.item"), rowHeader: true, accessor: (r) => rowOf<Fan>(r).item.name, cell: fanItemCell },
    { id: "kind", header: t("p.technology.kind"), accessor: (r) => rowOf<Fan>(r).item.kind },
    { id: "dependents", header: t("p.technology.dependents"), align: "end" },
    { id: "rag", header: t("p.technology.status"), headerCell: ragHeader, format: (v) => (v ? String(v) : "—") },
  ]);
  const redColumns: DataGridColumn[] = $derived([
    { id: "predecessor", header: t("p.technology.predecessor_red"), rowHeader: true, accessor: (r) => rowOf<Edge>(r).predecessor.name },
    { id: "successor", header: t("p.technology.successor_at_risk"), accessor: (r) => rowOf<Edge>(r).successor.name },
  ]);
  const crossColumns: DataGridColumn[] = $derived([
    { id: "predecessor", header: t("p.technology.predecessor"), rowHeader: true, accessor: (r) => `${rowOf<Edge>(r).predecessor.name} (${rowOf<Edge>(r).predecessor.kind})` },
    { id: "successor", header: t("p.technology.successor"), accessor: (r) => `${rowOf<Edge>(r).successor.name} (${rowOf<Edge>(r).successor.kind})` },
  ]);
  const debtColumns: DataGridColumn[] = $derived([
    { id: "title", header: t("p.technology.risk"), rowHeader: true, format: (v, r) => `${v}${rowOf<DebtRow>(r).escalated ? " ⚠" : ""}` },
    { id: "item", header: t("p.technology.item"), accessor: (r) => rowOf<DebtRow>(r).item?.name ?? "—" },
    { id: "status", header: t("p.technology.status_2") },
    { id: "exposure", header: t("p.technology.exposure"), align: "end" },
    { id: "owner_ref", header: t("p.technology.owner"), format: (v) => (v ? String(v) : "—") },
  ]);
  const flowRows = $derived(flow ? Object.entries(flow.throughput_by_month).map(([month, count]) => ({ month, count })) : []);
  const flowColumns: DataGridColumn[] = $derived([
    { id: "month", header: t("p.technology.month"), rowHeader: true },
    { id: "count", header: t("p.technology.milestones_completed"), align: "end" },
  ]);
</script>

{#snippet ragHeader()}{t("p.technology.status")}<Term t="RAG" />){/snippet}

{#snippet fanItemCell({ row, formatted }: DataGridCellContext)}
  {@const f = rowOf<Fan>(row)}
  <a href={localeHref(`/${f.item.kind.toLowerCase()}s/${f.item.pid}`)}>{formatted}</a>
{/snippet}


<svelte:head><title>{t("p.technology.ppm", { t: t("ppm.nav.technology") })}</title></svelte:head>

<h1>{t("ppm.nav.technology")}</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if radar}
  <h2>{t("p.technology.technology_radar")}</h2>
  <p class="muted">{t("p.technology.as_of", { as_of: radar.as_of, convention: radar.convention })}</p>
  {#each RINGS as ring (ring)}
    {@const inRing = radar.technologies.filter((tech) => tech.ring === ring)}
    {#if inRing.length > 0}
      <h3 class="ring-title">{ring}</h3>
      <ul class="ring" data-testid="radar-{ring}">
        {#each inRing as tech (tech.technology)}
          <li>
            <strong>{tech.technology}</strong>
            <span class="muted">
              {Object.entries(tech.per_collection)
                .map(([kind, count]) => `${count} ${kind}`)
                .join(" · ")}
              {#if tech.ring_votes > 0}{t("p.technology.ring_votes", { ring_votes: tech.ring_votes })}{/if}
            </span>
          </li>
        {/each}
      </ul>
    {/if}
  {/each}
  {#if radar.technologies.length === 0}
    <p class="muted">{t("p.technology.no_tech_tags_recorded_yet")}</p>
  {/if}
{/if}

{#if risk}
  <h2>{t("p.technology.dependency_risk")}</h2>
  <p class="muted">{t("p.technology.as_of_edges", { as_of: risk.as_of, edges: risk.edges })}</p>

  <h3>{t("p.technology.most_depended_on_single_point")}</h3>
  <DataTable label={t("p.technology.item")} columns={fanColumns} rows={risk.top_fan_out} rowId={(r) => String(rowOf<Fan>(r).item.pid)} emptyText={t("p.technology.no_dependencies_recorded_yet")} testid="tech-fan-out" />

  <h3>{t("p.technology.edges_importing_risk_from_a")}</h3>
  <DataTable label={t("p.technology.predecessor_red")} columns={redColumns} rows={risk.red_predecessor_edges} rowId={(r) => String(r.edge)} emptyText={t("p.technology.none_no_red_predecessors")} testid="tech-red-edges" />

  <h3>{t("p.technology.cross_portfolio_edges")}</h3>
  <DataTable label={t("p.technology.predecessor")} columns={crossColumns} rows={risk.cross_portfolio} rowId={(r) => String(r.edge)} emptyText={t("p.technology.none_all_edges_stay_within")} testid="tech-cross-portfolio" />
{/if}

{#if debt}
  <h2>{t("p.technology.technical_debt_register")}</h2>
  <p class="muted">{t("p.technology.open_exposure", { note: debt.note, open_exposure: debt.open_exposure })}</p>
  <DataTable label={t("p.technology.risk")} columns={debtColumns} rows={debt.register} rowId={(r) => String(r.pid)} emptyText={t("p.technology.no_tech_debt_risks_recorded")} testid="tech-debt" searchable />
{/if}

{#if flow}
  <h2>{t("p.technology.delivery_flow")}</h2>
  <p class="muted">{flow.derivation}</p>
  <DataTable label={t("p.technology.month")} columns={flowColumns} rows={flowRows} rowId={(r) => String(r.month)} emptyText={t("p.technology.no_timed_completions_in_the")} testid="tech-flow" />
  <p>
    {t("p.technology.median_lead")}
    {#if flow.median_lead_days === null}
      <span class="muted">{t("p.technology.no_timed_completions")}</span>
    {:else}
      <strong>{t("p.technology.calendar_days", { median_lead_days: flow.median_lead_days })}</strong>
    {/if}
    {t("p.technology.over_timed_completions", { timed_completions: flow.timed_completions })}
    {#if flow.undated_completions > 0}
      {t("p.technology.completed_before_timing_existed_unti", { undated_completions: flow.undated_completions })}
    {/if}
  </p>
{/if}

<style>
  .ring-title { text-transform: capitalize; margin-bottom: 0.25rem; }
  .ring { list-style: none; padding: 0; margin: 0 0 1rem; }
  .ring li { padding: 0.2rem 0; }
</style>
