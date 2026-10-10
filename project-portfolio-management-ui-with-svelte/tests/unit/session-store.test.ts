// RG-12: the shared session store. The RESP client is tested against a fake server that speaks the
// subset it needs; the Redis store against that fake; and, when REDIS_TEST_URL is set (CI sets it),
// the same checks against a real Redis.

import net from "node:net";
import { describe, expect, it } from "vitest";
import { RedisClient, RespParser, encodeCommand } from "../../src/lib/server/resp";
import { MemorySessionStore, RedisSessionStore, type SessionStore } from "../../src/lib/server/session-store";
import { OidcStore, accessTokenFor } from "../../src/lib/server/oidc-session";

const claims = { iss: "i", sub: "s", aud: "a", exp: 9e9, name: "Ada", roles: ["Portfolio.Editor"] } as never;

describe("RESP", () => {
  it("encodes a command as bulk strings, counting bytes not characters", () => {
    expect(encodeCommand(["SET", "k", "é"]).toString()).toBe("*3\r\n$3\r\nSET\r\n$1\r\nk\r\n$2\r\né\r\n");
  });

  it("parses every reply type, across arbitrary chunk boundaries", () => {
    const wire = Buffer.from("+OK\r\n:42\r\n$5\r\nhello\r\n$-1\r\n*2\r\n$1\r\na\r\n:7\r\n-ERR boom\r\n");
    for (const cut of [1, 3, 7, 11, wire.length - 2]) {
      const p = new RespParser();
      const out = [...p.feed(wire.subarray(0, cut)), ...p.feed(wire.subarray(cut))];
      expect(out.slice(0, 5)).toEqual(["OK", 42, "hello", null, ["a", 7]]);
      expect(out[5]).toBeInstanceOf(Error);
      expect((out[5] as Error).message).toBe("ERR boom");
    }
  });

  it("refuses an unknown reply type instead of guessing", () => {
    expect(() => new RespParser().feed(Buffer.from("?x\r\n"))).toThrow();
  });

  it("refuses absurd lengths instead of waiting for them", () => {
    expect(() => new RespParser().feed(Buffer.from("$99999999999\r\n"))).toThrow();
    expect(() => new RespParser().feed(Buffer.from("*99999999999\r\n"))).toThrow();
    expect(() => new RespParser().feed(Buffer.from("$abc\r\n"))).toThrow();
  });

  it("never loops or crashes on random bytes: it answers or throws", () => {
    let x = 123456789;
    const rnd = () => ((x = (x * 1103515245 + 12345) & 0x7fffffff) >>> 0);
    const starts = ["+", "-", ":", "$", "*", "?", ""];
    for (let i = 0; i < 3000; i++) {
      const len = rnd() % 60;
      const body = Buffer.from(Array.from({ length: len }, () => rnd() % 256));
      const wire = Buffer.concat([Buffer.from(starts[rnd() % starts.length]!), body]);
      const p = new RespParser();
      try {
        p.feed(wire);
        p.feed(Buffer.from("\r\n"));
      } catch {
        // refusing is fine
      }
    }
  });

  it("refuses a URL that is not redis:// or rediss://", () => {
    expect(() => new RedisClient({ url: "http://x" })).toThrow();
  });
});

