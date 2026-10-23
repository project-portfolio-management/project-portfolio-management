<!--
  The demonstration tour (`/demo`): where the sign-in on a demonstration stack lands. Every feature, grouped,
  each with what to look for in the fictitious Northgate data and a link. The copy is in the catalogue
  (`p.demo.*`); the screens it points at are in `$lib/demo-tour.ts`.
-->
<script lang="ts">
  import { t, tDynamic, localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PlanRepository } from "#lib/api/plans.js";
  import { GROUPS, FEATURES, demoHref } from "#lib/demo-tour.js";

  const NORTH = "Interlocking Replacement, North Line";
  const repo = PlanRepository.withFetch();
  let northPid = $state<string | null>(null);
  let looked = $state(false);

  onMount(async () => {
    try {
      northPid = (await repo.listPage()).items.find((p) => p.name === NORTH)?.pid ?? null;
    } catch {
      northPid = null;
    } finally {
      looked = true;
    }
  });

  const word = (key: string) => tDynamic(`p.demo.${key}`);
  const shown = (g: string) =>
    FEATURES.filter((f) => f.group === g)
      .map((f) => ({ f, href: demoHref(f, northPid) }))
      .filter((x) => x.href !== null);
</script>

<svelte:head><title>{t("p.demo.tour_title")}</title></svelte:head>

<h1>{t("p.demo.tour_h1")}</h1>
<p class="lede">{t("p.demo.tour_lede")}</p>
<section aria-labelledby="demo-story">
  <h2 id="demo-story">{t("p.demo.tour_story_h2")}</h2>
  <p>{t("p.demo.tour_story")}</p>
</section>
{#if looked && !northPid}<p class="banner" role="note">{t("p.demo.tour_no_plan")}</p>{/if}

<nav aria-label={t("p.demo.tour_toc")}>
  <ul class="toc">
    {#each GROUPS as g (g)}<li><a href={`#demo-${g}`}>{word(`g_${g}`)}</a></li>{/each}
  </ul>
</nav>

{#each GROUPS as g (g)}
  {@const items = shown(g)}
  {#if items.length > 0}
    <section aria-labelledby={`demo-${g}`}>
      <h2 id={`demo-${g}`}>{word(`g_${g}`)}</h2>
      <ul class="cards">
        {#each items as { f, href } (f.id)}
          <li class="card">
            <h3>{word(`f_${f.id.replaceAll("-", "_")}_t`)}</h3>
            <p>{word(`f_${f.id.replaceAll("-", "_")}_d`)}</p>
            <a class="button" href={localeHref(href ?? "/")} aria-label={`${t("p.demo.tour_open")}: ${word(`f_${f.id.replaceAll("-", "_")}_t`)}`}>{t("p.demo.tour_open")}</a>
          </li>
        {/each}
      </ul>
    </section>
  {/if}
{/each}

<style>
  .toc { display: flex; flex-wrap: wrap; gap: 0.5rem 1.25rem; list-style: none; padding: 0; margin: 1rem 0; }
  .cards { display: grid; grid-template-columns: repeat(auto-fill, minmax(18rem, 1fr)); gap: 1rem; list-style: none; padding: 0; }
  .card { border: 1px solid var(--lily-border-strong, #ccc); border-radius: 6px; padding: 0.75rem 1rem; display: flex; flex-direction: column; gap: 0.5rem; }
  .card h3 { margin: 0; font-size: 1.05rem; }
  .card p { margin: 0; flex: 1; }
  section { margin-block-start: 1.5rem; }
</style>
