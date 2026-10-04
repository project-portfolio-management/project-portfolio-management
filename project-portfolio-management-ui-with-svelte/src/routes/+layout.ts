import { redirect } from "@sveltejs/kit";
import type { LayoutLoad } from "./$types";
import { LOCALE_KEY } from "#lib/i18n.svelte.js";
import {
  DEFAULT_LOCALE,
  type Locale,
  localeFromAcceptLanguage,
  localeOfPath,
  localeRedirect,
  normaliseLocale,
} from "#lib/locales.js";

// SPA mode: no SSR, no prerender. The session lives in the browser
// (localStorage), so every route renders client-side.
export const ssr = false;
export const prerender = false;

// The reader's own choice for a URL that names none: the remembered locale,
// else the browser's languages, else English.
function preferredLocale(): Locale {
  if (typeof localStorage === "undefined") return DEFAULT_LOCALE;
  return (
    normaliseLocale(localStorage.getItem(LOCALE_KEY)) ??
    localeFromAcceptLanguage(navigator.languages?.join(",")) ??
    DEFAULT_LOCALE
  );
}

// The locale is part of the URL (`/cy-001/plans`): read it here, once, for the
// whole tree. `+layout.svelte` hands it to the i18n store. A client-side
// navigation to a bare or alias URL (`goto("/plans")`) never reaches the
// server's redirect, so it is localized here, the same way.
// A universal load replaces the server layout's data, so pass it through.
export const load: LayoutLoad = ({ url, data }) => {
  const to = localeRedirect(url, preferredLocale());
  if (to) redirect(to.status, to.location);
  return { ...data, locale: localeOfPath(url.pathname) ?? DEFAULT_LOCALE };
};
