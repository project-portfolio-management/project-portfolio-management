<!--
  The critical chain and its buffers (`/plans/[pid]/critical-chain`, CC-6). Advice only. The chain is
  shown on a timeline of labelled bars with a table that gives the same figures, beside the critical
  path (never blended with it). The buffers are bars too: the work of a chain in a solid bar and its
  buffer in a dashed one, labelled in words, with a table twin. A bar is never the only signal: each row
  has its words and, where a state matters, a shape.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { rowOf } from "#lib/row-of.js";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { PpmClient, type ChainBuffers, type CriticalChain } from "#lib/api/ppm.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { layoutBars } from "#lib/chain-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");
  let chain = $state<CriticalChain | null>(null);
  let buffers = $state<ChainBuffers | null>(null);
  let error = $state<string | null>(null);
  let loaded = $state(false);

  onMount(async () => {
    const [c, b] = await Promise.allSettled([ppm.criticalChain(pid), ppm.chainBuffers(pid)]);
    if (c.status === "fulfilled") chain = c.value;
    else error = errorText(c.reason, "p.critical_chain.could_not_load_chain");
    if (b.status === "fulfilled") buffers = b.value;
    loaded = true;
  });

  const reasonText = (code: string | undefined): string =>
    code === "no_assignments" ? t("p.critical_chain.reason_no_assignments")
    : code === "no_scheduled_tasks" ? t("p.critical_chain.reason_no_scheduled_tasks")
    : code === "estimates_missing" ? t("p.critical_chain.reason_estimates_missing")
    : (code ?? "");
  const ruleText = (r: string): string =>
    r === "root_sum_squares" ? t("p.critical_chain.rule_root_sum_squares") : t("p.critical_chain.rule_cut_and_paste");

  const tasks = $derived(chain?.tasks ?? []);
  const heldBy = $derived(new Map((chain?.links ?? []).map((l) => [l.to, l])));
  const bars = $derived(layoutBars(tasks.filter((x) => x.start && x.finish).map((x) => ({ start: x.start as string, end: x.finish as string }))));

  // The buffers as bars: the work of a chain solid, its buffer dashed, on one scale of business days.
  interface Row { key: string; label: string; work: number; buffer: number | null }
  const rows = $derived.by((): Row[] => {
    if (!buffers || buffers.state !== "buffers") return [];
    const out: Row[] = [];
    if (buffers.project_buffer) {
      out.push({ key: "project", label: t("p.critical_chain.project_buffer_bar"), work: buffers.chain_days_at_aggressive ?? 0, buffer: buffers.project_buffer.days });
    }
    (buffers.feeding_buffers ?? []).forEach((f, i) => {
      out.push({ key: `f${i}`, label: f.tasks.map((x) => x.title ?? x.pid).join(" → "), work: f.duration_days, buffer: f.buffer_days });
    });
    return out;
  });
  type Task = NonNullable<CriticalChain["tasks"]>[number];
  type Feeding = NonNullable<ChainBuffers["feeding_buffers"]>[number];
  type ResBuf = NonNullable<ChainBuffers["resource_buffers"]>[number];
  const heldText = (task: Task): string => {
    const link = heldBy.get(task.pid);
    if (!link) return t("p.critical_chain.held_first");
    if (link.kind === "dependency") return t("p.critical_chain.held_dep");
    if (link.masked) return t("p.critical_chain.held_person_hidden");
    return t("p.critical_chain.held_person", { person: link.person_ref ?? link.person ?? "" });
  };
  const chainColumns: DataGridColumn[] = $derived([
    { id: "title", header: t("p.critical_chain.col_task"), rowHeader: true, accessor: (r) => rowOf<Task>(r).title ?? rowOf<Task>(r).pid },
    { id: "duration_days", header: t("p.critical_chain.col_days"), align: "end" },
    { id: "start", header: t("p.critical_chain.col_start"), format: (v) => (v ? String(v) : "") },
    { id: "finish", header: t("p.critical_chain.col_finish"), format: (v) => (v ? String(v) : "") },
    { id: "held", header: t("p.critical_chain.col_held_by"), accessor: (r) => heldText(rowOf<Task>(r)) },
    { id: "on_critical_path", header: t("p.critical_chain.col_on_path"), format: (v) => (v ? t("p.critical_chain.yes") : t("p.critical_chain.no")) },
  ]);
  const feedingColumns: DataGridColumn[] = $derived([
    { id: "tasks", header: t("p.critical_chain.col_feeding_tasks"), rowHeader: true, accessor: (r) => rowOf<Feeding>(r).tasks.map((x) => x.title ?? x.pid).join(" → ") },
    { id: "joins", header: t("p.critical_chain.col_joins"), accessor: (r) => rowOf<Feeding>(r).joins.title ?? rowOf<Feeding>(r).joins.pid },
    { id: "duration_days", header: t("p.critical_chain.col_days"), align: "end" },
    { id: "buffer_days", header: t("p.critical_chain.col_buffer"), align: "end", format: (v) => (v === null || v === undefined ? t("p.critical_chain.unsized") : String(v)) },
    { id: "room_before_join_days", header: t("p.critical_chain.col_room"), align: "end", format: (v) => (v === null || v === undefined ? "" : String(v)) },
    { id: "push", header: t("p.critical_chain.col_push"), accessor: (r) => { const f = rowOf<Feeding>(r); return f.would_push_chain ? `▲ ${t("p.critical_chain.push_yes", { days: f.push_days ?? 0 })}` : f.buffer_days !== null ? t("p.critical_chain.no") : ""; } },
  ]);
  const resourceColumns: DataGridColumn[] = $derived([
    { id: "task", header: t("p.critical_chain.col_task"), rowHeader: true, accessor: (r) => rowOf<ResBuf>(r).task.title ?? rowOf<ResBuf>(r).task.pid },
    { id: "person", header: t("p.critical_chain.col_person"), accessor: (r) => { const x = rowOf<ResBuf>(r); return x.masked ? `${x.person} ${t("p.critical_chain.hidden")}` : (x.person_ref ?? x.person); } },
    { id: "alert_on", header: t("p.critical_chain.col_alert_on") },
    { id: "starts", header: t("p.critical_chain.col_task_starts") },
  ]);
  const scale = $derived(Math.max(1, ...rows.map((r) => r.work + (r.buffer ?? 0))));
