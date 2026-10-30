// CF-1: a very large plan is never silently shortened. The service caps the task list at 2,000 and
// says so (`truncated`, `total`); the WBS and earned-value pages must say so too, and the critical
// path page must offer the critical part when the full answer is refused (413).

import { test, expect, type Page } from "@playwright/test";
import { PID } from "./fixtures";

const task = (n: number) => ({
  pid: `t${n}`,
  title: `Task ${n}`,
  description: null,
  status: "todo",
  assignee_ref: null,
  points: null,
  due_date: null,
  sprint_pid: null,
  created_at: "2026-10-01T00:00:00Z",
  status_changed_at: "2026-10-01T00:00:00Z",
  done_at: null,
  blocked_days: null,
  parent_task_pid: null,
});
/** The first 2,000 of a 2,001-task plan, as the service answers with no paging parameters. */
const FIRST_PAGE = {
  tasks: Array.from({ length: 2000 }, (_, i) => task(i)),
  counts: { todo: 2001 },
  total: 2001,
  truncated: true,
};

async function listTasks(page: Page, body: unknown) {
  await page.route(`**/api/plans/${PID}/tasks`, (r) =>
    r.fulfill({ json: body }),
  );
}

test("the WBS says when a 2,001-task plan is only partly listed", async ({
  page,
}) => {
  await listTasks(page, FIRST_PAGE);
  await page.route(`**/api/plans/${PID}/task-link-counts`, (r) =>
    r.fulfill({ json: { plan_pid: PID, counts: {} } }),
  );
  await page.goto(`/en-001/plans/${PID}/wbs`);
  await expect(page.getByTestId("wbs-truncated")).toHaveText(
    /Showing 2000 of 2001 tasks/,
  );
});

test("the WBS shows no notice when the whole plan is listed", async ({
  page,
}) => {
  await listTasks(page, {
    tasks: [task(1)],
    counts: { todo: 1 },
    total: 1,
    truncated: false,
  });
  await page.route(`**/api/plans/${PID}/task-link-counts`, (r) =>
    r.fulfill({ json: { plan_pid: PID, counts: {} } }),
  );
  await page.goto(`/en-001/plans/${PID}/wbs`);
  await expect(page.getByRole("heading", { level: 1 })).toBeVisible();
  await expect(page.getByTestId("wbs-truncated")).toHaveCount(0);
});

test("earned value says task names may be missing on a very large plan", async ({
  page,
}) => {
  await listTasks(page, FIRST_PAGE);
  await page.route(`**/api/plans/${PID}/earned-value*`, (r) => {
    const path = new URL(r.request().url()).pathname;
    if (path.endsWith("/forecast")) return r.fulfill({ json: {} });
    if (path.endsWith("/series"))
      return r.fulfill({ json: { curves: [], note: "" } });
    if (path.endsWith("/rollup"))
      return r.fulfill({ json: { currencies: [], plans_without_figures: 0 } });
    return r.fulfill({
      json: {
        plan_pid: PID,
        as_of: "2026-10-12",
        blocks: [],
        note: "Figures are advisory.",
        percent_complete: null,
      },
    });
  });
  await page.goto(`/en-001/plans/${PID}/earned-value`);
  await expect(page.getByTestId("ev-names-truncated")).toContainText(
    "the figures cover every task",
  );
});

test("the critical path offers the critical part when the full answer is refused", async ({
  page,
}) => {
  const asked: string[] = [];
  const node = {
    pid: "c1",
    title: "Design",
    status: "todo",
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
    constraint_date: null,
    deadline: null,
    hard_conflict: false,
    calendar: "plan",
  };
  await page.route(`**/api/plans/${PID}/**`, (r) => {
    const url = new URL(r.request().url());
    const path = url.pathname;
    if (path.endsWith("/cpm")) {
      asked.push(url.search);
      if (!url.search.includes("only=critical")) {
        return r.fulfill({
          status: 413,
          json: {
            error: "too_large",
            message: "6000 tasks: ask for ?only=critical",
          },
        });
      }
      return r.fulfill({
        json: {
          plan_pid: PID,
          calendar: "default",
          calendar_conflicts: [],
          project_finish: "2026-10-09",
          nodes: [node],
          edges: [],
          unscheduled: [],
          ignored_edges: [],
          critical_chains: [["c1"]],
          chains_truncated: false,
        },
      });
    }
    if (path.endsWith("/schedule-baselines")) return r.fulfill({ json: [] });
    if (path.endsWith("/schedule-variance"))
      return r.fulfill({
        json: {
          baseline: null,
          reason: "no_baseline",
          note: "none",
          variance: null,
        },
      });
    return r.fulfill({ status: 404, json: { error: "unhandled in stub" } });
  });
  await page.goto(`/en-001/plans/${PID}/critical-path`);
  await expect(page.getByTestId("cpm-too-large")).toContainText("too large");
  await expect(page.getByTestId("cpm-critical-only")).toHaveCount(0);
  await page
    .getByRole("button", { name: "Show only the critical part" })
    .click();
  await expect(page.getByTestId("cpm-critical-only")).toContainText(
    "critical and near-critical",
  );
  await expect(page.getByText("Finishes 9 Oct 2026.")).toBeVisible();
  expect(asked.at(-1)).toContain("only=critical");
});
