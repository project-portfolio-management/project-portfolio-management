// Earned value (CQ-24): the figures and their sentences, how percent complete is counted and
// changed, the snapshot button, and the states where there is nothing to show.
import { test, expect, type Page } from "@playwright/test";

const PID = "77777777-7777-4777-8777-777777777777";
const BLOCK = {
  currency: "GBP",
  baseline_version: 2,
  bac_minor: 10_000_000,
  pv_minor: 4_000_000,
  ev_minor: 3_000_000,
  ac_minor: 3_500_000,
  sv_minor: -1_000_000,
  cv_minor: -500_000,
  spi: { bp: 7500 },
  cpi: { bp: 8571 },
  earned_schedule_days: 20,
  actual_days: 28,
  spi_t: { bp: 7143 },
};
const EV = {
  plan_pid: PID,
  as_of: "2026-10-12",
  blocks: [BLOCK],
  note: "Figures are advisory.",
  percent_complete: {
    percent_bp: 3000,
    default_method: "zero_hundred",
    weighting: "equal",
    tasks_counted: 2,
    unweighted_tasks: [],
    manual_tasks: ["t2"],
    manual: true,
    tasks: [
      { pid: "t1", weight: 1, percent_bp: 0, method: "zero_hundred" },
      { pid: "t2", weight: 1, percent_bp: 6000, method: "manual" },
    ],
  },
};

async function stub(page: Page, ev: unknown = EV) {
  const sent: Record<string, unknown> = {};
  const calls: string[] = [];
  await page.route(`**/api/plans/${PID}/**`, async (r) => {
    const req = r.request();
    const path = new URL(req.url()).pathname.replace(/^\/api\/proxy/, "");
    calls.push(`${req.method()} ${path}`);
    if (req.method() !== "GET") {
      sent[path] = req.postData() ? req.postDataJSON() : null;
      return r.fulfill({ json: {} });
    }
    if (path.endsWith("/earned-value")) return r.fulfill({ json: ev });
    if (path.endsWith("/earned-value/forecast")) return r.fulfill({ json: {} });
    if (path.endsWith("/earned-value/series"))
      return r.fulfill({ json: { curves: [], note: "" } });
    if (path.endsWith("/earned-value/rollup"))
      return r.fulfill({ json: { currencies: [], plans_without_figures: 0 } });
    if (path.endsWith("/tasks"))
      return r.fulfill({
        json: {
          tasks: [
            { pid: "t1", title: "Design" },
            { pid: "t2", title: "Build" },
          ],
          total: 2,
        },
      });
    return r.fulfill({ status: 404, json: { error: "unhandled in stub" } });
  });
  return { sent, calls };
}

