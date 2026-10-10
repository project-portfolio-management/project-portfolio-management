<!--
  Compliance area (`/compliance`): the compliance risk register and
  the derived conformance findings (a findings list with each rule
  disclosed — no invented compliance score). English-first, like the
  other PPM views.
-->
<script lang="ts">
  import { t } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import {
    PpmClient,
    type ConformanceFindings,
    type RiskRegister,
  } from "#lib/api/ppm.js";

  const ppm = PpmClient.withFetch();
  let register = $state<RiskRegister | null>(null);
  let findings = $state<ConformanceFindings | null>(null);
  let error = $state<string | null>(null);

  onMount(async () => {
    try {
      register = await ppm.complianceRegister();
      findings = await ppm.complianceFindings();
    } catch (err) {
      error = err instanceof Error ? err.message : t("ppm.common.loadFailed");
    }
  });
</script>

<svelte:head><title>{t("p.compliance.ppm", { t: t("ppm.nav.compliance") })}</title></svelte:head>

<h1>{t("ppm.nav.compliance")}</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if findings}
  <h2>{t("p.compliance.conformance_findings")}</h2>
  <p class="muted">{t("p.compliance.gate_review_recency_threshold_calend", { review_days: findings.review_days })}</p>
  <table data-testid="compliance-findings">
    <thead><tr><th>{t("p.compliance.rule")}</th><th>{t("p.compliance.detail")}</th></tr></thead>
    <tbody>
      {#each findings.findings as finding, index (index)}
        <tr><td><code>{finding.rule}</code></td><td>{finding.detail}</td></tr>
      {:else}
        <tr><td colspan="2" class="muted">{t("p.compliance.no_conformance_findings")}</td></tr>
      {/each}
    </tbody>
  </table>
{/if}

{#if register}
  <h2>{t("p.compliance.compliance_risk_register")}</h2>
  <p class="muted">{t("p.compliance.open_exposure", { note: register.note, open_exposure: register.open_exposure })}</p>
  <table data-testid="compliance-register">
    <thead>
      <tr><th>{t("p.compliance.risk")}</th><th>{t("p.compliance.item")}</th><th>{t("p.compliance.status")}</th><th>{t("p.compliance.exposure")}</th><th>{t("p.compliance.owner")}</th></tr>
    </thead>
    <tbody>
      {#each register.register as row (row.pid)}
        <tr>
          <td>{row.title}{#if row.escalated}&nbsp;⚠{/if}</td>
          <td>{row.item?.name ?? "—"}</td>
          <td>{row.status}</td>
          <td>{row.exposure}</td>
          <td>{row.owner_ref ?? "—"}</td>
        </tr>
      {:else}
        <tr><td colspan="5" class="muted">{t("p.compliance.no_compliance_risks_recorded")}</td></tr>
      {/each}
    </tbody>
  </table>
{/if}
