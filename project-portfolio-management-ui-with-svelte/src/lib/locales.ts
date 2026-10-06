// The locale set and the pure functions over it — no runes, no `$app/*`
// imports — so the server hooks, the universal `reroute` hook, the i18n store
// and unit tests all share one definition.
//
// Locale codes are lowercase and hyphenated ("en-001", "zh-cn"). `-001` is the
// UN M.49 code for "world": a language with no regional variant.
//
// URLs: every app page lives under a locale prefix (`/en-001/plans`). A bare
// two-letter code is an **alias** for the one locale of that language and
// redirects to it (`/en/plans` -> `/en-001/plans`), so each page has exactly one
// canonical address. The routes themselves are not moved: the universal
// `reroute` hook (`src/hooks.ts`) strips the prefix before route matching.

/**
 * Locales for which the UI is translated, sorted alphabetically by code
 * (the LocalePicker shows them in this order). To add one, extend this
 * tuple AND add a matching entry to {@link LOCALE_LABELS} and `STRINGS`
 * (in `i18n.svelte.ts`).
 */
export const LOCALES = [
  "ar-001",
  "cy-001",
  "de-001",
  "de-de",
  "en-001",
  "es-001",
  "fr-001",
  "hi-001",
  "zh-001",
  "zh-cn",
] as const;

/** A supported locale code (one of {@link LOCALES}). */
export type Locale = (typeof LOCALES)[number];

/** Fallback locale for an unknown key, locale, or missing translation. */
export const DEFAULT_LOCALE: Locale = "en-001";

/**
 * Human-readable name for the locale switcher, written in that locale.
 * A `-001` (world) locale is labelled by its language alone; a regional
 * locale reads "<language> - <region>" (never "<language> (<region>)").
 */
export const LOCALE_LABELS: Record<Locale, string> = {
  "ar-001": "العربية",
  "cy-001": "Cymraeg",
  "de-001": "Deutsch",
  "de-de": "Deutsch - Deutschland",
  "en-001": "English",
  "es-001": "Español",
  "fr-001": "Français",
  "hi-001": "हिन्दी",
  "zh-001": "中文",
  "zh-cn": "中文 - 中国",
};

/**
 * Right-to-left locales. The layout mirrors `<html dir>` to `rtl` for
 * these (and `ltr` for everyone else) via {@link isRtl}.
 */
export const RTL_LOCALES = ["ar-001"] as const satisfies readonly Locale[];

/**
 * Normalise raw input to a supported locale, or null if unsupported.
 * Case-insensitive and hyphen/underscore-insensitive. A region or legacy
 * code resolves by primary language (`es-MX`, `en_US`, `zh`, `en` all map
 * to the one supported locale of that language).
 */
export function normaliseLocale(raw: string | null | undefined): Locale | null {
  if (!raw) return null;
  const normalized = raw.trim().replace(/_/g, "-").toLowerCase();
  const exact = LOCALES.find((l) => l === normalized);
  if (exact) return exact;
  const primary = normalized.split("-")[0] ?? "";
  return LOCALES.find((l) => l.split("-")[0] === primary) ?? null;
}

/**
 * Whether `locale` is written right-to-left. Tolerant of region subtags via
 * {@link normaliseLocale}.
 */
export function isRtl(locale: string): boolean {
  const primary = normaliseLocale(locale);

  return (
    primary !== null && (RTL_LOCALES as readonly string[]).includes(primary)
  );
}

/**
 * Two-letter alias -> the locale it stands for: `en` -> `en-001`, `cy` ->
 * `cy-001`, and likewise for every language that has a `-001` locale; a
 * language with exactly one (regional) locale would alias to it. Never shadows a real locale code.
 */
export function localeAliases(): Record<string, Locale> {
  const out: Record<string, Locale> = {};
  const byLanguage = new Map<string, Locale[]>();
  for (const l of LOCALES) {
    const lang = l.split("-")[0] ?? l;
    byLanguage.set(lang, [...(byLanguage.get(lang) ?? []), l]);
  }
  for (const [lang, codes] of byLanguage) {
    const world = codes.find((c) => c === `${lang}-001`);
    const target = world ?? (codes.length === 1 ? codes[0] : undefined);
    if (target && !(LOCALES as readonly string[]).includes(lang)) {
      out[lang] = target;
    }
  }
  return out;
}

/** The alias table, built once. */
export const LOCALE_ALIASES: Readonly<Record<string, Locale>> = localeAliases();

