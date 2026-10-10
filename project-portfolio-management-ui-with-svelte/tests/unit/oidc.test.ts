// The OpenID Connect client (HD-12): token validation, PKCE, discovery, the
// transaction and session store, and the safe-return rule. No network: `fetch` is
// injected and the keys are generated here.

import { createHash, createSign, generateKeyPairSync, randomBytes } from "node:crypto";
import { describe, expect, it, vi } from "vitest";
import {
  OidcError,
  authorizationUrl,
  discover,
  exchangeCode,
  pkcePair,
  safeReturnPath,
  verifyIdToken,
  type Jwk,
  type OidcConfig,
  type ProviderMetadata,
} from "../../src/lib/server/oidc";
import { OidcStore, accessTokenFor } from "../../src/lib/server/oidc-session";

const ISSUER = "https://login.example.test/11111111-2222-3333-4444-555555555555/v2.0";
const CLIENT = "client-123";
const CFG: OidcConfig = { issuer: ISSUER, clientId: CLIENT, scopes: "openid profile email" };
const META: ProviderMetadata = {
  issuer: ISSUER,
  authorization_endpoint: `${ISSUER}/authorize`,
  token_endpoint: `${ISSUER}/token`,
  jwks_uri: `${ISSUER}/keys`,
};

const pair = generateKeyPairSync("rsa", { modulusLength: 2048 });
const jwk: Jwk = { ...(pair.publicKey.export({ format: "jwk" }) as Jwk), kid: "k1", use: "sig" };
const other = generateKeyPairSync("rsa", { modulusLength: 2048 });

const b64 = (o: unknown) => Buffer.from(JSON.stringify(o)).toString("base64url");
function sign(claims: Record<string, unknown>, opts: { alg?: string; kid?: string; key?: typeof pair.privateKey } = {}) {
  const header = { alg: opts.alg ?? "RS256", kid: opts.kid ?? "k1", typ: "JWT" };
  const input = `${b64(header)}.${b64(claims)}`;
  const sig = createSign("RSA-SHA256").update(input).sign(opts.key ?? pair.privateKey);
  return `${input}.${sig.toString("base64url")}`;
}
const NOW = 1_800_000_000;
const good = (over: Record<string, unknown> = {}) => ({
  iss: ISSUER,
  aud: CLIENT,
  sub: "user-1",
  exp: NOW + 600,
  iat: NOW,
  nonce: "n-1",
  name: "Alice Example",
  tid: "11111111-2222-3333-4444-555555555555",
  ...over,
});
const expectOk = { issuer: ISSUER, audience: CLIENT, nonce: "n-1", now: NOW };

