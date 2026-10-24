<!--
  Compliance area (`/compliance`): the compliance risk register and
  the derived conformance findings (a findings list with each rule
  disclosed — no invented compliance score). English-first, like the
  other PPM views.
-->
<script lang="ts">
  import { rowOf } from "#lib/row-of.js";
  import { errorText } from "#lib/error-text.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
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
      error = errorText(err, "ppm.common.loadFailed");
    }
  });

  type Row = RiskRegister["register"][number];
  const findingColumns: DataGridColumn[] = $derived([
    { id: "rule", header: t("p.compliance.rule"), rowHeader: true, cell: codeCell },
    { id: "detail", header: t("p.compliance.detail") },
  ]);
  const registerColumns: DataGridColumn[] = $derived([
    { id: "title", header: t("p.compliance.risk"), rowHeader: true, format: (v, r) => `${v}${rowOf<Row>(r).escalated ? " ⚠" : ""}` },
    { id: "item", header: t("p.compliance.item"), accessor: (r) => rowOf<Row>(r).item?.name ?? "—" },
    { id: "status", header: t("p.compliance.status") },
    { id: "exposure", header: t("p.compliance.exposure"), align: "end" },
    { id: "owner_ref", header: t("p.compliance.owner"), format: (v) => (v ? String(v) : "—") },
  ]);
</script>

{#snippet codeCell({ formatted }: DataGridCellContext)}<code>{formatted}</code>{/snippet}


<svelte:head><title>{t("p.compliance.ppm", { t: t("ppm.nav.compliance") })}</title></svelte:head>

<h1>{t("ppm.nav.compliance")}</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if findings}
  <h2>{t("p.compliance.conformance_findings")}</h2>
  <p class="muted">{t("p.compliance.gate_review_recency_threshold_calend", { review_days: findings.review_days })}</p>
  <DataTable label={t("p.compliance.rule")} columns={findingColumns} rows={findings.findings} rowId={(_r, i) => String(i)} emptyText={t("p.compliance.no_conformance_findings")} testid="compliance-findings" searchable />
{/if}

{#if register}
  <h2>{t("p.compliance.compliance_risk_register")}</h2>
  <p class="muted">{t("p.compliance.open_exposure", { note: register.note, open_exposure: register.open_exposure })}</p>
  <DataTable label={t("p.compliance.risk")} columns={registerColumns} rows={register.register} rowId={(r) => String(r.pid)} emptyText={t("p.compliance.no_compliance_risks_recorded")} testid="compliance-register" searchable />
{/if}
