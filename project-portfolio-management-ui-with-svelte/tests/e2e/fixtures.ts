// Shared by the phone-viewport audit and the accessibility audit.
import { readdirSync, statSync } from "node:fs";
import { join } from "node:path";
import type { Page } from "@playwright/test";

export const PID = "11111111-1111-4111-8111-111111111111";

const WORK_ITEM = {
  kind: "Project",
  name: "Website replatform",
  alternate_names: [],
  code: "WEB-42",
  status: "Active",
  goals: [],
  keywords: ["platform"],
  tags: [],
  identifiers: [],
  relationships: [],
  same_as: [],
  in_language: ["en"],
};

/** Stub `/api/plans*` (mirrors `smoke.spec.ts`) so `/plans` and
 *  `/plans/{pid}*` render their real, populated layout rather than an
 *  error banner — the only routes whose SVAR grid/Gantt this suite
 *  needs to see in its non-empty, real state. */
export async function stubPlansApi(page: Page) {
  await page.route("**/api/plans**", async (route) => {
    const req = route.request();
    const url = new URL(req.url());
    const method = req.method();
    const path = url.pathname.replace(/^\/api\/proxy/, "");

    if (path === "/api/plans" && method === "GET") {
      return route.fulfill({ json: [{ pid: PID, name: WORK_ITEM.name }] });
    }
    if (path.endsWith("/check-duplicates")) {
      return route.fulfill({ json: [] });
    }
    if (path === `/api/plans/${PID}` && method === "GET") {
      return route.fulfill({ json: WORK_ITEM });
    }
    if (path === `/api/plans/${PID}/schedule` && method === "GET") {
      return route.fulfill({ json: { items: [], edges: [] } });
    }
    if (path === "/api/me/tasks" && method === "GET") {
      const empty = {
        overdue: [],
        today: [],
        this_week: [],
        later: [],
        undated: [],
      };
      return route.fulfill({
        json: {
          tz: "UTC",
          as_of: "2026-10-02T00:00:00Z",
          today: "2026-10-02",
          buckets: empty,
          counts: { overdue: 0, today: 0, this_week: 0, later: 0, undated: 0 },
          total: 0,
          omitted_unreadable: 0,
          truncated: false,
          reason: null,
        },
      });
    }
    return route.fulfill({ status: 404, json: { error: "unhandled in stub" } });
  });
}

function pages(dir: string, out: string[] = []): string[] {
  for (const name of readdirSync(dir)) {
    const full = join(dir, name);
    if (statSync(full).isDirectory()) pages(full, out);
    else if (name === "+page.svelte") out.push(full);
  }
  return out;
}

/**
 * Every real page route, **enumerated from `src/routes` when the suite runs**, so
 * a route nobody has heard of cannot go unaudited. `[pid]` is filled with the
 * fixture `PID`; `[locale]`-style prefixes do not exist (the locale is a URL
 * prefix handled by `reroute`, not a route directory).
 */
export const ROUTES: string[] = pages("src/routes")
  .map(
    (f) => f.slice("src/routes".length).replace(/\/\+page\.svelte$/, "") || "/",
  )
  .map((r) => r.replace(/\[pid\]/g, PID))
  .sort();

/** The visible primary buttons on screen, by name (LT-12: there is at most one). */
export async function primaryButtons(page: Page): Promise<string[]> {
  return page
    .locator("button.primary:visible, a.primary:visible")
    .evaluateAll((els) => els.map((e) => (e.textContent ?? "").trim()));
}

/**
 * Stop CSS transitions and animations so an accessibility audit reads the final colours: a
 * button that has just changed from primary to secondary is otherwise measured halfway
 * through its fade, which axe reports as a (false) colour-contrast failure about 1 run in 20.
 */
export async function settle(page: Page): Promise<void> {
  // A constructed stylesheet, not an injected <style> tag: the app's CSP (HD-4) forbids
  // inline style elements, and adopted stylesheets are exempt from it.
  await page.evaluate(() => {
    const sheet = new CSSStyleSheet();
    sheet.replaceSync(
      "*, *::before, *::after { transition: none !important; animation: none !important; }",
    );
    document.adoptedStyleSheets = [...document.adoptedStyleSheets, sheet];
  });
}
