<!--
  Plan search (`/search?<terms>`): where the header's search picker goes. The picker sends the bare query
  (`/search?foo`), so the terms are the whole query string.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { page } from "$app/state";
  import { PlanRepository } from "#lib/api/plans.js";
  import type { PlanRef } from "#lib/api/types.js";

  const repo = PlanRepository.withFetch();
  // The picker's query is bare: `?foo%20bar` is "foo bar". A `?q=foo` link works as well.
  const query = $derived.by(() => {
    const raw = page.url.search.replace(/^\?/, "");
    const text = raw.startsWith("q=") ? raw.slice(2) : raw;
    try {
      return decodeURIComponent(text.replace(/\+/g, " ")).trim();
    } catch {
      return text.trim();
    }
  });
  let results = $state<PlanRef[] | null>(null);
  let error = $state<string | null>(null);
  let asked = $state("");

  $effect(() => {
    const q = query;
    asked = q;
    results = null;
    error = null;
    if (!q) return;
    repo.search(q).then(
      (r) => { if (asked === q) results = r; },
      (err) => { if (asked === q) error = errorText(err, "p.search.could_not_search"); },
    );
  });
</script>

<svelte:head><title>{t("p.search.title")}</title></svelte:head>

<h1>{t("p.search.h1")}</h1>
<p class="lede">{t("p.search.lede")}</p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}
{#if !query}
  <p>{t("p.search.none_asked")}</p>
{:else if !results && !error}
  <p class="muted">{t("p.search.searching")}</p>
{:else if results}
  <p role="status">{results.length === 0 ? t("p.search.no_results", { q: query }) : t("p.search.results", { n: results.length, q: query })}</p>
  <ul>
    {#each results as r (r.pid)}
      <li><a href={localeHref(`/plans/${r.pid}`)}>{r.name}</a></li>
    {/each}
  </ul>
{/if}
