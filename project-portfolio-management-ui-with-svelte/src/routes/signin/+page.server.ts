// Sign-in action (BFF, per-app magic-link login): the server asks the
// authentication service for a magic link that returns to THIS app's own
// `/verify` (via `return_url` = this origin, honoured by the auth
// service's allow-list). No credential is held client-side.

import type { Actions, PageServerLoad } from "./$types";
import { redirect } from "@sveltejs/kit";
import { requestMagicLink } from "#lib/server/auth.js";
import { pageTitle } from "#lib/i18n.svelte.js";
import { DEFAULT_LOCALE, localizePath } from "#lib/locales.js";
import { demoClaims, demoEnabled, isDemoEmail, DEMO_TOKEN } from "#lib/server/demo.js";
import { OIDC_COOKIE, OIDC_COOKIE_OPTIONS } from "#lib/server/oidc-session.js";
import { sessions } from "#lib/server/session-store-instance.js";
import { oidcOptions } from "#lib/server/config.js";

// `page.data.title` convention (see `../+layout.svelte`): mirrors this
// route's own <svelte:head><title> so SharePicker gets the right title
// without reading the DOM.
export const load: PageServerLoad = ({ url }) => {
  return { title: pageTitle(url, "p.signin.sign_in"), demo: demoEnabled() };
};

export const actions: Actions = {
  default: async ({ request, fetch, url, cookies, locals }) => {
    const form = await request.formData();
    const email = String(form.get("email") ?? "").trim();
    const localeRaw = form.get("locale");
    const locale = localeRaw ? String(localeRaw) : undefined;
    if (!email) {
      return { sent: false, error: "email-required" };
    }
    if (demoEnabled()) {
      // The demonstration: one fixed address, no email, a session in this server's own store.
      if (!isDemoEmail(email)) return { sent: false, error: "demo-email" };
      const now = Math.floor(Date.now() / 1000);
      const session = await sessions().createSession({
        accessToken: DEMO_TOKEN,
        sessionSeconds: oidcOptions().sessionSeconds,
        claims: demoClaims(now),
      });
      cookies.set(OIDC_COOKIE, session.id, { ...OIDC_COOKIE_OPTIONS, maxAge: oidcOptions().sessionSeconds });
      redirect(303, localizePath("/demo", locals.locale ?? DEFAULT_LOCALE));
    }
    // Return to this app's own origin so the emailed link lands on our
    // /verify (the auth service honours it only if allow-listed).
    const ok = await requestMagicLink(fetch, email, url.origin, locale);
    return ok ? { sent: true, error: null } : { sent: false, error: "failed" };
  },
};