describe("verifyIdToken", () => {
  it("accepts a well-formed token and returns its claims", () => {
    const c = verifyIdToken(sign(good()), [jwk], expectOk);
    expect(c.sub).toBe("user-1");
    expect(c.name).toBe("Alice Example");
  });

  it("refuses a token signed by another key", () => {
    const t = sign(good(), { key: other.privateKey });
    expect(() => verifyIdToken(t, [jwk], expectOk)).toThrow(/bad signature/);
  });

  it("refuses alg none, HS256 and anything but RS256", () => {
    for (const alg of ["none", "HS256", "ES256", "PS256"]) {
      const t = sign(good(), { alg });
      expect(() => verifyIdToken(t, [jwk], expectOk)).toThrow(/only RS256/);
    }
    // the classic confusion attack: an HMAC token keyed with the public key
    const input = `${b64({ alg: "HS256", kid: "k1" })}.${b64(good())}`;
    expect(() => verifyIdToken(`${input}.AAAA`, [jwk], expectOk)).toThrow(OidcError);
  });

  it("refuses a tampered payload", () => {
    const [h, , s] = sign(good()).split(".");
    const forged = `${h}.${b64(good({ sub: "admin" }))}.${s}`;
    expect(() => verifyIdToken(forged, [jwk], expectOk)).toThrow(/bad signature/);
  });

  it.each([
    ["issuer", { iss: "https://evil.example/v2.0" }, /wrong issuer/],
    ["audience", { aud: "someone-else" }, /wrong audience/],
    ["nonce", { nonce: "replayed" }, /wrong nonce/],
    ["subject", { sub: "" }, /no subject/],
    ["expiry", { exp: NOW - 3600 }, /expired/],
    ["not-before", { nbf: NOW + 3600 }, /not yet valid/],
  ])("refuses a wrong %s", (_n, over, re) => {
    expect(() => verifyIdToken(sign(good(over)), [jwk], expectOk)).toThrow(re);
  });

  it("accepts an audience list that includes the client, tolerates small clock skew", () => {
    expect(() => verifyIdToken(sign(good({ aud: ["x", CLIENT] })), [jwk], expectOk)).not.toThrow();
    expect(() => verifyIdToken(sign(good({ exp: NOW - 30 })), [jwk], expectOk)).not.toThrow();
  });

  it("enforces the tenant when one is configured", () => {
    const t = sign(good({ tid: "99999999-0000-0000-0000-000000000000" }));
    expect(() =>
      verifyIdToken(t, [jwk], { ...expectOk, tenantId: "11111111-2222-3333-4444-555555555555" }),
    ).toThrow(/wrong tenant/);
    expect(() =>
      verifyIdToken(sign(good()), [jwk], { ...expectOk, tenantId: "11111111-2222-3333-4444-555555555555" }),
    ).not.toThrow();
  });

  it("picks the key by kid and refuses an unknown kid", () => {
    expect(() => verifyIdToken(sign(good(), { kid: "unknown" }), [jwk], expectOk)).toThrow(
      /no matching signing key/,
    );
  });

  it("refuses malformed input", () => {
    for (const bad of ["", "a.b", "a.b.c.d", "!!!.???.***"]) {
      expect(() => verifyIdToken(bad, [jwk], expectOk)).toThrow(OidcError);
    }
  });
});

describe("PKCE and the authorization URL", () => {
  it("the challenge is the S256 of the verifier, and verifiers are unique and long enough", () => {
    const a = pkcePair();
    const b = pkcePair();
    expect(a.verifier).not.toBe(b.verifier);
    expect(a.verifier.length).toBeGreaterThanOrEqual(43);
    expect(a.challenge).toBe(createHash("sha256").update(a.verifier).digest("base64url"));
  });

  it("carries state, nonce, challenge, the exact redirect URI, and always asks for openid", () => {
    const url = new URL(
      authorizationUrl(META, { ...CFG, scopes: "profile email", idpHint: "entra" }, {
        state: "s1",
        nonce: "n1",
        challenge: "c1",
        redirectUri: "https://app.example/signin/oidc/callback",
      }),
    );
    expect(url.origin + url.pathname).toBe(META.authorization_endpoint);
    const q = url.searchParams;
    expect(q.get("response_type")).toBe("code");
    expect(q.get("client_id")).toBe(CLIENT);
    expect(q.get("redirect_uri")).toBe("https://app.example/signin/oidc/callback");
    expect(q.get("scope")?.split(" ")).toEqual(["openid", "profile", "email"]);
    expect(q.get("state")).toBe("s1");
    expect(q.get("nonce")).toBe("n1");
    expect(q.get("code_challenge")).toBe("c1");
    expect(q.get("code_challenge_method")).toBe("S256");
    expect(q.get("kc_idp_hint")).toBe("entra");
  });
});

describe("discover", () => {
  const reply = (body: unknown, status = 200) =>
    vi.fn(async () => new Response(JSON.stringify(body), { status })) as unknown as typeof fetch;

  it("returns the metadata when the issuer matches", async () => {
    const meta = await discover(reply(META), ISSUER);
    expect(meta.token_endpoint).toBe(META.token_endpoint);
  });

  it("refuses an issuer that differs, or a template (Entra common)", async () => {
    await expect(discover(reply({ ...META, issuer: "https://evil.example" }), ISSUER)).rejects.toThrow(
      /different issuer/,
    );
    await expect(
      discover(reply({ ...META, issuer: "https://login.example.test/{tenantid}/v2.0" }), ISSUER),
    ).rejects.toThrow(/template|specific tenant/);
  });

  it("refuses a failing or incomplete document", async () => {
    await expect(discover(reply({}, 500), ISSUER)).rejects.toThrow(OidcError);
    const { token_endpoint: _t, ...partial } = META;
    await expect(discover(reply(partial), ISSUER)).rejects.toThrow(/no token_endpoint/);
  });
});

