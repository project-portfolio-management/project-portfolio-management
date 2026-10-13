<!--
  Engineering estate area (`/engineering`): blocked-work aging (calendar days
  since entering blocked), the MoSCoW scope cut from `moscow:<band>`
  tags (convention shown verbatim; untagged counted, never guessed),
  and the delivery-links panel (which items are tracked in which
  external tool, plus the untracked list). English-first, like the
  other PPM views.
-->
<script lang="ts">
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import {
    PpmClient,
    type BlockedWork,
    type DeliveryLinks,
    type DevopsMetrics,
    type DevopsReleases,
    type MoscowView,
  } from "#lib/api/ppm.js";

  const ppm = PpmClient.withFetch();
  let blocked = $state<BlockedWork | null>(null);
  let moscow = $state<MoscowView | null>(null);
  let links = $state<DeliveryLinks | null>(null);
  let metrics = $state<DevopsMetrics | null>(null);
  let releases = $state<DevopsReleases | null>(null);
  let error = $state<string | null>(null);

  onMount(async () => {
    try {
      blocked = await ppm.engineeringBlocked();
      moscow = await ppm.engineeringMoscow();
      links = await ppm.engineeringDeliveryLinks();
      metrics = await ppm.devopsMetrics();
      releases = await ppm.devopsReleases();
    } catch (err) {
      error = err instanceof Error ? err.message : t("ppm.common.loadFailed");
    }
  });

  type Blocked = BlockedWork["blocked"][number];
  type Tracked = DeliveryLinks["tracked"][number];
  type Release = DevopsReleases["releases"][number];
  const blockedColumns: DataGridColumn[] = $derived([
    { id: "title", header: t("p.engineering.task"), rowHeader: true },
    { id: "item", header: t("p.engineering.item"), accessor: (r) => (r as unknown as Blocked).item?.name ?? "—" },
    { id: "assignee_ref", header: t("p.engineering.assignee"), format: (v) => (v ? String(v) : "—") },
    { id: "blocked_days", header: t("p.engineering.blocked_calendar_days"), align: "end" },
  ]);
  const linkColumns: DataGridColumn[] = $derived([
    { id: "item", header: t("p.engineering.item"), rowHeader: true, accessor: (r) => (r as unknown as Tracked).item.name },
    { id: "links", header: t("p.engineering.trackers"), sortable: false, accessor: (r) => (r as unknown as Tracked).links.map((l) => `${l.scheme}:${l.value}`).join(" "), cell: trackersCell },
  ]);
  const releaseColumns: DataGridColumn[] = $derived([
    { id: "occurred_at", header: t("p.engineering.when") },
    { id: "version", header: t("p.engineering.version"), format: (v) => (v ? String(v) : "—") },
    { id: "environment", header: t("p.engineering.environment"), format: (v) => (v ? String(v) : "—") },
    { id: "item", header: t("p.engineering.item"), accessor: (r) => (r as unknown as Release).item?.name ?? "—" },
  ]);
</script>

{#snippet trackersCell({ row }: DataGridCellContext)}
  {#each (row as unknown as Tracked).links as link (link.scheme + link.value)}<code>{link.scheme}:{link.value}</code>&nbsp;{/each}
{/snippet}


<svelte:head><title>{t("p.engineering.ppm", { t: t("ppm.nav.engineering") })}</title></svelte:head>

<h1>{t("ppm.nav.engineering")}</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if blocked}
  <h2>{t("p.engineering.blocked_work")}</h2>
  <p class="muted">{blocked.derivation}</p>
  <DataTable label={t("p.engineering.task")} columns={blockedColumns} rows={blocked.blocked} rowId={(r) => String(r.pid)} emptyText={t("p.engineering.nothing_blocked_ship_it")} testid="eng-blocked" />
{/if}

{#if moscow}
  <h2>{t("p.engineering.moscow_scope")}</h2>
  <p class="muted">{moscow.convention}</p>
  <div class="bands" data-testid="eng-moscow">
    {#each Object.entries(moscow.bands) as [band, items] (band)}
      <section>
        <h3>{band} ({items.length})</h3>
        <ul>
          {#each items as item (item.pid)}
            <li>{item.name} <span class="muted">({item.kind})</span></li>
          {:else}
            <li class="muted">—</li>
          {/each}
        </ul>
      </section>
    {/each}
  </div>
  <p class="muted">{t("p.engineering.items_carry_no_moscow_tag", { untagged: moscow.untagged })}</p>
{/if}

{#if links}
  <h2>{t("p.engineering.delivery_links")}</h2>
  <DataTable label={t("p.engineering.item")} columns={linkColumns} rows={links.tracked} rowId={(r) => String((r as unknown as Tracked).item.pid)} emptyText={t("p.engineering.no_externally_tracked_items")} testid="eng-links" />
  {#if links.untracked.length > 0}
    <p data-testid="eng-untracked">
      {t("p.engineering.untracked")}
      {#each links.untracked as item (item.pid)}
        {item.name}&nbsp;
      {/each}
    </p>
  {/if}
{/if}

{#if metrics}
  <h2>{t("p.engineering.devops_metrics")}</h2>
  <p class="muted">{metrics.derivation}</p>
  <section class="tiles" data-testid="devops-metrics">
    <div class="tile"><strong>{metrics.deploys}</strong><span>{t("p.engineering.deploys_mo", { window_months: metrics.window_months })}</span></div>
    <div class="tile"><strong>{metrics.incidents}</strong><span>{t("p.engineering.incidents")}</span></div>
    <div class="tile">
      <strong>{metrics.median_recovery_hours ?? "—"}</strong>
      <span>{t("p.engineering.median_recovery_h")}</span>
    </div>
    <div class="tile">
      <strong>
        {metrics.change_failure_rate === null
          ? "—"
          : `${(metrics.change_failure_rate * 100).toFixed(0)}%`}
      </strong>
      <span>{t("p.engineering.declared_cause_failure_rate")}</span>
    </div>
  </section>
  {#if metrics.unresolved_incidents > 0}
    <p class="muted">{t("p.engineering.incidents_unresolved_counted_never_t", { unresolved_incidents: metrics.unresolved_incidents })}</p>
  {/if}
{/if}

{#if releases}
  <h2>{t("p.engineering.releases")}</h2>
  <DataTable label={t("p.engineering.when")} columns={releaseColumns} rows={releases.releases} rowId={(r) => String(r.pid)} emptyText={t("p.engineering.no_deploy_events_ingested_yet")} testid="devops-releases" />
{/if}

<style>
  .bands { display: flex; gap: 2rem; flex-wrap: wrap; }
  .bands section { min-width: 10rem; }
  .bands h3 { text-transform: capitalize; }
  .bands ul { list-style: none; padding: 0; margin: 0; }
</style>
