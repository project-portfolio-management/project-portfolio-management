<!--
  Data-driven prioritization (`/prioritization`): the Smart Score queue.
  Highest score first, with each plan's evidence coverage on show, and
  an expandable per-component breakdown — a score that ranks somebody's
  work has to be able to explain itself.

  Unscored plans keep the `unscored` band and sort last: no evidence is
  reported as no evidence, never as a low score.
-->
<script lang="ts">
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { enumLabel } from "#lib/enum-labels.js";
  import { onMount } from "svelte";
  import {
    CapabilityClient,
    type Prioritization,
    type PlanSmartScore,
  } from "#lib/api/capabilities.js";

  const api = CapabilityClient.withFetch();
  let view = $state<Prioritization | null>(null);
  let error = $state<string | null>(null);
  let band = $state("");
  let expanded = $state<string | null>(null);
  let detail = $state<PlanSmartScore | null>(null);

  async function refresh() {
    try {
      view = await api.prioritization({ band: band || undefined });
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.prioritization.could_not_load_the_queue");
    }
  }

  async function toggle(pid: string) {
    if (expanded === pid) {
      expanded = null;
      detail = null;
      return;
    }
    expanded = pid;
    detail = null;
    try {
      detail = await api.smartScore(pid);
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.prioritization.could_not_load_the_score");
    }
  }

  onMount(refresh);
</script>

<svelte:head><title>{t("p.prioritization.prioritization_ppm")}</title></svelte:head>

<h1>{t("p.prioritization.prioritization")}</h1>
<p class="small muted">
  {t("p.prioritization.smart_score_over_roi_strategic")} <em>{t("p.prioritization.coverage")}</em> {t("p.prioritization.shows_how_much_of_the")}
</p>

{#if error}<p class="banner" role="alert">{error}</p>{/if}

<form
  class="row"
  onsubmit={(e) => {
    e.preventDefault();
    refresh();
  }}
>
  <label class="small">
    {t("p.prioritization.band")}
    <select bind:value={band}>
      <option value="">{t("p.prioritization.all")}</option>
      <option value="high">{t("p.prioritization.high")}</option>
      <option value="medium">{t("p.prioritization.medium")}</option>
      <option value="low">{t("p.prioritization.low")}</option>
      <option value="unscored">{t("p.prioritization.unscored")}</option>
    </select>
  </label>
  <button class="button small primary" type="submit">{t("p.prioritization.apply")}</button>
</form>

{#if view}
  <p class="small muted">{t("p.prioritization.of_plan_s", { length: view.plans.length, scored_of: view.scored_of })}</p>
  <table>
    <thead>
      <tr>
        <th>{t("p.prioritization.plan")}</th><th>{t("p.prioritization.score")}</th><th>{t("p.prioritization.band_2")}</th><th>{t("p.prioritization.coverage_2")}</th><th>{t("p.prioritization.missing_evidence")}</th><th></th>
      </tr>
    </thead>
    <tbody>
      {#each view.plans as plan (plan.pid)}
        <tr>
          <td><a href={localeHref(`/plans/${plan.pid}`)}>{plan.name}</a></td>
          <td>{plan.score === null ? "—" : plan.score.toFixed(1)}</td>
          <td><span class={`chip ${plan.band}`}>{enumLabel("band", plan.band)}</span></td>
          <td>{Math.round(plan.coverage * 100)}%</td>
          <td class="small muted">
            {plan.missing_evidence.length === 0
              ? t("p.prioritization.none")
              : plan.missing_evidence.join(", ")}
          </td>
          <td>
            <button class="button small" type="button" onclick={() => toggle(plan.pid)}>
              {expanded === plan.pid ? t("p.prioritization.hide") : t("p.prioritization.explain")}
            </button>
          </td>
        </tr>
        {#if expanded === plan.pid}
          <tr>
            <td colspan="6">
              {#if detail}
                <table class="inner">
                  <thead>
                    <tr><th>{t("p.prioritization.component")}</th><th>{t("p.prioritization.weight")}</th><th>{t("p.prioritization.value")}</th><th>{t("p.prioritization.points")}</th></tr>
                  </thead>
                  <tbody>
                    {#each detail.smart_score.components as component (component.name)}
                      <tr>
                        <td>{component.name}</td>
                        <td>{Math.round(component.weight * 100)}%</td>
                        <td>{component.raw.toFixed(2)}</td>
                        <td>{component.contribution.toFixed(1)}</td>
                      </tr>
                    {/each}
                  </tbody>
                </table>
                {#if detail.smart_score.components.length === 0}
                  <p class="small muted">
                    {t("p.prioritization.no_evidence_for_any_component")}
                  </p>
                {/if}
              {:else}
                <p class="small muted">{t("p.prioritization.loading_the_breakdown")}</p>
              {/if}
            </td>
          </tr>
        {/if}
      {/each}
    </tbody>
  </table>
  {#if view.plans.length === 0}
    <p class="small muted">{t("p.prioritization.no_plans_match_that_filter")}</p>
  {/if}
{/if}

<style>
  .row { display: flex; gap: 0.5rem; flex-wrap: wrap; align-items: center; margin: 0.8rem 0; }
  .chip {
    display: inline-block;
    border-radius: 999px;
    padding: 0 0.5rem;
    font-size: 0.75rem;
    border: 1px solid currentColor;
  }
  .chip.high { color: #1d8a4e; }
  .chip.medium { color: #8a6d1d; }
  .chip.low { color: #a4262c; }
  .chip.unscored { color: var(--text-2, #666); }
  table.inner { width: 100%; font-size: 0.85rem; }
</style>
