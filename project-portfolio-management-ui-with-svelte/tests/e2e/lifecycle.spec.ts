// The lifecycle funnel (CQ-24): every phase with its live and stalled counts, over a stubbed answer.
import { test, expect } from "@playwright/test";

const FUNNEL = {
  phases: [
    { phase: "idea", live: 6, stalled: 0 },
    { phase: "in_review", live: 3, stalled: 2 },
    { phase: "closed", live: 0, stalled: 0 },
  ],
  unknown_phase: 1,
  stall_days: 14,
  as_of: "2026-10-12T09:00:00Z",
};

test.describe("lifecycle", () => {
  test.beforeEach(async ({ page }) => {
    await page.route("**/api/lifecycle**", (r) => r.fulfill({ json: FUNNEL }));
  });

  test("shows every phase, even an empty one, with its counts", async ({
    page,
  }) => {
    await page.goto("/en-001/lifecycle", { waitUntil: "networkidle" });
    await expect(
      page.getByRole("heading", { level: 1, name: "Lifecycle" }),
    ).toBeVisible();
    await expect(
      page.getByText("9 live item(s) · stall threshold 14 calendar day(s)"),
    ).toBeVisible();
    await expect(page.getByText("in an unrecognized phase")).toBeVisible();
    await expect(
      page.getByRole("row", { name: /^idea\s+6\s+0/ }),
    ).toBeVisible();
    await expect(
      page.getByRole("row", { name: /^in review\s+3\s+2/ }),
    ).toBeVisible();
    await expect(
      page.getByRole("row", { name: /^closed\s+0\s+0/ }),
    ).toBeVisible();
    await expect(page.getByText("as of 2026-10-12T09:00:00Z")).toBeVisible();
  });

  test("says so when the funnel cannot be loaded", async ({ page }) => {
    await page.route("**/api/lifecycle**", (r) =>
      r.fulfill({ status: 500, json: { error: "boom" } }),
    );
    await page.goto("/en-001/lifecycle", { waitUntil: "networkidle" });
    await expect(page.getByRole("alert")).toBeVisible();
  });

  test("passes axe with the funnel populated", async ({ page }) => {
    await page.goto("/en-001/lifecycle", { waitUntil: "networkidle" });
    await expect(page.getByRole("row", { name: /^idea/ })).toBeVisible();
    const { default: AxeBuilder } = await import("@axe-core/playwright");
    const r = await new AxeBuilder({ page })
      .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
      .analyze();
    expect(
      r.violations
        .filter((v) => v.impact === "serious" || v.impact === "critical")
        .map(
          (v) =>
            `${v.id}: ${v.nodes.map((n) => n.target.join(" ")).join(" | ")}`,
        ),
    ).toEqual([]);
  });
});
