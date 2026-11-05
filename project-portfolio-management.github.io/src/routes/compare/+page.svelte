<script lang="ts">
  import { href } from "#lib/href.js";
  import { siteDir, siteTag, t } from "#lib/i18n.svelte.js";
  import type { PageProps } from "./$types";

  let { data }: PageProps = $props();
</script>

<svelte:head><title>{t("compare.title")} · {t("brand")}</title></svelte:head>

<div lang={siteTag()} dir={siteDir()}>
  <h1>{t("compare.title")}</h1>
  <p class="lede">{t("compare.lede")}</p>
</div>
<ul class="products" lang="en">
  {#each data.products as p (p.slug)}
    <li>
      <a href={href(`/compare/${p.slug}/`)}>{p.title}</a>
      {#if p.summary}<p>{p.summary}</p>{/if}
    </li>
  {/each}
</ul>

<style>
  .lede {
    color: var(--mxi-color-muted);
  }
  .products {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(16rem, 1fr));
    gap: 1rem;
    margin: 0;
    padding: 0;
    list-style: none;
  }
  .products li {
    padding: 0.75rem 1rem;
    border: 1px solid var(--mxi-color-border);
    border-radius: var(--mxi-radius);
    background: var(--mxi-color-surface);
  }
  .products a {
    font-weight: 600;
  }
  .products p {
    margin: 0.25rem 0 0;
  }
</style>