/** A fake Redis: GET, SET (EX/XX/KEEPTTL), GETDEL, DEL, AUTH, SELECT, with a clock the test moves. */
async function fakeRedis(requirePassword?: string) {
  const data = new Map<string, { v: string; exp: number | null }>();
  const clock = { now: 1_000_000 };
  const log: string[][] = [];
  const server = net.createServer((sock) => {
    let authed = !requirePassword;
    let buf = Buffer.alloc(0);
    sock.on("data", (chunk) => {
      buf = Buffer.concat([buf, chunk]);
      for (;;) {
        const text = buf.toString("utf8");
        const m = /^\*(\d+)\r\n/.exec(text);
        if (!m) return;
        const argc = Number(m[1]);
        const args: string[] = [];
        let pos = m[0].length;
        let ok = true;
        for (let i = 0; i < argc; i++) {
          const h = /^\$(\d+)\r\n/.exec(text.slice(pos));
          if (!h) { ok = false; break; }
          const len = Number(h[1]);
          const start = pos + h[0].length;
          if (Buffer.byteLength(text.slice(start)) < len + 2) { ok = false; break; }
          const val = Buffer.from(text.slice(start)).subarray(0, len).toString();
          args.push(val);
          pos = start + val.length + 2;
        }
        if (!ok) return;
        buf = Buffer.from(text.slice(pos));
        log.push(args);
        sock.write(reply(args));
      }
    });
    const live = (k: string) => {
      const e = data.get(k);
      if (!e) return undefined;
      if (e.exp !== null && e.exp <= clock.now) { data.delete(k); return undefined; }
      return e;
    };
    function reply(a: string[]): string {
      const cmd = a[0]!.toUpperCase();
      if (cmd === "AUTH") { authed = a[a.length - 1] === requirePassword; return authed ? "+OK\r\n" : "-WRONGPASS invalid\r\n"; }
      if (!authed) return "-NOAUTH Authentication required.\r\n";
      if (cmd === "SELECT") return "+OK\r\n";
      if (cmd === "GET") { const e = live(a[1]!); return e ? `$${Buffer.byteLength(e.v)}\r\n${e.v}\r\n` : "$-1\r\n"; }
      if (cmd === "GETDEL") { const e = live(a[1]!); data.delete(a[1]!); return e ? `$${Buffer.byteLength(e.v)}\r\n${e.v}\r\n` : "$-1\r\n"; }
      if (cmd === "DEL") { const had = live(a[1]!) ? 1 : 0; data.delete(a[1]!); return `:${had}\r\n`; }
      if (cmd === "INCR") { const e = live(a[1]!); const n = Number(e?.v ?? 0) + 1; data.set(a[1]!, { v: String(n), exp: e?.exp ?? null }); return `:${n}\r\n`; }
      if (cmd === "EXPIRE") { const e = live(a[1]!); if (e) e.exp = clock.now + Number(a[2]) * 1000; return `:${e ? 1 : 0}\r\n`; }
      if (cmd === "SET") {
        const opts = a.slice(3).map((x) => x.toUpperCase());
        const existing = live(a[1]!);
        if (opts.includes("XX") && !existing) return "$-1\r\n";
        let exp: number | null = null;
        const ex = opts.indexOf("EX");
        if (ex >= 0) exp = clock.now + Number(a[3 + ex + 1]) * 1000;
        else if (opts.includes("KEEPTTL") && existing) exp = existing.exp;
        data.set(a[1]!, { v: a[2]!, exp });
        return "+OK\r\n";
      }
      return `-ERR unknown command ${cmd}\r\n`;
    }
  });
  await new Promise<void>((r) => server.listen(0, "127.0.0.1", r));
  const port = (server.address() as net.AddressInfo).port;
  return { port, clock, data, log, close: () => server.close() };
}

describe("RedisClient", () => {
  it("authenticates and runs commands in order, returning each reply to its caller", async () => {
    const fake = await fakeRedis("s3cret");
    const c = new RedisClient({ url: `redis://:s3cret@127.0.0.1:${fake.port}/2` });
    const results = await Promise.all([c.command("SET", "a", "1"), c.command("GET", "a"), c.command("GET", "missing")]);
    expect(results).toEqual(["OK", "1", null]);
    expect(fake.log[0]).toEqual(["AUTH", "s3cret"]);
    expect(fake.log[1]).toEqual(["SELECT", "2"]);
    c.close();
    fake.close();
  });

  it("rejects on a wrong password and reconnects for the next command after a failure", async () => {
    const fake = await fakeRedis("right");
    const bad = new RedisClient({ url: `redis://:wrong@127.0.0.1:${fake.port}` });
    await expect(bad.command("GET", "a")).rejects.toThrow(/WRONGPASS/);
    bad.close();
    const good = new RedisClient({ url: `redis://:right@127.0.0.1:${fake.port}` });
    expect(await good.command("SET", "a", "1")).toBe("OK");
    good.close();
    fake.close();
  });

  it("rejects when nothing is listening", async () => {
    const l = net.createServer();
    await new Promise<void>((r) => l.listen(0, "127.0.0.1", r));
    const port = (l.address() as net.AddressInfo).port;
    await new Promise((r) => l.close(r));
    await expect(new RedisClient({ url: `redis://127.0.0.1:${port}`, timeoutMs: 500 }).command("GET", "a")).rejects.toThrow();
  });
});

