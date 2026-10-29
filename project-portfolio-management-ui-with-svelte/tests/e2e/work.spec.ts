// PJ-2 / PJ-5 / PJ-6 in a real browser: the assignment form sends one change and
// shows what the rule did, the work table adds up, an unplaced task is named, a
// finance 404 is a plain note, and each page passes the axe audit.

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

const NODE = {
  pid: "t1",
  title: "Build",
  status: null,
  early_start: "2026-10-05",
  early_finish: "2026-10-09",
  late_start: "2026-10-05",
  late_finish: "2026-10-09",
  total_float: 0,
  free_float: 0,
  critical: true,
  near_critical: false,
  planned_start: "2026-10-05",
  duration_days: 5,
  pushed_days: 0,
  constraint: null,
  work_minutes: 2400,
  units_percent: 100,
  scheduling_rule: "fixed_units",
  contour: "flat",
};
const CPM = {
  plan_pid: PID,
  calendar: "plan",
  calendar_conflicts: [],
  project_finish: "2026-10-09",
  nodes: [
    NODE,
    {
      ...NODE,
      pid: "t2",
      title: "Loose",
      work_minutes: null,
      units_percent: null,
      scheduling_rule: null,
    },
  ],
  edges: [],
  unscheduled: [],
  ignored_edges: [],
  critical_chains: [],
  chains_truncated: false,
};
const day = (n: number) => `2026-10-0${n}`;
const PROFILE = {
  plan_pid: PID,
  period: "week",
  tasks: [
    {
      pid: "t1",
      title: "Build",
      contour: "flat",
      work_minutes: 2400,
      placed: true,
      buckets: [{ from: day(5), to: day(11), minutes: 2400 }],
    },
  ],
  totals: {
    buckets: [{ from: day(5), to: day(11), minutes: 2400 }],
    work_minutes: 2880,
    unplaced_work_minutes: 480,
  },
  unplaced: [
    { pid: "t3", title: "Floating", work_minutes: 480, reason: "unscheduled" },
  ],
  note: "",
};

test("the assignment form sends one change, says what the rule did, and the table adds up", async ({
  page,
}) => {
  let sent: unknown = null;
  await page.route(`**/api/plans/${PID}/cpm**`, (r) =>
    r.fulfill({ json: CPM }),
  );
  await page.route(`**/api/plans/${PID}/work-profile**`, (r) =>
    r.fulfill({ json: PROFILE }),
  );
  await page.route(`**/api/plans/${PID}/tasks/t1/assignment`, (r) => {
    sent = r.request().postDataJSON();
    return r.fulfill({
      json: {
        task_pid: "t1",
        rule: "fixed_units",
        duration_days: 10,
        units_percent: 100,
        work_minutes: 4800,
        derived: "duration",
        exact: true,
        capacity: {
          checked: true,
          over_capacity: true,
          days_over: 2,
          from: day(5),
          to: "2026-10-16",
        },
        note: "",
      },
    });
  });
  await page.goto(`/en-001/plans/${PID}/work`);
  await expect(
    page.getByText("5 business days · 100% of a day · 40 h of work"),
  ).toBeVisible();
  await expect(page.getByText("No assignment yet")).toBeVisible();
  await page.getByText("Edit assignment").first().click();
  await page.getByRole("spinbutton", { name: "Work (hours)" }).fill("80");
  await page.getByRole("button", { name: "Save assignment" }).click();
  await expect(page.getByRole("status")).toContainText(
    "10 business days at 100% is 80 h. The duration was worked out.",
  );
  await expect(page.getByRole("status")).toContainText(
    "Over capacity on 2 calendar days",
  );
  await expect
    .poll(() => sent)
    .toEqual({ rule: "fixed_units", work_minutes: 4800 });
  // The profile: the plan's total, and the unplaced work named rather than dropped.
  await expect(page.getByTestId("work-profile").locator("tfoot")).toContainText(
    "48 h",
  );
  await expect(page.getByText("Floating (8 h) has no dates yet")).toBeVisible();
  await audit(page);
});