test.describe("earned value", () => {
  test("states the figures, and what each ratio means in words", async ({
    page,
  }) => {
    await stub(page);
    await page.goto(`/en-001/plans/${PID}/earned-value`, {
      waitUntil: "networkidle",
    });
    await expect(
      page.getByRole("heading", {
        level: 1,
        name: "Progress against budget (earned value)",
      }),
    ).toBeVisible();
    await expect(page.getByText("Measured as of 2026-10-12")).toBeVisible();
    await expect(page.getByRole("heading", { name: /^GBP/ })).toContainText(
      "baseline version 2",
    );
    const figures = page.getByRole("table", {
      name: "GBP earned value figures",
    });
    await expect(
      figures.getByRole("row", { name: /^Budget at completion/ }),
    ).toContainText("100,000");
    await expect(
      figures.getByRole("row", { name: /^Planned value/ }),
    ).toContainText("40,000");
    await expect(
      figures.getByRole("row", { name: /^Earned value/ }),
    ).toContainText("30,000");
    await expect(
      figures.getByRole("row", { name: /^Actual cost/ }),
    ).toContainText("35,000");
    await expect(page.getByText(/behind schedule/i).first()).toBeVisible();
    await expect(page.getByText(/over budget/i).first()).toBeVisible();
  });

  test("says how percent complete is counted, and flags a hand-entered percent", async ({
    page,
  }) => {
    await stub(page);
    await page.goto(`/en-001/plans/${PID}/earned-value`, {
      waitUntil: "networkidle",
    });
    await expect(
      page.getByRole("heading", { name: "30% complete" }),
    ).toBeVisible();
    await expect(
      page.getByRole("note").filter({ hasText: "Build" }),
    ).toBeVisible();
    await expect(
      page.getByRole("listitem").filter({ hasText: "Build: 60%" }),
    ).toContainText("manual");
    await expect(
      page.getByRole("listitem").filter({ hasText: "Design: 0%" }),
    ).toContainText("finished only");
  });

  test("changing the plan's method sends it", async ({ page }) => {
    const s = await stub(page);
    await page.goto(`/en-001/plans/${PID}/earned-value`, {
      waitUntil: "networkidle",
    });
    await page.getByLabel(/^Count tasks/).selectOption("fifty_fifty");
    await expect
      .poll(() => s.sent[`/api/plans/${PID}/earned-value-settings`])
      .toEqual({ default_method: "fifty_fifty" });
  });

  test("a hand-entered percent needs a reason, and is sent with it", async ({
    page,
  }) => {
    const s = await stub(page);
    await page.goto(`/en-001/plans/${PID}/earned-value`, {
      waitUntil: "networkidle",
    });
    await page
      .getByRole("listitem")
      .filter({ hasText: "Design: 0%" })
      .getByRole("button", { name: "Change" })
      .click();
    await page.getByLabel(/^Method/).selectOption("manual");
    await page.getByLabel(/^Percent complete/).fill("40");
    await page.getByRole("button", { name: "Save", exact: true }).click();
    expect(
      Object.keys(s.sent).filter((k) => k.endsWith("/tasks/t1/percent")),
    ).toHaveLength(0);
    await page.getByLabel(/^Why/).fill("Half the screens are done");
    await page.getByRole("button", { name: "Save", exact: true }).click();
    await expect
      .poll(
        () =>
          Object.entries(s.sent).find(([k]) => k.includes("/tasks/t1/"))?.[1],
      )
      .toMatchObject({
        method: "manual",
        percent: 40,
        note: "Half the screens are done",
      });
  });

  test("recording today's figures posts a snapshot and says so", async ({
    page,
  }) => {
    const s = await stub(page);
    await page.goto(`/en-001/plans/${PID}/earned-value`, {
      waitUntil: "networkidle",
    });
    await page
      .getByRole("button", { name: "Record today's figures now" })
      .click();
    await expect
      .poll(() => s.calls)
      .toContain(`POST /api/plans/${PID}/earned-value/snapshot`);
    await expect(
      page.getByRole("status").filter({ hasText: "Recorded" }),
    ).toBeVisible();
  });

  test("a plan with no baseline says why there are no figures", async ({
    page,
  }) => {
    await stub(page, {
      plan_pid: PID,
      as_of: "2026-10-12",
      blocks: null,
      absent: "no_baseline",
      reason: "Take a cost baseline first.",
    });
    await page.goto(`/en-001/plans/${PID}/earned-value`, {
      waitUntil: "networkidle",
    });
    await expect(page.getByTestId("ev-absent")).toHaveText(
      "Take a cost baseline first.",
    );
  });

  test("says so when the figures cannot be loaded", async ({ page }) => {
    await page.route(`**/api/plans/${PID}/**`, (r) =>
      r.fulfill({ status: 500, json: { error: "boom" } }),
    );
    await page.goto(`/en-001/plans/${PID}/earned-value`, {
      waitUntil: "networkidle",
    });
    await expect(page.getByRole("alert")).toBeVisible();
  });

  test("passes axe with the figures shown", async ({ page }) => {
    await stub(page);
    await page.goto(`/en-001/plans/${PID}/earned-value`, {
      waitUntil: "networkidle",
    });
    await expect(
      page.getByRole("table", { name: "GBP earned value figures" }),
    ).toBeVisible();
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
