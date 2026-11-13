<script lang="ts">
  import "../app.css";
  import type { Snippet } from "svelte";
  import { page } from "$app/state";
  import { onMount } from "svelte";
  import { browser } from "$app/env";
  import { goto } from "$app/navigation";
  import PickerBar from "@lilydesignsystem/svelte-picker-bar";
  import type { ShareTarget } from "@lilydesignsystem/svelte-share-picker";
  import { href } from "#lib/href.js";
  import { choose, homeRoute, restoreChoice, shownLocale, siteDir, siteTag, t, topicRoute, type MessageKey } from "#lib/i18n.svelte.js";
  import {
    BRIEF_LOCALES,
    PATH_LOCALES,
    isPathLocale,
    DOC_PAGES,
    LOCALE_LABELS,
    REPO_URL,
    isBriefLocale,
    isHomeLocale,
    isRtl,
    langTag,
    topicSlug,
    type TopicKey,
  } from "#lib/site.js";

  // Lily ships no third-party URLs: these are the UI's share destinations. SharePicker supplies the page's URL.
  const shareTargets: ShareTarget[] = $derived([
    {
      id: "email",
      label: t("share.email"),
      href: (url, title) => `mailto:?subject=${encodeURIComponent(title)}&body=${encodeURIComponent(url)}`,
      newTab: false,
    },
    {
      id: "linkedin",
      label: "LinkedIn",
      href: (url) => `https://www.linkedin.com/sharing/share-offsite/?url=${encodeURIComponent(url)}`,
    },
    {
      id: "reddit",
      label: "Reddit",
      href: (url, title) =>
        `https://www.reddit.com/submit?url=${encodeURIComponent(url)}&title=${encodeURIComponent(title)}`,
    },
    {
      id: "bluesky",
      label: "Bluesky",
      href: (url, title) => `https://bsky.app/intent/compose?text=${encodeURIComponent(`${title} ${url}`)}`,
    },
    {
      id: "mastodon",
      label: "Mastodon",
      href: (url, title) =>
        `https://mastodonshare.com/?text=${encodeURIComponent(title)}&url=${encodeURIComponent(url)}`,
    },
  ]);

  // Lily's seven steps, named in the site's language.
  const sizeLabels = $derived(
    Object.fromEntries(
      (["largest", "larger", "large", "normal", "small", "smaller", "smallest"] as const).map((s) => [
        s,
        t(`size.${s}`),
      ]),
    ),
  );

  // LocalePicker writes `lang` on its target as it mounts, and with no locale it would pick the first in the list, so
  // an English page would claim to be Welsh. This layout owns <html lang>: the picker writes to an element nobody sees.
  const localeSink = browser ? document.createElement("span") : undefined;

  let { children }: { children: Snippet } = $props();

  // The role-brief locale of this page (null on the English documents).
  const locale = $derived(page.data.locale ?? null);
  const role = $derived(typeof page.params.role === "string" ? page.params.role : null);

  // Keep <html lang dir> in step on client-side navigation; the prerendered HTML already carries them
  // (hooks.server.ts).
  $effect(() => {
    document.documentElement.lang = locale ? langTag(locale) : "en";
    document.documentElement.dir = locale && isRtl(locale) ? "rtl" : "ltr";
  });

  onMount(restoreChoice);

  // Arriving on a brief keeps the site in its language when the reader moves on to a document.
  $effect(() => {
    // A page-only locale (German, Spanish, French) has no site words of its own, so it does not become the choice.
    if (locale && isBriefLocale(locale)) choose(locale, false);
  });

  // The locale switcher opens the same brief (or the same list) in the chosen locale; on any other page the site's words
  // change and the reader stays where they are. The picker also reports the locale it shows when it starts: nothing to
  // do then.
  function switchLocale(code: string) {
    // The path to production also exists in locales with no briefs (German, Spanish, French): open it there, and keep
    // the site's own words in the last brief locale chosen (English if none).
    if (pathToProduction) {
      if (!isPathLocale(code) || code === page.data.locale) return;
      if (isBriefLocale(code)) choose(code, true);
      void goto(href(`/${code}/path-to-production/`));
      return;
    }
    if (!isBriefLocale(code) || code === shownLocale()) return;
    choose(code, true);
    const target = peerRoute(code);
    if (target) void goto(href(target));
  }

  const isHome = $derived(page.route.id === "/" || page.route.id === "/[locale=home]");
  const topic = $derived<TopicKey | null>(page.data.topic ?? null);
  const pathToProduction = $derived(page.data.pathToProduction === true);

  // This page in another locale, or null where it has only the one version (the documents, the search page). Hindi has
  // no home page or topics: its briefs stand in for them.
  function peerRoute(code: string): string | null {
    if (role) return `/${code}/for/${role}/`;
    if (pathToProduction) return `/${code}/path-to-production/`;
    if (isHome) return isHomeLocale(code) ? `/${code}/` : `/${code}/for/`;
    if (topic) return isHomeLocale(code) ? `/${code}/${topicSlug(code, topic)}/` : `/${code}/for/`;
    return locale ? `/${code}/for/` : null;
  }

  const nav = $derived([
    { label: t("nav.home"), route: homeRoute() },
    { label: t("nav.benefits"), route: topicRoute("benefits") },
    { label: t("nav.features"), route: topicRoute("features") },
    { label: t("nav.comparisons"), route: topicRoute("comparisons") },
    { label: t("nav.tutorial"), route: topicRoute("tutorial") },
    { label: t("nav.briefs"), route: `/${shownLocale()}/for/` },
    { label: t("nav.path"), route: `/${shownLocale()}/path-to-production/` },
    { label: t("nav.docs"), route: "/docs/" },
  ]);
  // The link picker adds each reference document.
  const reference = $derived(
    DOC_PAGES.filter((d) => d.slug !== "docs").map((d) => ({
      label: t(`nav.${d.slug}` as MessageKey),
      route: `/${d.slug}/`,
    })),
  );
  function current(route: string): boolean {
    const here = page.url.pathname;
    const target = href(route);
    if (route.endsWith("/for/")) return /\/for\//.test(here);
    if (route === topicRoute("comparisons") && here.startsWith(href("/compare/"))) return true;
    return here === target || here === new URL(target, page.url.href).pathname;
  }

  const links = $derived(
    [...nav, ...reference].map((item) => ({
      label: item.label,
      href: href(item.route),
      current: current(item.route),
    })),
  );

  const themesUrl = $derived(`${href("/")}assets/themes/`.replace(/\/{2,}assets/, "/assets"));
