// CQ-18: following a link from one plan to another reuses the plan page, and the page must then
// show the second plan, not keep the first one under the new address.

import { test, expect } from "@playwright/test";
import { PID } from "./fixtures";

const OTHER = "22222222-2222-4222-8222-222222222222";
const plan = (pid: string, name: string) => ({ pid, kind: "Project", name, in_language: ["en"] });

test("a duplicate link opens the other plan on the same page", async ({ page }) => {
  await page.route("**/api/**", (route) => {
    const path = new URL(route.request().url()).pathname.replace(/^\/api\/proxy/, "");
    if (path === `/api/plans/${PID}`) return route.fulfill({ json: plan(PID, "Website replatform") });
    if (path === `/api/plans/${OTHER}`) return route.fulfill({ json: plan(OTHER, "Website rebuild") });
    if (path.endsWith("/check-duplicates"))
      return route.fulfill({
        json: [{ pid: OTHER, name: "Website rebuild", score: 0.91, confidence: "Probable", is_match: true }],
      });
    return route.fulfill({ status: 404, json: { error: "not_found" } });
  });
  await page.goto(`/en-001/plans/${PID}`, { waitUntil: "networkidle" });
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("Website replatform");
  await page.getByRole("button", { name: "Check duplicates" }).click();
  await page.getByRole("link", { name: "Website rebuild" }).click();
  await expect(page).toHaveURL(new RegExp(`/plans/${OTHER}$`));
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("Website rebuild");
  // The first plan's duplicate list does not follow it.
  await expect(page.getByRole("heading", { name: "Potential duplicates" })).toHaveCount(0);
});

// CQ-18, every `[pid]` page: SvelteKit reuses a page component when only the id in the address
// changes, so a page that loads once on mount would keep the first plan's data under the second
// plan's address. The WBS stands for the 22 pages that load in `onMount`.
test("a plan sub-page shows the second plan's data after a client-side move between plans", async ({ page }) => {
  const task = (pid: string, title: string) => ({
    pid, title, description: null, status: "todo", assignee_ref: null, points: null, due_date: null, sprint_pid: null,
    created_at: "2026-10-01T00:00:00Z", status_changed_at: "2026-10-01T00:00:00Z", done_at: null, blocked_days: null, parent_task_pid: null,
  });
  await page.route("**/api/**", (route) => {
    const path = new URL(route.request().url()).pathname.replace(/^\/api\/proxy/, "");
    if (path === `/api/plans/${PID}/tasks`) return route.fulfill({ json: { tasks: [task("a1", "Alpha task")], counts: {} } });
    if (path === `/api/plans/${OTHER}/tasks`) return route.fulfill({ json: { tasks: [task("b1", "Beta task")], counts: {} } });
    if (path.endsWith("/task-link-counts")) return route.fulfill({ json: { counts: {} } });
    return route.fulfill({ status: 404, json: { error: "not_found" } });
  });
  await page.goto(`/en-001/plans/${PID}/wbs`, { waitUntil: "networkidle" });
  await expect(page.getByText("Alpha task").first()).toBeVisible();
  // A link the router handles, as one in a page would be.
  await page.evaluate((href) => {
    const a = document.createElement("a");
    a.href = href;
    a.id = "to-other";
    a.textContent = "other plan";
    document.querySelector("main")?.append(a);
  }, `/en-001/plans/${OTHER}/wbs`);
  await page.locator("#to-other").click();
  await expect(page).toHaveURL(new RegExp(`/plans/${OTHER}/wbs$`));
  await expect(page.getByText("Beta task").first()).toBeVisible();
  await expect(page.getByText("Alpha task")).toHaveCount(0);
});
