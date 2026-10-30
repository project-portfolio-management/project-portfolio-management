// LT-12: the display preferences in a real browser — simple view, the
// dyslexia-friendly font and reduced motion — and the worker's path to "what do
// I do today?". Signed out, so choices are kept on the device.

import { test, expect, type Page } from "@playwright/test";
import { stubPlansApi } from "./fixtures";

const EMPTY = { overdue: [], today: [], this_week: [], later: [], undated: [] };

async function stubMe(page: Page) {
  await stubPlansApi(page);
  await page.route("**/api/me/tasks**", (route) =>
    route.fulfill({
      json: {
        tz: "UTC",
        as_of: "2026-10-05T00:00:00Z",
        today: "2026-10-05",
        buckets: {
          ...EMPTY,
          today: [
            {
              pid: "t1",
              title: "Review the plan",
              status: "todo",
              due_date: "2026-10-05",
              points: null,
              plan_pid: "p1",
              plan_name: "Apollo",
              schedule: null,
            },
          ],
        },
        counts: { overdue: 0, today: 1, this_week: 0, later: 0, undated: 0 },
        total: 1,
        omitted_unreadable: 0,
        truncated: false,
        reason: null,
      },
    }),
  );
}

test("simple view moves the advanced destinations behind More, drops none, and persists", async ({
  page,
}) => {
  await stubPlansApi(page);
  await page.goto("/en-001/plans");
  const menu = page.getByRole("button", { name: /Toggle navigation/i });
  const nav = page.locator("#primary-nav");
  // Off by default for someone who is not the worker persona: everything is in front.
  await menu.click();
  await expect(
    nav.getByRole("link", { name: "Capacity", exact: true }),
  ).toBeVisible();
  await expect(nav.getByText("More", { exact: true })).toHaveCount(0);
  const toggle = page.getByRole("button", { name: /Simple view/ });
  await expect(toggle).toHaveAttribute("aria-pressed", "false");
  await toggle.click();
  await expect(toggle).toHaveAttribute("aria-pressed", "true");
  await expect(page.locator("html")).toHaveAttribute("data-simple", "on");
  // The advanced links are one step away, not gone.
  const more = nav.getByText("More", { exact: true });
  await expect(more).toBeVisible();
  await expect(
    nav.getByRole("link", { name: "Capacity", exact: true }),
  ).toBeHidden();
  await more.click();
  await expect(
    nav.getByRole("link", { name: "Capacity", exact: true }),
  ).toBeVisible();
  // Everything still works at its address.
  await page.goto("/en-001/capacity");
  await expect(page.locator("h1").first()).toBeVisible();
  // It stays chosen across a reload.
  await page.reload();
  await expect(page.locator("html")).toHaveAttribute("data-simple", "on");
  await expect(
    page.getByRole("button", { name: /Simple view/ }),
  ).toHaveAttribute("aria-pressed", "true");
});

test("the plan page keeps governance, board and schedule in front and the rest behind More tools", async ({
  page,
}) => {
  await stubPlansApi(page);
  await page.route("**/api/plans/*/modules", (r) =>
    r.fulfill({
      json: {
        modules: [
          "governance",
          "tasks",
          "milestones",
          "financials",
          "time",
          "risks",
        ],
      },
    }),
  );
  await page.goto("/en-001/plans");
  await page.getByRole("button", { name: /Simple view/ }).click();
  await page.goto("/en-001/plans/11111111-1111-4111-8111-111111111111");
  await expect(page.getByRole("heading", { level: 1 })).toBeVisible();
  const more = page.locator("details.more-tools");
  await expect(more).toHaveCount(1);
  await expect(page.getByRole("link", { name: "Critical path" })).toBeHidden();
  await more.locator("summary").click();
  await expect(page.getByRole("link", { name: "Critical path" })).toBeVisible();
});

test("the dyslexia-friendly font applies, persists, and can be turned off", async ({
  page,
}) => {
  await stubPlansApi(page);
  await page.goto("/en-001/display");
  const fontOf = () =>
    page.evaluate(() => getComputedStyle(document.body).fontFamily);
  expect(await fontOf()).not.toContain("OpenDyslexic");
  await page.getByLabel("Use the dyslexia-friendly font").check();
  await expect(page.locator("html")).toHaveAttribute("data-font", "dyslexia");
  expect(await fontOf()).toContain("OpenDyslexic");
  await expect(page.getByText("Saved on this device.")).toBeVisible();
  await page.reload();
  await expect(page.locator("html")).toHaveAttribute("data-font", "dyslexia");
  await page.getByLabel("Use the dyslexia-friendly font").uncheck();
  await expect(page.locator("html")).toHaveAttribute("data-font", "default");
});

test("a person can go back to the default for their role after choosing", async ({
  page,
}) => {
  await stubPlansApi(page);
  await page.goto("/en-001/display");
  await expect(
    page.getByText(/This is the default for your role: off/),
  ).toBeVisible();
  await page.getByLabel("Use simple view").check();
  await expect(page.getByText("You chose this.")).toBeVisible();
  await page
    .getByRole("button", { name: "Go back to the default for my role" })
    .click();
  await expect(
    page.getByText(/This is the default for your role: off/),
  ).toBeVisible();
  await expect(page.locator("html")).toHaveAttribute("data-simple", "off");
});

test("reduced motion switches every animation and transition off", async ({
  page,
}) => {
  await stubPlansApi(page);
  const probe = () =>
    page.evaluate(() => {
      const el = document.createElement("div");
      el.style.transition = "all 5s";
      el.style.animation = "x 5s infinite";
      document.body.append(el);
      const cs = getComputedStyle(el);
      const out = {
        transition: cs.transitionDuration,
        animation: cs.animationDuration,
        scroll: getComputedStyle(document.documentElement).scrollBehavior,
      };
      el.remove();
      return out;
    });
  await page.emulateMedia({ reducedMotion: "no-preference" });
  await page.goto("/en-001/tour");
  expect((await probe()).transition).toBe("5s");
  await page.emulateMedia({ reducedMotion: "reduce" });
  const reduced = await probe();
  expect(reduced.transition).toBe("1e-06s");
  expect(reduced.animation).toBe("1e-06s");
  expect(reduced.scroll).toBe("auto");
  // The splash and the tour are covered by the same rule.
  await page.goto("/en-001");
  await expect(page.locator("h1").first()).toBeVisible();
  expect((await probe()).transition).toBe("1e-06s");
});

test("a worker sees today's tasks on the first screen with no clicks", async ({
  page,
}) => {
  await stubMe(page);
  await page.goto("/en-001/me");
  await expect(
    page.getByRole("heading", { name: "My work", level: 1 }),
  ).toBeVisible();
  await expect(
    page.getByRole("link", { name: "Review the plan" }),
  ).toBeVisible();
  await expect(page.getByRole("heading", { name: /Today/ })).toBeVisible();
  // And in simple view My work is the first destination in the nav after Home.
  await page.getByRole("button", { name: /Simple view/ }).click();
  await page.getByRole("button", { name: /Toggle navigation/i }).click();
  const links = page.locator("#primary-nav > ul > li:not(.more) a");
  await expect(links.nth(1)).toHaveText("My work");
});
