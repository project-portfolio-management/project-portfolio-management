<!--
  Engineering estate area (`/engineering`): blocked-work aging (calendar days
  since entering blocked), the MoSCoW scope cut from `moscow:<band>`
  tags (convention shown verbatim; untagged counted, never guessed),
  and the delivery-links panel (which items are tracked in which
  external tool, plus the untracked list). English-first, like the
  other PPM views.
-->
<script lang="ts">
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
</script>

<svelte:head><title>{t("p.engineering.ppm", { t: t("ppm.nav.engineering") })}</title></svelte:head>

<h1>{t("ppm.nav.engineering")}</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if blocked}
  <h2>{t("p.engineering.blocked_work")}</h2>
  <p class="muted">{blocked.derivation}</p>
  <table data-testid="eng-blocked">
    <thead><tr><th>{t("p.engineering.task")}</th><th>{t("p.engineering.item")}</th><th>{t("p.engineering.assignee")}</th><th>{t("p.engineering.blocked_calendar_days")}</th></tr></thead>
    <tbody>
      {#each blocked.blocked as task (task.pid)}
        <tr>
          <td>{task.title}</td>
          <td>{task.item?.name ?? "—"}</td>
          <td>{task.assignee_ref ?? "—"}</td>
          <td>{task.blocked_days}</td>
        </tr>
      {:else}
        <tr><td colspan="4" class="muted">{t("p.engineering.nothing_blocked_ship_it")}</td></tr>
      {/each}
    </tbody>
  </table>
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
  <table data-testid="eng-links">
    <thead><tr><th>{t("p.engineering.item")}</th><th>{t("p.engineering.trackers")}</th></tr></thead>
    <tbody>
      {#each links.tracked as row (row.item.pid)}
        <tr>
          <td>{row.item.name}</td>
          <td>
            {#each row.links as link (link.scheme + link.value)}
              <code>{link.scheme}:{link.value}</code>&nbsp;
            {/each}
          </td>
        </tr>
      {:else}
        <tr><td colspan="2" class="muted">{t("p.engineering.no_externally_tracked_items")}</td></tr>
      {/each}
    </tbody>
  </table>
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
  <table data-testid="devops-releases">
    <thead><tr><th>{t("p.engineering.when")}</th><th>{t("p.engineering.version")}</th><th>{t("p.engineering.environment")}</th><th>{t("p.engineering.item")}</th></tr></thead>
    <tbody>
      {#each releases.releases as release (release.pid)}
        <tr>
          <td>{release.occurred_at}</td>
          <td>{release.version ?? "—"}</td>
          <td>{release.environment ?? "—"}</td>
          <td>{release.item?.name ?? "—"}</td>
        </tr>
      {:else}
        <tr><td colspan="4" class="muted">{t("p.engineering.no_deploy_events_ingested_yet")}</td></tr>
      {/each}
    </tbody>
  </table>
{/if}

<style>
  .bands { display: flex; gap: 2rem; flex-wrap: wrap; }
  .bands section { min-width: 10rem; }
  .bands h3 { text-transform: capitalize; }
  .bands ul { list-style: none; padding: 0; margin: 0; }
</style>
