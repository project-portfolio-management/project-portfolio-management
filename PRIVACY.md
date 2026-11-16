# Privacy notice for the documentation site

This notice covers the documentation site at <https://project-portfolio-management.github.io>, built from this
repository's `project-portfolio-management.github.io/` directory. It does not cover the Project Portfolio Management
service or its user interface: an organization that runs them writes its own notice, and
[`governance/dpia.md`](governance/dpia.md) is the starting point for it.

## What the site keeps in your browser

The site keeps a choice only when you make it in one of the pickers at the top of each page. A first visit stores
nothing. What it keeps stays in your own browser. The site sends none of it anywhere, and none of it identifies you.

| What | Name | When it is kept | Example value |
| --- | --- | --- | --- |
| Your theme | `ppm-theme-choice` | You pick a theme other than the one your browser's light or dark setting asks for. Picking your browser's own theme again removes it. | `nord` |
| The language of the site's own words | `lily-locale` | You pick a language in the language picker. Opening a page in a language is not a choice and is not kept. | `cy-001` |
| Your time zone | `lily-time-zone` | You pick a time zone in the time zone picker. The zone the site guesses from your browser is not kept. | `Asia/Tokyo` |
| Your text size | `lily-text-size` | You pick a text size in the text size picker. | `larger` |

**Why:** so that the next page you open, and your next visit, look the way you chose, without an account. Without
them, every page would start again from your browser's settings.

**Where:** in your browser's `localStorage` for this site. A build of the site may keep the same four choices in
first-party cookies with the same names instead (each for a year, for this site only, `SameSite=Lax`, and `Secure`
over HTTPS). The published site uses `localStorage`. GitHub Pages serves the site as static files and does not read
either.

**How long:** until you change or clear them. `localStorage` has no expiry. A cookie expires a year after the choice
was last changed.

**How to remove them:** pick your browser's own theme to remove the theme choice, or clear this site's data in your
browser's settings to remove them all. In a private window, or where your browser blocks site data, nothing is kept
past the page.

Before 16 November 2026 the site kept the theme, time zone and text size the pickers guessed from your browser, even
when you had chosen nothing, and the theme under the name `lily-theme`. On your next visit, a `lily-theme` value of
`light` or `dark` is removed as most likely that guess, and any other theme moves to `ppm-theme-choice`. A time zone
or text size kept that way stays until you change or clear it.

## What the site does not do

The site has no accounts, sign-in, analytics, advertising, tracking or third-party scripts, and it sets no cookie
unless it is built to keep your choices in cookies. Its pages, styles, themes and search index load from the
site itself. The share picker opens another site (for example LinkedIn or Mastodon) only when you choose one, and that
site's own notice then applies. The search runs in your browser over the site's own index; the words you search for
are in the search page's address, which GitHub receives like any other request.

## GitHub

GitHub hosts the site. GitHub, not this project, receives each request your browser makes for a page, including your
IP address, and its [privacy statement](https://docs.github.com/en/site-policy/privacy-policies/github-general-privacy-statement)
says what it keeps. This project sees none of it.

## Changes and questions

Changes to this notice are made in this file and recorded in [`CHANGELOG.md`](CHANGELOG.md). Questions go to the
maintainers named in [`MAINTAINERS.md`](MAINTAINERS.md).
