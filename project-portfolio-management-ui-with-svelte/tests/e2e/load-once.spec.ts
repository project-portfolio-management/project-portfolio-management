// CQ-17: changing a filter loads once, not twice. These pages used to load both from an
// `$effect` that tracked the filter and from the filter's change handler.

import { test, expect, type Page } from "@playwright/test";

const PLAN = "33333333-3333-4333-8333-333333333333";

/** Fulfils every API call from `answer`, and counts the calls whose path starts with `counted`. */
async function stub(page: Page, counted: string, answer: (path: string) => unknown): Promise<() => number> {
  let n = 0;
  await page.route("**/api/**", (route) => {
    const path = new URL(route.request().url()).pathname.replace(/^\/api\/proxy/, "");
    if (path.startsWith(counted)) n += 1;
    const json = answer(path);
    return json === undefined ? route.fulfill({ status: 404, json: { error: "unstubbed" } }) : route.fulfill({ json });
  });
  return () => n;
}

test("the delivery calendar loads once per kind change", async ({ page }) => {
  const calls = await stub(page, "/api/engineering/milestone-calendar", (path) =>
    path.startsWith("/api/engineering/milestone-calendar")
      ? { as_of: "2026-07-19T00:00:00Z", kinds: ["milestone", "demo"], milestones: [] }
      : undefined,
  );
  await page.goto("/en-001/calendar", { waitUntil: "networkidle" });
  expect(calls()).toBe(1);
  await page.getByLabel("Kind").selectOption("demo");
  // The new load arrives, and no second one follows it.
  await expect.poll(calls).toBe(2);
  await page.waitForTimeout(500);
  expect(calls()).toBe(2);
});

test("the flow page loads once per window change", async ({ page }) => {
  const calls = await stub(page, `/api/plans/${PLAN}/cumulative-flow`, () => undefined);
  await page.goto(`/en-001/plans/${PLAN}/flow`, { waitUntil: "networkidle" });
  expect(calls()).toBe(1);
  await page.getByLabel("Window").selectOption("90");
  // The new load arrives, and no second one follows it.
  await expect.poll(calls).toBe(2);
  await page.waitForTimeout(500);
  expect(calls()).toBe(2);
});
