<!--
  OKRs (`/okrs`): objectives and their key results across the portfolio, with
  scores derived on the server. Least progressed first; an objective nobody can
  measure yet says "Not measured" and sorts last — never 0%. Confidence sits
  beside a key result and is not part of any score.

  English-first, like the other recent views.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { t } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient, type OkrList } from "#lib/api/ppm.js";
  import ObjectiveCard from "#lib/components/ObjectiveCard.svelte";

  const ppm = PpmClient.withFetch();
  let list = $state<OkrList | null>(null);
  let error = $state<string | null>(null);
  let period = $state("");

  async function refresh() {
    try {
      list = await ppm.okrs(period.trim() || undefined);
      error = null;
    } catch (err) {
      error = errorText(err, "p.okrs.could_not_load_the_okrs");
    }
  }
  onMount(refresh);

  let title = $state("");
  let newPeriod = $state("");
  async function addObjective(e: SubmitEvent) {
    e.preventDefault();
    try {
      await ppm.createObjective({ title, period: newPeriod.trim() || null });
      title = "";
      await refresh();
    } catch (err) {
      error = errorText(err, "p.okrs.could_not_create_the_objective");
    }
  }
</script>

<svelte:head><title>{t("p.okrs.okrs_ppm")}</title></svelte:head>

<h1>{t("p.okrs.goals_and_key_results_okrs")}</h1>
<p class="lede">{t("p.okrs.a_goal_with_a_few")}</p>
<p class="small muted">
  {t("p.okrs.objectives_and_key_results_scores")}
</p>

{#if error}<p class="banner" role="alert">{error}</p>{/if}

<form class="filter" onsubmit={(e) => { e.preventDefault(); void refresh(); }}>
  <label>{t("p.okrs.period")} <input type="text" bind:value={period} placeholder={t("p.okrs.2026_q4_blank_for_all")} /></label>
  <button class="button small" type="submit">{t("p.okrs.show")}</button>
</form>

<details class="surface">
  <summary>{t("p.okrs.new_objective")}</summary>
  <form class="filter" onsubmit={addObjective}>
    <label>{t("p.okrs.title")} <input type="text" required bind:value={title} /></label>
    <label>{t("p.okrs.period")} <input type="text" bind:value={newPeriod} placeholder="2026-Q4" /></label>
    <button class="button small primary" type="submit">{t("p.okrs.create")}</button>
  </form>
</details>

{#if list}
  {#if list.objectives.length === 0}
    <p>{list.period ? t("p.okrs.none_for_period", { period: list.period }) : t("p.okrs.none")}</p>
  {:else}
    <p class="small muted" aria-live="polite">{(list.objectives.length === 1 ? t("p.okrs.objective_least_progressed_first_one", { length: list.objectives.length }) : t("p.okrs.objective_least_progressed_first_other", { length: list.objectives.length }))}</p>
    {#each list.objectives as item (item.objective.pid)}
      <ObjectiveCard {item} {ppm} onchange={refresh} onerror={(m) => (error = m)} />
    {/each}
  {/if}
{:else if !error}
  <p class="muted">{t("p.okrs.loading")}</p>
{/if}

<style>
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
