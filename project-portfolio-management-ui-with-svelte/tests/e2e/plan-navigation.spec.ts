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
