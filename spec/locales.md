# Locales

> **Source of truth for locale naming, the URL scheme and how a locale is chosen.**
> The supported set is `LOCALES` in
> [`project-portfolio-management-ui-with-svelte/src/lib/locales.ts`](../project-portfolio-management-ui-with-svelte/src/lib/locales.ts);
> this chapter says what the rules are, the code and its tests enforce them.
> Requirement text: [NFR-4 and NFR-4a](07-non-functional-requirements.md).

## 1. Naming

Every locale is `<language>-<region>`, **lowercase and hyphenated**: `en-001`, `cy-001`,
`de-de`, `zh-cn`. `-001` is the UN M.49 code for "world" and stands for a language with no
regional variant. This one form is used everywhere a locale is named: URL prefixes,
`<html lang>`, the remembered-choice cookie and storage key, string-catalogue keys, and any
directory or file stem that is named for a locale.

- **A bare two-letter code is not a locale.** `en` is never a URL prefix, a directory or a
  file. `/en/plans` is an ordinary unprefixed path: it gains the reader's own locale prefix
  like any other bare path and then finds no page. There is **no forwarding** from `/en/…`
  to `/en-001/…` (removed 2026-10-07).
- **No `locales/en/` directory exists, or may.** The repository has no locale directories at
  all: strings live in `src/lib/i18n.svelte.ts`, keyed by these codes. Should locale files
  ever be split out, they are named `<language>-<region>`.
- `tests/unit/locale-names.test.ts` fails the build if a bare two-letter locale directory or
  file appears, or if a locale directory holds a badly named entry.

## 2. Supported locales

Ten, sorted by code (the picker shows them in this order):

| Code | Language | Direction |
|---|---|---|
| `ar-001` | Arabic | right-to-left |
| `cy-001` | Welsh | left-to-right |
| `de-001` | German (world) | left-to-right |
| `de-de` | German (Germany) | left-to-right |
| `en-001` | English (world) — the default and the source of truth for strings | left-to-right |
| `es-001` | Spanish | left-to-right |
| `fr-001` | French | left-to-right |
| `hi-001` | Hindi | left-to-right |
| `zh-001` | Chinese (world) | left-to-right |
| `zh-cn` | Chinese (China) | left-to-right |

Only `ar-001` is right-to-left (`RTL_LOCALES`). The static shell carries the right `lang` and
`dir` before any script runs, and the layout keeps `document.documentElement.dir` in step.

To add one: extend `LOCALES`, add its entry to `LOCALE_LABELS` (written in that language) and
add a full catalogue to `STRINGS`. A parity test requires every catalogue to have exactly
the English keys, no more and no fewer.

## 3. URLs

Every app page is served under its locale prefix: `/en-001/plans`, `/cy-001/plans/{pid}`.
The root is `/<locale>` (no trailing slash). The routes themselves are not moved: the
universal `reroute` hook strips a supported prefix before route matching, while `url`,
links and `page.url` keep it. A path whose first segment is not a supported locale is
**bare**. Unlocalized paths are `/api/…`, `/_app/…`, `/assets/…`, `/.well-known/…`,
`/favicon.svg`, `/robots.txt` and `/sitemap.xml`.

## 4. Choosing a locale for a bare URL

For a bare GET (the root `/`, a bookmark, the magic-link email) the reader's locale is, in
order:

1. the **remembered choice** — the `locale` cookie (server) or `mxi.case.locale` in
   `localStorage` (client), set whenever a localized page is served or the picker is used;
2. the browser's language — `Accept-Language` on the server; `navigator.languages`, then
   `navigator.language`, in the client;
3. `en-001`.

A browser language resolves to the **nearest supported locale**: an exact match first
(`de-DE` or `de_DE` → `de-de`), otherwise the first supported locale of the same language
(`cy-GB` → `cy-001`, `de-AT` → `de-001`, `es-MX` → `es-001`); a language with none
(`ja-JP`) falls through to the next rule. Case, hyphen and underscore are ignored.
`q`-values are honoured and `q=0` refuses a tag. The redirect is temporary (302 on the
server, 307 client-side) and keeps the query string; a non-GET is not redirected. A path
that already names a locale is never redirected, and the **URL decides what is shown**: the
remembered choice only picks the locale for a bare URL.

Covered by `tests/unit/{locales,hooks}.test.ts` and `tests/e2e/root-locale.spec.ts`.

## 5. Not the same thing: the language of the data

A plan's `in_language` is the language its **content** is written in. It is a free **BCP 47
tag** (`en`, `en-GB`, `cy`), validated for shape by the API (`validation.rs`) and carried
verbatim in the DTO. It is not a UI locale and is not constrained to the ten above or to the
`<language>-<region>` form. Text in any language is Unicode-correct end to end, and the
matcher folds with NFKC and preserves diacritics (NFR-4).

## 6. The sitemap

`/sitemap.xml` lists every fixed page once per locale, with `hreflang` alternates for all ten
and an `x-default` pointing at `en-001` (BCP 47 form: `en`, `de-DE`, `zh-CN`). Pages whose
path names a record (`[pid]`), sign-in and token pages and `/api` are not listed.
