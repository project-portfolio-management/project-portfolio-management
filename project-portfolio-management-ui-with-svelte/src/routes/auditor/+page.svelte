<!--
  Auditor area (`/auditor`): the estate-wide audit-trail explorer
  (actor/action filters + integrity stats), segregation-of-duties
  findings (rules disclosed by the server), and the evidence-pack CSV
  download. English-first, like the other PPM views.
-->
<script lang="ts">
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
</script>

<svelte:head><title>{t("p.auditor.ppm", { t: t("ppm.nav.auditor") })}</title></svelte:head>

<h1>{t("ppm.nav.auditor")}</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if findings}
  <h2>{t("p.auditor.findings")}</h2>
  <p class="muted">{findings.note}</p>
  <table data-testid="auditor-findings">
    <thead><tr><th>{t("p.auditor.rule")}</th><th>{t("p.auditor.detail")}</th></tr></thead>
    <tbody>
      {#each findings.findings as finding, index (index)}
        <tr><td><code>{finding.rule}</code></td><td>{finding.detail}</td></tr>
      {:else}
        <tr><td colspan="2" class="muted">{t("p.auditor.no_findings")}</td></tr>
      {/each}
    </tbody>
  </table>
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
  <table data-testid="auditor-trail">
    <thead><tr><th>{t("p.auditor.when")}</th><th>{t("p.auditor.actor")}</th><th>{t("p.auditor.action")}</th><th>{t("p.auditor.entity")}</th></tr></thead>
    <tbody>
      {#each trail.rows as row (row.created_at + row.action + row.entity_pid)}
        <tr>
          <td>{row.created_at}</td>
          <td>{row.actor ?? "—"}</td>
          <td><code>{row.action}</code></td>
          <td class="muted">{row.entity_pid.slice(0, 8)}</td>
        </tr>
      {:else}
        <tr><td colspan="4" class="muted">{t("p.auditor.no_audit_rows_match")}</td></tr>
      {/each}
    </tbody>
  </table>
{/if}

<style>
  .filters { display: flex; gap: 1rem; align-items: end; flex-wrap: wrap; }
</style>
