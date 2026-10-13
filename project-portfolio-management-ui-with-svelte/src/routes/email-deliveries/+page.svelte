<!--
  Email deliveries (`/email-deliveries`): one row for each recipient of each email the service tried to send.
  Administrators only (the API answers 404 to anyone else).
-->
<script lang="ts">
  import { t } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient, type EmailDeliveries } from "#lib/api/ppm.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";

  const ppm = PpmClient.withFetch();
  let data = $state<EmailDeliveries | null>(null);
  let error = $state<string | null>(null);
  let status = $state("");

  async function load() {
    try {
      data = await ppm.emailDeliveries(status || undefined);
      error = null;
    } catch (err) {
      error = err instanceof Error && /404|not found/i.test(err.message)
        ? t("p.email_deliveries.not_allowed")
        : err instanceof Error ? err.message : t("p.email_deliveries.could_not_load");
    }
  }
  onMount(() => void load());

  const word = (s: string) => t(`p.email_deliveries.status_${s}` as never);
  const when = (v: unknown) => (typeof v === "string" ? v.slice(0, 16).replace("T", " ") : "");
  const columns: DataGridColumn[] = $derived([
    { id: "recipient", header: t("p.email_deliveries.col_recipient"), rowHeader: true },
    { id: "subject", header: t("p.email_deliveries.col_subject") },
    { id: "status", header: t("p.email_deliveries.col_status"), format: (v) => word(String(v)), cell: statusCell, cellClass: (c) => ({ sent: "cell-green", failed: "cell-red", retry: "cell-amber", queued: "cell-amber" })[String(c.value)] ?? "" },
    { id: "attempts", header: t("p.email_deliveries.col_attempts"), align: "end" },
    { id: "last_error", header: t("p.email_deliveries.col_error"), format: (v) => (v ? String(v) : "") },
    { id: "next_attempt_at", header: t("p.email_deliveries.col_next"), format: when },
    { id: "sent_at", header: t("p.email_deliveries.col_sent"), format: when },
  ]);
</script>

<svelte:head><title>{t("p.email_deliveries.title")}</title></svelte:head>

{#snippet statusCell({ value, formatted }: DataGridCellContext)}
  <span aria-hidden="true">{value === "sent" ? "●" : value === "failed" ? "✖" : "◆"}</span> {formatted}
{/snippet}

<h1>{t("p.email_deliveries.h1")}</h1>
<p class="lede">{t("p.email_deliveries.lede")}</p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}
{#if !data && !error}<p class="muted">{t("p.email_deliveries.loading")}</p>{/if}

{#if data}
  <p>{t("p.email_deliveries.counts", data.counts)}</p>
  <label>{t("p.email_deliveries.filter")}
    <select bind:value={status} onchange={() => void load()}>
      <option value="">{t("p.email_deliveries.all")}</option>
      {#each ["queued", "retry", "sent", "failed"] as s (s)}<option value={s}>{word(s)}</option>{/each}
    </select>
  </label>
  <DataTable
    label={t("p.email_deliveries.h1")}
    columns={columns}
    rows={data.deliveries}
    rowId={(r) => String(r.pid)}
    emptyText={t("p.email_deliveries.none")}
    searchable
    pageSize={25}
  />
{/if}
