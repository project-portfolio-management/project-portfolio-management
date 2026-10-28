// CQ-19: a page that reads something best-effort stays quiet only when the answer is "not there
// for you" (404, 403); any other failure is said, so a fault never looks like an empty answer.

import { test, expect, type Page } from "@playwright/test";

const PID = "44444444-4444-4444-8444-444444444444";
const fail = {
  status: 500,
  json: { error: "internal_server_error", description: "boom" },
};
const absent = { status: 404, json: { error: "not_found" } };

async function wbs(page: Page, counts: { status: number; json: unknown }) {
  await page.route(`**/api/plans/${PID}/tasks`, (r) =>
    r.fulfill({
      json: {
        counts: {},
        tasks: [
          {
            pid: "t1",
            title: "Design",
            description: null,
            status: "todo",
            assignee_ref: null,
            points: null,
            due_date: null,
            sprint_pid: null,
            created_at: "2026-10-01T00:00:00Z",
            parent_task_pid: null,
            position: 1,
          },
        ],
      },
    }),
  );
  await page.route(`**/api/plans/${PID}/task-link-counts`, (r) =>
    r.fulfill(counts),
  );
  await page.goto(`/en-001/plans/${PID}/wbs`, { waitUntil: "networkidle" });
}

test("the WBS says when link counts could not be loaded, and not when there are none", async ({
  page,
}) => {
  await wbs(page, fail);
  await expect(page.getByTestId("link-counts-failed")).toContainText(
    "could not be loaded",
  );
  await expect(
    page
      .getByRole("list", { name: "Work breakdown tree" })
      .getByRole("link", { name: "Design" }),
  ).toBeVisible();
  await page.unrouteAll();
  await wbs(page, absent);
  await expect(page.getByTestId("link-counts-failed")).toHaveCount(0);
});

async function planPage(
  page: Page,
  modules: { status: number; json: unknown },
) {
  await page.route("**/api/**", (r) => {
    const path = new URL(r.request().url()).pathname.replace(
      /^\/api\/proxy/,
      "",
    );
    if (path === `/api/plans/${PID}`)
      return r.fulfill({
        json: {
          pid: PID,
          kind: "Project",
          name: "Signal upgrade",
          in_language: ["en"],
        },
      });
    if (path === `/api/plans/${PID}/modules`) return r.fulfill(modules);
    return r.fulfill(absent);
  });
  await page.goto(`/en-001/plans/${PID}`, { waitUntil: "networkidle" });
}

test("the plan page says when it could not check the plan's tools", async ({
  page,
}) => {
  await planPage(page, fail);
  await expect(page.getByTestId("modules-failed")).toContainText(
    "every tool is listed",
  );
  await page.unrouteAll();
  await planPage(page, absent);
  await expect(page.getByRole("heading", { level: 1 })).toHaveText(
    "Signal upgrade",
  );
  await expect(page.getByTestId("modules-failed")).toHaveCount(0);
});

const SCHEDULE = {
  portfolio_pid: "p1",
  items: [
    {
      pid: "a",
      kind: "project",
      name: "Design",
      stage: null,
      start: "2026-10-05",
      end: "2026-10-09",
      on_critical_path: true,
    },
  ],
  edges: [],
  violations: [],
  critical_path: ["a"],
  undated: [],
};

async function gantt(page: Page, calendar: { status: number; json: unknown }) {
  await page.route("**/api/plans/p1/schedule", (r) =>
    r.fulfill({ json: SCHEDULE }),
  );
  await page.route("**/api/plans/p1/calendar", (r) => r.fulfill(calendar));
  await page.route(/\/api\/plans(\?.*)?$/, (r) =>
    r.fulfill({ json: [{ pid: "p1", name: "North Line", kind: "portfolio" }] }),
  );
  await page.goto("/en-001/gantt", { waitUntil: "networkidle" });
}

test("the Gantt chart says when the working calendar could not be loaded, and still draws", async ({
  page,
}) => {
  await gantt(page, fail);
  await expect(page.getByTestId("calendar-failed")).toContainText("not shaded");
  await expect(
    page
      .getByTestId("ppm-gantt")
      .getByRole("rowheader", { name: "Design (critical path)" }),
  ).toBeVisible();
  await page.unrouteAll();
  await gantt(page, absent);
  await expect(page.getByTestId("calendar-failed")).toHaveCount(0);
});

const FEED = {
  as_of: "2026-10-05T12:00:00Z",
  total: 1,
  truncated: false,
  announcements: [
    {
      pid: "a0",
      title: "Public notice",
      body: "Body",
      severity: "info",
      pinned: false,
      publish_at: "2026-10-05T10:00:00Z",
      expires_at: null,
      author_ref: null,
      state: "live",
    },
  ],
};

async function announcements(
  page: Page,
  all: { status: number; json: unknown },
) {
  await page.route("**/api/announcements**", (r) =>
    new URL(r.request().url()).searchParams.get("include") === "all"
      ? r.fulfill(all)
      : r.fulfill({ json: FEED }),
  );
  await page.goto("/en-001/announcements", { waitUntil: "networkidle" });
}

test("announcements fall back to the live feed only when the full list is refused", async ({
  page,
}) => {
  await announcements(page, { status: 403, json: { error: "forbidden" } });
  await expect(page.getByText("Public notice")).toBeVisible();
  await expect(page.getByRole("alert")).toHaveCount(0);
  await page.unrouteAll();
  await announcements(page, fail);
  await expect(page.getByRole("alert")).toBeVisible();
  await expect(page.getByText("Public notice")).toHaveCount(0);
});
