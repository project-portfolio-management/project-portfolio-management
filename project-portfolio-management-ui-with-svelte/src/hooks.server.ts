import type { Handle } from "@sveltejs/kit/hooks";
import { redirect } from "@sveltejs/kit";
import type { Cookies } from "@sveltejs/kit";

// BFF entry point: read the httpOnly session cookie on every request and
// expose the opaque session id to server `load`/actions/endpoints via
// `locals`. The browser never reads the cookie (httpOnly); only the
// SvelteKit server does, and only the server talks to the services.
import { SESSION_COOKIE } from "#lib/server/session.js";
import {
  DEFAULT_LOCALE,
  type Locale,
  isRtl,
  isUnlocalized,
  localeFromAcceptLanguage,
  localizePath,
  normaliseLocale,
  splitPath,
} from "#lib/locales.js";

/** Remembers the reader's locale, to choose one for a bare, unprefixed URL. */
export const LOCALE_COOKIE = "locale";

/**
 * The locale for a request that names none: the remembered one, else the
 * browser's `Accept-Language`, else English.
 */
export function negotiateLocale(
  cookies: Pick<Cookies, "get">,
  acceptLanguage: string | null,
): Locale {
  return (
    normaliseLocale(cookies.get(LOCALE_COOKIE)) ??
    localeFromAcceptLanguage(acceptLanguage) ??
    DEFAULT_LOCALE
  );
}

export const handle: Handle = async ({ event, resolve }) => {
  event.locals.sessionId = event.cookies.get(SESSION_COOKIE) ?? null;

  const { pathname, search } = event.url;
  if (!isUnlocalized(pathname)) {
    const { locale, alias, rest } = splitPath(pathname);
    const reading =
      event.request.method === "GET" || event.request.method === "HEAD";
    if (alias) {
      // `/en/plans` is another name for `/en-001/plans`: one canonical address
      // per page, so redirect rather than serve the same page twice.
      redirect(301, `${localizePath(rest, alias)}${search}`);
    }
    if (!locale && reading) {
      // A bare URL (a bookmark, the magic-link email) gains its locale prefix.
      const to = negotiateLocale(
        event.cookies,
        event.request.headers.get("accept-language"),
      );
      redirect(302, `${localizePath(rest, to)}${search}`);
    }
    if (locale) {
      event.locals.locale = locale;
      if (event.cookies.get(LOCALE_COOKIE) !== locale) {
        event.cookies.set(LOCALE_COOKIE, locale, {
          path: "/",
          maxAge: 60 * 60 * 24 * 365,
          sameSite: "lax",
          httpOnly: false,
          secure: event.url.protocol === "https:",
        });
      }
    }
  }
  const locale = event.locals.locale ?? DEFAULT_LOCALE;
  return resolve(event, {
    // The static shell carries the right `lang`/`dir` before any script runs.
    transformPageChunk: ({ html }) =>
      html
        .replace('lang="%lang%"', `lang="${locale}"`)
        .replace('dir="%dir%"', `dir="${isRtl(locale) ? "rtl" : "ltr"}"`),
  });
};