</script>

<svelte:head><title>{t("p.critical_chain.chain_ppm")}</title></svelte:head>

<h1>{t("p.critical_chain.chain_h1")}</h1>
<p class="lede">{t("p.critical_chain.chain_lede")}</p>
<p class="small">
  <a href={localeHref(`/plans/${pid}`)}>{t("p.critical_chain.link_plan")}</a> ·
  <a href={localeHref(`/plans/${pid}/critical-path`)}>{t("p.critical_chain.link_critical_path")}</a> ·
  <a href={localeHref(`/plans/${pid}/buffers`)}>{t("p.critical_chain.link_buffers")}</a>
</p>
<p class="banner" role="note">{t("p.critical_chain.advice")}</p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}
{#if !loaded}<p class="muted">{t("p.critical_chain.loading")}</p>{/if}

{#if chain}
  <section aria-labelledby="cc-chain">
    <h2 id="cc-chain">{t("p.critical_chain.chain_h2")}</h2>
    {#if chain.state === "no_critical_chain"}
      <p>{t("p.critical_chain.no_chain", { reason: reasonText(chain.reason) })}</p>
    {:else}
      <ul>
        <li>{t("p.critical_chain.chain_work", { days: chain.chain_days ?? 0 })}</li>
        <li>{t("p.critical_chain.chain_tasks_count", { tasks: tasks.length })}</li>
        <li>{t("p.critical_chain.chain_finish", { finish: chain.levelled_finish ?? "" })}
          {chain.finish_delay_days > 0 ? t("p.critical_chain.chain_delay", { days: chain.finish_delay_days }) : t("p.critical_chain.chain_delay_none")}</li>
        <li><span aria-hidden="true">{chain.crosses_paths ? "◆" : "●"}</span> {chain.crosses_paths ? t("p.critical_chain.crosses_yes") : t("p.critical_chain.crosses_no")}</li>
        {#if chain.tied_chains}<li>{t("p.critical_chain.tied")}</li>{/if}
      </ul>
      <figure class="bars" aria-label={t("p.critical_chain.timeline_label")}>
        <figcaption class="small muted">{t("p.critical_chain.timeline_caption")}</figcaption>
        <ol>
          {#each tasks as task, i (task.pid)}
            <li>
              <span class="name">{task.title ?? task.pid}</span>
              <span class="track" aria-hidden="true">
                {#if bars[i]}<span class="bar solid" style:left="{bars[i].left}%" style:width="{bars[i].width}%"></span>{/if}
              </span>
            </li>
          {/each}
        </ol>
      </figure>
      <!-- A wide table scrolls on its own; the region is focusable so a keyboard can scroll it. -->
      <DataTable label={t("p.critical_chain.chain_h2")} columns={chainColumns} rows={tasks} rowId={(r) => String(r.pid)} />
    {/if}
  </section>

  <section aria-labelledby="cc-path">
    <h2 id="cc-path">{t("p.critical_chain.cp_h2")}</h2>
    {#if chain.critical_path.tasks.length === 0}
      <p>{t("p.critical_chain.cp_none")}</p>
    {:else}
      <p>{t("p.critical_chain.cp_finish", { finish: chain.critical_path.finish ?? "" })}</p>
      <ol>{#each chain.critical_path.tasks as task (task.pid)}<li>{task.title ?? task.pid}</li>{/each}</ol>
    {/if}
  </section>
{/if}

{#if buffers}
  <section aria-labelledby="cc-buffers">
    <h2 id="cc-buffers">{t("p.critical_chain.buffers_h2")}</h2>
    {#if buffers.state === "no_buffers"}
      <p>{t("p.critical_chain.no_buffers", { reason: reasonText(buffers.reason) })}</p>
      {#if (buffers.chain_tasks_missing_estimates ?? []).length > 0}
        <h3>{t("p.critical_chain.missing_h3")}</h3>
        <ul>{#each buffers.chain_tasks_missing_estimates ?? [] as task (task.pid)}<li>{task.title ?? task.pid}</li>{/each}</ul>
      {/if}
    {:else}
      <p>{t("p.critical_chain.buffers_lede", { rule: ruleText(buffers.buffer_rule) })}</p>
      {#if buffers.project_buffer}
        <p>{t("p.critical_chain.project_buffer", { days: buffers.project_buffer.days, finish: buffers.project_buffer.chain_finish, with: buffers.project_buffer.finish_with_buffer })}</p>
        <p class="small">{t("p.critical_chain.project_safety", { days: buffers.project_buffer.safety_removed_days })}</p>
      {/if}
      <figure class="bars" aria-label={t("p.critical_chain.buffers_h2")}>
        <ol>
          {#each rows as r (r.key)}
            <li>
              <span class="name">{r.label}</span>
              <span class="track" aria-hidden="true">
                <span class="bar solid" style:left="0" style:width="{(r.work / scale) * 100}%"></span>
                {#if r.buffer}<span class="bar dashed" style:left="{(r.work / scale) * 100}%" style:width="{(r.buffer / scale) * 100}%"></span>{/if}
              </span>
            </li>
          {/each}
        </ol>
      </figure>

      <h3>{t("p.critical_chain.feeding_h3")}</h3>
      {#if (buffers.feeding_buffers ?? []).length === 0}
        <p>{t("p.critical_chain.feeding_none")}</p>
      {:else}
        <DataTable label={t("p.critical_chain.feeding_h3")} columns={feedingColumns} rows={buffers.feeding_buffers ?? []} rowId={(_r, i) => String(i)} />
      {/if}

      <h3>{t("p.critical_chain.resource_h3")}</h3>
      <p>{t("p.critical_chain.resource_lede", { lead: buffers.resource_buffer_lead_days })}</p>
      {#if (buffers.resource_buffers ?? []).length > 0}
        <DataTable label={t("p.critical_chain.resource_h3")} columns={resourceColumns} rows={buffers.resource_buffers ?? []} rowId={(r) => String(rowOf<ResBuf>(r).task.pid)} />
      {/if}

      {#if (buffers.scheduled_at_likely_duration ?? []).length > 0}
        <h3>{t("p.critical_chain.likely_h3")}</h3>
        <ul>{#each buffers.scheduled_at_likely_duration ?? [] as task (task.pid)}<li>{task.title ?? task.pid}</li>{/each}</ul>
      {/if}
    {/if}
  </section>
{/if}

<style>
  .bars ol { list-style: none; padding: 0; margin: 0 0 1rem; }
  .bars li { display: grid; grid-template-columns: minmax(8rem, 14rem) 1fr; gap: 0.5rem; align-items: center; margin-block: 0.25rem; }
  .track { position: relative; block-size: 1.1rem; background: #f3f3f3; border-radius: 2px; }
  .bar { position: absolute; top: 0; bottom: 0; border: 2px solid #1b3a6b; box-sizing: border-box; }
  .bar.solid { background: #5b7fb8; }
  /* A dashed outline and stripes so the buffer is told apart without colour. */
  .bar.dashed { background: repeating-linear-gradient(45deg, #fff, #fff 4px, #d7dfee 4px, #d7dfee 8px); border-style: dashed; }
</style>
