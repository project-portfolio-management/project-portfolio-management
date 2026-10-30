// FR-53 in a real browser: the tolerance board and a plan's tolerances show red / amber / green as a word and a
// shape (never colour alone), say plainly when something could not be measured, and let a limit be set and
// removed. The service is stubbed; both pages pass the axe audit.

import { test, expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle } from "./fixtures";

async function audit(page: Page) {
  await settle(page);
  const r = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    r.violations
      .filter((v) => v.impact === "serious" || v.impact === "critical")
      .map((v) => v.id),
  ).toEqual([]);
}

const dimension = (name: string, over: Record<string, unknown>) => ({
  dimension: name,
  unit: "u",
  basis: `How ${name} is measured.`,
  status: "not_set",
  limit: null,
  source: "none",
  ...over,
});

const picture = (
  overall: string,
  dimensions: unknown[],
  pid = PID,
  name = "Apollo",
) => ({
  plan_pid: pid,
  plan_name: name,
  overall,
  as_of: "2026-10-20T10:00:00Z",
  dimensions,
  how_to_read: "x",
});

test("the board shows each plan's colours as words and shapes, worst first", async ({
  page,
}) => {
  const apollo = picture("red", [
    dimension("time", {
      status: "red",
      limit: 5,
      measured: 9,
      usage_percent: 180,
      source: "own",
      amber_from_percent: 75,
    }),
    dimension("cost", {
      status: "unknown",
      limit: 500,
      measured: null,
      reason: "no baseline",
      source: "inherited",
    }),
    dimension("risk", {
      status: "green",
      limit: 25,
      measured: 4,
      usage_percent: 16,
      source: "own",
      amber_from_percent: 75,
    }),
    dimension("resources", {}),
  ]);
  const gemini = picture(
    "amber",
    [
      dimension("time", {
        status: "amber",
        limit: 10,
        measured: 8,
        usage_percent: 80,
        source: "own",
        amber_from_percent: 75,
      }),
      dimension("cost", {}),
      dimension("risk", {}),
      dimension("resources", {}),
    ],
    "p-gemini",
    "Gemini",
  );
  await page.route("**/api/tolerances**", (r) => {
    const status = new URL(r.request().url()).searchParams.get("status");
    const plans = [apollo, gemini].filter(
      (p) => !status || p.overall === status,
    );
    return r.fulfill({
      json: { plans, truncated: false, measured_at_most: 200, note: "n" },
    });
  });
  await page.goto("/en-001/tolerances");
  const rows = page.getByRole("row");
  await expect(rows.nth(1)).toContainText("Apollo");
  await expect(rows.nth(1)).toContainText("over tolerance");
  await expect(rows.nth(1)).toContainText("not measurable");
  await expect(rows.nth(1)).toContainText("no limit set");
  await expect(rows.nth(2)).toContainText("Gemini");
  await expect(rows.nth(2)).toContainText("near the limit");
  await expect(page.getByRole("link", { name: "Apollo" })).toHaveAttribute(
    "href",
    /\/plans\/.*\/tolerances$/,
  );
  await audit(page);
  await page.getByLabel("Show").selectOption("amber");
  await expect(page.getByRole("link", { name: "Apollo" })).toHaveCount(0);
  await expect(page.getByRole("link", { name: "Gemini" })).toBeVisible();
});

