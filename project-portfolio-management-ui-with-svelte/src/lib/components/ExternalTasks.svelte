<!--
  The tasks of other plans in this plan's network (`/plans/[pid]/critical-path`, NC-1 slice 12 part i, PJ-13): the
  critical path already carries every task of a linked plan the reader can read (`external` nodes of `GET
  /api/plans/{pid}/cpm`) and the links that cross (`cross_plan` edges); here they are drawn as bars on one timeline,
  by plan, with a table that gives the same dates, and the links in words. The finish across the network is said
  beside this plan's own; a network cut short is said. Links are added and removed on a task's page. Rules in
  `src/lib/cross-plan-view.ts`.
-->
<script lang="ts">
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { arrow, formatDate } from "#lib/locale-format.js";
  import { rowOf } from "#lib/row-of.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import type { Cpm, CpmNode } from "#lib/api/ppm.js";
  import { layoutBars } from "#lib/chain-view.js";
  import { LINK_KIND_KEY, crossPlanEdges, externalPlans } from "#lib/cross-plan-view.js";

  let { cpm }: { cpm: Cpm } = $props();

  const groups = $derived(externalPlans(cpm));
  const external = $derived(groups.flatMap((g) => g.nodes));
  const bars = $derived(layoutBars(external.map((n) => ({ start: n.early_start, end: n.early_finish }))));
  const barOf = $derived(new Map(external.map((n, i) => [n.pid, bars[i]])));
  const links = $derived(crossPlanEdges(cpm));
  const name = (n: CpmNode | undefined) => n?.title ?? t("p.crossplan.untitled");
  const columns: DataGridColumn[] = $derived([
    { id: "title", header: t("p.crossplan.col_task"), rowHeader: true, accessor: (r) => name(rowOf<CpmNode>(r)) },
    { id: "plan_name", header: t("p.crossplan.col_plan") },
    { id: "early_start", header: t("p.crossplan.col_start"), format: (v) => formatDate(String(v)) },
    { id: "early_finish", header: t("p.crossplan.col_finish"), format: (v) => formatDate(String(v)) },
    { id: "critical", header: t("p.crossplan.col_critical"), accessor: (r) => (rowOf<CpmNode>(r).critical ? t("p.crossplan.yes") : t("p.crossplan.no")) },
  ]);
</script>

{#if external.length > 0 || links.length > 0}
  <section aria-labelledby="ext-h" data-testid="external-tasks">
    <h2 id="ext-h">{t("p.crossplan.heading")}</h2>
    <p class="small muted">{t("p.crossplan.says")}</p>
    {#if cpm.network_finish}
      <p>{t("p.crossplan.network_finish", { date: formatDate(cpm.network_finish) })}</p>
    {/if}
    {#if cpm.external_truncated}<p class="banner" role="note">{t("p.crossplan.truncated")}</p>{/if}

    {#if links.length > 0}
      <h3>{t("p.crossplan.links_heading")}</h3>
      <ul class="small">
        {#each links as l (l.edge.pid)}
          <li>
            {name(l.from)}{l.from?.external ? ` (${l.from.plan_name ?? ""})` : ""} {arrow()} {name(l.to)}{l.to?.external ? ` (${l.to.plan_name ?? ""})` : ""}
            · {t(LINK_KIND_KEY[l.edge.kind])}{l.edge.lag_days !== 0 ? ` · ${t("p.crossplan.lag", { days: l.edge.lag_days })}` : ""}
          </li>
        {/each}
      </ul>
    {/if}

    {#each groups as g (g.planPid)}
      <figure class="bars" aria-label={t("p.crossplan.timeline", { plan: g.planName })}>
        <figcaption><a href={localeHref(`/plans/${g.planPid}/critical-path`)}>{g.planName}</a></figcaption>
        <ol>
          {#each g.nodes as n (n.pid)}
            {@const b = barOf.get(n.pid)}
            <li>
              <span class="name">{n.critical ? "◆ " : ""}{name(n)}</span>
              <span class="track" aria-hidden="true">
                {#if b}<span class="bar" class:critical={n.critical} style:left="{b.left}%" style:width="{b.width}%"></span>{/if}
              </span>
            </li>
          {/each}
        </ol>
      </figure>
    {/each}
    {#if external.length > 0}
      <DataTable label={t("p.crossplan.table")} {columns} rows={external} rowId={(r) => String(rowOf<CpmNode>(r).pid)} />
    {/if}
  </section>
{/if}

<style>
  .bars ol { list-style: none; padding: 0; margin: 0 0 1rem; }
  .bars li { display: grid; grid-template-columns: minmax(8rem, 14rem) 1fr; gap: 0.5rem; align-items: center; margin-block: 0.25rem; }
  .track { position: relative; block-size: 1.1rem; background: var(--mxi-color-surface); border: 1px solid var(--mxi-color-border); border-radius: 2px; }
  .bar { position: absolute; top: 0; bottom: 0; border: 2px dashed var(--mxi-color-primary); box-sizing: border-box; background: var(--mxi-color-bg); }
  .bar.critical { border-style: solid; background: var(--mxi-color-primary); }
</style>
