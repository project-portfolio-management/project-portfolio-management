<!--
  The mix of finished work (`/plans/[pid]/flow`, NC-1 slice 12 part i): `GET /api/plans/{pid}/flow-distribution`, read
  on asking. A window of days (90, the default, not sent; the service quietly reads one outside 1 to 3650 as 90, so the
  page refuses it first), and optionally the plans this one contains to a depth (32 not sent). Each kind of work in
  words with its count and share, and, where the plan declared the mix it intends, the intended share and the gap; no
  finished work is a sentence. Rules in `src/lib/plan-readings-view.ts`.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { t, tn } from "#lib/i18n.svelte.js";
  import { formatBasisPoints, formatInteger } from "#lib/locale-format.js";
  import { rowOf } from "#lib/row-of.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { PpmClient, type FlowDistribution } from "#lib/api/ppm.js";
  import { DISTRIBUTION_WINDOW, ROLLUP_DEPTH_MAX, distributionQuery, flowTypeKey, readingRefusalKey, type DistributionForm } from "#lib/plan-readings-view.js";

  let { pid }: { pid: string } = $props();

  const ppm = PpmClient.withFetch();
  let form = $state<DistributionForm>({ windowDays: null, rollup: false, depth: null });
  let data = $state<FlowDistribution | null>(null);
  let problem = $state<string | null>(null);
  let busy = $state(false);

  async function read(e: SubmitEvent) {
    e.preventDefault();
    const r = distributionQuery(form);
    if ("problems" in r) {
      problem = r.problems.map((k) => t(k, { max: ROLLUP_DEPTH_MAX })).join(" ");
      return;
    }
    problem = null;
    busy = true;
    try {
      data = await ppm.flowDistribution(pid, r.query);
    } catch (err) {
      const key = readingRefusalKey("distribution", err);
      problem = key ? t(key) : errorText(err, "p.readings.could_not_read");
    } finally {
      busy = false;
    }
  }

  type Share = FlowDistribution["distribution"]["shares"][number];
  const bp = (v: number | null) => (v === null ? "" : formatBasisPoints(v, 0));
  const columns: DataGridColumn[] = $derived.by(() => {
    const cols: DataGridColumn[] = [
      { id: "flow_type", header: t("p.flowmix.col_kind"), rowHeader: true, accessor: (r) => t(flowTypeKey(rowOf<Share>(r).flow_type)) },
      { id: "count", header: t("p.flowmix.col_count"), align: "end", format: (v) => formatInteger(Number(v)) },
      { id: "share", header: t("p.flowmix.col_share"), align: "end", accessor: (r) => bp(rowOf<Share>(r).basis_points) },
    ];
    if (data?.distribution.intent_declared)
      cols.push(
        { id: "intended", header: t("p.flowmix.col_intended"), align: "end", accessor: (r) => bp(rowOf<Share>(r).intended_basis_points) },
        { id: "gap", header: t("p.flowmix.col_gap"), align: "end", accessor: (r) => bp(rowOf<Share>(r).gap_basis_points) },
      );
    return cols;
  });
</script>

<section aria-labelledby="flowmix-h" data-testid="flow-mix">
  <h2 id="flowmix-h">{t("p.flowmix.heading")}</h2>
  <p class="small muted">{t("p.flowmix.says", { n: DISTRIBUTION_WINDOW })}</p>
  <form class="row" novalidate onsubmit={read} aria-label={t("p.flowmix.heading")}>
    <label>{t("p.flowmix.f_window", { n: DISTRIBUTION_WINDOW })}<input type="number" min="1" max="3650" step="1" bind:value={form.windowDays} /></label>
    <label class="check"><input type="checkbox" bind:checked={form.rollup} /> {t("p.flowmix.f_rollup")}</label>
    {#if form.rollup}
      <label>{t("p.flowmix.f_depth", { max: ROLLUP_DEPTH_MAX })}<input type="number" min="0" max={ROLLUP_DEPTH_MAX} step="1" bind:value={form.depth} /></label>
    {/if}
    <button class="button" type="submit" disabled={busy}>{t("p.flowmix.read")}</button>
  </form>
  {#if problem}<p class="banner error" role="alert">{problem}</p>{/if}
  {#if data}
    <p>{tn("p.flowmix.window", data.window_days, { n: formatInteger(data.window_days) })}
      {#if data.rollup}{tn("p.flowmix.plans", data.plans_counted, { n: formatInteger(data.plans_counted) })}{/if}</p>
    {#if data.truncated}<p class="banner" role="note">{t("p.flowmix.truncated")}</p>{/if}
    {#if data.distribution.total === 0}
      <p>{t("p.flowmix.none")}</p>
    {:else}
      {#if !data.distribution.intent_declared}<p class="small">{t("p.flowmix.no_intent")}</p>{/if}
      <DataTable label={t("p.flowmix.table")} {columns} rows={data.distribution.shares} rowId={(r) => String(rowOf<Share>(r).flow_type)} />
    {/if}
  {/if}
</section>

<style>
  .row { display: flex; flex-wrap: wrap; gap: 0.5rem 1rem; align-items: end; }
  .row label { display: grid; gap: 0.2rem; }
  .row label.check { display: inline-flex; gap: 0.3rem; }
</style>