test("a plan's tolerances can be set and removed, and an unmeasurable one says why", async ({
  page,
}) => {
  let timeLimit: number | null = null;
  const current = () =>
    picture(timeLimit === null ? "unknown" : "red", [
      timeLimit === null
        ? dimension("time", {})
        : dimension("time", {
            status: "red",
            limit: timeLimit,
            measured: 9,
            usage_percent: 180,
            source: "own",
            amber_from_percent: 75,
          }),
      dimension("schedule", {
        status: "unknown",
        limit: 5,
        measured: null,
        reason:
          "the plan has no schedule baseline to hold the forecast against",
        source: "own",
        amber_from_percent: 75,
      }),
      dimension("cost", {
        status: "unknown",
        limit: 1000,
        measured: null,
        reason: "there is no approved budget baseline",
        source: "inherited",
        from_plan_pid: "p-prog",
      }),
      dimension("risk", {}),
      dimension("resources", {}),
    ]);
  let sent: unknown = null;
  await page.route(`**/api/plans/${PID}/tolerances`, (r) =>
    r.fulfill({ json: current() }),
  );
  await page.route(`**/api/plans/${PID}/tolerances/time`, (r) => {
    sent = r.request().postDataJSON();
    timeLimit = (sent as { limit: number }).limit;
    return r.fulfill({ json: current() });
  });
  await page.route(`**/api/plans/${PID}/tolerances/time/delete`, (r) => {
    timeLimit = null;
    return r.fulfill({ json: current() });
  });
  await page.goto(`/en-001/plans/${PID}/tolerances`);
  await expect(
    page.getByRole("heading", { name: "Tolerances: Apollo" }),
  ).toBeVisible();
  const schedule = page.getByRole("region", {
    name: /Schedule against baseline/,
  });
  await expect(schedule).toContainText("not measurable");
  await expect(schedule).toContainText("no schedule baseline");
  const cost = page.getByRole("region", { name: /Cost/ });
  await expect(cost).toContainText("not measurable");
  await expect(cost).toContainText("there is no approved budget baseline");
  await expect(cost).toContainText("10%");
  await expect(cost).toContainText("Inherited from a plan above");
  await audit(page);

  const time = page.getByRole("region", { name: /Time/ });
  await expect(time).toContainText("no limit set");
  await time.getByLabel("Limit (calendar days late)").fill("5");
  await time.getByRole("button", { name: "Set the limit" }).click();
  await expect.poll(() => sent).toEqual({ limit: 5, note: null });
  await expect(page.getByRole("status")).toContainText("Saved the Time limit.");
  await expect(time).toContainText("over tolerance");
  await expect(time).toContainText("9 days");
  await time
    .getByRole("button", { name: "Remove this plan's own limit" })
    .click();
  await expect(page.getByRole("status")).toContainText(
    "Removed the Time limit.",
  );
  await expect(time).toContainText("no limit set");

  // A blank or non-numeric limit is refused here, in words, before anything is sent.
  sent = null;
  await time.getByRole("button", { name: "Set the limit" }).click();
  await expect(page.getByRole("status")).toContainText(
    "Give a limit as a number.",
  );
  expect(sent).toBeNull();
});

test("a plan with red tolerances can escalate them all in one go", async ({
  page,
}) => {
  let escalated = false;
  const current = () => ({
    ...picture("red", [
      dimension("time", {
        status: "red",
        limit: 5,
        measured: 9,
        usage_percent: 180,
        source: "own",
        amber_from_percent: 75,
        escalation_needed: !escalated,
        open_escalations: escalated ? 1 : 0,
      }),
      dimension("cost", {}),
      dimension("risk", {}),
      dimension("resources", {}),
    ]),
    escalation_needed: !escalated,
  });
  await page.route(`**/api/plans/${PID}/tolerances`, (r) =>
    r.fulfill({ json: current() }),
  );
  await page.route(`**/api/plans/${PID}/tolerances/escalate`, (r) => {
    escalated = true;
    return r.fulfill({
      json: {
        raised: [{ pid: "e1", tolerance_dimension: "time" }],
        tolerances: current(),
      },
    });
  });
  await page.goto(`/en-001/plans/${PID}/tolerances`);
  await page
    .getByRole("button", { name: "Escalate every red tolerance" })
    .click();
  await expect(page.getByRole("status")).toContainText("Raised 1 escalation.");
  await expect(
    page.getByRole("button", { name: "Escalate every red tolerance" }),
  ).toHaveCount(0);
  await audit(page);
});
