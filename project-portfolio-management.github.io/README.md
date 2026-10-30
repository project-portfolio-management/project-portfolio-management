# project-portfolio-management.github.io

The Project Portfolio Management documentation site for GitHub Pages: a static SvelteKit build (`adapter-static`,
Svelte 5 runes, Lily Design System widgets) of the repository's own documents.

This directory lives inside the monorepo, `project-portfolio-management/project-portfolio-management`, and is
**exported** to its own repository, `project-portfolio-management/project-portfolio-management.github.io`, with
`git subtree` (the family pattern: the WPM repository's `spec/monorepo-github-pages/`). Change the site here; never
work in the exported repository. The specification is [`../spec/github-pages-site.md`](../spec/github-pages-site.md).

**Status: built, not published.** No `github-pages` remote is configured and the exported repository does not exist
yet. The steps to publish are below.

## What it shows

| Route | Document |
| --- | --- |
| `/` | [`index.md`](../index.md) |
| `/readme/` | [`README.md`](../README.md) |
| `/install/` | [`INSTALL.md`](../INSTALL.md) |
| `/wpm-bridge/` | [`operations/wpm-bridge.md`](../operations/wpm-bridge.md) |
| `/spec/` | [`spec/index.md`](../spec/index.md) |
| `/skill/` | [`project-portfolio-management-skill/SKILL.md`](../project-portfolio-management-skill/SKILL.md) |
| `/<locale>/for/` and `/<locale>/for/<role>/` | the role briefs, [`locales/<locale>/for/<role>/index.md`](../locales/) (10 locales, 11 roles) |

A link between two of these documents stays on the site; a link to any other file in the repository goes to that
file on GitHub. Raw HTML in the Markdown is shown as text. On a role brief the header's language button (Lily
`LocalePicker`) opens the same brief in another locale, and `<html lang>` is the brief's locale. The theme follows the
system's light or dark preference (Lily `ThemePicker`, the `light` and `dark` themes only); the colour tokens are the
PPM UI's, with its light and dark contrast fix (commit `49675dc`). Text size uses Lily `TextSizePicker`.

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
PW_PORT=4189 CI=true pnpm exec playwright test   # every page answers; axe in light and dark; locale switch; rtl
```

The monorepo's CI job `site` runs the same commands. `BASE_PATH` (empty by default) serves the site from a sub-path
if it is ever published as a project site rather than the organization site.

## Publishing

The user who owns the GitHub organization does this once; nothing here has done any of it.

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

- The site's own words (navigation, headings, footer) are English only; the role briefs are in their locales.
- No brief locale is right-to-left today. The layout uses logical properties and the tests force `dir="rtl"` on four
  pages, but no right-to-left brief has been read by a person.
- Only the `light` and `dark` Lily themes are offered and audited.
- Before the theme stylesheet loads (the first paint, or with JavaScript off) the page uses the light fallbacks of the
  tokens, whatever the system preference.
- GitHub Pages cannot send HTTP headers, so the content security policy is a `<meta http-equiv>` tag, which cannot
  carry `frame-ancestors`.
