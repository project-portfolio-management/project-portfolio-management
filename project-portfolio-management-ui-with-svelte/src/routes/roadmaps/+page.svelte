<!--
  Product roadmaps (`/roadmaps`, RE-3, RE-5; NC-1 slice 12 part l): the list an editor opens first. The roadmaps due a
  review (`GET /api/public-roadmaps/stale`: never reviewed first, then stale past the deployment's days), every roadmap of
  a live plan the reader may read, published or not (`GET /api/public-roadmaps`, every page: its title linking to its
  editor, published or not, how fresh, how many items placed and not, its family), and the product families
  (`GET /api/roadmap-families`, every page). Nothing is changed here.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { errorText } from "#lib/error-text.js";
  import { localeHref, t, tDynamic } from "#lib/i18n.svelte.js";
  import { formatDate, formatInteger } from "#lib/locale-format.js";
  import { PpmClient, type ListedRoadmap, type RoadmapFamily, type StaleRoadmaps } from "#lib/api/ppm.js";

  const ppm = PpmClient.withFetch();
  let all = $state<ListedRoadmap[] | null>(null);
  let stale = $state<StaleRoadmaps | null>(null);
  let families = $state<RoadmapFamily[] | null>(null);
  let loadError = $state<string | null>(null);

  onMount(async () => {
    try {
      [all, stale, families] = await Promise.all([ppm.publicRoadmaps(), ppm.staleRoadmaps(), ppm.roadmapFamilies()]);
    } catch (err) {
      loadError = errorText(err, "p.product_roadmaps.could_not_load");
    }
  });

  const editor = (planPid: string) => localeHref(`/plans/${planPid}/roadmap`);
  const freshWord = (s: string | null) => tDynamic(`p.product_roadmap.fresh.${s ?? "unknown"}`);
</script>

<svelte:head><title>{t("p.product_roadmaps.title_ppm")}</title></svelte:head>

<h1>{t("p.product_roadmaps.title")}</h1>
<p class="lede">{t("p.product_roadmaps.intro")}</p>
{#if loadError}<p class="banner" role="alert">{loadError}</p>{/if}

{#if !all && !loadError}
  <p class="muted">{t("p.product_roadmaps.loading")}</p>
{:else if all && stale && families}
  <section class="surface stack" aria-labelledby="due-h">
    <h2 id="due-h">{t("p.product_roadmaps.due_h")}</h2>
    <p class="small muted">{t("p.product_roadmap.stale_rule", { days: formatInteger(stale.stale_after_days) })}</p>
    {#if stale.roadmaps.length === 0}
      <p data-testid="due-none">{t("p.product_roadmaps.due_none")}</p>
    {:else}
      <ul data-testid="due">
        {#each stale.roadmaps as r (r.plan_pid)}
          <li>
            <a href={editor(r.plan_pid)}>{r.title}</a> ·
            {freshWord(r.state)}{#if r.reviewed_on} · {t("p.product_roadmap.reviewed_on", { date: formatDate(r.reviewed_on) })}{/if}
          </li>
        {/each}
      </ul>
    {/if}
  </section>

  <section class="surface stack" aria-labelledby="all-h">
    <h2 id="all-h">{t("p.product_roadmaps.all_h")}</h2>
    {#if all.length === 0}
      <p data-testid="all-none">{t("p.product_roadmaps.all_none")}</p>
    {:else}
      <ul class="items" data-testid="all">
        {#each all as r (r.pid)}
          <li>
            <p><a href={editor(r.plan_pid)}>{r.title}</a> <span class="small muted">/{r.slug}</span></p>
            <p class="small">
              {r.published ? t("p.product_roadmap.published") : t("p.product_roadmap.not_published")} · {freshWord(r.freshness)} ·
              {t("p.product_roadmaps.counts", { placed: formatInteger(r.items.placed), unplaced: formatInteger(r.items.unplaced) })}
              {#if r.family?.name} · {t("p.product_roadmaps.in_family", { family: r.family.name })}{/if}
            </p>
          </li>
        {/each}
      </ul>
    {/if}
  </section>

  <section class="surface stack" aria-labelledby="fam-h">
    <h2 id="fam-h">{t("p.product_roadmaps.families_h")}</h2>
    {#if families.length === 0}
      <p data-testid="families-none">{t("p.product_roadmaps.families_none")}</p>
    {:else}
      <ul data-testid="families">
        {#each families as f (f.plan_pid)}
          <li><a href={localeHref(`/plans/${f.plan_pid}`)}>{f.name}</a> · {t("p.product_roadmaps.products", { n: formatInteger(f.products) })}</li>
        {/each}
      </ul>
    {/if}
  </section>
{/if}

<style>
  .items {
    list-style: none;
    padding: 0;
    display: grid;
    gap: 0.75rem;
  }
  h2 {
    font-size: 1.1rem;
  }
</style>
