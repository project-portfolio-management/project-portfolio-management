<!--
  FB-4: a programme's people impact from the workforce service (WPM), on the plan page. Asked only when the
  reader presses the button (the service asks WPM under the reader's own sign-in). WPM's answer is shown as it
  came: each part known or not known with WPM's reason, a count under WPM's floor shown as hidden (never zero),
  and no total. When WPM was not asked or did not answer, the reason is shown and no figure.
-->
<script lang="ts">
  import { PpmClient } from "#lib/api/ppm.js";
  import { errorText } from "#lib/error-text.js";
  import { t } from "#lib/i18n.svelte.js";
  import { peopleRows, type ImpactRow, type PeopleImpact } from "#lib/people-impact.js";

  let { pid }: { pid: string } = $props();

  let impact = $state<PeopleImpact | null>(null);
  let asking = $state(false);
  let failed = $state<string | null>(null);

  async function ask() {
    asking = true;
    failed = null;
    try {
      impact = await PpmClient.withFetch().planPeopleImpact(pid);
    } catch (err) {
      impact = null;
      failed = errorText(err, "p.people_impact.failed");
    } finally {
      asking = false;
    }
  }

  const fallback = $derived(t("p.people_impact.not_known"));
  const parts = $derived(
    impact
      ? [
          { title: t("p.people_impact.committed"), view: peopleRows(impact.workers_committed, fallback) },
          { title: t("p.people_impact.ending"), view: peopleRows(impact.contingent_ending_before_end, fallback) },
        ]
      : [],
  );
  const over = $derived(impact?.over_committed_pool_months ?? null);

  function rowName(r: ImpactRow): string {
    if (r.kind === "in_no_pool") return t("p.people_impact.in_no_pool");
    if (r.kind === "end_not_recorded") return t("p.people_impact.end_not_recorded");
    return r.name ?? "";
  }
  function rowCount(r: ImpactRow): string {
    return r.hidden || r.count === null
      ? t("p.people_impact.hidden", { floor: impact?.floor ?? "?" })
      : String(r.count);
  }
</script>

<section class="surface stack" data-testid="people-impact" aria-labelledby="people-impact-h">
  <h2 id="people-impact-h">{t("p.people_impact.title")}</h2>
  <p class="small">{t("p.people_impact.intro")}</p>
  <div>
    <button class="button small" onclick={ask} disabled={asking}>
      {asking ? t("p.people_impact.asking") : t("p.people_impact.ask")}
    </button>
  </div>
  {#if failed}<p class="banner" role="alert">{failed}</p>{/if}
  {#if impact}
    {#if impact.status === "live"}
      <p data-testid="people-impact-source">{t("p.people_impact.source", { date: impact.as_of ?? "?" })}</p>
    {:else}
      <p class="banner" role="note" data-testid="people-impact-unknown">
        {t("p.people_impact.unknown", { reason: impact.reason ?? fallback })}
      </p>
    {/if}
    <p class="small">
      {impact.plan_ends_on
        ? t("p.people_impact.end_sent", { date: impact.plan_ends_on })
        : (impact.plan_ends_on_reason ?? "")}
    </p>
    {#if impact.status === "live"}
      {#each parts as part (part.title)}
        <h3>{part.title}</h3>
        {#if part.view.known}
          {#if part.view.rows.length === 0}
            <p>{t("p.people_impact.no_pools")}</p>
          {:else}
            <ul>
              {#each part.view.rows as r, i (i)}
                <li>{rowName(r)}: <strong>{rowCount(r)}</strong></li>
              {/each}
            </ul>
          {/if}
        {:else}
          <p>{t("p.people_impact.part_unknown", { reason: part.view.reason })}</p>
        {/if}
      {/each}
      <h3>{t("p.people_impact.over")}</h3>
      {#if over?.known}
        <p>
          {t("p.people_impact.over_counts", {
            over: over.over ?? "?",
            considered: over.considered ?? "?",
            unknown: over.unknown ?? "?",
          })}
        </p>
      {:else}
        <p>{t("p.people_impact.part_unknown", { reason: over?.reason ?? fallback })}</p>
      {/if}
      <p class="small">{t("p.people_impact.no_total")}</p>
    {/if}
  {/if}
</section>
