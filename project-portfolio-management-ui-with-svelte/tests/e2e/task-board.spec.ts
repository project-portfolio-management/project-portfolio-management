// The plan's task board (/<locale>/plans/<pid>/board) on Lily's KanbanBoard, over stubbed API answers.
import { test, expect, type Page } from "@playwright/test";

const TASK = (
  pid: string,
  title: string,
  status: string,
  over: Record<string, unknown> = {},
) => ({
  pid,
  title,
  status,
  assignee_ref: "person:ada",
  points: 3,
  blocked_days: null,
  sprint_pid: null,
  ...over,
});
const TASKS = [
  TASK("t1", "Wire the API", "todo"),
  TASK("t2", "Survey the route", "in_progress", {
    points: null,
    assignee_ref: null,
  }),
  TASK("t3", "Fix the signal", "blocked", { blocked_days: 2 }),
];

async function stub(
  page: Page,
  moves: { url: string; body: string }[],
  fail = false,
) {
  await page.route(/\/api\/plans\/w-1\/tasks(\?.*)?$/, (r) =>
    r.fulfill({ json: { tasks: TASKS, truncated: false, total: 3 } }),
  );
  await page.route(/\/api\/plans\/w-1\/tasks\/[^/]+$/, (r) => {
    if (r.request().method() !== "PATCH") return r.fallback();
    moves.push({ url: r.request().url(), body: r.request().postData() ?? "" });
    return fail
      ? r.fulfill({ status: 409, json: { error: "x" } })
      : r.fulfill({ json: TASKS[0] });
  });
  await page.route("**/api/plans/w-1/sprints", (r) => r.fulfill({ json: [] }));
  await page.route("**/api/plans/w-1/standup", (r) =>
    r.fulfill({
      json: { tasks_created: [], tasks_moved: [], blocked_now: [] },
    }),
  );
  await page.route("**/api/plans/w-1/velocity", (r) =>
    r.fulfill({ json: { sprints: [], note: "" } }),
  );
}

test.describe("task board", () => {
  test("shows a card's assignee and points as fields, and says a blocked task is blocked in words", async ({
    page,
  }) => {
    await stub(page, []);
    await page.goto("/en-001/plans/w-1/board", { waitUntil: "networkidle" });
    const board = page.getByTestId("task-board");
    for (const col of ["Todo", "In progress", "In review", "Done", "Blocked"])
      await expect(
        board.getByRole("columnheader", { name: new RegExp(col) }),
      ).toBeVisible();
    await expect(board.getByText("Wire the API")).toBeVisible();
    await expect(board.getByText("person:ada").first()).toBeVisible();
    await expect(board.getByText("unassigned")).toBeVisible();
    await expect(board.getByText("blocked 2 calendar days")).toBeVisible();
  });

  test("moves a card from its menu and saves the status", async ({ page }) => {
    const moves: { url: string; body: string }[] = [];
    await stub(page, moves);
    await page.goto("/en-001/plans/w-1/board", { waitUntil: "networkidle" });
    await page.getByRole("button", { name: /^Move Wire the API/ }).click();
    await page.getByRole("option", { name: "In review" }).click();
    await expect.poll(() => moves.length).toBe(1);
    expect(moves[0]!.url).toContain("/api/plans/w-1/tasks/t1");
    expect(moves[0]!.body).toBe('{"status":"in_review"}');
  });

  test("puts the card back and says so when the service refuses the move", async ({
    page,
  }) => {
    await stub(page, [], true);
    await page.goto("/en-001/plans/w-1/board", { waitUntil: "networkidle" });
    await page.getByRole("button", { name: /^Move Wire the API/ }).click();
    await page.getByRole("option", { name: "Done" }).click();
    await expect(page.getByRole("alert")).toContainText(
      "Could not move the task",
    );
  });

  test("exports the board as CSV", async ({ page }) => {
    await stub(page, []);
    await page.goto("/en-001/plans/w-1/board", { waitUntil: "networkidle" });
    const download = page.waitForEvent("download");
    await page
      .getByTestId("task-board")
      .getByRole("button", { name: "CSV" })
      .click();
    expect((await download).suggestedFilename()).toBe("tasks.csv");
  });

  for (const locale of ["en-001", "cy-001"]) {
    test(`passes axe in ${locale}`, async ({ page }) => {
      await stub(page, []);
      await page.goto(`/${locale}/plans/w-1/board`, {
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
