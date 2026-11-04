<script lang="ts">
  import { href } from "#lib/href.js";
  import { siteTag, t } from "#lib/i18n.svelte.js";
  import { langTag } from "#lib/site.js";
  import type { PageProps } from "./$types";

  let { data }: PageProps = $props();

  // The language's own name for itself, from the build machine's Intl data (not a translation written here).
  const endonym = $derived.by(() => {
    try {
      return new Intl.DisplayNames([langTag(data.locale)], { type: "language" }).of(langTag(data.locale)) ?? data.locale;
    } catch {
      return data.locale;
    }
  });
</script>

<svelte:head><title>{t("briefs.title")} ({data.locale}) · {t("brand")}</title></svelte:head>

<h1 lang={siteTag()} dir="ltr">{t("briefs.title")}</h1>
<p class="lede"><span lang={siteTag()} dir="ltr">{t("briefs.locale")}</span> {endonym} (<code>{data.locale}</code>)</p>
<p class="lede" lang={siteTag()} dir="ltr">{t("briefs.lede")}</p>
<ul class="briefs">
  {#each data.briefs as b (b.role)}
    <li>
      <a href={href(`/${data.locale}/for/${b.role}/`)}>{b.title}</a>
      {#if b.summary}<p>{b.summary}</p>{/if}
    </li>
  {/each}
</ul>

<style>
  .lede {
    color: var(--mxi-color-muted);
    max-width: 60rem;
  }
  .briefs {
    list-style: none;
    padding: 0;
    display: grid;
    gap: 1rem;
  }
  .briefs li {
    border: 1px solid var(--mxi-color-border);
    border-radius: var(--mxi-radius);
    padding: 0.75rem 1rem;
    background: var(--mxi-color-surface);
  }
  .briefs a {
    font-weight: 600;
    text-decoration: underline;
  }
  .briefs p {
    margin: 0.25rem 0 0;
  }
</style>
