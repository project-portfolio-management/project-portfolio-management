<!--
  The drum and the pipeline (`/pipeline`, CC-6). Which skill pool is the drum, and the staggered starts the
  service proposes for projects that need it at once. A proposal: nothing is moved. Each project's current
  and proposed use of the drum are two labelled bars, and a table gives the same dates with the reason.
-->
<script lang="ts">
  import { t } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient, type Pipeline, type SkillPool } from "#lib/api/ppm.js";
  import { layoutBars, outcomeLook } from "#lib/chain-view.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";

  const ppm = PpmClient.withFetch();
  let pipe = $state<Pipeline | null>(null);
  let pools = $state<SkillPool[]>([]);
  let pick = $state("");
  let error = $state<string | null>(null);
  let busy = $state(false);

  async function load() {
    try {
      pipe = await ppm.portfolioPipeline();
      pools = (await ppm.skillPools()).skill_pools.filter((p) => p.active);
      if (!pick && pools[0]) pick = pools[0].pid;
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.critical_chain.could_not_load_pipeline");
    }
  }
  onMount(() => void load());

  async function name(poolPid: string | null) {
    busy = true;
    try {
      await ppm.setPortfolioDrum(poolPid);
      await load();
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.critical_chain.could_not_save");
    } finally {
      busy = false;
    }
  }

  const projects = $derived(pipe?.projects ?? []);
  // Two bars per project on one shared scale: when it uses the drum now, and when it would as proposed.
  const bars = $derived(
    layoutBars(
      projects.flatMap((p) => {
        const end = shift(p.proposed_first_drum_use, span(p.first_drum_use, p.last_drum_use));
        return [
          { start: p.first_drum_use, end: p.last_drum_use },
          { start: p.proposed_first_drum_use, end },
        ];
      }),
    ),
  );
  type Proj = NonNullable<NonNullable<typeof pipe>["projects"]>[number];
  const columns: DataGridColumn[] = $derived([
    { id: "name", header: t("p.critical_chain.col_project"), rowHeader: true },
    { id: "needs", header: t("p.critical_chain.col_needs"), accessor: (r) => `${(r as unknown as Proj).first_drum_use} → ${(r as unknown as Proj).last_drum_use}` },
    { id: "peak_fte", header: t("p.critical_chain.col_peak"), align: "end" },
    { id: "outcome", header: t("p.critical_chain.col_proposal"), format: (v) => t(outcomeLook(String(v)).word as never), cell: outcomeCell, cellClass: (c) => `cell-${outcomeLook(String(c.value)).tone}` },
    { id: "proposed_first_drum_use", header: t("p.critical_chain.col_proposed") },
    { id: "delay_days", header: t("p.critical_chain.col_delay"), align: "end" },
    { id: "explanation", header: t("p.critical_chain.col_why") },
  ]);
  function span(a: string, b: string): number {
    return Math.round((Date.parse(b.slice(0, 10)) - Date.parse(a.slice(0, 10))) / 86_400_000);
  }
  function shift(d: string, days: number): string {
    return new Date(Date.parse(d.slice(0, 10)) + days * 86_400_000).toISOString().slice(0, 10);
  }
</script>

