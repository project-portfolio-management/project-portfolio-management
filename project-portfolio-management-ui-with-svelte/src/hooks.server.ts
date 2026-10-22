import type { Handle } from "@sveltejs/kit/hooks";
import { redirect } from "@sveltejs/kit";
import type { Cookies } from "@sveltejs/kit";

// BFF entry point: read the httpOnly session cookie on every request and
// expose the opaque session id to server `load`/actions/endpoints via
// `locals`. The browser never reads the cookie (httpOnly); only the
// SvelteKit server does, and only the server talks to the services.
import { SESSION_COOKIE } from "#lib/server/session.js";
import { OIDC_COOKIE, OIDC_COOKIE_OPTIONS } from "#lib/server/oidc-session.js";
import { sessions } from "#lib/server/session-store-instance.js";
import { oidcOptions } from "#lib/server/config.js";
import {
  DEFAULT_LOCALE,
  type Locale,
  isRtl,
  isUnlocalized,
  localeFromAcceptLanguage,
  localizePath,
  normalizeLocale,
  splitPath,
} from "#lib/locales.js";

import { applySecurityHeaders, isHttps } from "#lib/security-headers.js";

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
    normalizeLocale(cookies.get(LOCALE_COOKIE)) ??
    localeFromAcceptLanguage(acceptLanguage) ??
    DEFAULT_LOCALE
  );
}

export const handle: Handle = async (input) => {
  const response = await localeHandle(input);
  const { event } = input;
  const https = isHttps(event.url, event.request.headers.get("x-forwarded-proto"));
  try {
    applySecurityHeaders(response.headers, https);
    return response;
  } catch {
    // Immutable headers (e.g. a response passed through from `fetch`): copy the response
    // and set them on the copy, so no answer leaves without them (CQ-19).
    const copy = new Response(response.body, {
      status: response.status,
      statusText: response.statusText,
      headers: new Headers(response.headers),
    });
    applySecurityHeaders(copy.headers, https);
    return copy;
  }
};

sessions().setIdleSeconds(oidcOptions().idleSeconds);

const localeHandle: Handle = async ({ event, resolve }) => {
  event.locals.sessionId = event.cookies.get(SESSION_COOKIE) ?? null;
  const oidcCookie = event.cookies.get(OIDC_COOKIE);
  event.locals.oidc = await sessions().get(oidcCookie);
  if (!event.locals.oidc && (await sessions().endedIdle(oidcCookie))) {
    // The session sat idle too long (RG-2): clear the cookie and, for a page the person
    // asked for, say why they are back at the sign-in page. API calls just see "signed out".
    event.cookies.delete(OIDC_COOKIE, OIDC_COOKIE_OPTIONS);
    const { pathname } = event.url;
    if (
      event.request.headers.get("sec-fetch-dest") === "document" &&
      !pathname.startsWith("/api/") &&
      !pathname.includes("/signin")
    ) {
      const to = negotiateLocale(event.cookies, event.request.headers.get("accept-language"));
      redirect(303, `${localizePath("/signin", to)}?error=oidc_idle`);
    }
  }

  const { pathname, search } = event.url;
  if (!isUnlocalized(pathname)) {
    const { locale, rest } = splitPath(pathname);
    const reading =
      event.request.method === "GET" || event.request.method === "HEAD";
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
