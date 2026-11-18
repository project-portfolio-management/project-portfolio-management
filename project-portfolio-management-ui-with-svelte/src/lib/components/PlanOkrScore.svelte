<!--
  A plan's goal score (`/okrs`, NC-1 slice 12 part j, KO-2), read on asking: `GET /api/plans/{pid}/okr`, the score
  weighted by how strongly each objective is aligned with the plan, or "not measured" (never 0%); each objective
  named from the page's list with its weight and score, those left out of the score because nothing in them can be
  measured said apart; an objective's key results read on asking (`GET /api/objectives/{pid}/key-results`), each with
  its progress or why it has none. The plan list from `GET /api/plans`, the choice disabled until read. Rules in
  `src/lib/okr-reads-view.ts`.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { errorText } from "#lib/error-text.js";
  import { t, tn } from "#lib/i18n.svelte.js";
  import { formatDateTime, formatInteger } from "#lib/locale-format.js";
  import { PlanRepository } from "#lib/api/plans.js";
  import type { PlanRef } from "#lib/api/types.js";
  import { PpmClient, type ObjectiveKeyResult, type PlanOkr } from "#lib/api/ppm.js";
  import { percentText } from "#lib/okr-view.js";
  import { okrReadRefusalKey, scoredSplit, unmeasurableKey } from "#lib/okr-reads-view.js";

  let { titleOf }: { titleOf: (objectivePid: string) => string | undefined } = $props();

  const ppm = PpmClient.withFetch();
  let plans = $state<PlanRef[] | null>(null);
  let plan = $state("");
  let data = $state<PlanOkr | null>(null);
  let problem = $state<string | null>(null);
  let busy = $state(false);
  let keyResults = $state<Record<string, ObjectiveKeyResult[]>>({});

  onMount(async () => {
    try {
      plans = await PlanRepository.withFetch().list();
    } catch (err) {
      plans = [];
      problem = errorText(err, "p.planokr.could_not_load_plans");
    }
  });

  async function read(e: SubmitEvent) {
    e.preventDefault();
    if (!plan) {
      problem = t("p.planokr.problem_plan");
      return;
    }
    problem = null;
    busy = true;
    keyResults = {};
    try {
      data = await ppm.planOkr(plan);
    } catch (err) {
      data = null;
      const key = okrReadRefusalKey(err);
      problem = key ? t(key) : errorText(err, "p.planokr.could_not");
    } finally {
      busy = false;
    }
  }

  async function readKeyResults(e: Event, objectivePid: string) {
    if (!(e.currentTarget as HTMLDetailsElement).open || keyResults[objectivePid]) return;
    try {
      keyResults[objectivePid] = await ppm.objectiveKeyResults(objectivePid);
    } catch (err) {
      const key = okrReadRefusalKey(err);
      problem = key ? t(key) : errorText(err, "p.planokr.could_not");
    }
  }

  const name = (pid: string) => titleOf(pid) ?? t("p.planokr.unnamed", { id: pid.slice(0, 8) });
  const split = $derived(data ? scoredSplit(data.objectives) : null);
</script>

<section class="surface" aria-labelledby="planokr-h" data-testid="plan-okr">
  <h2 id="planokr-h">{t("p.planokr.heading")}</h2>
  <p class="small muted">{t("p.planokr.says")}</p>
  <form class="row" novalidate onsubmit={read} aria-label={t("p.planokr.form")}>
    <label>{t("p.planokr.f_plan")}
      <select bind:value={plan} disabled={plans === null}>
        <option value="">{t("p.planokr.choose_plan")}</option>
        {#each plans ?? [] as p (p.pid)}<option value={p.pid}>{p.name}</option>{/each}
      </select>
    </label>
    <button class="button small" type="submit" disabled={busy || plans === null}>{t("p.planokr.read")}</button>
  </form>
  {#if problem}<p class="banner error" role="alert">{problem}</p>{/if}
  {#if data && split}
    <p data-testid="plan-okr-score">
      <strong>{data.measured ? t("p.planokr.score", { score: percentText(data.score_basis_points) }) : t("p.planokr.not_measured")}</strong>
      <span class="small muted"> · {t("p.planokr.as_of", { at: formatDateTime(data.as_of) })}</span>
    </p>
    {#if data.objectives.length === 0}
      <p>{t("p.planokr.no_objectives")}</p>
    {:else}
      {#if split.left_out.length > 0}
        <p>{tn("p.planokr.left_out", split.left_out.length, { n: formatInteger(split.left_out.length) })}</p>
      {/if}
      <ul class="objectives">
        {#each data.objectives as o (o.objective_pid)}
          <li>
            <strong>{name(o.objective_pid)}</strong>:
            {o.measured ? percentText(o.score_basis_points) : t("p.planokr.objective_not_measured")}
            · {t("p.planokr.weight", { weight: formatInteger(o.weight) })}
            · {tn("p.planokr.key_results", o.key_results, { n: formatInteger(o.key_results) })}
            {#if !o.single_currency}· {t("p.planokr.currencies")}{/if}
            {#if o.key_results > 0}
              <details ontoggle={(e) => void readKeyResults(e, o.objective_pid)}>
                <summary>{t("p.planokr.its_key_results", { title: name(o.objective_pid) })}</summary>
                {#if keyResults[o.objective_pid]}
                  <ul>
                    {#each keyResults[o.objective_pid] ?? [] as kr (kr.key_result.pid)}
                      <li>
                        {kr.key_result.title}:
                        {kr.progress_basis_points !== null ? percentText(kr.progress_basis_points) : t(unmeasurableKey(kr.unmeasurable))}
                      </li>
                    {/each}
                  </ul>
                {/if}
              </details>
            {/if}
          </li>
        {/each}
      </ul>
    {/if}
  {/if}
</section>

<style>
  section { margin-block: 1rem; }
  .row { display: flex; gap: 0.75rem; align-items: end; flex-wrap: wrap; }
  .row label { display: grid; gap: 0.2rem; font-size: 0.85rem; }
  .objectives { padding-inline-start: 1.25rem; }
</style>
