// BFF layout load: expose whether a session is present (resolved
// server-side from the httpOnly cookie) so the chrome can show signed-in
// state without the browser ever holding a token. When signed in, also
// resolves the caller's `view` ABAC attribute (T-28f, repo `tasks.md`
// EV-1) for role-tailored nav ordering / landing route — a presentation
// choice only; every route stays reachable by URL regardless.

import type { LayoutServerLoad } from "./$types";
import { currentUser } from "#lib/server/auth.js";
import { demoEnabled } from "#lib/server/demo.js";
import { KNOWN_NAV_HREFS, landingRouteForView, viewAttr } from "#lib/nav.js";

const DEFAULT_LANDING_ROUTE = "/plans";

export const load: LayoutServerLoad = async (event) => {
  const data = await loadSession(event);
  return { ...data, demo: demoEnabled() };
};

const loadSession = async ({
  locals,
  fetch,
}: Parameters<LayoutServerLoad>[0]) => {
  if (!locals.sessionId && locals.oidc) {
    // A direct OpenID Connect sign-in: no authentication-service session to ask
    // about attributes, so the default landing route applies.
    return {
      signedIn: true,
      view: null,
      landingRoute: DEFAULT_LANDING_ROUTE,
      userName: locals.oidc.user.name,
    };
  }
  if (!locals.sessionId) {
    return { signedIn: false, view: null, landingRoute: DEFAULT_LANDING_ROUTE };
  }
  const user = await currentUser(fetch, locals.sessionId);
  const view = viewAttr(user?.attrs);
  return {
    signedIn: true,
    view,
    landingRoute: landingRouteForView(
      KNOWN_NAV_HREFS,
      view,
      DEFAULT_LANDING_ROUTE,
    ),
  };
};
