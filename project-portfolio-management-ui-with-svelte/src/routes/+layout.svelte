<script lang="ts">
  import "../app.css";
  import { browser } from '$app/env';
  import { page } from "$app/state";
  import { enhance } from "$app/forms";
  import { goto } from "$app/navigation";
  import type { Snippet } from "svelte";
  import type { LayoutData } from "./$types";
  import {
    i18n,
    t,
    isRtl,
    localeHref,
    LOCALES,
    LOCALE_LABELS,
  } from "#lib/i18n.svelte.js";
  import { localizePath, normalizeLocale, splitPath } from "#lib/locales.js";
  import { display } from "#lib/preferences.svelte.js";
  import { effectiveSimple, htmlAttributes, splitNav } from "#lib/preferences.js";
  import { COLLECTIONS } from "#lib/api/types.js";
  import { PpmClient } from "#lib/api/ppm.js";
  import { isAbsent } from "#lib/absent.js";
  import { orderNavForView } from "#lib/nav.js";
  import ConfirmHost from "#lib/components/ConfirmHost.svelte";
  import PickerBar from "@lilydesignsystem/svelte-picker-bar";
  import type { ShareTarget } from "@lilydesignsystem/svelte-share-picker";

  // Share destinations for the Lily SharePicker. Lily ships no
  // third-party URLs — each `href` builder is ours. `url`/`title` are
  // supplied by SharePicker at share time (current page URL; the leaf
  // page's title, sourced from `page.data.title` below — the
  // `page.data.title` convention, set per-route by each route's load
  // function so it stays in sync with that page's own <svelte:head>
  // <title> without SharePicker having to read the DOM).
  // The handling mark (security classification) of the plan being looked at, when the deployment
  // has a scheme (RG-7). Nothing is shown when there is no scheme, or when the plan is not there for
  // this reader (404, 403). Any other failure says the mark could not be checked (CQ-19): hiding the
  // banner then would read as "unmarked", and showing a mark would be a guess.
  const ppmClient = PpmClient.withFetch();
  let handlingMark = $state<string | null>(null);
  let handlingMarkUnknown = $state(false);
  $effect(() => {
    const pid = page.params?.pid;
    handlingMark = null;
    handlingMarkUnknown = false;
    if (!browser || !pid) return;
    let live = true;
    ppmClient
      .planHandlingMark(pid)
      .then((r) => {
        if (live) handlingMark = r.scheme.length > 0 ? r.effective : null;
      })
      .catch((err: unknown) => {
        if (!live) return;
        handlingMarkUnknown = !isAbsent(err);
      });
    return () => {
      live = false;
    };
  });

  const SHARE_TARGETS: ShareTarget[] = $derived([
    {
      id: "email",
      label: t("share.email"),
      href: (url, title) =>
        `mailto:?subject=${encodeURIComponent(title)}&body=${encodeURIComponent(url)}`,
      newTab: false,
    },
    {
      id: "linkedin",
      label: t("share.linkedin"),
      href: (url) =>
        `https://www.linkedin.com/sharing/share-offsite/?url=${encodeURIComponent(url)}`,
    },
    {
      id: "reddit",
      label: t("share.reddit"),
      href: (url, title) =>
        `https://www.reddit.com/submit?url=${encodeURIComponent(url)}&title=${encodeURIComponent(title)}`,
    },
    {
      id: "bluesky",
      label: t("share.bluesky"),
      href: (url, title) =>
        `https://bsky.app/intent/compose?text=${encodeURIComponent(`${title} ${url}`)}`,
    },
    {
      id: "mastodon",
      label: t("share.mastodon"),
      href: (url, title) =>
        `https://mastodonshare.com/?text=${encodeURIComponent(title)}&url=${encodeURIComponent(url)}`,
    },
  ]);

  // `data.signedIn` is resolved server-side from the httpOnly session
  // cookie (`+layout.server.ts`).
  let { children, data }: { children: Snippet; data: LayoutData } = $props();

  // The locale is in the URL (`/cy-001/plans`): hand it to the i18n store
  // before first render, and again whenever navigation changes it.
  // svelte-ignore state_referenced_locally
  i18n.set(data.locale ?? "en-001");
  $effect.pre(() => {
    i18n.set(data.locale ?? "en-001");
  });

  // The page's path without its locale prefix, for the active-nav check.
  const here = $derived(splitPath(page.url.pathname).rest);

  // The link picker: a short list of the places people go most. `current` marks the page you are on.
  const pickerLinks = $derived(
    [
      { href: "/", label: t("p.layout.lp_home") },
      { href: "/plans", label: t("p.layout.lp_plans") },
      { href: "/dashboard", label: t("p.layout.lp_dashboard") },
      { href: "/kanban", label: t("p.kanban.nav") },
      { href: "/milestones", label: t("p.layout.lp_milestones") },
      { href: "/proposals", label: t("p.layout.lp_proposals") },
      { href: "/director-packs", label: t("p.director_pack.director_packs") },
      { href: "/start-queue", label: t("p.start_queue.nav") },
      { href: "/organizations", label: t("p.organizations.nav") },
      { href: "/display", label: t("p.layout.display") },
    ].map((l) => ({ label: l.label, href: localeHref(l.href), current: l.href === "/" ? here === "/" : here === l.href || here.startsWith(`${l.href}/`) })),
  );

  // Picking a language goes to the same page under that language's prefix.
  function switchLocale(code: string): void {
    const next = normalizeLocale(code);
    if (!next || next === i18n.locale) return;
    void goto(
      localizePath(`${here}${page.url.search}${page.url.hash}`, next),
    );
  }

  // The `page.data.title` convention: each route's own load function
  // (`+page.ts`/`+page.server.ts`) returns a plain `title` string that
  // mirrors what that route's `<svelte:head><title>` renders, so the
  // layout — which does not know which leaf page is active — can read
  // it here for SharePicker without scraping `document.title`. Falls
  // back to the brand name for the rare route that sets none.
  const pageTitle = $derived(page.data?.title ?? t("brand.name"));

  // Hamburger toggle state for the top navigation bar (narrow viewports).
  let menuOpen = $state(false);

  // The i18n store is the single source of truth for the locale; mirror it
  // onto `<html lang>` / `<html dir>` (RTL for ar-001) whenever it changes.
  // SSR-guarded — `document` is only touched in the browser.
  $effect(() => {
    const locale = i18n.locale;
    if (!browser || typeof document === "undefined") return;
    // `lang` must read BCP47-hyphenated; the locale codes already are
    // ("ar-001", "zh-cn"), and this must agree with what PickerBar's
    // LocalePicker itself writes, since both write the same attribute.
    document.documentElement.lang = locale.replace("_", "-");
    document.documentElement.dir = isRtl(locale) ? "rtl" : "ltr";
  });

  // Sidebar navigation targets; `key` is an i18n key so labels follow the
  // selected locale. `aria-current` is set per item below.
  const navItems: { href: string; label: string }[] = [
    { href: "/", label: t("brand.name") },
    ...COLLECTIONS.map((collection) => ({
      href: `/${collection}`,
      label: collection,
    })),
    // Record merge (fold a confirmed duplicate plan into a survivor).
    { href: "/plans/merge", label: t("nav.merge") },
    // PPM catalogue views (fully localized 2026-07-18).
    // LT-3: the individual worker's personal task view. A deployment opts a
    // persona into it as its landing page with `view=me` (T-28f).
    { href: "/me", get label() { return t("p.me.my_work"); } },
    { href: "/dashboard", label: t("ppm.nav.dashboard") },
    { href: "/prioritization", get label() { return t("p.prioritization.prioritization"); } },
    { href: "/lifecycle", get label() { return t("p.lifecycle.lifecycle"); } },
    { href: "/reviews", get label() { return t("p._layout_svelte.reviews"); } },
    { href: "/automations", get label() { return t("p._layout_svelte.automations"); } },
    { href: "/proposals", label: t("ppm.nav.proposals") },
    { href: "/ideas", label: t("ppm.nav.ideas") },
    { href: "/scenarios", label: t("ppm.nav.scenarios") },
    { href: "/objectives", label: t("ppm.nav.objectives") },
    { href: "/okrs", get label() { return t("p._layout_svelte.goals_okrs"); } },
    { href: "/milestones", get label() { return t("p.milestones.milestones"); } },
    { href: "/kpis", get label() { return t("p.kpis.key_measures_kpis"); } },
    { href: "/gantt", label: t("ppm.nav.gantt") },
    { href: "/engineering", label: t("ppm.nav.engineering") },
    { href: "/calendar", label: t("ppm.nav.calendar") },
    { href: "/ceo", get label() { return t("p.ceo.ceo_dashboard"); } },
    { href: "/announcements", get label() { return t("p.ceo.announcements"); } },
    { href: "/executive", label: t("ppm.nav.executive") },
    { href: "/financials", label: t("ppm.nav.financials") },
    { href: "/technology", label: t("ppm.nav.technology") },
    { href: "/board", label: t("ppm.nav.board") },
    { href: "/auditor", label: t("ppm.nav.auditor") },
    { href: "/compliance", label: t("ppm.nav.compliance") },
    { href: "/risk", label: t("ppm.nav.risk") },
    { href: "/issues", get label() { return t("p.issues.issues"); } },
    { href: "/change-requests", get label() { return t("p.change_requests.change_requests"); } },
    { href: "/director-packs", get label() { return t("p.director_pack.director_packs"); } },
    { href: "/pipeline", get label() { return t("p.critical_chain.nav_pipeline"); } },
    { href: "/start-queue", get label() { return t("p.start_queue.nav"); } },
    { href: "/kanban", get label() { return t("p.kanban.nav"); } },
    { href: "/ledger", get label() { return t("p.ledger.nav"); } },
    { href: "/milestone-signoffs", get label() { return t("p.signoffs.nav"); } },
    { href: "/key-persons", get label() { return t("p.key_persons.nav"); } },
    { href: "/organizations", get label() { return t("p.organizations.nav"); } },
    { href: "/email-deliveries", get label() { return t("p.email_deliveries.nav"); } },
    { href: "/security", label: t("ppm.nav.security") },
    { href: "/regulator", label: t("ppm.nav.regulator") },
    { href: "/capacity", label: t("ppm.nav.capacity") },
    { href: "/resources", get label() { return t("p.resources.resources"); } },
    { href: "/cost-risk", get label() { return t("p.plans.pid.joint_risk.cost_risk"); } },
    { href: "/cost-rates", get label() { return t("p.plans.pid.costs.cost_rates"); } },
    { href: "/reports", label: t("ppm.nav.reports") },
    { href: "/onboarding", get label() { return t("p._layout_svelte.onboarding"); } },
    { href: "/documents", get label() { return t("p.documents.documents"); } },
    { href: "/approval-processes", get label() { return t("p.processes.processes"); } },
    { href: "/tolerances", get label() { return t("p.tolerances.tolerances"); } },
    { href: "/escalations", get label() { return t("p.escalations.escalations"); } },
    { href: "/tour", label: t("nav.tour") },
  ];

  // T-28f: reorders `navItems` around `data.view` (the deployment-declared
  // ABAC attribute) — recomputes whenever `data` changes (sign-in/out,
  // navigation), since `data` is a reactive prop. `navItems` unchanged
  // when `data.view` is absent/unmatched, so this is a no-op for every
  // deployment that has not opted in.
  const orderedNavItems = $derived(orderNavForView(navItems, data.view));

  // Reactive: tracks the server-resolved session presence.
  const signedIn = $derived(data.signedIn);

  // Display preferences (LT-12). Simple mode moves the advanced destinations
  // behind "More" — prominence, not capability: every route still works. It is on
  // by default for the worker persona (`view=me`); an explicit choice wins.
  const simple = $derived(effectiveSimple(display.prefs, data.view));
  const nav = $derived(splitNav(orderedNavItems, simple));
  $effect(() => {
    if (!browser) return;
    const a = htmlAttributes(simple, display.dyslexia);
    document.documentElement.setAttribute("data-simple", a["data-simple"]);
    document.documentElement.setAttribute("data-font", a["data-font"]);
  });
  $effect(() => {
    if (browser) void display.sync(signedIn);
  });
  const toggleSimple = () => void display.set({ simple: !simple }, signedIn);
