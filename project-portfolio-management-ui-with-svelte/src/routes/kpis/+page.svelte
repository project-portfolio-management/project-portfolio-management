<!--
  KPIs (`/kpis`): standing measures with a target and a red line. Status comes
  from the latest reading and is worked out each time — never stored. "No data
  yet" is its own state, not red and not green; status is a word and a shape,
  never colour alone.

  English-first, like the other recent views.
-->
<script lang="ts">
  import { t } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient, type KpiList, type KpiStatus, type KpiDirection, type KpiCadence } from "#lib/api/ppm.js";
  import KpiCard from "#lib/components/KpiCard.svelte";
  import { STATUS_SYMBOL, summaryParts } from "#lib/kpi-view.js";

  const ppm = PpmClient.withFetch();
  let list = $state<KpiList | null>(null);
  let error = $state<string | null>(null);
  let status = $state<KpiStatus | "">("");

  async function refresh() {
    try {
      list = await ppm.kpis(status ? { status } : {});
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.kpis.could_not_load_the_kpis");
    }
  }
  onMount(refresh);

  let name = $state("");
  let unit = $state("");
  let direction = $state<KpiDirection>("higher_better");
  let target = $state("");
  let red = $state("");
  let cadence = $state<KpiCadence | "">("");
  let planPid = $state("");
  async function addKpi(e: SubmitEvent) {
    e.preventDefault();
    try {
      await ppm.createKpi({
        name,
        unit: unit.trim() || null,
        direction,
        target_value: Number(target),
        red_value: Number(red),
        cadence: cadence || null,
        plan_pid: planPid.trim() || null,
      });
      name = "";
      target = "";
      red = "";
      await refresh();
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.kpis.could_not_define_the_kpi");
    }
  }
</script>

<svelte:head><title>{t("p.kpis.kpis_ppm")}</title></svelte:head>

<h1>{t("p.kpis.key_measures_kpis")}</h1>
<p class="lede">{t("p.kpis.something_you_track_over_time")}</p>
<p class="small muted">
  {t("p.kpis.standing_measures_green_is_at")}
</p>

{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if list}
  <ul class="summary" aria-label={t("p.kpis.kpis_by_status")}>
    {#each summaryParts(list.summary) as part (part.status)}
      <li><span aria-hidden="true">{STATUS_SYMBOL[part.status]}</span> {part.label}: <strong>{part.count}</strong></li>
    {/each}
  </ul>
{/if}

<form class="filter" onsubmit={(e) => { e.preventDefault(); void refresh(); }}>
  <label>{t("p.kpis.show")}
    <select bind:value={status} onchange={refresh}>
      <option value="">{t("p.kpis.all")}</option>
      <option value="red">{t("p.kpis.off_target")}</option>
      <option value="amber">{t("p.kpis.watch")}</option>
      <option value="no_data">{t("p.kpis.no_data_yet")}</option>
      <option value="green">{t("p.kpis.on_target")}</option>
    </select>
  </label>
</form>

<details class="surface">
  <summary>{t("p.kpis.define_a_key_measure_kpi")}</summary>
  <form class="filter" onsubmit={addKpi}>
    <label>{t("p.kpis.name")} <input type="text" required bind:value={name} /></label>
    <label>{t("p.kpis.unit")} <input type="text" bind:value={unit} placeholder={t("p.kpis.percent_defects_gbp")} /></label>
    <label>{t("p.kpis.which_way_is_good")}
      <select bind:value={direction}>
        <option value="higher_better">{t("p.kpis.higher_is_better")}</option>
        <option value="lower_better">{t("p.kpis.lower_is_better")}</option>
      </select>
    </label>
    <label>{t("p.kpis.target")} <input type="number" step="1" required bind:value={target} /></label>
    <label>{t("p.kpis.red_line")} <input type="number" step="1" required bind:value={red} /></label>
    <label>{t("p.kpis.read_how_often")}
      <select bind:value={cadence}>
        <option value="">{t("p.kpis.not_set")}</option>
        <option value="daily">{t("p.kpis.daily")}</option>
        <option value="weekly">{t("p.kpis.weekly")}</option>
        <option value="monthly">{t("p.kpis.monthly")}</option>
        <option value="quarterly">{t("p.kpis.quarterly")}</option>
      </select>
    </label>
    <label>{t("p.kpis.plan_optional_pid")} <input type="text" bind:value={planPid} placeholder={t("p.kpis.blank_for_the_whole_portfolio")} /></label>
    <button class="button small primary" type="submit">{t("p.kpis.define")}</button>
  </form>
  <p class="small muted">{t("p.kpis.the_red_line_must_be")}</p>
</details>

{#if list}
  {#if list.kpis.length === 0}
    <p>{status ? t("p.kpis.none_with_status") : t("p.kpis.none")}</p>
  {:else}
    {#each list.kpis as item (item.kpi.pid)}
      <KpiCard {item} {ppm} onchange={refresh} onerror={(m) => (error = m)} />
    {/each}
  {/if}
{:else if !error}
  <p class="muted">{t("p.kpis.loading")}</p>
{/if}

<style>
  .summary {
    display: flex;
    flex-wrap: wrap;
    gap: 1.25rem;
    list-style: none;
    padding: 0;
  }
  .filter {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: end;
    margin: 0.75rem 0;
  }
  .filter label {
    display: grid;
    gap: 0.2rem;
    font-size: 0.85rem;
  }
</style>
