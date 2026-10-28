// Server-side BFF endpoints (never bundled into the browser).

import * as env from "$app/env/private";
import { baseUrl, type Bridges } from "../bridge.js";

/** Portfolio service base URL — the proxy forwards entity-API calls here with
 *  a server-injected PASETO. (loco dev default 5150.) */
export const PROJECT_PORTFOLIO_MANAGEMENT_API_URL =
  env.PROJECT_PORTFOLIO_MANAGEMENT_API_URL || "http://localhost:5150";

/** Authentication service base URL — for the session→PASETO exchange and
 *  the magic-link login flow. */
export const AUTH_API_URL = env.AUTH_API_URL || "http://localhost:5150";

import type { OidcConfig } from "./oidc";

/**
 * Direct OpenID Connect sign-in (HD-12), or `null` when it is not configured
 * (`OIDC_ISSUER` and `OIDC_CLIENT_ID` both required). Entra: the issuer is
 * `https://login.microsoftonline.com/<tenant-id>/v2.0`. Keycloak: the realm URL
 * `https://<host>/realms/<realm>`.
 */
export function oidcConfig(): OidcConfig | null {
  const issuer = (env.OIDC_ISSUER ?? "").trim();
  const clientId = (env.OIDC_CLIENT_ID ?? "").trim();
  if (!issuer || !clientId) return null;
  return {
    issuer,
    clientId,
    clientSecret: (env.OIDC_CLIENT_SECRET ?? "").trim() || undefined,
    scopes:
      (env.OIDC_SCOPES ?? "").trim() || "openid profile email offline_access",
    tenantId: (env.OIDC_TENANT_ID ?? "").trim() || undefined,
    idpHint: (env.OIDC_IDP_HINT ?? "").trim() || undefined,
  };
}

/** Deployment options for the OIDC flow that are not part of the protocol. */
export function oidcOptions(): {
  /** Fixed redirect URI; otherwise built from the request's origin. */
  redirectUri?: string;
  /** Hard session length in seconds (default 8 hours). */
  sessionSeconds: number;
  /** Sign out after this many seconds without a request; 0 turns it off (default 30 minutes). */
  idleSeconds: number;
  /** Also end the provider's session on sign-out (RP-initiated logout). */
  endProviderSession: boolean;
} {
  const idleMinutes = Number.parseFloat(env.OIDC_IDLE_MINUTES ?? "");
  const hours = Number.parseFloat(env.OIDC_SESSION_HOURS ?? "");
  return {
    redirectUri: (env.OIDC_REDIRECT_URI ?? "").trim() || undefined,
    sessionSeconds: Math.round(
      (Number.isFinite(hours) && hours > 0 ? hours : 8) * 3600,
    ),
    idleSeconds: Math.round(
      (Number.isFinite(idleMinutes) && idleMinutes >= 0 ? idleMinutes : 30) *
        60,
    ),
    endProviderSession: (env.OIDC_END_SESSION ?? "").trim() === "true",
  };
}

/** The sibling apps' base URLs (`CONTACTS_APP_URL`, `WORKFORCE_APP_URL`), each `null` unless a web URL. */
export function bridgeConfig(): Bridges {
  return {
    contacts: baseUrl(env.CONTACTS_APP_URL),
    workforce: baseUrl(env.WORKFORCE_APP_URL),
  };
}
