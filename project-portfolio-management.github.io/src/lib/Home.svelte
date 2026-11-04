<script lang="ts">
  import { href } from "#lib/href.js";
  import { siteTag, t, topicRoute } from "#lib/i18n.svelte.js";
  import { LOCALE_LABELS, REPO_URL, WORLD_LOCALES, langTag, type BriefLocale } from "#lib/site.js";
  import type { Home } from "#lib/server/content.js";

  let { home, lang }: { home: Home; lang: string } = $props();
</script>

<div {lang}>
  <section class="home-hero">
    <h1>{home.headline}</h1>
    <p class="home-subheadline">{home.subheadline}</p>
    <p class="home-intro">{home.intro}</p>
    <p class="home-actions">
      <a class="home-action primary" href={href(topicRoute("tutorial"))}>{t("hero.tutorial")}</a>
      <a class="home-action" href={href(topicRoute("features"))}>{t("hero.features")}</a>
      <a class="home-action" href={REPO_URL}>{t("hero.github")}</a>
    </p>
    <p class="home-translations" lang={siteTag()}>
      {t("hero.translations")}
      {#each WORLD_LOCALES as code, i (code)}
        {i > 0 ? ", " : ""}<a href={href(`/${code}/`)} lang={langTag(code)}>{LOCALE_LABELS[code as BriefLocale]}</a>
      {/each}
    </p>
  </section>

  <!-- From the repository's own Markdown, with raw HTML escaped ($lib/server/content.ts). -->
  <article class="home-landing doc">
    <!-- eslint-disable-next-line svelte/no-at-html-tags -->
    {@html home.html}
  </article>
</div>

<style>
  .home-hero {
    padding-block: 2.5rem 2rem;
    text-align: center;
  }
  h1 {
    margin: 0;
    font-size: 2.5rem;
    line-height: 1.15;
  }
  .home-subheadline {
    max-width: 42rem;
    margin: 0.75rem auto 0;
    font-size: 1.375rem;
    line-height: 1.35;
  }
  .home-intro {
    max-width: 42rem;
    margin: 1rem auto 0;
    color: var(--mxi-color-muted);
  }
  .home-actions {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    gap: 0.75rem;
    margin-block: 1.5rem 0;
  }
  .home-action {
    display: inline-block;
    padding: 0.625rem 1.125rem;
    border: 1px solid var(--mxi-color-border);
    border-radius: var(--mxi-radius);
    background: var(--mxi-color-surface);
    color: var(--mxi-color-fg);
    font-weight: 600;
    text-decoration: none;
  }
  .home-action.primary {
    border-color: var(--mxi-color-primary);
    background: var(--mxi-color-primary);
    color: var(--mxi-color-primary-fg);
  }
  .home-action:hover {
    outline: 2px solid var(--mxi-color-link);
  }
  .home-translations {
    margin-block: 1.25rem 0;
    color: var(--mxi-color-muted);
  }
  .home-landing :global(h2) {
    margin-block-start: 2.5rem;
    text-align: center;
  }
  .home-landing :global(ul) {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(16rem, 1fr));
    gap: 1rem;
    margin: 1rem 0;
    padding: 0;
    list-style: none;
  }
  .home-landing :global(li) {
    padding: 0.875rem 1rem;
    border: 1px solid var(--mxi-color-border);
    border-radius: var(--mxi-radius);
    background: var(--mxi-color-surface);
  }
  .home-landing :global(li strong) {
    display: block;
    margin-block-end: 0.25rem;
  }
  .home-landing > :global(p) {
    text-align: center;
  }
  @media (max-width: 40rem) {
    h1 {
      font-size: 2rem;
    }
  }
</style>
