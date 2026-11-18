<!--
  Each task's aggressive and safe estimates (`/plans/[pid]/critical-chain`, NC-1 slice 12 part i, CC-2): `GET
  /api/plans/{pid}/ccpm-estimates` (the first 1,000 tasks; more said), the deployment's rule in words, every figure with
  where it came from and every task's standing in a word and, where someone must act, a shape; one task's two
  estimates recorded, or cleared after a question (`PUT /api/plans/{pid}/tasks/{t_pid}/ccpm-estimate`); nothing sent
  for no change; every problem said at once, first. The inputs are disabled until the list is read. Rules in
  `src/lib/ccpm-estimate-view.ts`.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { errorText } from "#lib/error-text.js";
  import { t, tn } from "#lib/i18n.svelte.js";
  import { formatInteger } from "#lib/locale-format.js";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import { rowOf } from "#lib/row-of.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { PpmClient, type CcpmEstimates, type CcpmTask } from "#lib/api/ppm.js";
  import {
    CCPM_MAX_DAYS,
    RULE_KEY,
    SOURCE_KEY,
    estimable,
    estimateBody,
    estimateFormOf,
    estimateRefusalKey,
    needsAttention,
    stateKey,
    type EstimateForm,
  } from "#lib/ccpm-estimate-view.js";

  let { pid, onsaved }: { pid: string; onsaved: () => void } = $props();

  const ppm = PpmClient.withFetch();
  let data = $state<CcpmEstimates | null>(null);
  let chosen = $state("");
  let form = $state<EstimateForm>({ aggressive: null, safe: null });
  let loadError = $state<string | null>(null);
  let problem = $state<string | null>(null);
  let notice = $state("");
  let busy = $state(false);

  async function load() {
    try {
      data = await ppm.ccpmEstimates(pid);
      loadError = null;
    } catch (err) {
      loadError = errorText(err, "p.ccpm.could_not_load");
    }
  }
  onMount(load);

  const editable = $derived((data?.tasks ?? []).filter(estimable));
  const task = $derived(editable.find((x) => x.pid === chosen) ?? null);
  function choose(p: string) {
    chosen = p;
    problem = null;
    notice = "";
    form = estimateFormOf(editable.find((x) => x.pid === p) ?? null);
  }

  async function save(e: SubmitEvent) {
    e.preventDefault();
    notice = "";
    if (!task) {
      problem = t("p.ccpm.problem_task");
      return;
    }
    const r = estimateBody(form, task);
    if (r === null) {
      problem = null;
      notice = t("p.ccpm.unchanged");
      return;
    }
    if ("problems" in r) {
      problem = r.problems.map((k) => t(k, { max: formatInteger(CCPM_MAX_DAYS) })).join(" ");
      return;
    }
    problem = null;
    const title = task.title;
    if (r.clears && !(await askConfirm(t("p.ccpm.confirm_clear", { title })))) return;
    busy = true;
    try {
      await ppm.setCcpmEstimate(pid, task.pid, r.body);
      notice = t(r.clears ? "p.ccpm.cleared" : "p.ccpm.saved", { title });
      await load();
      form = estimateFormOf(editable.find((x) => x.pid === chosen) ?? null);
      onsaved();
    } catch (err) {
      const key = estimateRefusalKey(err);
      problem = key ? t(key) : errorText(err, "p.ccpm.could_not_save");
      await load();
    } finally {
      busy = false;
    }
  }

  const figure = (f: CcpmTask["aggressive"]) => (f ? t("p.ccpm.figure", { days: formatInteger(f.days), source: t(SOURCE_KEY[f.source]) }) : t("p.ccpm.none"));
  const columns: DataGridColumn[] = $derived([
    { id: "title", header: t("p.ccpm.col_task"), rowHeader: true },
    { id: "likely_days", header: t("p.ccpm.col_likely"), align: "end", format: (v) => (v === null || v === undefined ? "" : formatInteger(Number(v))) },
    { id: "aggressive", header: t("p.ccpm.col_aggressive"), accessor: (r) => figure(rowOf<CcpmTask>(r).aggressive) },
    { id: "safe", header: t("p.ccpm.col_safe"), accessor: (r) => figure(rowOf<CcpmTask>(r).safe) },
    { id: "state", header: t("p.ccpm.col_state"), accessor: (r) => { const s = rowOf<CcpmTask>(r).state; return `${needsAttention(s) ? "▲ " : ""}${t(stateKey(s))}`; } },
  ]);
</script>

<section aria-labelledby="ccpm-h" data-testid="ccpm-estimates">
  <h2 id="ccpm-h">{t("p.ccpm.heading")}</h2>
  <p class="small muted">{t("p.ccpm.says", { max: formatInteger(CCPM_MAX_DAYS) })}</p>
  {#if loadError}<p class="banner" role="alert">{loadError}</p>{/if}
  {#if data}
    <p>{t(RULE_KEY[data.rule] ?? "p.ccpm.rule_other")}</p>
    {#if data.rule_setting_unusable}<p class="banner" role="note">{t("p.ccpm.rule_unusable")}</p>{/if}
    {#if data.needing_attention.length > 0}
      <p><span aria-hidden="true">▲ </span>{tn("p.ccpm.attention", data.needing_attention.length, { n: data.needing_attention.length })}</p>
    {/if}
    {#if data.total > data.tasks.length}<p class="small">{t("p.ccpm.more", { shown: formatInteger(data.tasks.length), total: formatInteger(data.total) })}</p>{/if}
    {#if data.tasks.length === 0}
      <p>{t("p.ccpm.no_tasks")}</p>
    {:else}
      <DataTable label={t("p.ccpm.table")} {columns} rows={data.tasks} rowId={(r) => String(rowOf<CcpmTask>(r).pid)} />
    {/if}
  {/if}

  {#if problem}<p class="banner error" role="alert">{problem}</p>{/if}
  <p class="small" role="status">{notice}</p>
  <form class="stack" novalidate onsubmit={save} aria-label={t("p.ccpm.form")}>
    <label>{t("p.ccpm.f_task")}
      <select value={chosen} onchange={(e) => choose((e.currentTarget as HTMLSelectElement).value)} disabled={!data}>
        <option value="">{t("p.ccpm.choose_task")}</option>
        {#each editable as x (x.pid)}<option value={x.pid}>{x.title}</option>{/each}
      </select>
    </label>
    <label>{t("p.ccpm.f_aggressive")}<input type="number" min="1" max={CCPM_MAX_DAYS} step="1" bind:value={form.aggressive} disabled={!task} /></label>
    <label>{t("p.ccpm.f_safe")}<input type="number" min="1" max={CCPM_MAX_DAYS} step="1" bind:value={form.safe} disabled={!task} /></label>
    <div><button class="button" type="submit" disabled={!data || busy}>{t("p.ccpm.save")}</button></div>
  </form>
</section>

<style>
  form { max-inline-size: 28rem; margin-block: 1rem; }
  label { display: grid; gap: 0.2rem; }
</style>
