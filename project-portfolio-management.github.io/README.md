# project-portfolio-management.github.io

The Project Portfolio Management public site for GitHub Pages: a static SvelteKit build (`adapter-static`, Svelte 5
runes, Lily's whole PickerBar) for newcomers (home, benefits, features, comparisons, a tutorial) over the
repository's own reference documents, in English, Arabic, Hindi, Welsh and Mandarin.

This directory lives inside the monorepo, `project-portfolio-management/project-portfolio-management`, and is
**exported** to its own repository, `project-portfolio-management/project-portfolio-management.github.io`, with
`git subtree` (the family pattern: the WPM repository's `spec/monorepo-github-pages/`). Change the site here; never
work in the exported repository. The specification is [`../spec/github-pages-site.md`](../spec/github-pages-site.md).

**Status: published** at <https://project-portfolio-management.github.io/> (first deployed 2026-11-04, GP-2). The
exported repository's own `git subtree` history is what GitHub Pages builds; the steps are below.

## What it shows

| Route | Source |
| --- | --- |
| `/`, `/<locale>/` | [`locales/<locale>/index.md`](../locales/) (eleven locales; `/` is `en-001`) |
| `/<locale>/<topic>/` | `locales/<locale>/topics/<slug>/index.md`: benefits, features, comparisons, tutorial, with a slug in the language |
| `/compare/`, `/compare/<product>/` | `locales/en-001/compare/<product>/index.md` (English) |
| `/docs/` | [`index.md`](../index.md) |
| `/readme/`, `/install/`, `/wpm-bridge/`, `/spec/`, `/skill/` | [`README.md`](../README.md), [`INSTALL.md`](../INSTALL.md), [`operations/wpm-bridge.md`](../operations/wpm-bridge.md), [`spec/index.md`](../spec/index.md), [`SKILL.md`](../project-portfolio-management-skill/SKILL.md) |
| `/<locale>/for/`, `/<locale>/for/<role>/` | the role briefs, [`locales/<locale>/for/<role>/index.md`](../locales/) (11 locales, 11 roles) |
| `/search/`, `/search-index.json` | search, in the browser |
| `/sitemap.xml`, `/<locale>/sitemap.xml` | the sitemaps |

A link between two of these stays on the site; a link to any other file in the repository goes to that file on GitHub.
Raw HTML in the Markdown is shown as text. Every claim on the site must be one the repository supports; see "Claims" in
[the spec](../spec/github-pages-site.md).

The site's own words are in `src/lib/messages.json`. After changing them, or a translated page, run
`python3 scripts/export-site-translation-review.py` from the monorepo root. To add a page to a locale, add its `index.md`,
a `README.md` symlink and the same `.locale-peer-id` as its peers.

## Content

The site does not read `..` when it builds: the exported repository has no parent. `scripts/sync-content.mjs` copies
the documents into `src/content/docs.json`, which is committed. After editing any of the documents above, run in this
directory:

```sh
pnpm run sync          # rewrite src/content/docs.json
pnpm run sync:check    # CI runs this: fails if the copy is stale
```

## Build and test

Node 26 and pnpm.

```sh
pnpm install
pnpm run check                               # svelte-check
pnpm run build                               # prerenders every page into build/
PW_PORT=4189 CI=true pnpm exec playwright test   # the whole suite; see spec/github-pages-site.md
```

The monorepo's CI job `site` runs the same commands. `BASE_PATH` (empty by default) serves the site from a sub-path
if it is ever published as a project site rather than the organization site.

## Publishing

Steps 1 to 3 were done once, on 2026-11-04. A new clone of the monorepo needs only step 3 (the remote lives in
`.git/config`, not in the repository); every publish after that is step 4.

1. On GitHub, create the empty public repository `project-portfolio-management/project-portfolio-management.github.io`
   (no README, licence or `.gitignore`, so the first push is not rejected).
2. In that repository, open Settings, then Pages, and set Source to **GitHub Actions**. The workflow
   `.github/workflows/deploy.yml` in this directory becomes that repository's root workflow; it builds the site and
   deploys it.
3. In the monorepo clone, add the remote (the spec's `.git/config` entry):

   ```sh
   git remote add github-pages git@github.com:project-portfolio-management/project-portfolio-management.github.io.git
   ```

4. Commit any change to this directory, then from the monorepo root run:

   ```sh
   make github-pages
   ```

   which runs `bin/make-github-pages`: it checks that the remote exists, that `src/content/docs.json` is current and
   that this directory has no uncommitted change, then runs
   `git subtree push --prefix=project-portfolio-management.github.io github-pages main`.
5. Watch the Deploy to GitHub Pages run in the exported repository; the site is then at
   <https://project-portfolio-management.github.io/>.

Every later publish is step 4 again.

## Not done

See "Not built" in [the spec](../spec/github-pages-site.md). In short: Welsh and Mandarin are unreviewed machine drafts;
the product comparisons are English only and have not been checked against each product's current documentation;
the tutorial's commands come from `operations/demo.md` and were not run for this site; 15 of the 57 themes fail contrast
on the home page; the role briefs keep English slugs. GitHub Pages cannot send HTTP headers, so the content security
policy is a `<meta http-equiv>` tag, which cannot carry `frame-ancestors`.