test("a bad value is stopped in the form with a sentence, not sent", async ({
  page,
}) => {
  let called = false;
  await page.route(`**/api/plans/${PID}/cpm**`, (r) =>
    r.fulfill({ json: CPM }),
  );
  await page.route(`**/api/plans/${PID}/work-profile**`, (r) =>
    r.fulfill({
      json: {
        ...PROFILE,
        reason: "no_assignments",
        totals: { buckets: null, work_minutes: 0 },
        tasks: [],
        unplaced: [],
      },
    }),
  );
  await page.route(`**/api/plans/${PID}/tasks/*/assignment`, (r) => {
    called = true;
    return r.fulfill({ json: {} });
  });
  await page.goto(`/en-001/plans/${PID}/work`);
  await page.getByText("Edit assignment").first().click();
  await page.getByRole("spinbutton", { name: "Work (hours)" }).fill("0");
  await page.getByRole("button", { name: "Save assignment" }).click();
  await expect(page.getByRole("status")).toContainText(
    "Work must be a number of hours above zero.",
  );
  expect(called).toBe(false);
});

test("costs show each currency alone, name unrated work, and hide behind a plain note for non-finance", async ({
  page,
}) => {
  await page.route(`**/api/plans/${PID}/cpm**`, (r) =>
    r.fulfill({ json: CPM }),
  );
  await page.route(`**/api/plans/${PID}/cost-plan**`, (r) =>
    r.fulfill({
      json: {
        plan_pid: PID,
        note: "",
        unplaced: [],
        unassigned: [],
        unrated: [
          { pid: "t1", title: "Build", work_minutes: 480, reason: "no_rate" },
        ],
        currencies: [
          {
            currency: "GBP",
            labour: {
              total_minor: 128000,
              buckets: [{ from: day(5), to: day(11), minor: 128000 }],
            },
            fixed: {
              total_minor: 100,
              buckets: [{ from: day(5), to: day(11), minor: 100 }],
            },
            total: {
              total_minor: 128100,
              buckets: [{ from: day(5), to: day(11), minor: 128100 }],
            },
          },
        ],
      },
    }),
  );
  await page.route(`**/api/plans/${PID}/labour-costs`, (r) =>
    r.fulfill({
      json: {
        currencies: [
          { currency: "GBP", cost_minor: 10000, minutes: 120, entries: 1 },
        ],
        unpriced: {
          entries: 2,
          minutes: 60,
          reason: "not_recorded_or_no_rate",
        },
        note: "",
      },
    }),
  );
  await page.goto(`/en-001/plans/${PID}/costs`);
  await expect(page.getByTestId("cost-GBP")).toContainText("1,281.00");
  await expect(
    page.getByText(
      "Build (8 h): no rate applies, so its labour is not costed.",
    ),
  ).toBeVisible();
  await expect(page.getByText("unknown, not zero")).toBeVisible();
  await audit(page);

  await page.unroute(`**/api/plans/${PID}/cost-plan**`);
  await page.route(`**/api/plans/${PID}/cost-plan**`, (r) =>
    r.fulfill({ status: 404, json: { error: "not_found" } }),
  );
  await page.goto(`/en-001/plans/${PID}/costs`);
  await expect(page.getByTestId("denied")).toBeVisible();
  await audit(page);
});

test("cost rates: a table with no rate says its work is not costed; a 404 is a plain note", async ({
  page,
}) => {
  await page.route("**/api/cost-rate-tables", (r) =>
    r.fulfill({
      json: {
        tables: [
          {
            pid: "x1",
            name: "standard",
            person_ref: null,
            role: "Engineer",
            currency: "GBP",
            rates: [{ effective_from: "2026-01-01", per_hour_minor: 5000 }],
          },
          {
            pid: "x2",
            name: "overtime",
            person_ref: null,
            role: "Engineer",
            currency: "GBP",
            rates: [],
          },
        ],
      },
    }),
  );
  await page.goto("/en-001/cost-rates");
  await expect(page.getByText("£50.00")).toBeVisible();
  await expect(
    page.getByText("No rate yet: work by this role is not costed."),
  ).toBeVisible();
  // WW-1.3: no currency is built in, so the new-table form starts with none
  await expect(page.getByLabel("Currency")).toHaveValue("");
  await expect(page.getByLabel("Currency")).toHaveAttribute(
    "placeholder",
    "for example EUR, JPY or GBP",
  );
  await audit(page);
  await page.unroute("**/api/cost-rate-tables");
  await page.route("**/api/cost-rate-tables", (r) =>
    r.fulfill({ status: 404, json: { error: "not_found" } }),
  );
  await page.goto("/en-001/cost-rates");
  await expect(page.getByTestId("denied")).toBeVisible();
});