/** The first path segment, if it is shaped like a locale or an alias. */
const FIRST_SEGMENT = /^\/([a-z]{2,3}(?:-[a-z0-9]{2,3})?)(?=\/|$)/;

/** What a pathname's locale prefix says. */
export interface SplitPath {
  /** The supported locale in the prefix, if any. */
  locale: Locale | null;
  /** The two-letter alias in the prefix, if any (and its target). */
  alias: Locale | null;
  /** The path without the prefix (always starts with `/`). */
  rest: string;
}

/**
 * Split a pathname into its locale prefix and the rest.
 * `/cy-001/plans/1` -> `{ locale: "cy-001", rest: "/plans/1" }`;
 * `/en/plans` -> `{ alias: "en-001", rest: "/plans" }`; `/plans` -> neither.
 */
export function splitPath(pathname: string): SplitPath {
  const m = FIRST_SEGMENT.exec(pathname);
  const seg = m?.[1];
  if (seg) {
    const rest = pathname.slice(seg.length + 1) || "/";
    if ((LOCALES as readonly string[]).includes(seg)) {
      return { locale: seg as Locale, alias: null, rest };
    }
    const target = LOCALE_ALIASES[seg];
    if (target) return { locale: null, alias: target, rest };
  }
  return { locale: null, alias: null, rest: pathname || "/" };
}

/** The locale named by a pathname's prefix (an alias counts), or null. */
export function localeOfPath(pathname: string): Locale | null {
  const s = splitPath(pathname);
  return s.locale ?? s.alias;
}

/** Paths that are never localized: the BFF API, framework assets, static files. */
const UNLOCALIZED =
  /^\/(api|_app|assets|\.well-known)(\/|$)|^\/(favicon\.svg|robots\.txt|sitemap\.xml)$/;

/** Whether `pathname` belongs outside the locale tree. */
export function isUnlocalized(pathname: string): boolean {
  return UNLOCALIZED.test(pathname);
}

/**
 * Put `locale` in front of an app path: `localizePath("/plans?x=1", "cy-001")`
 * -> `/cy-001/plans?x=1`; `/` -> `/cy-001`. A path that already carries a locale prefix has it
 * replaced; an external or fragment-only or unlocalized path is returned as is.
 */
export function localizePath(path: string, locale: Locale): string {
  if (!path.startsWith("/") || path.startsWith("//") || isUnlocalized(path)) {
    return path;
  }
  const cut = path.search(/[?#]/);
  const pathname = cut === -1 ? path : path.slice(0, cut);
  const tail = cut === -1 ? "" : path.slice(cut);
  const { rest } = splitPath(pathname);
  // The root is `/<locale>`: SvelteKit does not keep a trailing slash.
  return `/${locale}${rest === "/" ? "" : rest}${tail}`;
}

/**
 * The best supported locale for an `Accept-Language` header, or null.
 * Tags are taken in the order given; q-values below 1 are ordered after
 * higher ones (stable), and `q=0` is refused.
 */
export function localeFromAcceptLanguage(
  header: string | null | undefined,
): Locale | null {
  if (!header) return null;
  const tags = header
    .split(",")
    .map((part, i) => {
      const [tag = "", ...params] = part.trim().split(";");
      const q = params
        .map((p) => /^\s*q=([0-9.]+)\s*$/.exec(p)?.[1])
        .find((v) => v !== undefined);
      return { tag: tag.trim(), q: q === undefined ? 1 : Number(q), i };
    })
    .filter((x) => x.tag && x.tag !== "*" && x.q > 0)
    .sort((a, b) => b.q - a.q || a.i - b.i);
  for (const { tag } of tags) {
    const l = normaliseLocale(tag);
    if (l) return l;
  }
  return null;
}

/**
 * Where a URL that is not in the locale tree should go, or null if it is fine
 * where it is: an alias goes to its locale (permanent), a bare path gains
 * `preferred`'s prefix (temporary). Used by the root layout's `load`, which
 * catches client-side navigations the server never sees.
 */
export function localeRedirect(
  url: { pathname: string; search: string; hash: string },
  preferred: Locale,
): { status: 301 | 307; location: string } | null {
  if (isUnlocalized(url.pathname)) return null;
  const { locale, alias, rest } = splitPath(url.pathname);
  if (locale) return null;
  return {
    status: alias ? 301 : 307,
    location: `${localizePath(rest, alias ?? preferred)}${url.search}${url.hash}`,
  };
}
