<!--
  Auditor area (`/auditor`): the estate-wide audit-trail explorer
  (actor/action filters + integrity stats), segregation-of-duties
  findings (rules disclosed by the server), and the evidence-pack CSV
  download. English-first, like the other PPM views.
-->
<script lang="ts">
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import {
    PpmClient,
    type AuditTrail,
    type AuditorFindings,
  } from "#lib/api/ppm.js";

  const ppm = PpmClient.withFetch();
  let trail = $state<AuditTrail | null>(null);
  let findings = $state<AuditorFindings | null>(null);
  let error = $state<string | null>(null);
  let actorFilter = $state("");
  let actionFilter = $state("");

  async function loadTrail() {
    try {
      trail = await ppm.auditorTrail({
        actor: actorFilter.trim() || undefined,
        action: actionFilter.trim() || undefined,
      });
    } catch (err) {
      error = err instanceof Error ? err.message : t("ppm.common.loadFailed");
    }
  }
  onMount(async () => {
    await loadTrail();
    try {
      findings = await ppm.auditorFindings();
    } catch (err) {
      error = err instanceof Error ? err.message : t("ppm.common.loadFailed");
    }
  });

  const findingColumns: DataGridColumn[] = $derived([
    { id: "rule", header: t("p.auditor.rule"), rowHeader: true, cell: codeCell },
    { id: "detail", header: t("p.auditor.detail") },
  ]);
  const trailColumns: DataGridColumn[] = $derived([
    { id: "created_at", header: t("p.auditor.when") },
    { id: "actor", header: t("p.auditor.actor"), format: (v) => (v ? String(v) : "—") },
    { id: "action", header: t("p.auditor.action"), cell: codeCell },
    { id: "entity_pid", header: t("p.auditor.entity"), format: (v) => String(v).slice(0, 8) },
  ]);
</script>

{#snippet codeCell({ formatted }: DataGridCellContext)}<code>{formatted}</code>{/snippet}


<svelte:head><title>{t("p.auditor.ppm", { t: t("ppm.nav.auditor") })}</title></svelte:head>

<h1>{t("ppm.nav.auditor")}</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if findings}
  <h2>{t("p.auditor.findings")}</h2>
  <p class="muted">{findings.note}</p>
  <DataTable label={t("p.auditor.rule")} columns={findingColumns} rows={findings.findings} rowId={(_r, i) => String(i)} emptyText={t("p.auditor.no_findings")} testid="auditor-findings" searchable />
  <p data-testid="auditor-actorless">
    {t("p.auditor.actor_less_recorded_actions")} <strong>{findings.actorless_actions}</strong>
    <span class="muted">{t("p.auditor.submitted_without_a_verified_token")}</span>
  </p>
{/if}

<h2>{t("p.auditor.audit_trail")}</h2>
<form
  class="filters"
  onsubmit={(event) => {
    event.preventDefault();
    void loadTrail();
  }}
>
  <label>{t("p.auditor.actor")} <input bind:value={actorFilter} placeholder={t("p.auditor.bearer_sub")} /></label>
  <label>{t("p.auditor.action")} <input bind:value={actionFilter} placeholder={t("p.auditor.e_g_merged")} /></label>
  <button type="submit">{t("p.auditor.filter")}</button>
  <a href={ppm.evidencePackUrl()} download>{t("p.auditor.evidence_pack_csv")}</a>
</form>

{#if trail}
  <p class="muted" data-testid="trail-stats">
    {t("p.auditor.rows_distinct_actors_actor_less", { returned: trail.returned, distinct_actors: trail.stats.distinct_actors, actorless: trail.stats.actorless })}
  </p>
  <DataTable label={t("p.auditor.when")} columns={trailColumns} rows={trail.rows} rowId={(r) => String(r.created_at) + String(r.action) + String(r.entity_pid)} emptyText={t("p.auditor.no_audit_rows_match")} testid="auditor-trail" searchable pageSize={50} />
{/if}

<style>
  .filters { display: flex; gap: 1rem; align-items: end; flex-wrap: wrap; }
</style>
