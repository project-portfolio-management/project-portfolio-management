// The sitemap: every static page of the app, once per locale, each with `hreflang`
// alternates for the others. Pure — the route files below are found by the caller
// (`import.meta.glob`) and passed in, so this is unit-tested without a build.
//
// Only pages with a fixed address are listed. A page whose path has a parameter
// (`/plans/[pid]/...`) names a record, and a sitemap must not enumerate records.
// Sign-in and token-verification pages are left out on purpose, and so is anything
// under `/api`.

import {
  DEFAULT_LOCALE,
  LOCALES,
  localizePath,
  type Locale,
} from "./locales.js";

/** Route prefixes that are never listed. */
const EXCLUDED = ["/signin", "/verify", "/api", "/display"];

/**
 * The public URL path of each page file (`/src/routes/plans/+page.svelte` ->
 * `/plans`, the home page -> `/`). Parameterised, grouped-only-segment and
 * excluded pages are dropped; the result is sorted and has no duplicates.
 */
export function pagePaths(files: readonly string[]): string[] {
  const out = new Set<string>();
  for (const file of files) {
    const m =
      /^\/?(?:src\/)?routes\/(.*?)\/?\+page\.svelte$/.exec(file) ??
      /^\/?(?:src\/)?routes\/\+page\.svelte$/.exec(file);
    if (!m) continue;
    const segments = (m[1] ?? "")
      .split("/")
      .filter(Boolean)
      .filter((s) => !/^\(.*\)$/.test(s));
    if (segments.some((s) => s.includes("[") || s.includes("]"))) continue;
    const path = "/" + segments.join("/");
    if (EXCLUDED.some((x) => path === x || path.startsWith(x + "/"))) continue;
    out.add(path);
  }
  return [...out].sort();
}

/** `&`, `<`, `>`, quotes: the characters an XML attribute or text node cannot hold raw. */
export function xmlEscape(s: string): string {
  return s
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&apos;");
}

/** `en-001` -> `en`, `de-de` -> `de-DE`, `zh-cn` -> `zh-CN`: the BCP 47 form `hreflang` wants. */
export function hreflang(locale: Locale): string {
  const [lang, region] = locale.split("-");
  if (!region || region === "001") return lang ?? locale;
  return `${lang}-${region.toUpperCase()}`;
}

/** The origin (`https://host`), without a trailing slash; `null` if it is not an http(s) origin. */
export function cleanOrigin(raw: string): string | null {
  try {
    const u = new URL(raw);
    if (u.protocol !== "http:" && u.protocol !== "https:") return null;
    return u.origin;
  } catch {
    return null;
  }
}

/** The sitemap XML for `paths` under `origin`. */
export function buildSitemap(origin: string, paths: readonly string[]): string {
  const url = (path: string, locale: Locale) =>
    xmlEscape(origin + localizePath(path, locale));
  const entries: string[] = [];
  for (const path of paths) {
    const alternates = [
      ...LOCALES.map(
        (l) =>
          `    <xhtml:link rel="alternate" hreflang="${hreflang(l)}" href="${url(path, l)}"/>`,
      ),
      `    <xhtml:link rel="alternate" hreflang="x-default" href="${url(path, DEFAULT_LOCALE)}"/>`,
    ].join("\n");
    for (const locale of LOCALES) {
      entries.push(
        `  <url>\n    <loc>${url(path, locale)}</loc>\n${alternates}\n  </url>`,
      );
    }
  }
  return (
    `<?xml version="1.0" encoding="UTF-8"?>\n` +
    `<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:xhtml="http://www.w3.org/1999/xhtml">\n` +
    entries.join("\n") +
    `\n</urlset>\n`
  );
}

/** `robots.txt`: nothing is disallowed that is not behind sign-in anyway, and the sitemap is named. */
export function buildRobots(origin: string): string {
  return `User-agent: *\nDisallow: /api/\nSitemap: ${origin}/sitemap.xml\n`;
}
