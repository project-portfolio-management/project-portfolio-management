# GitHub Pages site

The public site, `project-portfolio-management.github.io/` in this repository (tasks GP-1 to GP-6 in
[`tasks.md`](../tasks.md)): a welcoming site for newcomers (home, benefits, features, comparisons, a tutorial) over the
repository's reference documents. It follows the family pattern written down in the workforce repository
(`workforce-planning-management/spec/monorepo-github-pages/`): a SvelteKit and Lily Design System subproject named
`<repo>.github.io/` inside the monorepo, exported with `git subtree` to a read-only sibling repository, published by
`make publish-github-pages`.

## What it is

- **Static.** SvelteKit 3 with `@sveltejs/adapter-static`; every page is prerendered (`prerender = true`,
  `trailingSlash = "always"`, so `/install/` is `install/index.html`). No server, no API call, no form.
- **Svelte 5 runes** and Lily's whole `PickerBar` on every page: link, search, theme (all 57, following the system
  preference until one is chosen), locale, time zone, text size and share (the UI's five destinations and copy link).
  The time zone picker records the choice in `data-time-zone`; nothing on the site shows a time yet.
- **Search.** `/search/` runs in the reader's browser over `/search-index.json`, prerendered from the same documents
  (`src/lib/search.ts`): a page must hold every word, ignoring case, and title matches come first. The index is fetched
  on the first search. Lily's search picker writes `/search/?foo%20bar`, the whole query string with no key.
- **Pages.** `/` and `/<locale>/`: the home page, from `locales/<locale>/index.md`, whose heading, italic line and
  first paragraph are the hero. `/<locale>/<topic>/`: benefits, features, comparisons and tutorial, from
  `locales/<locale>/topics/<slug>/index.md`. `/compare/` and `/compare/<product>/`, from
  `locales/en-001/compare/<product>/index.md`. `/docs/` (`index.md`), `README.md`, `INSTALL.md`,
  `operations/wpm-bridge.md`, `spec/index.md` and the agent skill's `SKILL.md` (front matter dropped). Every role brief
  `locales/<locale>/for/<role>/index.md` at `/<locale>/for/<role>/`, with a list per locale at `/<locale>/for/`.
- **Path to production.** `/<locale>/path-to-production/`, from `locales/<locale>/path-to-production/index.md` (the
  same slug in every locale, and in the navigation): what "production" means for this software, then an honest list of
  what production use would still take. Each list item is written `- **Title** — NN%. Text`, NN a whole number from 0
  to 100 in ASCII digits; the page shows each item with its estimate and a Lily `Slider` (0 to 100, steps of 5) that
  starts at the estimate. A reader may move a slider to note their own view; nothing is saved or sent. An item that
  starts like one but has no figure, or a figure over 100, fails the build. The evidence is in
  [`governance/production-readiness.md`](../governance/production-readiness.md).
- **Voice.** The site reads as a neutral business analyst's summary: third person, declarative and specific, with no
  rhetorical questions, slogans or promotional adjectives. Imperatives are for instructions, in the tutorial.
- **Claims.** The site's words are held to rule 3 of `AGENTS.md`. Every claim about this project comes from its own
  documents, there are no invented users or figures, and the home page and every comparison link to the path to
  production. No page calls the software "production-ready" or "not ready for production": what production use would
  take is said once, item by item, on that page. A comparison page says that the other product was not
  installed or run, describes it in general terms only, scores nothing, says plainly when no importer exists, and
  says that the project is not affiliated with its owner. The tutorial is the seeded demonstration
  ([`operations/demo.md`](../operations/demo.md)).
- **`/.well-known/security.txt`** (RFC 9116, SM-1), from `static/`. The deploy sets `include-hidden-files`, or the action
  would leave it out. A test fails when `Expires` is under 60 days away.
- **Sitemaps.** `/sitemap.xml` lists every page and `/<locale>/sitemap.xml` that locale's pages.
- **Links.** A relative link to a document the site shows goes to its page (with its `#fragment`; headings get
  GitHub's anchors); a relative link to any other repository path goes to that path on GitHub (`blob`, or `tree` for a
  directory). External links are kept. The prerender fails on a link to a missing page or a missing anchor.
- **Raw HTML** in the Markdown is escaped and shown as text; the HTML on a page comes only from the Markdown renderer.

## Content copy

The exported repository has no parent directory, so the site cannot read `../README.md` when it builds. The site's
`scripts/sync-content.mjs` copies the documents into `src/content/docs.json`, and each locale's topic slugs, resolved
through `.locale-peer-id`, into `src/content/topics.json` (both committed). `--check` fails when the copy
differs from the repository files; CI runs it, and `bin/publish-github-pages` runs it before pushing. The document list
is in that script and in `src/lib/site.ts`.

## Locale and direction

This follows [locales for global sharing with Svelte](locales-for-global-sharing-with-svelte/index.md).

`<html lang>` is the locale's BCP 47 form (`en-gb-oxendict` → `en-GB-oxendict`) on a page with a locale in its address,
and `en` elsewhere, because those documents are in English; it is written into the prerendered HTML by
`hooks.server.ts` and kept in step on client-side navigation. `LocalePicker` is given an unseen target, because it
writes `lang` as it mounts. `dir` is `rtl` for a right-to-left language: Arabic, on the whole page in `ar-001`, and on the site's own words wherever a reader has chosen Arabic, while an English document stays left to right. Every layout
rule uses logical properties; code blocks stay left to right.

The site's own words (navigation, wordmark, pickers, footer, search) are in `src/lib/messages.json`, in English, Arabic,
Hindi, Welsh and Mandarin (Simplified), with their own `lang`. Their language is the address's on a page with a locale, so those
pages are prerendered in it. Elsewhere it is the reader's choice (`localStorage` `lily-locale`), so those pages are
prerendered in English and change as they load, and a note says the document itself is in English.

Each home page and topic has a `.locale-peer-id` that is byte-identical across locales, and a `README.md` symlink. Its
slug is in the language and need not be ASCII (`/cy-gb/nodweddion/`, `/zh-cn/功能/`). `cy-gb` is copied to `cy-001`, `hi-in` to `hi-001`
and `zh-cn` to `zh-001`; `ar-001` has no regional copy, as in the interface; the English dialects follow the rule's spellings. The locale picker opens the same page in the
chosen locale: a brief, a list, the home page or a topic (found by peer id). On a page with one version it stays put.
The comparison product pages are English only.

Arabic, Hindi, Welsh and Mandarin are machine-written until a speaker reviews them, and each page says so in its footer.
`scripts/export-site-translation-review.py` lists them in `governance/translation-reviews/site-*.csv`.

## Colour and accessibility

The colour tokens are the UI's (`--mxi-color-*` over the Lily `--color-*` tokens), including the contrast fix of
commit `49675dc`: `--mxi-color-link` (primary mixed 70/30 with the text colour) for every link, and
`--mxi-color-primary` (primary mixed 80/20 with the neutral) as a fill. A change to the UI's tokens is made there
first, then copied here. Links in running text are underlined. A table or code block that scrolls sideways is
focusable. The target is no serious or critical axe finding (WCAG 2.2 AA tags) under the light and the dark system
preference. The other 55 themes are offered but are not held to that: an audit of the home page on 2026-11-04 found
contrast failures under 15 of the 57. Lily's themes give a picker button no hover change, so the layout adds one, and
Lily's own class names (such as `.hero`, which a theme paints) are avoided. An open list that scrolls fails axe's
`scrollable-region-focusable`, which the popup test turns off: the list takes focus itself and moves through its options
with `aria-activedescendant`, and a test reaches the last theme by keyboard.

## Content security policy

GitHub Pages sends no custom headers, so the policy is SvelteKit's `<meta http-equiv>` with hashes of its own inline
scripts (`mode: "hash"`): `default-src 'self'`, `script-src 'self'` plus hashes, `style-src 'self'`, and the same
`style-src-attr` hash as the UI (SvelteKit's announcer only). Computed styles use Svelte's `style:` directive, never a
`style="…"` attribute. `frame-ancestors` cannot be set from a meta tag and is not set.

## Tests

`tests/e2e/site.spec.ts` (Playwright with `@axe-core/playwright`, against `vite preview` of the build): every page
answers 200; axe under light and dark on every document, home page and comparison, the topics of three locales, every
locale's list and one brief per locale; each picker (order, hover, a dropdown under its button that does not move the
page, axe with it open); search; every word in every language; the translated hero; peer ids, `README.md` symlinks,
the `-001` copies and the dialects; the sitemaps; the honest parts of each comparison; no CSP report
in the console; `lang` on a brief and on a document; the locale switcher; links on and off the site; four pages under
a forced `dir="rtl"` (brand at the inline start, no sideways scroll, axe clean in both schemes); no sideways scroll at
390 px; no element with a `style` attribute. CI job `site` runs the content check, `svelte-check`, the build and the
suite.

## Publishing

`make publish-github-pages` runs `bin/publish-github-pages`, which runs
`git subtree push --prefix=project-portfolio-management.github.io github-pages main` after checking that the
`github-pages` remote exists, the content copy is current and the directory has no uncommitted change. The exported
repository deploys with its own `.github/workflows/deploy.yml` (GitHub Pages' actions; inert inside the monorepo).
Each action there is pinned to a full commit SHA with its release tag in a comment (`# v7.0.1`), as the workforce
repository's site does, so a moved tag cannot change what runs. No Dependabot entry watches that file (the site's
entry is npm only, and the `github-actions` entry reads the root `.github`), so the pins are moved by hand.
Published 2026-11-04 (GP-2) at <https://project-portfolio-management.github.io/>. The exported repository exists with
its Pages source set to GitHub Actions; the `github-pages` remote is per clone. The steps are in the site's
[`README.md`](../project-portfolio-management.github.io/README.md#publishing).

## Not built

The comparison product pages are in
English; the role briefs' slugs are English in every locale, so they fall short of the rule that a segment is in the
language; search does not work with JavaScript off; 15 of the 57 themes fail contrast on the home page; the first paint (or
a visit with JavaScript off) uses the light fallback tokens whatever the system preference; a page without a locale in
its address is first painted in English for a reader of another language; nothing publishes the site on a push to the
monorepo (`make publish-github-pages` is run by hand), so it can lag `main`.
