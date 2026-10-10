// Sign-out action (BFF): revoke the session server-side, then clear the
// httpOnly cookie and redirect home.

import type { Actions, PageServerLoad } from "./$types";
import { redirect } from "@sveltejs/kit";
import { DEFAULT_LOCALE, localizePath } from "#lib/locales.js";
import { signout } from "#lib/server/auth.js";
import { SESSION_COOKIE } from "#lib/server/session.js";
import { oidcConfig, oidcOptions } from "#lib/server/config.js";
import { OIDC_COOKIE, oidcStore, providerMetadata } from "#lib/server/oidc-session.js";
import { t, pageTitle } from "#lib/i18n.svelte.js";

// `page.data.title` convention (see `./+layout.svelte`): mirrors this
// route's own <svelte:head><title>{t("brand.name")}</title> so
// SharePicker gets the right title without reading the DOM.
export const load: PageServerLoad = ({ url }) => {
  return { title: pageTitle(url, "p.home.main_x_plans") };
};

export const actions: Actions = {
  signout: async ({ locals, fetch, cookies, url }) => {
    const home = localizePath("/", locals.locale ?? DEFAULT_LOCALE);
    if (locals.sessionId) {
      await signout(fetch, locals.sessionId);
    }
    cookies.delete(SESSION_COOKIE, { path: "/" });
    if (locals.oidc) {
      // End the BFF session. Ending the provider's session too (RP-initiated logout) is
      // opt-in (OIDC_END_SESSION=true): the provider must have our post-logout address
      // registered, or it shows its own error page.
      const idToken = locals.oidc.idToken;
      oidcStore.delete(locals.oidc.id);
      cookies.delete(OIDC_COOKIE, { path: "/" });
      const cfg = oidcConfig();
      if (cfg && oidcOptions().endProviderSession) {
        try {
          const meta = await providerMetadata(fetch, cfg.issuer);
          if (meta.end_session_endpoint) {
            const q = new URLSearchParams({ post_logout_redirect_uri: `${url.origin}${home}` });
            if (idToken) q.set("id_token_hint", idToken);
            redirect(303, `${meta.end_session_endpoint}?${q.toString()}`, { external: true });
          }
        } catch (e) {
          if (e && typeof e === "object" && "status" in e) throw e; // a redirect
        }
      }
    }
    redirect(303, home);
  },
};
