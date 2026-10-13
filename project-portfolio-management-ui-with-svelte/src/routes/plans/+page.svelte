<!--
  Plans index route (`/plans`).

  Purpose: fetch the plan refs and show them on Lily's DataGrid (through DataTable): sortable, with a filter
  box. The plan's name links to its detail route.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { PlanRepository } from "#lib/api/plans.js";
  import type { PlanRef } from "#lib/api/types.js";
  import { t, localeHref } from "#lib/i18n.svelte.js";

  const repo = PlanRepository.withFetch();

  let items = $state<PlanRef[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);
  // Rows matching overall, from `X-Total-Count` — not `items.length`,
  // which is only this page. Shown so an operator can tell a short list
  // from a first page of a long one (agents/share/restful.md), matching
  // the organization front-end's `/organizations` list convention.
  let total = $state(0);

  onMount(async () => {
    try {
      const page = await repo.listPage();
      items = page.items;
      total = page.total;
    } catch (err) {
      error = err instanceof Error ? err.message : t("list.loadFailed");
    } finally {
      loading = false;
    }
  });

  // The list endpoint returns lightweight `{pid, name}` refs, so the grid carries those two columns;
  // `pid` stays a technical literal.
  const columns: DataGridColumn[] = $derived([
    { id: "name", header: t("form.title"), rowHeader: true, cell: nameCell },
    { id: "pid", header: "pid" },
  ]);
</script>

{#snippet nameCell({ row, formatted }: DataGridCellContext)}<a href={localeHref(`/plans/${row.pid}`)}>{formatted}</a>{/snippet}

<svelte:head><title>{t("p.plans.main_x", { t: t("list.title") })}</title></svelte:head>

<h1>{t("list.title")}</h1>
<p><a class="button" href={localeHref("/plans/new")}>{t("list.new")}</a></p>

{#if loading}
  <p>{t("list.loading")}</p>
{:else if error}
  <p class="banner" role="alert">{error}</p>
{:else if items.length === 0}
  <p class="surface">{t("list.empty")} <a href={localeHref("/plans/new")}>{t("list.createOne")}</a>.</p>
{:else}
  <p class="count">
    {items.length === total ? `${total}` : `${items.length} / ${total}`}
  </p>
  <DataTable label={t("list.title")} {columns} rows={items} rowId={(r) => String(r.pid)} searchable pageSize={100} />
{/if}

<style>
  /* The row count: just the total when the page holds everything, and "shown / total" when it does not, so a
     first page of a long list cannot be mistaken for a short list (same convention as the organization
     service's front-end /organizations list route). */
  .count {
    margin: 0 0 0.5rem;
    opacity: 0.75;
    font-variant-numeric: tabular-nums;
  }
</style>