{#snippet outcomeCell({ value, formatted }: DataGridCellContext)}<span aria-hidden="true">{outcomeLook(String(value)).symbol}</span> {formatted}{/snippet}

<svelte:head><title>{t("p.critical_chain.pipeline_ppm")}</title></svelte:head>

<h1>{t("p.critical_chain.pipeline_h1")}</h1>
<p class="lede">{t("p.critical_chain.pipeline_lede")}</p>
<p class="banner" role="note">{t("p.critical_chain.advice")}</p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}
{#if !pipe && !error}<p class="muted">{t("p.critical_chain.loading")}</p>{/if}

{#if pipe}
  <section aria-labelledby="pl-drum">
    <h2 id="pl-drum">{t("p.critical_chain.drum_h2")}</h2>
    {#if pipe.state === "no_drum"}
      <p>{t("p.critical_chain.no_drum", { why: pipe.why ?? pipe.reason ?? "" })}</p>
    {:else if pipe.drum}
      <p>{pipe.drum.source === "named" ? t("p.critical_chain.drum_named", { name: pipe.drum.name }) : t("p.critical_chain.drum_overloaded", { name: pipe.drum.name })}</p>
      {#if pipe.state === "no_capacity"}
        <p>{t("p.critical_chain.no_capacity")}</p>
      {:else}
        <p>{t("p.critical_chain.capacity", { fte: pipe.capacity_fte ?? 0, days: pipe.capacity_buffer_days })}</p>
      {/if}
    {/if}
    <h3>{t("p.critical_chain.set_h3")}</h3>
    <form onsubmit={(e) => { e.preventDefault(); if (pick) void name(pick); }}>
      <label>{t("p.critical_chain.pick_pool")}
        <select bind:value={pick}>{#each pools as p (p.pid)}<option value={p.pid}>{p.name}</option>{/each}</select>
      </label>
      <button class="button" type="submit" disabled={busy || !pick}>{t("p.critical_chain.set_button")}</button>
      <button class="button" type="button" disabled={busy} onclick={() => void name(null)}>{t("p.critical_chain.clear_button")}</button>
    </form>
  </section>

  {#if pipe.state === "pipeline"}
    <section aria-labelledby="pl-projects">
      <h2 id="pl-projects">{t("p.critical_chain.projects_h2")}</h2>
      {#if projects.length === 0}
        <p>{t("p.critical_chain.projects_none")}</p>
      {:else}
        <figure class="bars" aria-label={t("p.critical_chain.pipeline_timeline")}>
          <figcaption class="small muted">{t("p.critical_chain.pipeline_timeline_caption")}
            <span class="legend"><span class="sw now" aria-hidden="true"></span> {t("p.critical_chain.bar_now")} <span class="sw prop" aria-hidden="true"></span> {t("p.critical_chain.bar_proposed")}</span>
          </figcaption>
          <ol>
            {#each projects as p, i (p.plan_pid)}
              <li>
                <span class="name">{p.name}</span>
                <span class="track" aria-hidden="true">
                  <span class="bar now" style:top="0" style:bottom="50%" style:left="{bars[2 * i]?.left ?? 0}%" style:width="{bars[2 * i]?.width ?? 0}%"></span>
                  <span class="bar proposed" style:top="50%" style:bottom="0" style:left="{bars[2 * i + 1]?.left ?? 0}%" style:width="{bars[2 * i + 1]?.width ?? 0}%"></span>
                </span>
              </li>
            {/each}
          </ol>
        </figure>
        <DataTable label={t("p.critical_chain.projects_h2")} columns={columns} rows={projects} rowId={(r) => String((r as unknown as Proj).plan_pid)} />
      {/if}
    </section>
  {/if}
{/if}

<style>
  .bars ol { list-style: none; padding: 0; margin: 0 0 1rem; }
  .bars li { display: grid; grid-template-columns: minmax(8rem, 14rem) 1fr; gap: 0.5rem; align-items: center; margin-block: 0.35rem; }
  .track { position: relative; block-size: 1.6rem; background: #f3f3f3; border-radius: 2px; }
  .bar { position: absolute; border: 2px solid #1b3a6b; box-sizing: border-box; }
  .bar.now { background: #5b7fb8; }
  .bar.proposed { background: repeating-linear-gradient(45deg, #fff, #fff 4px, #d7dfee 4px, #d7dfee 8px); border-style: dashed; }
  .legend { margin-inline-start: 0.75rem; white-space: nowrap; }
  .sw { display: inline-block; inline-size: 1.2rem; block-size: 0.6rem; border: 2px solid #1b3a6b; box-sizing: border-box; vertical-align: middle; }
  .sw.now { background: #5b7fb8; }
  .sw.prop { background: repeating-linear-gradient(45deg, #fff, #fff 3px, #d7dfee 3px, #d7dfee 6px); border-style: dashed; }
</style>
