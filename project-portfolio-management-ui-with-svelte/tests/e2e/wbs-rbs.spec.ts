// PJ-12 in a real browser: the WBS tree shows generated codes but links by task id,
// a task under an unseen parent sits under "Restricted item", the RBS shows roles
// with people and skills as demand only, and each page passes the axe audit.

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

const task = (
  pid: string,
  title: string,
  parent: string | null,
  n: number,
) => ({
  pid,
  title,
  description: null,
  status: "todo",
  assignee_ref: null,
  points: null,
  due_date: null,
  sprint_pid: null,
  created_at: `2026-10-0${n}T00:00:00Z`,
  status_changed_at: `2026-10-0${n}T00:00:00Z`,
  done_at: null,
  blocked_days: null,
  parent_task_pid: parent,
});

test("the WBS shows codes, links by id, and hides what it must", async ({
  page,
}) => {
  await page.route(`**/api/plans/${PID}/tasks`, (r) =>
    r.fulfill({
      json: {
        counts: {},
        tasks: [
          task("a1", "Design", null, 1),
          task("b2", "Mockups", "a1", 2),
          task("c3", "Orphan", "hidden-parent", 3),
        ],
      },
    }),
  );
  await page.goto(`/en-001/plans/${PID}/wbs`);
  const tree = page.getByRole("list", { name: "Work breakdown tree" });
  await expect(tree).toContainText("1");
  await expect(tree).toContainText("1.1");
  await expect(tree).toContainText("Restricted item");
  await expect(tree.getByRole("link", { name: "Mockups" })).toHaveAttribute(
    "href",
    /\/tasks\/b2$/,
  );
  await expect(page.getByRole("table")).toContainText("2.1");
  await expect(page.locator("body")).not.toContainText("hidden-parent");
  await audit(page);
});

test("the RBS lists roles and skills, skills as demand only", async ({
  page,
}) => {
  const row = (key: string, d: number, s: number | null, h: number) => ({
    key,
    demand_fte: d,
    named_fte: d,
    placeholder_fte: 0,
    proposed_fte: 0,
    supply_fte: s,
    gap_fte: null,
    headcount: h,
  });
  const periods = (rows: unknown[]) => [
    {
      from: "2026-10-01",
      to: "2026-10-31",
      days: 31,
      rows,
      total_demand_fte: 1,
      total_placeholder_fte: 0,
      total_proposed_fte: 0,
    },
  ];
  const base = {
    scope: { kind: "all" },
    from: "2026-10-01",
    to: "2026-12-31",
    period: "month",
    placeholders: [],
    masked: false,
    note: "",
  };
  await page.route("**/api/resource-plan**", (r) => {
    const skill =
      new URL(r.request().url()).searchParams.get("group_by") === "skill";
    return r.fulfill({
      json: skill
        ? {
            ...base,
            group_by: "skill",
            overlapping: true,
            periods: periods([row("rust", 1, null, 0)]),
            people_by_role: null,
          }
        : {
            ...base,
            group_by: "role",
            overlapping: false,
            periods: periods([row("Engineer", 1.5, 2, 2)]),
            people_by_role: { Engineer: ["person:x", "person:y"] },
          },
    });
  });
  await page.goto("/en-001/resources/rbs");
  await expect(
    page.getByText("Engineer: 2 named, demand 1.5 FTE, supply 2 FTE"),
  ).toBeVisible();
  await expect(page.getByText("person:x")).toBeVisible();
  await expect(
    page
      .getByRole("cell", { name: "rust" })
      .or(page.getByRole("rowheader", { name: "rust" })),
  ).toBeVisible();
  await expect(
    page.getByText("Supply and people are not known for skills"),
  ).toBeVisible();
  await audit(page);
});

test("a task moves one place among its siblings and the order is sent to the service", async ({
  page,
}) => {
  let order = ["a1", "b2", "c3"];
  let sent: unknown = null;
  await page.route(`**/api/plans/${PID}/tasks`, (r) =>
    r.fulfill({
      json: {
        counts: {},
        tasks: order.map((pid, i) => ({
          ...task(pid, pid.toUpperCase(), null, Number(pid.slice(1))),
          sibling_position: i,
        })),
      },
    }),
  );
  await page.route(`**/api/plans/${PID}/task-order`, async (r) => {
    sent = r.request().postDataJSON();
    order = (sent as { order: string[] }).order;
    await r.fulfill({ json: sent as object });
  });
  await page.goto(`/en-001/plans/${PID}/wbs`);
  const tree = page.getByRole("list", { name: "Work breakdown tree" });
  await expect(tree.getByRole("button", { name: "Move A1 up" })).toBeDisabled();
  await expect(tree.getByRole("button", { name: "Move C3 down" })).toBeDisabled();
  await tree.getByRole("button", { name: "Move C3 up" }).click();
  await expect(page.getByRole("status")).toContainText("Moved C3.");
  expect(sent).toEqual({ parent_task_pid: null, order: ["a1", "c3", "b2"] });
  await expect(tree.getByRole("listitem").nth(1)).toContainText("C3");
  await audit(page);
});

test("a role opens onto the open tasks of its people", async ({ page }) => {
  const row = {
    key: "Engineer",
    demand_fte: 1,
    named_fte: 1,
    placeholder_fte: 0,
    proposed_fte: 0,
    supply_fte: 2,
    gap_fte: null,
    headcount: 2,
  };
  const plan = (group: string) => ({
    scope: { kind: "all" },
    from: "2026-10-01",
    to: "2026-12-31",
    period: "month",
    group_by: group,
    overlapping: group === "skill",
    periods: [
      {
        from: "2026-10-01",
        to: "2026-10-31",
        days: 31,
        rows: group === "role" ? [row] : [],
        total_demand_fte: 1,
        total_placeholder_fte: 0,
        total_proposed_fte: 0,
      },
    ],
    placeholders: [],
    people_by_role: group === "role" ? { Engineer: ["person:x", "person:y"] } : null,
    masked: false,
    note: "",
  });
  await page.route("**/api/resource-plan?**", (r) =>
    r.fulfill({
      json: plan(new URL(r.request().url()).searchParams.get("group_by") ?? "role"),
    }),
  );
  await page.route("**/api/resource-plan/role-tasks?**", (r) =>
    r.fulfill({
      json: {
        role: "Engineer",
        people: [
          {
            person_ref: "person:x",
            tasks: [
              { pid: "t1", title: "Fix the build", status: "todo", due_date: null, plan_pid: PID, plan_name: "Apollo" },
            ],
          },
          { person_ref: "person:y", tasks: [] },
        ],
        tasks: 1,
        omitted_unreadable: 2,
        truncated: false,
      },
    }),
  );
  await page.goto("/en-001/resources/rbs");
  const button = page.getByRole("button", { name: "Show the open tasks of Engineer" });
  await expect(button).toHaveAttribute("aria-expanded", "false");
  await button.click();
  const region = page.getByRole("region", { name: "Show the open tasks of Engineer" });
  await expect(region.getByRole("link", { name: "Fix the build" })).toHaveAttribute("href", /\/plans\/.*\/tasks\/t1$/);
  await expect(region).toContainText("no open tasks");
  await expect(region).toContainText("2 in plans you cannot read are not shown.");
  await expect(page.getByRole("button", { name: "Hide the open tasks of Engineer" })).toHaveAttribute("aria-expanded", "true");
  await audit(page);
  await page.getByRole("button", { name: "Hide the open tasks of Engineer" }).click();
  await expect(region).toHaveCount(0);
});
