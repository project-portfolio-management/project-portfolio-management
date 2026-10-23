// Start a direct OpenID Connect sign-in (HD-12): remember the login's state, nonce
// and PKCE verifier on the server, bind it to this browser with a short-lived
// cookie, and send the browser to the provider. A plain navigation (an <a href>),
// not a form post, so the content security policy's `form-action 'self'` is not in
// the way. 404 unless OIDC is configured.

import type { RequestHandler } from "./$types";
import { error, redirect } from "@sveltejs/kit";
import { oidcConfig, oidcOptions } from "#lib/server/config.js";
import {
  authorizationUrl,
  pkcePair,
  randomToken,
  safeReturnPath,
} from "#lib/server/oidc.js";
import {
  OIDC_COOKIE_OPTIONS,
  OIDC_TX_COOKIE,
  TX_TTL_MS,
  providerMetadata,
} from "#lib/server/oidc-session.js";
import { sessions } from "#lib/server/session-store-instance.js";
import { DEFAULT_LOCALE, localizePath } from "#lib/locales.js";

export const GET: RequestHandler = async ({ url, cookies, fetch, locals }) => {
  const cfg = oidcConfig();
  if (!cfg) error(404, "Sign-in with an identity provider is not enabled");
  const locale = locals.locale ?? DEFAULT_LOCALE;

  let meta;
  try {
    meta = await providerMetadata(fetch, cfg.issuer);
  } catch {
    redirect(303, `${localizePath("/signin", locale)}?error=oidc_unavailable`);
  }

  const state = randomToken();
  const nonce = randomToken();
  const { verifier, challenge } = pkcePair();
  const redirectUri =
    oidcOptions().redirectUri ?? `${url.origin}/signin/oidc/callback`;
  const returnTo = safeReturnPath(url.searchParams.get("return"));
  await sessions().startTransaction(state, {
    nonce,
    verifier,
    // Remember where to land, in the reader's own language.
    returnTo: localizePath(returnTo, locale),
    redirectUri,
    locale,
  });
  cookies.set(OIDC_TX_COOKIE, state, {
    ...OIDC_COOKIE_OPTIONS,
    maxAge: TX_TTL_MS / 1000,
  });
  redirect(
    303,
    authorizationUrl(meta, cfg, { state, nonce, challenge, redirectUri }),
    {
      external: true,
    },
  );
};
