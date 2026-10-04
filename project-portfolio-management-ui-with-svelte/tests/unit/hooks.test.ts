import { describe, it, expect } from "vitest";
import { handle, negotiateLocale } from "../../src/hooks.server";
import { reroute } from "../../src/hooks";

function jar(initial: Record<string, string> = {}) {
  const store = { ...initial };
  return {
    store,
    get: (k: string) => store[k],
    set: (k: string, v: string) => {
      store[k] = v;
    },
  };
}

type Outcome =
  | { status: number; location: string }
  | { html: string; locals: Record<string, unknown> };

async function run(
  path: string,
  opts: {
    method?: string;
    cookies?: Record<string, string>;
    lang?: string;
  } = {},
): Promise<Outcome> {
  const url = new URL(`http://localhost${path}`);
  const cookies = jar(opts.cookies);
  const locals: Record<string, unknown> = {};
  const event = {
    url,
    locals,
    cookies,
    request: new Request(url, {
      method: opts.method ?? "GET",
      headers: opts.lang ? { "accept-language": opts.lang } : {},
    }),
  };
  try {
    const res = await handle({
      event,
      resolve: async (
        _e: unknown,
        o: {
          transformPageChunk?: (c: { html: string; done: boolean }) => string;
        },
      ) => {
        const html = o.transformPageChunk?.({
          html: '<html lang="%lang%" dir="%dir%">',
          done: true,
        });
        return new Response(html);
      },
    } as never);
    return { html: await res.text(), locals };
  } catch (e) {
    const r = e as { status?: number; location?: string };
    if (r.status && r.location)
      return { status: r.status, location: r.location };
    throw e;
  }
}

describe("reroute", () => {
  const route = (p: string) =>
    reroute({ url: new URL(`http://x${p}`) } as never);
  it("strips a supported locale prefix", () => {
    expect(route("/cy-001/plans/1")).toBe("/plans/1");
    expect(route("/en-001")).toBe("/");
    expect(route("/en-001/")).toBe("/");
  });
  it("leaves everything else to the router", () => {
    expect(route("/plans")).toBeUndefined();
    expect(route("/en/plans")).toBeUndefined();
    expect(route("/api/x")).toBeUndefined();
  });
});

describe("negotiateLocale", () => {
  it("prefers the remembered locale, then Accept-Language, then English", () => {
    expect(negotiateLocale(jar({ locale: "cy-001" }), "de")).toBe("cy-001");
    expect(negotiateLocale(jar(), "fr-CA,fr;q=0.9")).toBe("fr-001");
    expect(negotiateLocale(jar({ locale: "junk" }), "es")).toBe("es-001");
    expect(negotiateLocale(jar(), null)).toBe("en-001");
  });
});

describe("handle", () => {
  it("redirects a two-letter alias to its locale, permanently, keeping the rest", async () => {
    expect(await run("/en/plans?x=1")).toEqual({
      status: 301,
      location: "/en-001/plans?x=1",
    });
    expect(await run("/cy")).toEqual({ status: 301, location: "/cy-001" });
    expect(await run("/cy/")).toEqual({ status: 301, location: "/cy-001" });
    expect(await run("/de/tour")).toEqual({
      status: 301,
      location: "/de-de/tour",
    });
  });

  it("redirects an alias for any method, so a form post is not served twice", async () => {
    expect(await run("/en/signin", { method: "POST" })).toMatchObject({
      status: 301,
    });
  });

  it("gives a bare URL a locale prefix from the cookie or Accept-Language", async () => {
    expect(await run("/plans", { cookies: { locale: "cy-001" } })).toEqual({
      status: 302,
      location: "/cy-001/plans",
    });
    expect(await run("/verify?token=abc", { lang: "fr" })).toEqual({
      status: 302,
      location: "/fr-001/verify?token=abc",
    });
    expect(await run("/")).toEqual({ status: 302, location: "/en-001" });
  });

  it("does not redirect a bare non-GET (a form post keeps working)", async () => {
    const out = await run("/signin", { method: "POST" });
    expect("status" in out).toBe(false);
  });

  it("never localizes the API, framework or static paths", async () => {
    for (const p of [
      "/api/proxy/x",
      "/_app/immutable/a.js",
      "/assets/a.css",
      "/robots.txt",
    ]) {
      const out = await run(p);
      expect("status" in out, p).toBe(false);
    }
  });

  it("serves a locale URL, remembers it, and sets lang and dir", async () => {
    const out = (await run("/ar-001/plans")) as {
      html: string;
      locals: Record<string, unknown>;
    };
    expect(out.locals.locale).toBe("ar-001");
    expect(out.html).toBe('<html lang="ar-001" dir="rtl">');
    const en = (await run("/en-001/plans")) as { html: string };
    expect(en.html).toBe('<html lang="en-001" dir="ltr">');
  });
});
