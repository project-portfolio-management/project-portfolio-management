// HD-4: security headers and a strict Content Security Policy. Asserts the
// headers on a page response, and that pages load and run under the CSP with
// no violations (console message or `securitypolicyviolation` event).

import { test, expect, type Page } from "@playwright/test";
import { ROUTES, stubPlansApi, PID } from "./fixtures";

/** Collects every CSP violation, from the console and from the DOM event. */
async function watchViolations(page: Page): Promise<string[]> {
  const found: string[] = [];
  await page.addInitScript(() => {
    document.addEventListener("securitypolicyviolation", (e) => {
      (window as unknown as { __csp: string[] }).__csp ??= [];
      (window as unknown as { __csp: string[] }).__csp.push(
        `${e.violatedDirective} blocked ${e.blockedURI} (${e.sample})`,
      );
    });
  });
  page.on("console", (m) => {
    const t = m.text();
    if (/content security policy|violates the following/i.test(t))
      found.push(t);
  });
  page.on("pageerror", (e) => {
    if (/content security policy/i.test(e.message)) found.push(e.message);
  });
  return found;
}

async function domViolations(page: Page): Promise<string[]> {
  return page.evaluate(
    () => (window as unknown as { __csp?: string[] }).__csp ?? [],
  );
}

test.describe("security headers", () => {
  test.beforeEach(async ({ page }) => {
    await stubPlansApi(page);
  });

  test("a page response carries the full header set and a strict CSP", async ({
    request,
  }) => {
    const res = await request.get("/en-001/plans");
    expect(res.status()).toBe(200);
    const h = res.headers();
    expect(h["x-content-type-options"]).toBe("nosniff");
    expect(h["x-frame-options"]).toBe("DENY");
    expect(h["referrer-policy"]).toBe("strict-origin-when-cross-origin");
    expect(h["cross-origin-opener-policy"]).toBe("same-origin");
    expect(h["cross-origin-resource-policy"]).toBe("same-origin");
    for (const f of ["camera", "microphone", "geolocation", "payment", "usb"]) {
      expect(h["permissions-policy"]).toContain(`${f}=()`);
    }
    // Plain HTTP: no HSTS.
    expect(h["strict-transport-security"]).toBeUndefined();

    const csp = h["content-security-policy"];
    expect(csp).toBeTruthy();
    for (const d of [
      "default-src 'self'",
      "img-src 'self' data:",
      "font-src 'self'",
      "connect-src 'self'",
      "object-src 'none'",
      "base-uri 'self'",
      "form-action 'self'",
      "frame-ancestors 'none'",
    ]) {
      expect(csp).toContain(d);
    }
    // Style attributes: one hash (SvelteKit's announcer), never a blanket 'unsafe-inline'.
    const attr = csp
      .split(";")
      .find((s) => s.trim().startsWith("style-src-attr"))!;
    expect(attr).toContain("'unsafe-hashes'");
    expect(attr).toMatch(/'sha256-[A-Za-z0-9+/=]+'/);
    expect(attr).not.toContain("'unsafe-inline'");
    const script = csp
      .split(";")
      .find((s) => s.trim().startsWith("script-src"))!;
    expect(script).toContain("'self'");
    expect(script).not.toContain("'unsafe-inline'");
    expect(script).not.toContain("'unsafe-eval'");
  });

  test("HSTS is sent when a proxy says the request was HTTPS", async ({
    request,
  }) => {
    const res = await request.get("/en-001/plans", {
      headers: { "x-forwarded-proto": "https" },
    });
    expect(res.headers()["strict-transport-security"]).toBe(
      "max-age=31536000; includeSubDomains",
    );
  });

  const sample = [
    "",
    "/plans",
    "/plans/new",
    `/plans/${PID}`,
    "/cost-risk",
    "/gantt",
    "/calendar",
    "/board",
    "/okrs",
  ];
  const locales = ["en-001", "cy-001", "ar-001"];

  for (const loc of locales) {
    for (const route of sample) {
      test(`no CSP violations: /${loc}${route}`, async ({ page }) => {
        const found = await watchViolations(page);
        await page.goto(`/${loc}${route}`, { waitUntil: "networkidle" });
        await expect(page.locator("h1").first()).toBeVisible();
        if (loc === "ar-001") {
          await expect(page.locator("html")).toHaveAttribute("dir", "rtl");
        }
        expect([...found, ...(await domViolations(page))]).toEqual([]);
      });
    }
  }

  test("no CSP violations on any route", async ({ page }) => {
    test.setTimeout(240_000);
    const found = await watchViolations(page);
    for (const route of ROUTES) {
      await page.goto(`/en-001${route === "/" ? "" : route}`, {
        waitUntil: "networkidle",
      });
      found.push(...(await domViolations(page)).map((v) => `${route}: ${v}`));
    }
    expect(found).toEqual([]);
  });
});
