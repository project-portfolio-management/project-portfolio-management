import { test, expect } from "@playwright/test";
import { ROUTES, stubPlansApi } from "./fixtures";

// T-28k (repo tasks.md EV-1): a responsive audit at a phone viewport
// (390x844, set via the "mobile" Playwright project in
// playwright.config.ts) across every real route in this app. Two
// invariants, checked per route so a failure names the route rather
// than producing a screenshot to eyeball:
//
//   1. No horizontal scroll on the document — a route that needs the
//      visitor to scroll sideways to read it is the definition of
//      "not responsive."
//   2. The page's primary heading (<h1>) is visible without horizontal
//      scroll — a proxy for "the page's primary content is reachable"
//      that does not require hand-picking a different "primary action"
//      selector per route (34 routes, each with a different one).
//
// Every route here is enumerated from the real `src/routes` tree (`find
// src/routes -name "+page.svelte"`), not hand-maintained, so a route
// this suite has never heard of cannot silently go unaudited.
//
// Data-fetching routes are NOT stubbed with a live backend (this is a
// structural/responsive audit, not a data-correctness one): the BFF's
// own upstream fetch fails (no Rust service listens on
// PROJECT_PORTFOLIO_MANAGEMENT_API_URL / AUTH_API_URL during this
// suite), and every route's own try/catch renders its error state —
// which must itself be responsive. `/plans` and `/plans/{pid}*` are the
// exception: they are stubbed (mirroring `smoke.spec.ts`'s fixture) so
// this suite also audits their real, populated, SVAR-widget-bearing
// happy-path layout, not just an error banner.

test.describe("phone-viewport responsive audit (T-28k)", () => {
  test.beforeEach(async ({ page }) => {
    await stubPlansApi(page);
  });

  for (const route of ROUTES) {
    test(`${route} has no horizontal scroll and shows its heading`, async ({
      page,
    }) => {
      await page.goto(route, { waitUntil: "networkidle" });

      const overflow = await page.evaluate(() => {
        const doc = document.documentElement;
        // Name what sticks out, so a failure says where (a bare width does not).
        const wide = [...document.querySelectorAll("body *")]
          .filter((e) => e.getBoundingClientRect().right > doc.clientWidth + 1)
          .slice(0, 6)
          .map((e) => `${e.tagName.toLowerCase()}.${String((e as HTMLElement).className).slice(0, 40)} left=${Math.round(e.getBoundingClientRect().left)} right=${Math.round(e.getBoundingClientRect().right)} text=${JSON.stringify((e.textContent ?? "").slice(0, 40))}`);
        return { scrollWidth: doc.scrollWidth, clientWidth: doc.clientWidth, wide };
      });
      expect(
        overflow.scrollWidth,
        `${route}: scrollWidth ${overflow.scrollWidth} > clientWidth ${overflow.clientWidth} (horizontal scroll) — wide: ${overflow.wide.join("; ")}`,
      ).toBeLessThanOrEqual(overflow.clientWidth);

      // The primary heading is reachable — visible within the viewport
      // width without a horizontal scroll (vertical scroll is fine).
      const heading = page.locator("h1").first();
      await expect(heading, `${route}: no visible <h1>`).toBeVisible();
    });
  }
});
