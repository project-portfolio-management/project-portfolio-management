// The user's kanban board (/<locale>/kanban), on Lily's KanbanBoard, over stubbed API answers.
import { test, expect, type Page } from "@playwright/test";

const T = (
  pid: string,
  title: string,
  status: string,
  due: string | null,
  points: number | null,
) => ({
  pid,
  title,
  status,
  due_date: due,
  points,
  plan_pid: "p1",
  plan_name: "North Line",
});
const WORK = {
  tz: "UTC",
  as_of: "2026-10-13T00:00:00Z",
  today: "2026-10-13",
  buckets: {
    overdue: [
      T("t1", "Survey the cable route", "in_progress", "2026-10-01", 3),
    ],
    today: [T("t2", "Brief the drivers", "todo", "2026-10-13", 2)],
    this_week: [T("t3", "Safety assurance", "blocked", "2026-10-15", 5)],
    later: [T("t4", "Install on site", "todo", "2026-11-02", 13)],
    undated: [],
  },
  counts: { overdue: 1, today: 1, this_week: 1, later: 1, undated: 0 },
  total: 4,
  omitted_unreadable: 0,
  truncated: false,
  reason: null,
};

async function stub(page: Page, patched: { body: string; url: string }) {
  await page.route("**/api/me/tasks*", (r) => r.fulfill({ json: WORK }));
  await page.route("**/api/plans/p1/tasks/*", (r) => {
    patched.url = r.request().url();
    patched.body = r.request().postData() ?? "";
    return r.fulfill({ json: { pid: "t2", status: "in_progress" } });
  });
}

test.describe("my kanban board", () => {
  test("shows my open tasks in columns by status", async ({ page }) => {
    await stub(page, { body: "", url: "" });
    await page.goto("/en-001/kanban", { waitUntil: "networkidle" });
    await expect(
      page.getByRole("heading", { level: 1, name: "My kanban board" }),
    ).toBeVisible();
    const board = page.getByRole("grid", { name: "Your tasks by status" });
    for (const col of ["To do", "In progress", "In review", "Blocked", "Done"])
      await expect(
        board.getByRole("columnheader", { name: new RegExp(col) }),
      ).toBeVisible();
    await expect(
      board.getByText(
        "Survey the cable route · North Line · due 2026-10-01 · 3 points",
      ),
    ).toBeVisible();
    await expect(
      board.getByText("Safety assurance", { exact: false }),
    ).toBeVisible();
    await expect(
      board.getByText("Install on site", { exact: false }),
    ).toBeVisible();
  });

  test("moves a card to another column from its menu and saves the status on the task's plan", async ({
    page,
  }) => {
    const patched = { body: "", url: "" };
    await stub(page, patched);
    await page.goto("/en-001/kanban", { waitUntil: "networkidle" });
    await page.getByRole("button", { name: /^Move Brief the drivers/ }).click();
    await page.getByRole("option", { name: "In progress" }).click();
    await expect.poll(() => patched.body).toBe('{"status":"in_progress"}');
    expect(patched.url).toContain("/api/plans/p1/tasks/t2");
  });

  test("puts the card back and says so when the move fails", async ({
    page,
  }) => {
    await stub(page, { body: "", url: "" });
    await page.route("**/api/plans/p1/tasks/*", (r) =>
      r.fulfill({ status: 500, json: { error: "x" } }),
    );
    await page.goto("/en-001/kanban", { waitUntil: "networkidle" });
    await page.getByRole("button", { name: /^Move Brief the drivers/ }).click();
    await page.getByRole("option", { name: "Blocked" }).click();
    await expect(page.getByRole("alert")).toContainText(
      "Could not move the task",
    );
  });

  test("asks to sign in when there is no session", async ({ page }) => {
    await page.route("**/api/me/tasks*", (r) =>
      r.fulfill({ status: 401, json: { error: "unauthorized" } }),
    );
    await page.goto("/en-001/kanban", { waitUntil: "networkidle" });
    await expect(page.getByRole("alert")).toContainText(
      "Sign in to see your board.",
    );
  });

  test("reads in Welsh", async ({ page }) => {
    await stub(page, { body: "", url: "" });
    await page.goto("/cy-001/kanban", { waitUntil: "networkidle" });
    await expect(
      page.getByRole("heading", { level: 1, name: "Fy mwrdd kanban" }),
    ).toBeVisible();
    await expect(
      page.getByRole("columnheader", { name: /I'w wneud/ }),
    ).toBeVisible();
  });

  for (const locale of ["en-001", "cy-001", "zh-cn"]) {
    test(`passes axe in ${locale}`, async ({ page }) => {
      await stub(page, { body: "", url: "" });
      await page.goto(`/${locale}/kanban`, { waitUntil: "networkidle" });
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
