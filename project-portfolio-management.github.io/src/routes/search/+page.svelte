<script lang="ts">
  import { browser } from "$app/env";
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import { href } from "#lib/href.js";
  import { siteDir, siteTag, t } from "#lib/i18n.svelte.js";
  import { queryOf, search, searchUrl, type SearchDoc } from "#lib/search.js";

  // The page is prerendered with no query: the address is read only in the browser.
  const query = $derived(browser ? queryOf(page.url.search) : "");

  let docs = $state<SearchDoc[] | null>(null);
  let failed = $state(false);
  let field = $state("");

  const hits = $derived(docs ? search(docs, query) : []);

  $effect(() => {
    field = query;
  });

  // The index is large next to a page, so it is fetched on the first search, not with the page.
  $effect(() => {
    if (!query || docs) return;
    fetch(href("/search-index.json"))
      .then((res) => {
        if (!res.ok) throw new Error(String(res.status));
        return res.json() as Promise<SearchDoc[]>;
      })
      .then((loaded) => {
        docs = loaded;
        failed = false;
      })
      .catch(() => {
        failed = true;
      });
  });

  function submit(event: SubmitEvent) {
    event.preventDefault();
    void goto(searchUrl(href("/search/"), field));
  }
</script>

<svelte:head><title>{t("search.title")} · {t("brand")}</title></svelte:head>

<div lang={siteTag()} dir={siteDir()}>
<h1>{t("search.title")}</h1>
<form role="search" onsubmit={submit}>
  <label for="search-field">{t("search.label")}</label>
  <div class="search-row">
    <input id="search-field" type="search" bind:value={field} />
    <button type="submit">{t("search.button")}</button>
  </div>
</form>

<p role="status">
  {#if failed}
    {t("search.failed")}
  {:else if !query}
    {t("search.prompt")}
  {:else if !docs}
    {t("search.loading")}
  {:else if hits.length === 0}
    {t("search.none", { q: query })}
  {:else if hits.length === 1}
    {t("search.one", { q: query })}
  {:else}
    {t("search.many", { n: hits.length, q: query })}
  {/if}
</p>
</div>

{#if hits.length > 0}
  <ol class="search-hits">
    {#each hits as hit (hit.route)}
      <li lang={hit.lang} dir="auto">
        <a href={href(hit.route)}>{hit.title}</a>
        <p>{hit.excerpt}</p>
      </li>
    {/each}
  </ol>
{/if}

<style>
  label {
    display: block;
    margin-block-end: 0.25rem;
  }
  .search-row {
    display: flex;
    gap: 0.5rem;
  }
  input {
    flex: 1;
    min-width: 0;
    min-height: 2.5rem;
    padding-inline: 0.5rem;
    font: inherit;
    color: var(--mxi-color-fg);
    background: var(--mxi-color-bg);
    border: 1px solid var(--mxi-color-muted);
    border-radius: var(--mxi-radius);
  }
  button {
    min-height: 2.5rem;
    padding-inline: 0.875rem;
    font: inherit;
    color: var(--mxi-color-primary-fg);
    background: var(--mxi-color-primary);
    border: 0;
    border-radius: var(--mxi-radius);
  }
  .search-hits {
    margin: 0;
    padding: 0;
    list-style: none;
  }
  .search-hits li {
    padding-block: 0.75rem;
    border-block-end: 1px solid var(--mxi-color-border);
  }
  .search-hits p {
    margin: 0.25rem 0 0;
    color: var(--mxi-color-muted);
    overflow-wrap: anywhere;
  }
</style>
