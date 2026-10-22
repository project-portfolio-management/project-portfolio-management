<!--
  CISO area (`/security`): the security risk register plus the
  server's disclosed heuristic — items at a late stage with no
  security-category risk ever recorded (a proxy, not proof).
  English-first, like the other PPM views.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient, type RiskRegister } from "#lib/api/ppm.js";

  const ppm = PpmClient.withFetch();
  let register = $state<RiskRegister | null>(null);
  let error = $state<string | null>(null);

  onMount(async () => {
    try {
      register = await ppm.securityRegister();
    } catch (err) {
      error = errorText(err, "ppm.common.loadFailed");
    }
  });

  type Row = RiskRegister["register"][number];
  const columns: DataGridColumn[] = $derived([
    { id: "title", header: t("p.security.risk"), rowHeader: true, format: (v, r) => `${v}${(r as unknown as Row).escalated ? " ⚠" : ""}` },
    { id: "item", header: t("p.security.item"), accessor: (r) => (r as unknown as Row).item?.name ?? "—" },
    { id: "status", header: t("p.security.status") },
    { id: "exposure", header: t("p.security.exposure"), align: "end" },
    { id: "owner_ref", header: t("p.security.owner"), format: (v) => (v ? String(v) : "—") },
  ]);
</script>

<svelte:head><title>{t("p.security.ppm", { t: t("ppm.nav.security") })}</title></svelte:head>

<h1>{t("ppm.nav.security")}</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if register}
  <h2>{t("p.security.security_risk_register")}</h2>
  <p class="muted">{t("p.security.open_exposure", { note: register.note, open_exposure: register.open_exposure })}</p>
  <DataTable label={t("p.security.risk")} {columns} rows={register.register} rowId={(r) => String(r.pid)} emptyText={t("p.security.no_security_risks_recorded")} testid="security-register" searchable />

  {#if register.unreviewed_at_late_stage}
    <h2>{t("p.security.late_stage_items_with_no")}</h2>
    <p class="muted">{register.unreviewed_at_late_stage.heuristic}</p>
    <ul data-testid="security-unreviewed">
      {#each register.unreviewed_at_late_stage.items as row (row.item.pid)}
        <li>{row.item.name} <span class="muted">({row.stage})</span></li>
      {:else}
        <li class="muted">{t("p.security.none_every_late_stage_item")}</li>
      {/each}
    </ul>
  {/if}
{/if}
