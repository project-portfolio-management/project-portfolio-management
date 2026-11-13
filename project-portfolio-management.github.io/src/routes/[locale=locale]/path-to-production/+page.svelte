<!--
  The path to production (`locales/<locale>/path-to-production/index.md`): what production means, and an honest list of
  what it would take, each item with the project's readiness estimate on a Lily slider ($lib/ReadinessList.svelte).
-->
<script lang="ts">
  import ReadinessList from "#lib/ReadinessList.svelte";
  import { siteDir, siteTag, t } from "#lib/i18n.svelte.js";
  import { REPO_URL } from "#lib/site.js";
  import type { PageProps } from "./$types";

  let { data }: PageProps = $props();

  // Where each run of items starts, so every slider on the page has its own id.
  const offsets = $derived.by(() => {
    let n = 0;
    return data.parts.map((p) => {
      const start = n;
      if (p.kind === "items") n += p.items.length;
      return start;
    });
  });
</script>

<svelte:head><title>{data.title} · {t("brand")}</title></svelte:head>

<!-- The HTML is rendered at build time from the repository's own Markdown, with raw HTML escaped
     ($lib/server/content.ts), so nothing a reader supplies reaches it. -->
<article class="doc">
  {#key data.path}
    {#each data.parts as part, p (p)}
      {#if part.kind === "html"}
        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
        {@html part.html}
      {:else}
        <ReadinessList items={part.items} offset={offsets[p]} />
      {/if}
    {/each}
  {/key}
</article>
<p class="source" lang={siteTag()} dir={siteDir()}>
  {t("doc.source")} <a href={`${REPO_URL}/blob/main/${data.path}`}><code dir="ltr">{data.path}</code></a>
</p>

<style>
  .source {
    margin-block-start: 2.5rem;
    padding-block-start: 0.75rem;
    border-block-start: 1px solid var(--mxi-color-border);
    color: var(--mxi-color-muted);
    font-size: 0.875rem;
  }
</style>
