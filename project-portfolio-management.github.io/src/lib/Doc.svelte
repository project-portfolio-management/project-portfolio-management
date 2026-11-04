<script lang="ts">
  import { page } from "$app/state";
  import { siteTag, t } from "#lib/i18n.svelte.js";
  import { REPO_URL } from "#lib/site.js";

  let { html, path }: { html: string; path: string } = $props();
</script>

{#if siteTag() !== "en" && !page.data.locale}
  <p class="english-only" role="note" lang={siteTag()}>{t("doc.englishOnly")}</p>
{/if}

<!-- The HTML is rendered at build time from the repository's own Markdown, with raw HTML escaped
     ($lib/server/content.ts), so nothing a reader supplies reaches it. -->
<article class="doc">
  <!-- eslint-disable-next-line svelte/no-at-html-tags -->
  {@html html}
</article>
<p class="source" lang={siteTag()} dir="ltr">
  {t("doc.source")} <a href={`${REPO_URL}/blob/main/${path}`}><code>{path}</code></a>
</p>

<style>
  .english-only {
    margin-block: 0 1rem;
    padding: 0.5rem 0.75rem;
    border-inline-start: 4px solid var(--mxi-color-link);
    background: var(--mxi-color-surface);
  }
  .source {
    margin-block-start: 2.5rem;
    padding-block-start: 0.75rem;
    border-block-start: 1px solid var(--mxi-color-border);
    color: var(--mxi-color-muted);
    font-size: 0.875rem;
  }
</style>
