import type { Reroute } from "@sveltejs/kit/hooks";
import { splitPath } from "#lib/locales.js";

// Locale-prefixed URLs (`/cy-001/plans`) are served by the routes that already
// exist (`/plans`): strip a supported locale prefix before route matching.
// Only the *matched route* changes — `url`, links and `page.url` keep the
// prefix. An alias (`/en/plans`) or an unprefixed path is left alone here; the
// server's `handle` redirects both before any page is rendered.
export const reroute: Reroute = ({ url }) => {
  const { locale, rest } = splitPath(url.pathname);
  return locale ? rest : undefined;
};
