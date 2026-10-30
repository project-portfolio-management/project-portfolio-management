# GitHub Pages site

The public documentation site, `project-portfolio-management.github.io/` in this repository (task GP-1 in
[`tasks.md`](../tasks.md)). It follows the family pattern written down in the workforce repository
(`workforce-planning-management/spec/monorepo-github-pages/`): a SvelteKit and Lily Design System subproject named
`<repo>.github.io/` inside the monorepo, exported with `git subtree` to a read-only sibling repository, published by
`make github-pages`.

## What it is

- **Static.** SvelteKit 3 with `@sveltejs/adapter-static`; every page is prerendered (`prerender = true`,
  `trailingSlash = "always"`, so `/install/` is `install/index.html`). No server, no API call, no form.
- **Svelte 5 runes** and Lily widgets as the operator UI uses them: `ThemePicker` (themes `light` and `dark`, following
  the system preference), `TextSizePicker` (the same seven steps and `data-text-size` rules), and `LocalePicker` on
  the role-brief pages. No search or share picker (there is nothing to search on a static site, and the UI's share
  targets are the UI's).
- **Pages.** `index.md` (`/`), `README.md`, `INSTALL.md`, `operations/wpm-bridge.md`, `spec/index.md`, the agent
  skill's `SKILL.md` (front matter dropped), and every role brief `locales/<locale>/for/<role>/index.md` at
  `/<locale>/for/<role>/`, with a list per locale at `/<locale>/for/`.
- **Links.** A relative link to a document the site shows goes to its page (with its `#fragment`; headings get
  GitHub's anchors); a relative link to any other repository path goes to that path on GitHub (`blob`, or `tree` for a
  directory). External links are kept. The prerender fails on a link to a missing page or a missing anchor.
- **Raw HTML** in the Markdown is escaped and shown as text; the HTML on a page comes only from the Markdown renderer.

## Content copy

The exported repository has no parent directory, so the site cannot read `../README.md` when it builds. The site's
`scripts/sync-content.mjs` copies the documents into `src/content/docs.json` (committed). `--check` fails when the copy
differs from the repository files; CI runs it, and `bin/make-github-pages` runs it before pushing. The document list
is in that script and in `src/lib/site.ts`.

## Locale and direction

`<html lang>` is the brief's locale in BCP 47 form (`en-gb-oxendict` → `en-GB-oxendict`) on a brief or a locale's
list, and `en` elsewhere; it is written into the prerendered HTML by `hooks.server.ts` and kept in step on client-side
navigation. The site's own words (navigation, headings, footer) are English and marked `lang="en"`. `dir` is `rtl`
for a right-to-left language (none among the brief locales today). The locale switcher opens the same brief in the
chosen locale. Every layout rule uses logical properties; code blocks stay left to right.

## Colour and accessibility

The colour tokens are the UI's (`--mxi-color-*` over the Lily `--color-*` tokens), including the contrast fix of
commit `49675dc`: `--mxi-color-link` (primary mixed 70/30 with the text colour) for every link, and
`--mxi-color-primary` (primary mixed 80/20 with the neutral) as a fill. A change to the UI's tokens is made there
first, then copied here. Links in running text are underlined. A table or code block that scrolls sideways is
focusable. The target is no serious or critical axe finding (WCAG 2.2 AA tags) under the light and the dark system
preference.

## Content security policy

GitHub Pages sends no custom headers, so the policy is SvelteKit's `<meta http-equiv>` with hashes of its own inline
scripts (`mode: "hash"`): `default-src 'self'`, `script-src 'self'` plus hashes, `style-src 'self'`, and the same
`style-src-attr` hash as the UI (SvelteKit's announcer only). Computed styles use Svelte's `style:` directive, never a
`style="…"` attribute. `frame-ancestors` cannot be set from a meta tag and is not set.

## Tests

`tests/e2e/site.spec.ts` (Playwright with `@axe-core/playwright`, against `vite preview` of the build): every page
answers 200; axe under light and dark on every document, every locale's list and one brief per locale; no CSP report
in the console; `lang` on a brief and on a document; the locale switcher; links on and off the site; four pages under
a forced `dir="rtl"` (brand at the inline start, no sideways scroll, axe clean in both schemes); no sideways scroll at
390 px; no element with a `style` attribute. CI job `site` runs the content check, `svelte-check`, the build and the
suite.

## Publishing

`make github-pages` runs `bin/make-github-pages`, which runs
`git subtree push --prefix=project-portfolio-management.github.io github-pages main` after checking that the
`github-pages` remote exists, the content copy is current and the directory has no uncommitted change. The exported
repository deploys with its own `.github/workflows/deploy.yml` (GitHub Pages' actions; inert inside the monorepo).
Each action there is pinned to a full commit SHA with its release tag in a comment (`# v7.0.1`), as the workforce
repository's site does, so a moved tag cannot change what runs. No Dependabot entry watches that file (the site's
entry is npm only, and the `github-actions` entry reads the root `.github`), so the pins are moved by hand.
The remote and the repository are created by the owner; the steps are in the site's
[`README.md`](../project-portfolio-management.github.io/README.md#publishing).

## Not built

The site's words are English only; no right-to-left brief exists to read; only the `light` and `dark` themes;
the first paint (or a visit with JavaScript off) uses the light fallback tokens whatever the system preference;
the site is not published.