</script>

<svelte:head>
  {#if role || isHome || topic || pathToProduction}
    {#each pathToProduction ? PATH_LOCALES : BRIEF_LOCALES as code (code)}
      {@const target = peerRoute(code)}
      {#if target}<link rel="alternate" hreflang={langTag(code)} href={href(target)} />{/if}
    {/each}
  {/if}
  {#if isHome}<link rel="alternate" hreflang="x-default" href={href("/")} />{/if}
</svelte:head>

<a class="skip" href="#main" lang={siteTag()} dir={siteDir()}>{t("skip")}</a>
<header class="site-header" lang={siteTag()} dir={siteDir()}>
  <a class="brand" href={href(homeRoute())}>{t("brand")}</a>
  <nav aria-label={t("nav.aria")}>
    <ul>
      {#each nav as item (item.route)}
        <li>
          <a href={href(item.route)} aria-current={current(item.route) ? "page" : undefined}>{item.label}</a>
        </li>
      {/each}
    </ul>
  </nav>
  <div class="pickers">
    <PickerBar
      {links}
      linkProps={{ navigate: (to: string) => void goto(to) }}
      searchProps={{ action: href("/search/"), navigate: (to: string) => void goto(to) }}
      labels={{
        link: t("picker.link"),
        search: t("picker.search"),
        searchInput: t("picker.searchInput"),
        searchSubmit: t("picker.searchSubmit"),
        theme: t("picker.theme"),
        locale: t("picker.locale"),
        timeZone: t("picker.timeZone"),
        textSize: t("picker.textSize"),
        share: t("picker.share"),
      }}
      {themesUrl}
      themeProps={{ storageKey: "lily-theme", detectFromSystem: true, defaultValue: "light" }}
      locales={pathToProduction ? [...PATH_LOCALES] : [...BRIEF_LOCALES]}
      localeProps={{
        value: pathToProduction ? page.data.locale : shownLocale(),
        localeLabels: LOCALE_LABELS,
        target: localeSink,
        applyDir: false,
        onChange: switchLocale,
      }}
      timeZoneProps={{ locale: siteTag(), storageKey: "lily-time-zone" }}
      textSizeProps={{ storageKey: "lily-text-size", sizeLabels }}
      {shareTargets}
      shareProps={{
        title: page.data.title ? `${page.data.title} · ${t("brand")}` : t("brand"),
        copyLabel: t("share.copy"),
        copiedLabel: t("share.copied"),
        copyFailedLabel: t("share.copyFailed"),
      }}
    />
  </div>
</header>

<main id="main" tabindex="-1">{@render children()}</main>

<footer class="site-footer" lang={siteTag()} dir={siteDir()}>
  <p>
    {t("footer.lead")} <a href={REPO_URL}>{t("footer.link")}</a>.
    {#each t("footer.tail").split(/(\{\w+\})/) as part, i (i)}
      {#if part === "{dir}"}<code dir="ltr">project-portfolio-management.github.io/</code
        >{:else if part === "{tool}"}<code dir="ltr">git subtree</code>{:else}{part}{/if}
    {/each}
  </p>
  {#if t("footer.draft")}<p class="draft">{t("footer.draft")}</p>{/if}
</footer>

<style>
  /* The wrapper div in app.html: a class, not an inline style, for the CSP. */
  :global(.app-root) {
    display: contents;
  }
  .skip {
    position: absolute;
    inset-inline-start: 0.5rem;
    inset-block-start: -3rem;
    padding: 0.5rem 0.75rem;
    background: var(--mxi-color-primary);
    color: var(--mxi-color-primary-fg);
    border-radius: var(--mxi-radius);
    z-index: 50;
  }
  .skip:focus {
    inset-block-start: 0.5rem;
  }
  .site-header {
    position: relative;
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem 1.25rem;
    padding: 0.75rem 1.5rem;
    border-block-end: 1px solid var(--mxi-color-border);
    background: var(--mxi-color-surface);
  }
  .brand {
    font-weight: 700;
    color: var(--mxi-color-fg);
  }
  nav ul {
    display: flex;
    flex-wrap: wrap;
    gap: 0.25rem 1rem;
    list-style: none;
    margin: 0;
    padding: 0;
  }
  nav a {
    display: inline-block;
    min-height: 24px;
    padding-block: 0.125rem;
  }
  nav a[aria-current="page"] {
    font-weight: 700;
    text-decoration: underline;
  }
  /* Lily PickerBar, as in the UI: the theme stylesheet styles the buttons and places each popup (CSS anchor
     positioning: do not set `top` or `inset-*` on a popup here), and this only lines the buttons up and sizes the
     popups. */
  .pickers {
    margin-inline-start: auto;
  }
  .pickers :global(.picker-bar) {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.375rem;
  }
  .pickers :global(.link-picker),
  .pickers :global(.search-picker),
  .pickers :global(.theme-picker),
  .pickers :global(.locale-picker),
  .pickers :global(.time-zone-picker),
  .pickers :global(.text-size-picker),
  .pickers :global(.share-picker) {
    display: inline-flex;
    align-items: center;
  }
  .pickers :global(.link-picker-button),
  .pickers :global(.search-picker-button),
  .pickers :global(.theme-picker-button),
  .pickers :global(.locale-picker-button),
  .pickers :global(.time-zone-picker-button),
  .pickers :global(.text-size-picker-button),
  .pickers :global(.share-picker-button) {
    box-sizing: border-box;
    width: 2.5rem;
    height: 2.5rem;
    margin: 0;
  }
  /* Lily's themes do not change a picker button on hover, so this does, in every theme. */
  .pickers :global(.link-picker-button:hover),
  .pickers :global(.search-picker-button:hover),
  .pickers :global(.theme-picker-button:hover),
  .pickers :global(.locale-picker-button:hover),
  .pickers :global(.time-zone-picker-button:hover),
  .pickers :global(.text-size-picker-button:hover),
  .pickers :global(.share-picker-button:hover) {
    background: var(--mxi-color-border);
  }
  .pickers :global(.link-picker-list),
  .pickers :global(.search-picker-panel),
  .pickers :global(.theme-picker-list),
  .pickers :global(.locale-picker-list),
  .pickers :global(.time-zone-picker-list),
  .pickers :global(.text-size-picker-list),
  .pickers :global(.share-picker-list) {
    z-index: 40;
    box-sizing: border-box;
    min-width: 0;
    width: max-content;
    max-width: calc(100vw - 2rem);
  }
  .pickers :global(.theme-picker-option),
  .pickers :global(.locale-picker-option),
  .pickers :global(.time-zone-picker-option),
  .pickers :global(.text-size-picker-option) {
    white-space: nowrap;
  }
  /* "Link copied" is a live region inside the share picker: in flow it would knock the button out of line. */
  .pickers :global(.share-picker-status) {
    position: absolute;
    inset-block-start: 100%;
    inset-inline-end: 1.5rem;
    z-index: 40;
    margin: 0.25rem 0 0;
    padding: 0.25rem 0.625rem;
    border-radius: var(--mxi-radius);
    background: var(--mxi-color-surface);
    border: 1px solid var(--mxi-color-border);
    font-size: 0.8125rem;
  }
  .pickers :global(.share-picker-status:empty) {
    display: none;
  }
  @media (max-width: 40rem) {
    .pickers {
      margin-inline-start: 0;
    }
    .pickers :global(.theme-picker-option),
    .pickers :global(.locale-picker-option),
    .pickers :global(.time-zone-picker-option),
    .pickers :global(.text-size-picker-option) {
      white-space: normal;
    }
  }
  main {
    max-width: 60rem;
    margin-inline: auto;
    padding: 1.5rem;
  }
  main:focus {
    outline: none;
  }
  .draft {
    font-style: italic;
  }
  .site-footer {
    max-width: 60rem;
    margin-inline: auto;
    padding: 1rem 1.5rem 2rem;
    color: var(--mxi-color-muted);
    font-size: 0.875rem;
    border-block-start: 1px solid var(--mxi-color-border);
  }
</style>