/** The checks every store must pass, run against the memory store and the Redis store. */
function behaves(name: string, make: () => Promise<{ store: SessionStore; advance: (s: number) => void; done?: () => void }>) {
  describe(name, () => {
    it("creates, finds, refreshes and deletes a session", async () => {
      const { store, done } = await make();
      const s = await store.createSession({ accessToken: "at", refreshToken: "rt", sessionSeconds: 3600, claims });
      expect(s.id.length).toBeGreaterThan(30);
      expect((await store.get(s.id))?.user).toMatchObject({ sub: "s", name: "Ada", roles: ["Portfolio.Editor"] });
      s.accessToken = "new";
      await store.save(s);
      expect((await store.get(s.id))?.accessToken).toBe("new");
      await store.delete(s.id);
      expect(await store.get(s.id)).toBeNull();
      expect(await store.get("not-a-real-id-not-a-real-id")).toBeNull();
      expect(await store.get(null)).toBeNull();
      done?.();
    });

    it("spends a login transaction exactly once", async () => {
      const { store, done } = await make();
      // A real state is a 43-character random token (see `randomToken`).
      const state = "Q".repeat(43);
      await store.startTransaction(state, { nonce: "n", verifier: "v", returnTo: "/", redirectUri: "r", locale: "cy-001" });
      const [a, b] = await Promise.all([store.takeTransaction(state), store.takeTransaction(state)]);
      expect([a, b].filter(Boolean)).toHaveLength(1);
      expect((a ?? b)?.locale).toBe("cy-001");
      expect(await store.takeTransaction("never")).toBeNull();
      done?.();
    });

    it("ends an idle session and says so once; a hard limit ends it regardless", async () => {
      const { store, advance, done } = await make();
      store.setIdleSeconds(1800);
      const s = await store.createSession({ accessToken: "at", sessionSeconds: 8 * 3600, claims });
      advance(1700);
      expect(await store.get(s.id)).not.toBeNull();
      advance(1900);
      expect(await store.get(s.id)).toBeNull();
      expect(await store.endedIdle(s.id)).toBe(true);
      expect(await store.endedIdle(s.id)).toBe(false);
      done?.();
    });
  });
}

behaves("MemorySessionStore", async () => {
  let now = 1_000_000;
  const store = new MemorySessionStore(new OidcStore(() => now));
  return { store, advance: (s) => (now += s * 1000) };
});

behaves("RedisSessionStore against a fake server", async () => {
  const fake = await fakeRedis();
  const client = new RedisClient({ url: `redis://127.0.0.1:${fake.port}` });
  const store = new RedisSessionStore(client, () => fake.clock.now);
  return { store, advance: (s) => (fake.clock.now += s * 1000), done: () => { client.close(); fake.close(); } };
});

describe("the Redis store, details", () => {
  it("keeps the same expiry when it marks a session used, and does not write on every request", async () => {
    const fake = await fakeRedis();
    const client = new RedisClient({ url: `redis://127.0.0.1:${fake.port}` });
    const store = new RedisSessionStore(client, () => fake.clock.now);
    const s = await store.createSession({ accessToken: "at", sessionSeconds: 3600, claims });
    const expiry = fake.data.get(`ppm:sess:${s.id}`)!.exp;
    const writes = () => fake.log.filter((a) => a[0] === "SET").length;
    const before = writes();
    await store.get(s.id);
    await store.get(s.id);
    expect(writes()).toBe(before);
    fake.clock.now += 15_000;
    await store.get(s.id);
    expect(writes()).toBe(before + 1);
    expect(fake.data.get(`ppm:sess:${s.id}`)!.exp).toBe(expiry);
    client.close();
    fake.close();
  });

  it("treats an unreachable Redis as signed out, not as a crash", async () => {
    const l = net.createServer();
    await new Promise<void>((r) => l.listen(0, "127.0.0.1", r));
    const port = (l.address() as net.AddressInfo).port;
    await new Promise((r) => l.close(r));
    const store = new RedisSessionStore(new RedisClient({ url: `redis://127.0.0.1:${port}`, timeoutMs: 500 }));
    expect(await store.get("a".repeat(43))).toBeNull();
    await expect(store.createSession({ accessToken: "at", sessionSeconds: 60, claims })).rejects.toThrow();
  });

  it("writes refreshed tokens back so another replica sees them", async () => {
    const fake = await fakeRedis();
    const client = new RedisClient({ url: `redis://127.0.0.1:${fake.port}` });
    const store = new RedisSessionStore(client, () => fake.clock.now);
    const s = await store.createSession({ accessToken: "old", refreshToken: "r1", expiresInSeconds: 10, sessionSeconds: 3600, claims });
    const fetchFn = (async () =>
      new Response(JSON.stringify({ access_token: "fresh", refresh_token: "r2", expires_in: 3600 }), {
        headers: { "content-type": "application/json" },
      })) as unknown as typeof fetch;
    const meta = { issuer: "i", authorization_endpoint: "a", token_endpoint: "http://idp/token", jwks_uri: "j" };
    const cfg = { issuer: "i", clientId: "c", scopes: "openid" };
    expect(await accessTokenFor(store, s, { fetchFn, cfg, meta, now: () => fake.clock.now })).toBe("fresh");
    const other = new RedisSessionStore(new RedisClient({ url: `redis://127.0.0.1:${fake.port}` }), () => fake.clock.now);
    const seen = await other.get(s.id);
    expect(seen?.accessToken).toBe("fresh");
    expect(seen?.refreshToken).toBe("r2");
    client.close();
    fake.close();
  });
});

