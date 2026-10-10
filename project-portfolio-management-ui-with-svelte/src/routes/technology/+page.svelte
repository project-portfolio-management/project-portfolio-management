<!--
  CTO technology area (`/technology`): the dependency-risk lens (top
  fan-out items, cross-portfolio edges, edges with a RAG-red
  predecessor) and the tag-encoded technology radar
  (`tech:<name>[:<ring>]`; the server's convention string is displayed
  verbatim). All derivations are server-side. English-first, like the
  other PPM views.
-->
<script lang="ts">
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
      error = err instanceof Error ? err.message : t("ppm.common.loadFailed");
    }
  });

  const RINGS = ["adopt", "trial", "assess", "hold", "unclassified"] as const;
</script>

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
  <table data-testid="tech-fan-out">
    <thead>
      <tr><th>{t("p.technology.item")}</th><th>{t("p.technology.kind")}</th><th>{t("p.technology.dependents")}</th><th>{t("p.technology.status")}<Term t="RAG" />)</th></tr>
    </thead>
    <tbody>
      {#each risk.top_fan_out as row (row.item.pid)}
        <tr>
          <td><a href={localeHref(`/${row.item.kind.toLowerCase()}s/${row.item.pid}`)}>{row.item.name}</a></td>
          <td>{row.item.kind}</td>
          <td>{row.dependents}</td>
          <td>{row.rag ?? "—"}</td>
        </tr>
      {:else}
        <tr><td colspan="4" class="muted">{t("p.technology.no_dependencies_recorded_yet")}</td></tr>
      {/each}
    </tbody>
  </table>

  <h3>{t("p.technology.edges_importing_risk_from_a")}</h3>
  <table data-testid="tech-red-edges">
    <thead>
      <tr><th>{t("p.technology.predecessor_red")}</th><th>{t("p.technology.successor_at_risk")}</th></tr>
    </thead>
    <tbody>
      {#each risk.red_predecessor_edges as edge (edge.edge)}
        <tr>
          <td>{edge.predecessor.name}</td>
          <td>{edge.successor.name}</td>
        </tr>
      {:else}
        <tr><td colspan="2" class="muted">{t("p.technology.none_no_red_predecessors")}</td></tr>
      {/each}
    </tbody>
  </table>

  <h3>{t("p.technology.cross_portfolio_edges")}</h3>
  <table data-testid="tech-cross-portfolio">
    <thead>
      <tr><th>{t("p.technology.predecessor")}</th><th>{t("p.technology.successor")}</th></tr>
    </thead>
    <tbody>
      {#each risk.cross_portfolio as edge (edge.edge)}
        <tr>
          <td>{edge.predecessor.name} <span class="muted">({edge.predecessor.kind})</span></td>
          <td>{edge.successor.name} <span class="muted">({edge.successor.kind})</span></td>
        </tr>
      {:else}
        <tr><td colspan="2" class="muted">{t("p.technology.none_all_edges_stay_within")}</td></tr>
      {/each}
    </tbody>
  </table>
{/if}

{#if debt}
  <h2>{t("p.technology.technical_debt_register")}</h2>
  <p class="muted">{t("p.technology.open_exposure", { note: debt.note, open_exposure: debt.open_exposure })}</p>
  <table data-testid="tech-debt">
    <thead>
      <tr><th>{t("p.technology.risk")}</th><th>{t("p.technology.item")}</th><th>{t("p.technology.status_2")}</th><th>{t("p.technology.exposure")}</th><th>{t("p.technology.owner")}</th></tr>
    </thead>
    <tbody>
      {#each debt.register as row (row.pid)}
        <tr>
          <td>{row.title}{#if row.escalated}&nbsp;⚠{/if}</td>
          <td>{row.item?.name ?? "—"}</td>
          <td>{row.status}</td>
          <td>{row.exposure}</td>
          <td>{row.owner_ref ?? "—"}</td>
        </tr>
      {:else}
        <tr><td colspan="5" class="muted">{t("p.technology.no_tech_debt_risks_recorded")}</td></tr>
      {/each}
    </tbody>
  </table>
{/if}

{#if flow}
  <h2>{t("p.technology.delivery_flow")}</h2>
  <p class="muted">{flow.derivation}</p>
  <table data-testid="tech-flow">
    <thead>
      <tr><th>{t("p.technology.month")}</th><th>{t("p.technology.milestones_completed")}</th></tr>
    </thead>
    <tbody>
      {#each Object.entries(flow.throughput_by_month) as [month, count] (month)}
        <tr><td>{month}</td><td>{count}</td></tr>
      {:else}
        <tr><td colspan="2" class="muted">{t("p.technology.no_timed_completions_in_the")}</td></tr>
      {/each}
    </tbody>
  </table>
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
