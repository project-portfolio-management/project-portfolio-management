<!--
  The start queue (`/start-queue`, EX-8): programmes refused a start, in the order they asked, and why
  (the work-in-progress limit, or a constraint pool short of supply). The order never changes.
-->
<script lang="ts">
  import { t } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient, type StartQueue, type StartRefusal } from "#lib/api/ppm.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";

  const ppm = PpmClient.withFetch();
  let queue = $state<StartQueue | null>(null);
  let error = $state<string | null>(null);
  let busy = $state(false);

  async function load() {
    try {
      queue = await ppm.startQueue();
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.start_queue.could_not_load");
    }
  }
  onMount(() => void load());

  async function withdraw(planPid: string) {
    busy = true;
    try {
      await ppm.withdrawFromStartQueue(planPid);
      await load();
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.start_queue.could_not_withdraw");
    } finally {
      busy = false;
    }
  }

  function why(r: StartRefusal): string {
    return r.reason === "limit_reached"
      ? t("p.start_queue.limit", { limit: r.limit, in_delivery: r.in_delivery })
      : t("p.start_queue.pool", { pool: r.pool, months: r.months.join(", ") });
  }

  const columns: DataGridColumn[] = $derived([
    { id: "position", header: t("p.start_queue.col_pos"), align: "end" },
    { id: "name", header: t("p.start_queue.col_prog"), rowHeader: true, accessor: (r) => (r.name as string | null) ?? String(r.plan_pid) },
    { id: "queued_on", header: t("p.start_queue.col_asked"), format: (v) => String(v).slice(0, 10) },
    { id: "refusals", header: t("p.start_queue.col_why"), sortable: false, format: (v) => (v as StartRefusal[]).map(why).join(" "), cell: whyCell },
    { id: "plan_pid", header: t("p.start_queue.col_act"), sortable: false, cell: actCell },
  ]);
</script>

{#snippet whyCell({ value }: DataGridCellContext)}
  <ul>{#each value as StartRefusal[] as r (r.reason)}<li>{why(r)}</li>{/each}</ul>
{/snippet}

{#snippet actCell({ row }: DataGridCellContext)}
  <button class="button" type="button" disabled={busy} aria-label={t("p.start_queue.withdraw", { name: String(row.name ?? row.plan_pid) })} onclick={() => void withdraw(String(row.plan_pid))}>{t("p.start_queue.withdraw_short")}</button>
{/snippet}

<svelte:head><title>{t("p.start_queue.title")}</title></svelte:head>

<h1>{t("p.start_queue.h1")}</h1>
<p class="lede">{t("p.start_queue.lede")}</p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}
{#if !queue && !error}<p class="muted">{t("p.start_queue.loading")}</p>{/if}

{#if queue}
  {#if queue.queue.length === 0}
    <p>{t("p.start_queue.none")}</p>
  {:else}
    <DataTable label={t("p.start_queue.h1")} {columns} rows={queue.queue} rowId={(r) => String(r.plan_pid)} />
  {/if}
{/if}

<style>
  ul { margin: 0; padding-inline-start: 1.1rem; }
</style>
