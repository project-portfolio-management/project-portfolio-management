// The site's own words (navigation, pickers, footer, search) in English, Arabic, Hindi, Welsh and Mandarin (Simplified). The documents
// are not translated here: the role briefs have their own files and the other documents are English.
//
// The language is the role brief's on a brief or a list (so those pages are prerendered in it), and otherwise the reader's
// choice in the locale picker, which only the browser knows: those pages are prerendered in English and change as they load.
import { page } from "$app/state";
import messages from "./messages.json" with { type: "json" };
import {
  DEFAULT_LOCALE,
  isBriefLocale,
  isHomeLocale,
  isRtl,
  langTag,
  topicSlug,
  type BriefLocale,
  type TopicKey,
} from "./site.js";

export type SiteLang = keyof typeof messages;
export type MessageKey = keyof typeof messages.en;

const STORAGE_KEY = "lily-locale";

/** The site's language for a locale. */
export function siteLangOf(locale: string): SiteLang {
  const base = locale.split("-")[0] ?? "";
  return base in messages ? (base as SiteLang) : "en";
}

/** Set only in the browser, so prerendering one page cannot leak a language into the next. */
let chosen = $state<BriefLocale | null>(null);

function pageLocale(): BriefLocale | null {
  const locale: unknown = page.data.locale;
  return typeof locale === "string" && isBriefLocale(locale) ? locale : null;
}

/** The locale the picker shows. */
export function shownLocale(): BriefLocale {
  return pageLocale() ?? chosen ?? DEFAULT_LOCALE;
}

/** The locale for links to the home page and the topics: a locale with neither falls back to English. */
function pagesLocale(): BriefLocale {
  const locale = shownLocale();
  return isHomeLocale(locale) ? locale : DEFAULT_LOCALE;
}

/** The home page: `/` until a locale is in play. */
export function homeRoute(): string {
  return pageLocale() || chosen ? `/${pagesLocale()}/` : "/";
}

/** A topic, in the current locale. */
export function topicRoute(key: TopicKey): string {
  const locale = pagesLocale();
  return `/${locale}/${topicSlug(locale, key)}/`;
}

/** The `lang` value for the site's own words. */
export function siteTag(): string {
  const locale = shownLocale();
  return siteLangOf(locale) === "en" ? "en" : langTag(locale);
}

/** The `dir` for the site's own words, which follow the reader's choice even on a page that is in English. */
export function siteDir(): "ltr" | "rtl" {
  return isRtl(shownLocale()) ? "rtl" : "ltr";
}

/** A site word in the current language; `{name}` is filled from `vars`. */
export function t(key: MessageKey, vars: Record<string, string | number> = {}): string {
  const text: string = messages[siteLangOf(shownLocale())][key];
  return text.replace(/\{(\w+)\}/g, (whole, name: string) => String(vars[name] ?? whole));
}

/** Read the reader's choice. Storage can throw (private window, blocked site data): the site then stays in English. */
export function restoreChoice(): void {
  try {
    const saved = localStorage.getItem(STORAGE_KEY);
    if (saved && isBriefLocale(saved)) chosen = saved;
  } catch {
    // English it is.
  }
}

export function choose(locale: BriefLocale, persist: boolean): void {
  chosen = locale;
  if (!persist) return;
  try {
    localStorage.setItem(STORAGE_KEY, locale);
  } catch {
    // The choice lasts until the page is reloaded.
  }
}