describe("exchangeCode", () => {
  it("posts the code with the PKCE verifier and the client id, and returns the tokens", async () => {
    let seen: URLSearchParams | undefined;
    const f = vi.fn(async (_u: unknown, init?: RequestInit) => {
      seen = init?.body as URLSearchParams;
      return new Response(JSON.stringify({ access_token: "at", id_token: "it", expires_in: 3599 }));
    }) as unknown as typeof fetch;
    const t = await exchangeCode(f, META, { ...CFG, clientSecret: "shh" }, {
      code: "abc",
      verifier: "ver",
      redirectUri: "https://app.example/cb",
    });
    expect(t.access_token).toBe("at");
    expect(seen?.get("grant_type")).toBe("authorization_code");
    expect(seen?.get("code")).toBe("abc");
    expect(seen?.get("code_verifier")).toBe("ver");
    expect(seen?.get("client_id")).toBe(CLIENT);
    expect(seen?.get("client_secret")).toBe("shh");
  });

  it("fails on a rejected exchange without echoing the response", async () => {
    const f = vi.fn(async () => new Response("secret detail", { status: 400 })) as unknown as typeof fetch;
    const err = await exchangeCode(f, META, CFG, { code: "x", verifier: "v", redirectUri: "r" }).catch(
      (e: Error) => e,
    );
    expect(err).toBeInstanceOf(OidcError);
    expect((err as Error).message).not.toContain("secret detail");
  });
});

describe("safeReturnPath", () => {
  it("keeps a same-site path, query and fragment", () => {
    expect(safeReturnPath("/plans?x=1#top")).toBe("/plans?x=1#top");
    expect(safeReturnPath("/")).toBe("/");
  });
  it.each([
    "https://evil.example/",
    "//evil.example/",
    "/\\evil.example",
    "javascript:alert(1)",
    "/ok\nHeader: x",
    "plans",
    "",
    null,
    undefined,
  ])("turns %j into /", (raw) => {
    expect(safeReturnPath(raw as string)).toBe("/");
  });
});

describe("OidcStore", () => {
  const claims = { iss: ISSUER, sub: "u1", aud: CLIENT, exp: NOW, name: "Alice", roles: ["a"] };

  it("a login transaction is single use and expires", () => {
    let now = 1000;
    const s = new OidcStore(() => now);
    s.startTransaction("st", { nonce: "n", verifier: "v", returnTo: "/", redirectUri: "r", locale: "en-001" });
    expect(s.takeTransaction("st")?.nonce).toBe("n");
    expect(s.takeTransaction("st")).toBeNull();
    s.startTransaction("st2", { nonce: "n", verifier: "v", returnTo: "/", redirectUri: "r", locale: "en-001" });
    now += 11 * 60 * 1000;
    expect(s.takeTransaction("st2")).toBeNull();
    expect(s.takeTransaction("never")).toBeNull();
  });

  it("a session is found by its id, ends at its hard limit, and can be deleted", () => {
    let now = 0;
    const s = new OidcStore(() => now);
    const a = s.createSession({ accessToken: "at", sessionSeconds: 60, claims });
    expect(a.id.length).toBeGreaterThanOrEqual(43);
    expect(s.get(a.id)?.user).toMatchObject({ sub: "u1", name: "Alice", roles: ["a"] });
    expect(s.get("nope")).toBeNull();
    expect(s.get(null)).toBeNull();
    now = 61_000;
    expect(s.get(a.id)).toBeNull();
    const b = s.createSession({ accessToken: "at", sessionSeconds: 60, claims });
    s.delete(b.id);
    expect(s.get(b.id)).toBeNull();
  });

  it("memory is bounded", () => {
    const s = new OidcStore(() => 0, 3);
    for (let i = 0; i < 20; i++) s.createSession({ accessToken: "t", sessionSeconds: 60, claims });
    expect(s.size).toBeLessThanOrEqual(3);
  });

  it("session ids are unpredictable", () => {
    const s = new OidcStore();
    const ids = new Set(
      Array.from({ length: 50 }, () => s.createSession({ accessToken: "t", sessionSeconds: 60, claims }).id),
    );
    expect(ids.size).toBe(50);
    void randomBytes;
  });
});