describe("the Redis store refuses what it did not write", () => {
  it("rejects identifiers that are not random tokens before they reach a key", async () => {
    const fake = await fakeRedis();
    const client = new RedisClient({ url: `redis://127.0.0.1:${fake.port}` });
    const store = new RedisSessionStore(client, () => fake.clock.now);
    for (const bad of ["", "short", "a b".repeat(10), "x".repeat(200), "../../etc/passwd" + "a".repeat(20), "ppm:sess:" + "a".repeat(20), "a\r\nDEL x".padEnd(30, "a")]) {
      expect(await store.get(bad)).toBeNull();
      expect(await store.takeTransaction(bad)).toBeNull();
      expect(await store.endedIdle(bad)).toBe(false);
      await expect(store.startTransaction(bad, { nonce: "n", verifier: "v", returnTo: "/", redirectUri: "r", locale: "en-001" })).rejects.toThrow();
    }
    expect(fake.log.filter((a) => a[0] !== "INCR" && a[0] !== "EXPIRE")).toEqual([]);
    client.close();
    fake.close();
  });

  it("treats a corrupt or foreign value under a session key as no session, and removes it", async () => {
    const fake = await fakeRedis();
    const client = new RedisClient({ url: `redis://127.0.0.1:${fake.port}` });
    const store = new RedisSessionStore(client, () => fake.clock.now);
    const id = "A".repeat(43);
    for (const junk of ["not json", "[]", "null", '"a string"', '{"expiresAt":"soon"}', "{}"]) {
      fake.data.set(`ppm:sess:${id}`, { v: junk, exp: null });
      expect(await store.get(id), junk).toBeNull();
      expect(fake.data.has(`ppm:sess:${id}`), `${junk} removed`).toBe(false);
    }
    fake.data.set(`ppm:tx:${id}`, { v: "{broken", exp: null });
    expect(await store.takeTransaction(id)).toBeNull();
    client.close();
    fake.close();
  });

  it("will not let a visitor start unlimited sign-ins", async () => {
    const fake = await fakeRedis();
    const client = new RedisClient({ url: `redis://127.0.0.1:${fake.port}` });
    const store = new RedisSessionStore(client, () => fake.clock.now);
    const tx = { nonce: "n", verifier: "v", returnTo: "/", redirectUri: "r", locale: "en-001" };
    const { MAX_LOGINS_PER_MINUTE } = await import("../../src/lib/server/session-store");
    for (let i = 0; i < MAX_LOGINS_PER_MINUTE; i++) await store.startTransaction(`state-${String(i).padStart(30, "0")}`, tx);
    await expect(store.startTransaction("x".repeat(40), tx)).rejects.toThrow(/too many/);
    // The next minute starts clean.
    fake.clock.now += 61_000;
    await store.startTransaction("y".repeat(40), tx);
    client.close();
    fake.close();
  });

  it("does not resurrect a deleted session when a late refresh writes back", async () => {
    const fake = await fakeRedis();
    const client = new RedisClient({ url: `redis://127.0.0.1:${fake.port}` });
    const store = new RedisSessionStore(client, () => fake.clock.now);
    const s = await store.createSession({ accessToken: "at", sessionSeconds: 3600, claims });
    await store.delete(s.id);
    s.accessToken = "late";
    await store.save(s); // SET ... XX: a no-op when the key is gone
    expect(await store.get(s.id)).toBeNull();
    client.close();
    fake.close();
  });
});

const REAL = process.env.REDIS_TEST_URL;
describe.skipIf(!REAL)("against a real Redis (REDIS_TEST_URL)", () => {
  behaves("RedisSessionStore, real", async () => {
    const client = new RedisClient({ url: REAL! });
    let skew = 0;
    const store = new RedisSessionStore(client, () => Date.now() + skew);
    return { store, advance: (s) => (skew += s * 1000), done: () => client.close() };
  });
});
