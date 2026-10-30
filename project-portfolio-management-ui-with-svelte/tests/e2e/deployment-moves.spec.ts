// The deployment moves screen (EX-5), over stubbed API answers.
import { test, expect } from "@playwright/test";

const PID = "11111111-1111-4111-8111-111111111111";
const MOVES = {
  moves: [
    {
      pid: "mv1",
      plan_pid: PID,
      task_pid: "t1",
      from_start: "2026-11-02",
      to_start: "2026-11-16",
      reason: "Vendor late",
      requested_by: "ana",
      approver_ref: "role:Programme director",
      status: "requested",
      decided_by: null,
      decided_on: null,
      decision_reason: null,
      knock_on: [
        {
          task_pid: "t2",
          title: "Test",
          plan_pid: PID,
          other_deployment: false,
          moved: true,
          start_days: 10,
          finish_days: 10,
        },
      ],
      knock_on_cost: { labour: [{ currency: "GBP", delta_minor: 120000 }] },
    },
    {
      pid: "mv2",
      plan_pid: PID,
      task_pid: "t1",
      from_start: "2026-10-01",
      to_start: "2026-10-08",
      reason: "Earlier",
      requested_by: "ana",
      approver_ref: "role:Programme director",
      status: "refused",
      decided_by: "dir",
      decided_on: "2026-10-02T09:00:00Z",
      decision_reason: "Too late",
      knock_on: [],
      knock_on_cost: { labour: [] },
    },
  ],
  cost_by_requester: [
    {
      requested_by: "ana",
      requests: 2,
      knock_on_labour: [{ currency: "GBP", delta_minor: 120000 }],
    },
  ],
  note: "x",
};

test.describe("deployment moves", () => {
  let posted = "";
  let decided = "";
  test.beforeEach(async ({ page }) => {
    posted = decided = "";
    await page.route(`**/api/plans/${PID}/deployment-moves`, (r) => {
      if (r.request().method() === "POST") {
        posted = r.request().postData() ?? "";
        return r.fulfill({ json: {} });
      }
      return r.fulfill({ json: MOVES });
    });
    await page.route(
      `**/api/plans/${PID}/deployment-moves/mv1/decision`,
      (r) => {
        decided = r.request().postData() ?? "";
        return r.fulfill({ json: {} });
      },
    );
    await page.route(`**/api/plans/${PID}/tasks`, (r) =>
      r.fulfill({
        json: {
          tasks: [{ pid: "t1", title: "Build the interface", status: "todo" }],
          counts: {},
        },
      }),
    );
  });

  test("lists moves with the knock-on and cost, requests one and decides one", async ({
    page,
  }) => {
    await page.goto(`/en-001/plans/${PID}/deployment-moves`, {
      waitUntil: "networkidle",
    });
    await expect(
      page.getByRole("heading", { level: 1, name: "Deployment moves" }),
    ).toBeVisible();
    await expect(
      page.getByRole("row", {
        name: /Build the interface.*2 Nov 2026 → 16 Nov 2026.*waiting for the approver.*1 other tasks move; labour cost \+£1,200\.00/,
      }),
    ).toBeVisible();
    await expect(
      page.getByRole("row", { name: /refused.*dir on 2 Oct 2026: Too late/ }),
    ).toContainText("✖");
    await page.getByLabel("New start").fill("2026-12-01");
    await page
      .getByLabel("Reason", { exact: true })
      .first()
      .fill("Customer asked");
    await page.getByRole("button", { name: "Request the move" }).click();
    await expect
      .poll(() => posted)
      .toBe(
        '{"task_pid":"t1","new_start":"2026-12-01","reason":"Customer asked"}',
      );
    await page.getByLabel("Reason for the decision").fill("Agreed");
    await page
      .getByRole("button", { name: "Approve the move of Build the interface" })
      .click();
    await expect
      .poll(() => decided)
      .toBe('{"decision":"approve","reason":"Agreed"}');
  });

  test("reads in Welsh", async ({ page }) => {
    await page.goto(`/cy-001/plans/${PID}/deployment-moves`, {
      waitUntil: "networkidle",
    });
    await expect(
      page.getByRole("heading", { level: 1, name: "Symud lleoliadau" }),
    ).toBeVisible();
  });

  for (const locale of ["en-001", "cy-001", "zh-cn"]) {
    test(`passes axe in ${locale}`, async ({ page }) => {
      await page.goto(`/${locale}/plans/${PID}/deployment-moves`, {
        waitUntil: "networkidle",
      });
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
  }
});
