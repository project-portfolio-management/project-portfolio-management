// BFF reverse proxy for entity-API calls.
//
// The browser calls this same-origin proxy (sending the httpOnly session
// cookie automatically, no token in JS). The server exchanges the session
// for a short-lived PASETO and forwards the request to the portfolio service
// with `Authorization: Bearer <paseto>`. The browser thus never holds a
// token, and pages keep using the existing client unchanged (its base URL
// points here).

import type { RequestHandler } from "./$types";
import { PROJECT_PORTFOLIO_MANAGEMENT_API_URL } from "#lib/server/config.js";
import { exchangeToken } from "#lib/server/auth.js";
import { oidcConfig } from "#lib/server/config.js";
import { accessTokenFor, providerMetadata } from "#lib/server/oidc-session.js";
import { sessions } from "#lib/server/session-store-instance.js";
import { demoEnabled } from "#lib/server/demo.js";
import { relayResponseHeaders } from "#lib/server/proxy-headers.js";

const proxy: RequestHandler = async ({
  request,
  params,
  url,
  locals,
  fetch,
}) => {
  const target = `${PROJECT_PORTFOLIO_MANAGEMENT_API_URL}/${params.path}${url.search}`;

  // Copy request headers, but drop hop-by-hop / origin-specific ones and
  // never forward the browser's cookie to the entity service.
  const headers = new Headers(request.headers);
  headers.delete("cookie");
  headers.delete("host");
  headers.delete("connection");
  headers.delete("content-length");

  // Negotiate the API version via header, not the URL (URLs are
  // version-free). See `agents/share/api-versioning.md` §6.4.
  headers.set("accepts-version", "1.0");

  // Inject the server-exchanged PASETO when a session is present.
  if (locals.sessionId) {
    const token = await exchangeToken(fetch, locals.sessionId);
    if (token) headers.set("authorization", `Bearer ${token}`);
  } else if (locals.oidc && demoEnabled()) {
    // A demonstration session: its access token was signed with the published demonstration key at sign-in.
    headers.set("authorization", `Bearer ${locals.oidc.accessToken}`);
  } else if (locals.oidc) {
    // A direct OpenID Connect session: the provider's access token (refreshed when it is
    // about to expire) goes to the API, which verifies it. The BFF never interprets it.
    const cfg = oidcConfig();
    if (cfg) {
      try {
        const meta = await providerMetadata(fetch, cfg.issuer);
        const token = await accessTokenFor(sessions(), locals.oidc, { fetchFn: fetch, cfg, meta });
        if (token) headers.set("authorization", `Bearer ${token}`);
      } catch {
        // Provider unreachable: forward without a token; the API answers 401.
      }
    }
  }

  const init: RequestInit = { method: request.method, headers };
  if (request.method !== "GET" && request.method !== "HEAD") {
    init.body = await request.arrayBuffer();
  }

  const upstream = await fetch(target, init);

  // Relay status + body, and the response headers the client reads (an allow-list).
  return new Response(upstream.body, {
    status: upstream.status,
    headers: relayResponseHeaders(upstream.headers),
  });
};

export const GET = proxy;
export const POST = proxy;
export const PUT = proxy;
export const PATCH = proxy;
export const DELETE = proxy;
