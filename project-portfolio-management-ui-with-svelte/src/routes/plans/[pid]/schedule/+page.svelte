<!--
  Plan schedule (`/plans/[pid]/schedule`, PPM-6): member timeframes as CSS
  bars, critical-path badges, finish-start violations, and undated
  members. Any plan may contain children, so this works for any plan.
-->
<script lang="ts">
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { formatDate } from "#lib/locale-format.js";
  import { onMount } from "svelte";
  import { resource } from "#lib/resource.svelte.js";
  import { page } from "$app/state";
  import { PpmClient } from "#lib/api/ppm.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");

  const loaded = resource(() => ppm.schedule(pid), "ppm.common.loadFailed");
  const schedule = $derived(loaded.data);
  const error = $derived(loaded.error);

  onMount(() => void loaded.reload());

  /** Bar geometry: percent offsets across the min..max date span. */
  const span = $derived.by(() => {
    if (!schedule) return null;
    const dates = schedule.items
      .flatMap((item) => [item.start, item.end])
      .filter((value): value is string => value !== null)
      .map((value) => new Date(value).getTime());
    if (dates.length === 0) return null;
    const min = Math.min(...dates);
    const max = Math.max(...dates);
    return { min, width: Math.max(max - min, 1) };
  });

  function bar(start: string | null, end: string | null) {
    if (!span || !start || !end) return null;
    const from = ((new Date(start).getTime() - span.min) / span.width) * 100;
    const to = ((new Date(end).getTime() - span.min) / span.width) * 100;
    return { left: `${from.toFixed(1)}%`, width: `${Math.max(to - from, 0.5).toFixed(1)}%` };
  }
</script>

<svelte:head><title>{t("p.plans.pid.schedule.schedule_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref(`/plans/${pid}/governance`)}>{t("ppm.common.governance")}</a></p>
<h1>{t("ppm.schedule.title")}</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if schedule}
  {#if schedule.violations.length > 0}
    <div class="banner" role="alert" data-testid="violations">
      {#each schedule.violations as violation (violation.edge_pid)}
        <p class="small">
          {t("p.plans.pid.schedule.dependency_violated_successor_starts", { actual_start: formatDate(violation.actual_start), earliest_start: formatDate(violation.earliest_start) })}
        </p>
      {/each}
    </div>
  {/if}

  <div class="gantt">
    {#each schedule.items as item (item.pid)}
      <div class="lane">
        <span class="label small">
          <a href={localeHref(`/plans/${item.pid}/governance`)}>{item.name}</a>
          {#if item.on_critical_path}<span class="chip critical">{t("ppm.schedule.critical")}</span>{/if}
          {#if item.stage}<span class="chip">{item.stage}</span>{/if}
        </span>
        <span class="track">
          {#if bar(item.start, item.end)}
            <span class="bar" class:critical={item.on_critical_path} style:left={bar(item.start, item.end)?.left} style:width={bar(item.start, item.end)?.width}
            ></span>
          {:else}
            <span class="small muted">{t("ppm.schedule.undated")}</span>
          {/if}
        </span>
      </div>
    {/each}
  </div>

  {#if schedule.edges.length > 0}
    <h2>{t("ppm.schedule.dependencies")}</h2>
    <ul class="small">
      {#each schedule.edges as edge (edge.pid)}
        <li>{edge.predecessor_pid.slice(0, 8)} → {edge.successor_pid.slice(0, 8)} (+{edge.lag_days}d)</li>
      {/each}
    </ul>
  {/if}
{/if}

<style>
  .gantt { display: flex; flex-direction: column; gap: 0.3rem; margin: 1rem 0; }
  .lane { display: grid; grid-template-columns: minmax(200px, 1fr) 3fr; align-items: center; gap: 0.6rem; }
  .track { position: relative; height: 1.1rem; background: var(--surface-2, #f0f0f0); border-radius: 4px; }
  .bar {
    position: absolute;
    top: 0.15rem;
    height: 0.8rem;
    border-radius: 4px;
    background: var(--accent, #35547c);
    opacity: 0.85;
  }
  .bar.critical { background: #a4262c; }
  .chip {
    display: inline-block;
    border: 1px solid var(--border, #ccc);
    border-radius: 999px;
    padding: 0 0.5rem;
    font-size: 0.72rem;
  }
  .chip.critical { color: #a4262c; border-color: #a4262c; }
</style>
