// A minimal Microsoft Entra ID-compatible OpenID Connect provider and a mock API, for the
// browser sign-in test (HD-12). Shapes follow Entra's v2.0 endpoints:
//   {base}/{tenant}/v2.0/.well-known/openid-configuration
//   {base}/{tenant}/oauth2/v2.0/{authorize,token,logout}
//   {base}/{tenant}/discovery/v2.0/keys
// ID tokens are RS256 with Entra's claims (`ver: "2.0"`, `tid`, `oid`, `preferred_username`,
// `roles`, `nonce`). The provider enforces what a real one does: an exact redirect URI,
// PKCE (S256) at the token endpoint, single-use codes.
//
// Port 4180: the provider. Port 4181: the "API", which answers only a bearer token this
// provider issued (so the test can tell the BFF sent the right one).

import http from "node:http";
import crypto from "node:crypto";

const IDP_PORT = Number(process.env.MOCK_IDP_PORT ?? 4180);
const API_PORT = Number(process.env.MOCK_API_PORT ?? 4181);
const TENANT = "11111111-2222-3333-4444-555555555555";
const OTHER_TENANT = "99999999-0000-0000-0000-000000000000";
const CLIENT_ID = "mock-client-id";
const REDIRECT_URI =
  process.env.MOCK_REDIRECT_URI ?? "http://localhost:4173/signin/oidc/callback";
const base = `http://localhost:${IDP_PORT}`;

const { publicKey, privateKey } = crypto.generateKeyPairSync("rsa", {
  modulusLength: 2048,
});
const jwk = {
  ...publicKey.export({ format: "jwk" }),
  kid: "mock-key-1",
  use: "sig",
  alg: "RS256",
};

const codes = new Map(); // code -> { nonce, challenge, user, tenant }
const refreshTokens = new Map(); // refresh token -> user
const accessTokens = new Map(); // access token -> { user, expires }
const stats = { codesIssued: 0, refreshes: 0, apiCalls: 0, pkceFailures: 0 };

const USERS = {
  alice: {
    sub: "alice-sub",
    oid: "oid-alice",
    name: "Alice Example",
    email: "alice@contoso.example",
    roles: ["Portfolio.Editor"],
    expiresIn: 3599,
  },
  short: {
    sub: "short-sub",
    oid: "oid-short",
    name: "Shorty Example",
    email: "short@contoso.example",
    roles: [],
    expiresIn: 1,
  },
};

const b64 = (o) => Buffer.from(JSON.stringify(o)).toString("base64url");
function idToken(user, tenant, nonce) {
  const now = Math.floor(Date.now() / 1000);
  const claims = {
    ver: "2.0",
    iss: `${base}/${tenant}/v2.0`,
    aud: CLIENT_ID,
    sub: user.sub,
    oid: user.oid,
    tid: tenant,
    name: user.name,
    preferred_username: user.email,
    email: user.email,
    roles: user.roles,
    nonce,
    iat: now,
    nbf: now,
    exp: now + 3600,
  };
  const input = `${b64({ alg: "RS256", typ: "JWT", kid: jwk.kid })}.${b64(claims)}`;
  const sig = crypto.createSign("RSA-SHA256").update(input).sign(privateKey);
  return `${input}.${sig.toString("base64url")}`;
}

function mintTokens(user, tenant, nonce) {
  const access = `at_${crypto.randomBytes(24).toString("base64url")}`;
  const refresh = `rt_${crypto.randomBytes(24).toString("base64url")}`;
  accessTokens.set(access, {
    user,
    expires: Date.now() + user.expiresIn * 1000,
  });
  refreshTokens.set(refresh, { user, tenant });
  return {
    token_type: "Bearer",
    scope: "openid profile email",
    expires_in: user.expiresIn,
    access_token: access,
    refresh_token: refresh,
    ...(nonce ? { id_token: idToken(user, tenant, nonce) } : {}),
  };
}

const json = (res, status, body) => {
  res.writeHead(status, {
    "content-type": "application/json",
    "cache-control": "no-store",
  });
  res.end(JSON.stringify(body));
};
const html = (res, body) => {
  res.writeHead(200, {
    "content-type": "text/html; charset=utf-8",
    "cache-control": "no-store",
  });
  res.end(
    `<!doctype html><meta charset="utf-8"><title>Mock Entra sign-in</title><body>${body}</body>`,
  );
};
const readForm = (req) =>
  new Promise((resolve) => {
    let data = "";
    req.on("data", (c) => (data += c));
    req.on("end", () => resolve(new URLSearchParams(data)));
  });

