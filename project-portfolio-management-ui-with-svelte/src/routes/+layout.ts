import { redirect } from "@sveltejs/kit";
import type { LayoutLoad } from "./$types";
import { LOCALE_KEY, loadLocale } from "#lib/i18n.svelte.js";
import {
  DEFAULT_LOCALE,
  type Locale,
  localeFromAcceptLanguage,
  localeOfPath,
  localeRedirect,
  normalizeLocale,
} from "#lib/locales.js";

// SPA mode: no SSR, no prerender. The session lives in the browser
// (localStorage), so every route renders client-side.
export const ssr = false;
export const prerender = false;

// What the browser says its languages are, most preferred first. `navigator.language`
// (one tag, maybe `cy_GB`) is the fallback for a browser that leaves `languages` empty.
function browserLanguages(): string {
  const list = navigator.languages?.length
    ? [...navigator.languages]
    : [navigator.language];
  return list.filter(Boolean).join(",");
}

// The reader's own choice for a URL that names none: the remembered locale,
// else the browser's languages, else English.
function preferredLocale(): Locale {
  if (typeof localStorage === "undefined") return DEFAULT_LOCALE;
  return (
    normalizeLocale(localStorage.getItem(LOCALE_KEY)) ??
    localeFromAcceptLanguage(browserLanguages()) ??
    DEFAULT_LOCALE
  );
}

// The locale is part of the URL (`/cy-001/plans`): read it here, once, for the
// whole tree. `+layout.svelte` hands it to the i18n store. A client-side
// navigation to a bare URL (`goto("/plans")`) never reaches the
// server's redirect, so it is localized here, the same way.
// A universal load replaces the server layout's data, so pass it through.
// The locale's page catalogue is loaded here, before the page is drawn (WW-1.1: they are loaded per locale).
export const load: LayoutLoad = async ({ url, data }) => {
  const to = localeRedirect(url, preferredLocale());
  if (to) redirect(to.status, to.location);
  const locale = localeOfPath(url.pathname) ?? DEFAULT_LOCALE;
  await loadLocale(locale);
  return { ...data, locale };
};
