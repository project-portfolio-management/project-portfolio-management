// RE-10: every e2e spec imports `test` and `expect` from here, not from "@playwright/test".
//
// The browser's only outside world is the app under test (the `baseURL` origin) and the mock
// identity provider the sign-in tests are redirected to. A request or WebSocket to any other
// origin is blocked on the browser context, so popups are covered too, and the test then fails
// with the blocked addresses. A route a test fakes with `page.route` takes precedence and is
// never blocked. Service workers bypass routing, so they are blocked.
//
// The server side of SvelteKit (load functions, the API proxy) is not a browser request and is
// not covered here: it reaches only PROJECT_PORTFOLIO_MANAGEMENT_API_URL, which playwright.config.ts
// points at the local mock.
import { test as base, expect } from "@playwright/test";

export * from "@playwright/test";

/** Origins the browser may reach besides the app itself: the mock identity provider. */
export const ALLOWED_ORIGINS: readonly string[] = ["http://localhost:4180"];

/** Whether the browser may reach `url` when the app is served from `appOrigin`. */
export function isAllowed(url: URL, appOrigin: string): boolean {
  if (
    url.protocol !== "http:" &&
    url.protocol !== "https:" &&
    url.protocol !== "ws:" &&
    url.protocol !== "wss:"
  ) {
    return true; // data:, blob:, about: never leave the browser
  }
  const origin = url.origin.replace(/^ws/, "http");
  return origin === appOrigin || ALLOWED_ORIGINS.includes(origin);
}

export const test = base.extend<{ blockedRequests: string[] }>({
  serviceWorkers: "block",
  // The app renders in the browser only (`ssr = false`), so a page's <title> exists once it has hydrated, and for a
  // locale whose catalogue loads on demand (zh-cn) that can be after `networkidle`. axe's `document-title` rule then
  // failed on a slow runner, in a different spec each time. After `goto` of an HTML page, wait (up to 5 s, never failing)
  // for a non-empty title, so every spec that audits a page starts from one that has finished its first render.
  page: async ({ page }, use) => {
    const goto = page.goto.bind(page);
    page.goto = async (url, options) => {
      const response = await goto(url, options);
      if (response?.headers()["content-type"]?.includes("text/html")) {
        await page
          .waitForFunction(() => document.title !== "", undefined, {
            timeout: 5_000,
          })
          .catch(() => undefined);
      }
      return response;
    };
    await use(page);
  },
  blockedRequests: [
    async ({ context, baseURL }, use) => {
      if (!baseURL) throw new Error("baseURL is not configured");
      const appOrigin = new URL(baseURL).origin;
      const blocked: string[] = [];
      const external = (url: URL) => !isAllowed(url, appOrigin);
      await context.route(external, (route) => {
        blocked.push(route.request().url());
        return route.abort("blockedbyclient");
      });
      await context.routeWebSocket(external, (socket) => {
        blocked.push(socket.url());
        return socket.close();
      });
      await use(blocked);
      expect(blocked, "requests to origins no test faked (RE-10)").toEqual([]);
    },
    { auto: true },
  ],
});

export { expect };