const idp = http.createServer(async (req, res) => {
  const url = new URL(req.url, base);
  const path = url.pathname;
  if (path === "/__health") return json(res, 200, { ok: true });
  if (path === "/__stats") return json(res, 200, stats);
  const m = path.match(/^\/([^/]+)\/(.*)$/);
  if (!m) return json(res, 404, { error: "not_found" });
  const [, tenant, rest] = m;

  if (rest === "v2.0/.well-known/openid-configuration") {
    return json(res, 200, {
      issuer: `${base}/${tenant}/v2.0`,
      authorization_endpoint: `${base}/${tenant}/oauth2/v2.0/authorize`,
      token_endpoint: `${base}/${tenant}/oauth2/v2.0/token`,
      jwks_uri: `${base}/${tenant}/discovery/v2.0/keys`,
      end_session_endpoint: `${base}/${tenant}/oauth2/v2.0/logout`,
      response_types_supported: ["code", "id_token", "code id_token"],
      id_token_signing_alg_values_supported: ["RS256"],
      subject_types_supported: ["pairwise"],
      scopes_supported: ["openid", "profile", "email", "offline_access"],
      code_challenge_methods_supported: ["S256"],
    });
  }
  if (rest === "discovery/v2.0/keys") return json(res, 200, { keys: [jwk] });

  if (rest === "oauth2/v2.0/authorize") {
    const q = url.searchParams;
    if (q.get("client_id") !== CLIENT_ID)
      return html(res, "<h1>AADSTS700016: unknown application</h1>");
    if (q.get("redirect_uri") !== REDIRECT_URI)
      return html(res, "<h1>AADSTS50011: the redirect URI does not match</h1>");
    if (
      q.get("response_type") !== "code" ||
      q.get("code_challenge_method") !== "S256" ||
      !q.get("code_challenge")
    ) {
      return html(res, "<h1>invalid_request: PKCE (S256) is required</h1>");
    }
    if (!(q.get("scope") ?? "").split(" ").includes("openid"))
      return html(res, "<h1>invalid_scope</h1>");
    const state = q.get("state") ?? "";
    const issue = (user, forTenant) => {
      const code = `code_${crypto.randomBytes(16).toString("base64url")}`;
      codes.set(code, {
        nonce: q.get("nonce"),
        challenge: q.get("code_challenge"),
        user,
        tenant: forTenant,
      });
      stats.codesIssued += 1;
      const back = new URL(REDIRECT_URI);
      back.searchParams.set("code", code);
      back.searchParams.set("state", state);
      return back.toString();
    };
    const deny = new URL(REDIRECT_URI);
    deny.searchParams.set("error", "access_denied");
    deny.searchParams.set(
      "error_description",
      "AADSTS65004: the user declined to consent",
    );
    deny.searchParams.set("state", state);
    return html(
      res,
      `<h1>Sign in</h1>
       <p><a id="as-alice" href="${issue(USERS.alice, TENANT)}">Continue as Alice Example</a></p>
       <p><a id="as-short" href="${issue(USERS.short, TENANT)}">Continue as Shorty Example (token lasts one second)</a></p>
       <p><a id="as-other-tenant" href="${issue(USERS.alice, OTHER_TENANT)}">Continue as someone from another tenant</a></p>
       <p><a id="deny" href="${deny.toString()}">Cancel</a></p>`,
    );
  }

  if (rest === "oauth2/v2.0/token" && req.method === "POST") {
    const f = await readForm(req);
    if (f.get("client_id") !== CLIENT_ID)
      return json(res, 401, { error: "invalid_client" });
    if (f.get("grant_type") === "authorization_code") {
      const entry = codes.get(f.get("code") ?? "");
      codes.delete(f.get("code") ?? ""); // single use
      if (!entry)
        return json(res, 400, {
          error: "invalid_grant",
          error_description: "AADSTS70008: code expired or used",
        });
      if (f.get("redirect_uri") !== REDIRECT_URI)
        return json(res, 400, { error: "invalid_grant" });
      const verifier = f.get("code_verifier") ?? "";
      const challenge = crypto
        .createHash("sha256")
        .update(verifier)
        .digest("base64url");
      if (!verifier || challenge !== entry.challenge) {
        stats.pkceFailures += 1;
        return json(res, 400, {
          error: "invalid_grant",
          error_description: "AADSTS501481: the code_verifier does not match",
        });
      }
      return json(res, 200, mintTokens(entry.user, entry.tenant, entry.nonce));
    }
    if (f.get("grant_type") === "refresh_token") {
      const entry = refreshTokens.get(f.get("refresh_token") ?? "");
      refreshTokens.delete(f.get("refresh_token") ?? "");
      if (!entry) return json(res, 400, { error: "invalid_grant" });
      stats.refreshes += 1;
      // A refreshed token lasts long enough to observe (the short user keeps rolling).
      const user = { ...entry.user, expiresIn: 3599 };
      return json(res, 200, mintTokens(user, entry.tenant, null));
    }
    return json(res, 400, { error: "unsupported_grant_type" });
  }

  if (rest === "oauth2/v2.0/logout")
    return html(res, "<h1>You signed out of the mock provider</h1>");
  return json(res, 404, { error: "not_found" });
});

const api = http.createServer((req, res) => {
  stats.apiCalls += 1;
  const bearer = (req.headers.authorization ?? "").replace(/^Bearer /i, "");
  const entry = accessTokens.get(bearer);
  if (!entry || entry.expires < Date.now())
    return json(res, 401, { error: "unauthorized" });
  json(res, 200, {
    ok: true,
    sub: entry.user.sub,
    name: entry.user.name,
    roles: entry.user.roles,
    path: req.url,
  });
});

idp.listen(IDP_PORT, () => console.log(`mock Entra ID on ${base}`));
api.listen(API_PORT, () =>
  console.log(`mock API on http://localhost:${API_PORT}`),
);
