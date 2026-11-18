<!--
  The frozen buffer plan (`/plans/[pid]/critical-chain`, NC-1 slice 12 part i, CC-3): `GET /api/plans/{pid}/buffer-plan`,
  the latest plan frozen on `/plans/[pid]/buffers` and the day each earlier one was taken. The project buffer and each
  feeding buffer with its days, its chain and its planned finish, in a table; the rule it was sized by in words. None
  frozen is said, with the way to freeze one (the service's reason is English and not shown).
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { errorText } from "#lib/error-text.js";
  import { t, tn, localeHref } from "#lib/i18n.svelte.js";
  import { arrow, formatDate, formatInteger } from "#lib/locale-format.js";
  import { rowOf } from "#lib/row-of.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { PpmClient, type BufferPlan, type FrozenBuffer } from "#lib/api/ppm.js";

  let { pid }: { pid: string } = $props();

  const ppm = PpmClient.withFetch();
  let plan = $state<BufferPlan | null>(null);
  let loadError = $state<string | null>(null);

  onMount(async () => {
    try {
      plan = await ppm.bufferPlan(pid);
    } catch (err) {
      loadError = errorText(err, "p.frozen.could_not_load");
    }
  });

  const rows = $derived(plan?.state === "buffer_plan" ? [plan.document.project, ...plan.document.feeding] : []);
  const chain = (b: FrozenBuffer) => b.tasks.map((x) => x.title ?? t("p.frozen.untitled")).join(` ${arrow()} `);
  const columns: DataGridColumn[] = $derived([
    { id: "buffer", header: t("p.frozen.col_buffer"), rowHeader: true, accessor: (r) => { const b = rowOf<FrozenBuffer>(r); return b.key === "project" ? t("p.frozen.project") : t("p.frozen.feeding", { joins: b.joins?.title ?? t("p.frozen.untitled") }); } },
    { id: "days", header: t("p.frozen.col_days"), align: "end", format: (v) => (v === null || v === undefined ? t("p.frozen.unsized") : formatInteger(Number(v))) },
    { id: "chain", header: t("p.frozen.col_chain"), accessor: (r) => chain(rowOf<FrozenBuffer>(r)) },
    { id: "planned_finish", header: t("p.frozen.col_finish"), format: (v) => (v ? formatDate(String(v)) : "") },
  ]);
</script>

<section aria-labelledby="frozen-h" data-testid="frozen-buffer-plan">
  <h2 id="frozen-h">{t("p.frozen.heading")}</h2>
  {#if loadError}<p class="banner" role="alert">{loadError}</p>{/if}
  {#if plan?.state === "no_buffer_plan"}
    <p>{t("p.frozen.none")} <a href={localeHref(`/plans/${pid}/buffers`)}>{t("p.frozen.freeze_link")}</a></p>
  {:else if plan?.state === "buffer_plan"}
    <p>{t("p.frozen.taken", { date: formatDate(plan.taken_on), rule: t(plan.document.buffer_rule === "root_sum_squares" ? "p.critical_chain.rule_root_sum_squares" : "p.critical_chain.rule_cut_and_paste") })}</p>
    <DataTable label={t("p.frozen.table")} {columns} rows={rows} rowId={(r) => String(rowOf<FrozenBuffer>(r).key)} />
    {#if plan.history.length > 1}
      <p class="small">{tn("p.frozen.history", plan.history.length, { n: plan.history.length, dates: plan.history.map((h) => formatDate(h.taken_on)).join(", ") })}</p>
    {/if}
  {/if}
</section>
