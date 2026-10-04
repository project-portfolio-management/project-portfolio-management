import type { LayoutLoad } from "./$types";
import { DEFAULT_LOCALE, localeOfPath } from "#lib/locales.js";

// SPA mode: no SSR, no prerender. The session lives in the browser
// (localStorage), so every route renders client-side.
export const ssr = false;
export const prerender = false;

// The locale is part of the URL (`/cy-001/plans`): read it here, once, for the
// whole tree. `+layout.svelte` hands it to the i18n store.
// A universal load replaces the server layout's data, so pass it through.
export const load: LayoutLoad = ({ url, data }) => ({
  ...data,
  locale: localeOfPath(url.pathname) ?? DEFAULT_LOCALE,
});