</script>

<!--
  Root layout — chrome shared by every route.

  Purpose:
    Renders the top navigation bar (brand, nav, session panel) plus the
    routed page.

  $props:
    - children: Snippet — the routed page content (`{@render children()}`).
    - data: LayoutData — `signedIn` resolved server-side from the httpOnly
            session cookie, `view` from the caller's ABAC attrs
            (`+layout.server.ts`).

  Session affordance: per-app magic-link login on this app's own `/signin`;
  sign-out posts to the root page's `signout` action (BFF: revokes the
  session server-side + clears the cookie). The browser never holds a token.

  Nav ordering (T-28f, repo `tasks.md` EV-1): `data.view` (a
  deployment-declared ABAC attribute, e.g. `view=executive`) moves the
  matching nav item to the front, via the pure `orderNavForView` helper
  (`#lib/nav.ts`) — presentation only; every route stays reachable by URL
  regardless. `view` absent ⇒ `navItems` unchanged, byte for byte.
-->

<div class="layout">
  <header class="topbar">
    <button
      type="button"
      class="hamburger"
      aria-expanded={menuOpen}
      aria-controls="primary-nav"
      aria-label={t("nav.toggle")}
      onclick={() => menuOpen = !menuOpen}
    ><span class="hamburger-box" aria-hidden="true"></span></button>

    <a href={localeHref("/")} class="brand">{t("brand.name")}</a>
    <nav id="primary-nav" class="primary-nav" class:open={menuOpen}>
      <ul>
        {#each nav.front as item (item.href)}
          <li>
            <a
              href={localeHref(item.href)}
              aria-current={here === item.href ? "page" : undefined}
              onclick={() => menuOpen = false}
            >{item.label}</a>
          </li>
        {/each}
        {#if nav.more.length > 0}
          <li class="more">
            <details>
              <summary>{t("p.layout.more")}</summary>
              <ul>
                {#each nav.more as item (item.href)}
                  <li>
                    <a
                      href={localeHref(item.href)}
                      aria-current={here === item.href ? "page" : undefined}
                      onclick={() => menuOpen = false}
                    >{item.label}</a>
                  </li>
                {/each}
              </ul>
            </details>
          </li>
        {/if}
      </ul>
    </nav>
    <div class="header-end">
      <button type="button" class="session-button" aria-pressed={simple} onclick={toggleSimple}>
        {t("p.layout.simple_view", { v: simple ? t("p.common.on") : t("p.common.off") })}
      </button>
      <a class="session-button display-link" href={localeHref("/display")}>{t("p.layout.display")}</a>
      {#if signedIn}
        <!-- Sign-out posts to the root page's \`signout\` action
             (BFF: revokes server-side + clears the cookie). -->
        <form method="POST" action={localeHref("/?/signout")} use:enhance>
          <button type="submit" class="session-button">
            {t("auth.signout")}
          </button>
        </form>
      {:else}
        <!-- Per-app magic-link login on this app's own origin. -->
        <a class="session-button signin" href={localeHref("/signin")}>{t("auth.signin")}</a>
      {/if}
      <PickerBar
        links={pickerLinks}
        linkProps={{ navigate: (href: string) => void goto(href) }}
        searchProps={{ action: localeHref("/search"), navigate: (href: string) => void goto(href) }}
        labels={{
          link: t("p.layout.link_picker"),
          search: t("p.layout.search_picker"),
          searchInput: t("p.layout.search_input"),
          searchSubmit: t("p.layout.search_submit"),
          theme: t("chrome.theme"),
          locale: t("chrome.language"),
          textSize: t("nav.text_size"),
          share: t("nav.share"),
        }}
        themesUrl="/assets/themes/"
        themeProps={{
          storageKey: "lily-theme",
          // Without a default no theme stylesheet loads until the user
          // picks one, leaving the pickers (which Lily's theme CSS styles)
          // unstyled on first visit. Follow the OS light/dark preference,
          // else fall back to "light".
          detectFromSystem: true,
          defaultValue: "light",
        }}
        locales={[...LOCALES]}
        localeProps={{
          value: i18n.locale,
          localeLabels: LOCALE_LABELS,
          applyDir: false,
          onChange: switchLocale,
        }}
        textSizeProps={{
          storageKey: "lily-text-size",
        }}
        shareTargets={SHARE_TARGETS}
        shareProps={{
          title: pageTitle,
          copyLabel: t("share.copy_link"),
          copiedLabel: t("share.copied"),
          copyFailedLabel: t("share.copy_failed"),
        }}
      />
    </div>
  </header>
  {#if handlingMark}<p class="handling-mark" role="note" data-testid="handling-mark">{t("p._layout_svelte.handling_mark", { mark: handlingMark })}</p>{:else if handlingMarkUnknown}<p class="handling-mark" role="note" data-testid="handling-mark">{t("p._layout_svelte.handling_mark_unknown")}</p>{/if}
  {#if data.demo}
    <p class="demo-banner" role="note">{t("p.demo.banner")} <a href={localeHref("/demo")}>{t("p.demo.banner_tour")}</a></p>
  {/if}
  <main>{@render children()}</main>
  <ConfirmHost />
</div>

<style>
  /* The wrapper div in app.html: a class, not an inline style, for the CSP. */
  :global(.app-root) {
    display: contents;
  }
  .layout {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
  }
  .topbar {
    position: relative;
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 1rem;
    padding: 0.75rem 1.5rem;
    background: var(--mxi-color-surface);
    border-bottom: 1px solid var(--mxi-color-border);
  }
  .brand {
    font-weight: 700;
    color: inherit;
    text-decoration: none;
    white-space: nowrap;
  }
  .hamburger {
    /* Always visible: the primary nav is collapsed behind it at every width. */
    display: block;
    width: 2.5rem;
    height: 2.5rem;
    padding: 0;
    background: transparent;
    border: 1px solid var(--mxi-color-border);
    border-radius: var(--mxi-radius);
    cursor: pointer;
  }
  .hamburger-box,
  .hamburger-box::before,
  .hamburger-box::after {
    display: block;
    width: 1.1rem;
    height: 2px;
    margin: 0 auto;
    background: currentColor;
    content: "";
  }
  .hamburger-box::before {
    transform: translateY(-5px);
  }
  .hamburger-box::after {
    transform: translateY(3px);
  }
  .primary-nav {
    /* Always collapsed behind the hamburger: hidden by default at every
       width, shown only when the toggle adds `.open`. Rendered as a dropdown
       panel overlaying content (position:absolute) so opening it does not
       reflow the header. */
    display: none;
    position: absolute;
    top: 100%;
    inset-inline-start: 1.5rem;
    z-index: 20;
    flex-direction: column;
    align-items: stretch;
    gap: 0.75rem;
    min-width: 16rem;
    padding: 0.75rem;
    background: var(--mxi-color-surface);
    border: 1px solid var(--mxi-color-border);
    border-radius: var(--mxi-radius);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
  }
  .primary-nav.open {
    display: flex;
  }
  .primary-nav ul {
    list-style: none;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    margin: 0;
    padding: 0;
  }
  .primary-nav a {
    display: block;
    text-decoration: none;
    padding: 0.5rem 0.625rem;
    border-radius: var(--mxi-radius);
    color: inherit;
  }
  .primary-nav a:hover {
    background: var(--mxi-color-bg);
  }
  .primary-nav a[aria-current="page"] {
    background: var(--mxi-color-primary);
    color: var(--mxi-color-primary-fg);
    font-weight: 600;
  }
  main {
    width: 100%;
    padding: 1.5rem 2rem;
  }
  .header-end {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: flex-end;
    gap: 0.5rem 0.75rem;
    margin-inline-start: auto;
    max-width: 100%;
  }
  .header-end form {
    margin: 0;
  }
  .session-button {
    display: inline-flex;
    align-items: center;
    height: 2.5rem;
    padding: 0 0.875rem;
    border: 1px solid var(--mxi-color-border);
    border-radius: var(--mxi-radius);
    background: transparent;
    color: var(--mxi-color-fg);
    font: inherit;
    font-size: 0.875rem;
    font-weight: 600;
    line-height: 1;
    white-space: nowrap;
    text-decoration: none;
    cursor: pointer;
  }
  .session-button:hover {
    background: var(--mxi-color-bg);
    text-decoration: none;
  }
  .session-button.signin {
    border-color: var(--mxi-color-primary);
    background: var(--mxi-color-primary);
    color: var(--mxi-color-primary-fg);
  }
  .session-button.signin:hover {
    background: var(--mxi-color-primary);
    filter: brightness(1.1);
  }

  /* Lily PickerBar. The Lily theme stylesheet (loaded by ThemePicker, see
     `defaultValue` below) styles the picker buttons and listboxes; this
     only places the buttons and sizes the popups. Since the 0.4 themes each popup is placed by the theme
     itself (CSS anchor positioning, `position: fixed`): under its own button, flipping to the button's end
     edge or above it when it would run off the window. Do not set `top` or `inset-*` on the popups here:
     that fights the anchor and throws them to the bottom of the window. A popup is as wide as its longest
     label (the theme names are long) and never pushes the page down. */
  .header-end :global(.picker-bar) {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.375rem;
  }
  .header-end :global(.link-picker),
  .header-end :global(.search-picker),
  .header-end :global(.theme-picker),
  .header-end :global(.locale-picker),
  .header-end :global(.text-size-picker),
  .header-end :global(.share-picker) {
    display: inline-flex;
    align-items: center;
  }
  /* "Copied" feedback: a live region Lily renders inside the share
     picker's root. In flow it made that root taller than the other
     three and knocked its button out of line with them, so it is lifted
     out of flow and shown as a small pill under the header instead. */
  .header-end :global(.share-picker-status) {
    position: absolute;
    top: 100%;
    inset-inline-end: 1.5rem;
    z-index: 40;
    margin: 0.25rem 0 0;
    padding: 0.25rem 0.625rem;
    border-radius: var(--mxi-radius);
    background: var(--mxi-color-surface);
    border: 1px solid var(--mxi-color-border);
    font-size: 0.8125rem;
  }
  .header-end :global(.share-picker-status:empty) {
    display: none;
  }
  .header-end :global(.link-picker-button),
  .header-end :global(.search-picker-button),
  .header-end :global(.theme-picker-button),
  .header-end :global(.locale-picker-button),
  .header-end :global(.text-size-picker-button),
  .header-end :global(.share-picker-button) {
    box-sizing: border-box;
    width: 2.5rem;
    height: 2.5rem;
    margin: 0;
    vertical-align: middle;
  }
  .header-end :global(.link-picker-list),
  .header-end :global(.search-picker-panel),
  .header-end :global(.theme-picker-list),
  .header-end :global(.locale-picker-list),
  .header-end :global(.text-size-picker-list),
  .header-end :global(.share-picker-list) {
    z-index: 40;
    box-sizing: border-box;
    min-width: 0;
    width: max-content;
    max-width: calc(100vw - 2rem);
  }
  .header-end :global(.theme-picker-option),
  .header-end :global(.locale-picker-option),
  .header-end :global(.text-size-picker-option) {
    white-space: nowrap;
  }
  .demo-banner {
    margin: 0;
    padding: 0.5rem 1.5rem;
    background: #fff4d6;
    color: #4a3b00;
    border-block-end: 1px solid #e6c65c;
    font-size: 0.9rem;
  }
  .demo-banner a {
    color: inherit;
    font-weight: 600;
  }
  /* Six pickers and the session buttons no longer fit one row on a tablet. Display is also in the link
     picker, so its button goes first. */
  @media (max-width: 64rem) {
    .display-link {
      display: none;
    }
  }
  @media (max-width: 40rem) {
    .topbar {
      padding-inline: 0.75rem;
      gap: 0.5rem;
    }
    .header-end {
      gap: 0.375rem;
    }
    .header-end :global(.theme-picker-option),
    .header-end :global(.locale-picker-option),
    .header-end :global(.text-size-picker-option) {
      white-space: normal;
    }
    .header-end :global(.share-picker-status) {
      inset-inline-end: 0.75rem;
    }
    .header-end :global(.link-picker-list),
    .header-end :global(.search-picker-panel),
    .header-end :global(.theme-picker-list),
    .header-end :global(.locale-picker-list),
    .header-end :global(.text-size-picker-list),
    .header-end :global(.share-picker-list) {
      max-width: calc(100vw - 1.5rem);
    }
  }
</style>