describe("accessTokenFor", () => {
  const claims = { iss: ISSUER, sub: "u1", aud: CLIENT, exp: NOW, name: "Alice" };
  const store = () => {
    let now = 0;
    const s = new OidcStore(() => now);
    return { s, advance: (ms: number) => (now += ms), now: () => now };
  };

  it("returns the current token while it is fresh", async () => {
    const { s, now } = store();
    const sess = s.createSession({ accessToken: "fresh", expiresInSeconds: 3600, sessionSeconds: 7200, claims });
    const f = vi.fn() as unknown as typeof fetch;
    expect(await accessTokenFor(s, sess, { fetchFn: f, cfg: CFG, meta: META, now })).toBe("fresh");
    expect(f).not.toHaveBeenCalled();
  });

  it("refreshes a token about to expire, and keeps the new refresh token", async () => {
    const { s, advance, now } = store();
    const sess = s.createSession({
      accessToken: "old",
      refreshToken: "r1",
      expiresInSeconds: 60,
      sessionSeconds: 7200,
      claims,
    });
    advance(45_000);
    const f = vi.fn(async () =>
      new Response(JSON.stringify({ access_token: "new", refresh_token: "r2", expires_in: 3600 })),
    ) as unknown as typeof fetch;
    expect(await accessTokenFor(s, sess, { fetchFn: f, cfg: CFG, meta: META, now })).toBe("new");
    expect(sess.refreshToken).toBe("r2");
    expect(s.get(sess.id)).not.toBeNull();
  });

  it("ends the session when it cannot be refreshed", async () => {
    const { s, advance, now } = store();
    const noRefresh = s.createSession({ accessToken: "x", expiresInSeconds: 10, sessionSeconds: 7200, claims });
    advance(20_000);
    const f = vi.fn() as unknown as typeof fetch;
    expect(await accessTokenFor(s, noRefresh, { fetchFn: f, cfg: CFG, meta: META, now })).toBeNull();
    expect(s.get(noRefresh.id)).toBeNull();

    const failing = s.createSession({ accessToken: "x", refreshToken: "r", expiresInSeconds: 10, sessionSeconds: 7200, claims });
    advance(20_000);
    const bad = vi.fn(async () => new Response("no", { status: 400 })) as unknown as typeof fetch;
    expect(await accessTokenFor(s, failing, { fetchFn: bad, cfg: CFG, meta: META, now })).toBeNull();
    expect(s.get(failing.id)).toBeNull();
  });
});

describe("the idle timeout (RG-2)", () => {
  const claims = { iss: "i", sub: "s", aud: "a", exp: 9e9, name: "Ada" } as never;
  const make = (idleSeconds: number) => {
    let now = 1_000_000;
    const store = new OidcStore(() => now, 100, idleSeconds);
    const s = store.createSession({ accessToken: "at", sessionSeconds: 8 * 3600, claims });
    return { store, s, advance: (seconds: number) => (now += seconds * 1000) };
  };

  it("ends a session that sat idle, and says it was the idle timeout once", () => {
    const { store, s, advance } = make(1800);
    advance(1799);
    expect(store.get(s.id)).not.toBeNull();
    advance(1800);
    expect(store.get(s.id)).toBeNull();
    expect(store.endedIdle(s.id)).toBe(true);
    expect(store.endedIdle(s.id)).toBe(false);
  });

  it("is sliding: each use pushes the idle end back", () => {
    const { store, s, advance } = make(1800);
    for (let i = 0; i < 10; i++) {
      advance(1000);
      expect(store.get(s.id)).not.toBeNull();
    }
  });

  it("does not outlast the hard limit however busy the person is", () => {
    const { store, s, advance } = make(1800);
    for (let i = 0; i < 40; i++) {
      advance(1000);
      store.get(s.id);
    }
    advance(8 * 3600);
    expect(store.get(s.id)).toBeNull();
    expect(store.endedIdle(s.id)).toBe(false);
  });

  it("is off when set to zero", () => {
    const { store, s, advance } = make(0);
    advance(7 * 3600);
    expect(store.get(s.id)).not.toBeNull();
  });
});
