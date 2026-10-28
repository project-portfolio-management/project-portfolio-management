// HD-12: signing in through an Entra-compatible OpenID Connect provider, end to end.
//
// A real browser, the real production build of the app, and a mock provider that follows
// Microsoft Entra ID's v2.0 shapes (see support/entra-mock.mjs): discovery, an exact redirect
// URI, PKCE, RS256 ID tokens with `tid`/`oid`/`roles`/`nonce`, refresh tokens. The mock API on
// the other side only answers a bearer token the provider issued, so a pass proves the app
// sent the right one. Nothing here is stubbed in the browser: the sign-in is server to server.

import { test, expect } from "@playwright/test";
import { settle } from "./fixtures";

const IDP = "http://localhost:4180";
// The message formatter wraps a placeholder value in invisible bidi-isolation marks, so match around them.
const ENTRA = /^Sign in with .*Microsoft Entra ID/;
const stats = async (request: import("@playwright/test").APIRequestContext) =>
  (await request.get(`${IDP}/__stats`)).json() as Promise<{
    codesIssued: number;
    refreshes: number;
    apiCalls: number;
    pkceFailures: number;
  }>;

test.describe("sign in with an Entra-compatible OpenID Connect provider", () => {
  test("the sign-in page offers it, and the whole round trip signs the person in", async ({
    page,
    request,
  }) => {
    await page.goto("/en-001/signin");
    const button = page.getByRole("link", { name: ENTRA });
    await expect(button).toBeVisible();

    // Not signed in: the API proxy has no token to send.
    expect((await page.request.get("/api/proxy/api/plans")).status()).toBe(401);

    await button.click();
    // Now on the provider's page (another origin).
    await expect(page).toHaveURL(new RegExp(`^${IDP}/`));
    await expect(page.getByRole("heading", { name: "Sign in" })).toBeVisible();
    await page.locator("#as-alice").click();

    // Back in the app, signed in, in the reader's language.
    await expect(page).toHaveURL(/\/en-001$/);
    await expect(page.getByRole("button", { name: "Sign out" })).toBeVisible();

    // The tokens are on the server: the browser holds one opaque, httpOnly cookie, no token.
    const cookies = await page.context().cookies();
    const session = cookies.find((c) => c.name === "__Host-mxi_oidc");
    expect(session?.httpOnly).toBe(true);
    expect(session?.secure).toBe(true);
    expect(session?.value).not.toMatch(/^eyJ|at_/);
    const stored = await page.evaluate(() =>
      JSON.stringify({ ...localStorage, ...sessionStorage }),
    );
    expect(stored).not.toMatch(/at_|eyJ/);

    // The proxy now sends the provider's access token, and the API accepts it.
    const api = await page.request.get("/api/proxy/api/plans");
    expect(api.status()).toBe(200);
    expect(await api.json()).toMatchObject({
      ok: true,
      sub: "alice-sub",
      roles: ["Portfolio.Editor"],
    });
    expect((await stats(request)).pkceFailures).toBe(0);
  });

  test("a token that is about to expire is refreshed rather than the person signed out", async ({
    page,
    request,
  }) => {
    const before = (await stats(request)).refreshes;
    await page.goto("/en-001/signin");
    await page.getByRole("link", { name: ENTRA }).click();
    await page.locator("#as-short").click();
    await expect(page.getByRole("button", { name: "Sign out" })).toBeVisible();
    // The access token lasted one second; the app refreshes it before using it.
    const api = await page.request.get("/api/proxy/api/plans");
    expect(api.status()).toBe(200);
    expect(await api.json()).toMatchObject({ sub: "short-sub" });
    expect((await stats(request)).refreshes).toBeGreaterThan(before);
  });

  test("cancelling at the provider returns to the sign-in page with a sentence, signed out", async ({
    page,
  }) => {
    await page.goto("/en-001/signin");
    await page.getByRole("link", { name: ENTRA }).click();
    await page.locator("#deny").click();
    await expect(page).toHaveURL(/\/en-001\/signin\?error=oidc_denied$/);
    await expect(page.getByTestId("oidc-error")).toHaveText(
      "Sign-in was cancelled or is not permitted by your organization.",
    );
    expect((await page.request.get("/api/proxy/api/plans")).status()).toBe(401);
  });

  test("a person from another tenant is refused", async ({ page }) => {
    await page.goto("/en-001/signin");
    await page.getByRole("link", { name: ENTRA }).click();
    await page.locator("#as-other-tenant").click();
    await expect(page).toHaveURL(/\/en-001\/signin\?error=oidc$/);
    await expect(page.getByTestId("oidc-error")).toBeVisible();
    expect((await page.request.get("/api/proxy/api/plans")).status()).toBe(401);
  });

  test("a callback with a made-up state or a replayed code is refused", async ({
    page,
    request,
  }) => {
    // A forged callback: no login was started in this browser.
    await page.goto("/signin/oidc/callback?code=anything&state=forged");
    await expect(page).toHaveURL(/\/signin\?error=oidc$/);
    expect((await page.request.get("/api/proxy/api/plans")).status()).toBe(401);

    // Replay: take a real callback URL and use it a second time.
    await page.goto("/en-001/signin");
    await page.getByRole("link", { name: ENTRA }).click();
    let callback = "";
    page.on("request", (r) => {
      if (r.url().includes("/signin/oidc/callback")) callback = r.url();
    });
    await page.locator("#as-alice").click();
    await expect(page.getByRole("button", { name: "Sign out" })).toBeVisible();
    expect(callback).toContain("code=");
    await page.context().clearCookies();
    await page.goto(callback);
    await expect(page).toHaveURL(/\/signin\?error=oidc$/);
    expect((await stats(request)).codesIssued).toBeGreaterThan(0);
  });

  test("signing out ends the session here and at the provider", async ({
    page,
  }) => {
    await page.goto("/en-001/signin");
    await page.getByRole("link", { name: ENTRA }).click();
    await page.locator("#as-alice").click();
    await page.getByRole("button", { name: "Sign out" }).click();
    // OIDC_END_SESSION is on for this suite, so the provider's sign-out page follows.
    await expect(page).toHaveURL(new RegExp(`^${IDP}/.*/logout`));
    expect((await page.request.get("/api/proxy/api/plans")).status()).toBe(401);
  });

  test("the sign-in page still passes the accessibility audit with the button and an error", async ({
    page,
  }) => {
    const { default: AxeBuilder } = await import("@axe-core/playwright");
    await page.goto("/en-001/signin?error=oidc", { waitUntil: "networkidle" });
    await expect(page.locator("h1").first()).toBeVisible();
    await settle(page);
    const r = await new AxeBuilder({ page })
      .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
      .analyze();
    expect(
      r.violations
        .filter((v) => v.impact === "serious" || v.impact === "critical")
        .map((v) => v.id),
    ).toEqual([]);
  });
});
