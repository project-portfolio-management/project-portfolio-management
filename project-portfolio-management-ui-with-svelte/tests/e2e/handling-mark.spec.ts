// RG-7: a plan page says its handling mark when the deployment has a scheme, in words, in the reader's
// language, and says nothing when there is none.

import { test, expect } from "@playwright/test";
import { PID, settle, stubPlansApi } from "./fixtures";

test.describe("handling mark banner", () => {
  test("shows the plan's effective mark when a scheme is configured", async ({
    page,
  }) => {
    await stubPlansApi(page);
    await page.route("**/api/plans/*/handling-mark", (route) =>
      route.fulfill({
        json: {
          plan_pid: PID,
          scheme: ["OFFICIAL", "OFFICIAL-SENSITIVE"],
          mark: "OFFICIAL-SENSITIVE",
          effective: "OFFICIAL-SENSITIVE",
        },
      }),
    );
    await page.goto(`/en-001/plans/${PID}`, { waitUntil: "networkidle" });
    await expect(page.getByTestId("handling-mark")).toHaveText(
      "Handling mark: OFFICIAL-SENSITIVE",
    );
    await page.goto(`/cy-001/plans/${PID}`, { waitUntil: "networkidle" });
    await expect(page.getByTestId("handling-mark")).toHaveText(
      "Marc trin: OFFICIAL-SENSITIVE",
    );
  });

  test("shows nothing when there is no scheme, and nothing off a plan page", async ({
    page,
  }) => {
    await stubPlansApi(page);
    await page.route("**/api/plans/*/handling-mark", (route) =>
      route.fulfill({
        json: { plan_pid: PID, scheme: [], mark: null, effective: null },
      }),
    );
    await page.goto(`/en-001/plans/${PID}`, { waitUntil: "networkidle" });
    await expect(page.getByTestId("handling-mark")).toHaveCount(0);
    await page.goto("/en-001/plans", { waitUntil: "networkidle" });
    await expect(page.getByTestId("handling-mark")).toHaveCount(0);
  });

  test("a failed lookup says the mark could not be checked, never a guess, and passes axe", async ({
    page,
  }) => {
    // CQ-19: this once showed nothing, which reads as "unmarked".
    await stubPlansApi(page);
    await page.route("**/api/plans/*/handling-mark", (route) =>
      route.fulfill({ status: 500, json: { error: "x" } }),
    );
    await page.goto(`/en-001/plans/${PID}`, { waitUntil: "networkidle" });
    await expect(page.getByTestId("handling-mark")).toHaveText(
      /could not be checked/,
    );
    await expect(page.getByTestId("handling-mark")).not.toContainText(
      "OFFICIAL",
    );
    const { default: AxeBuilder } = await import("@axe-core/playwright");
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

  test("a plan the reader cannot see (404) shows no banner", async ({
    page,
  }) => {
    await stubPlansApi(page);
    await page.route("**/api/plans/*/handling-mark", (route) =>
      route.fulfill({ status: 404, json: { error: "not_found" } }),
    );
    await page.goto(`/en-001/plans/${PID}`, { waitUntil: "networkidle" });
    await expect(page.getByTestId("handling-mark")).toHaveCount(0);
  });
});
