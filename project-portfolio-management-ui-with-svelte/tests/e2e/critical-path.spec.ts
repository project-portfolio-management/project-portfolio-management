// The critical path (CQ-24): the schedule as a table with its float in words, the chains, a task
// left unscheduled and why, the editor and what it says moved, and the baselines note.
import { test, expect, type Page } from "@playwright/test";

const PID = "66666666-6666-4666-8666-666666666666";
const node = (pid: string, title: string, over: Record<string, unknown>) => ({
  pid,
  title,
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
  ...over,
});
const CPM = {
  plan_pid: PID,
  calendar: "default",
  calendar_conflicts: [],
  project_finish: "2026-10-16",
  nodes: [
    node("c1", "Design", {}),
    node("c2", "Build", {
      early_start: "2026-10-12",
      early_finish: "2026-10-16",
      late_start: "2026-10-12",
      late_finish: "2026-10-16",
    }),
    node("c3", "Write docs", {
      critical: false,
      total_float: 6,
      free_float: 6,
      early_start: "2026-10-05",
      early_finish: "2026-10-07",
      late_start: "2026-10-13",
      late_finish: "2026-10-15",
    }),
  ],
  edges: [
    {
      pid: "e1",
      predecessor_pid: "c1",
      successor_pid: "c2",
      kind: "fs",
      lag_days: 0,
    },
  ],
  unscheduled: [{ pid: "c4", title: "Train staff", reason: "no_duration" }],
  ignored_edges: [],
  critical_chains: [["c1", "c2"]],
  chains_truncated: false,
};

async function stub(page: Page, cpm: unknown = CPM) {
  const puts: Record<string, unknown> = {};
  await page.route(`**/api/plans/${PID}/**`, async (r) => {
    const req = r.request();
    const path = new URL(req.url()).pathname.replace(/^\/api\/proxy/, "");
    if (req.method() === "PUT") {
      puts[path] = req.postDataJSON();
      return r.fulfill({
        json: {
          task: { pid: "c4" },
          moved: [{ pid: "c2", moved: "both", start_days: 2, finish_days: 2 }],
          note: null,
        },
      });
    }
    if (path.endsWith("/cpm")) return r.fulfill({ json: cpm });
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
  return puts;
}

test.describe("critical path", () => {
  test("the table says in words which tasks are critical and how much float the others have", async ({
    page,
  }) => {
    await stub(page);
    await page.goto(`/en-001/plans/${PID}/critical-path`, {
      waitUntil: "networkidle",
    });
    await expect(
      page.getByRole("heading", { level: 1, name: "Critical path" }),
    ).toBeVisible();
    await expect(page.getByText("Finishes 16 Oct 2026.")).toBeVisible();
    await page.getByRole("button", { name: "Table", exact: true }).click();
    await expect(page.getByRole("row", { name: /^Design/ })).toContainText(
      "On the critical path: any slip moves the finish",
    );
    await expect(
      page.getByRole("row", { name: /^Write docs/ }),
    ).not.toContainText("On the critical path");
    await expect(
      page.getByRole("row", {
        name: /^Write docs\s+5 Oct 2026\s+7 Oct 2026\s+6\s+6/,
      }),
    ).toBeVisible();
    await expect(
      page.getByRole("heading", { name: "Critical chains" }),
    ).toBeVisible();
    await expect(page.getByText("Design → Build")).toBeVisible();
    await expect(
      page.getByText("No baseline yet, so nothing can be said about slip."),
    ).toBeVisible();
  });

  test("a task that cannot be scheduled is listed with the reason, and opens the editor", async ({
    page,
  }) => {
    const puts = await stub(page);
    await page.goto(`/en-001/plans/${PID}/critical-path`, {
      waitUntil: "networkidle",
    });
    await expect(
      page.getByRole("heading", { name: "Not scheduled" }),
    ).toBeVisible();
    await page.getByRole("button", { name: "Train staff" }).click();
    await expect(
      page.getByRole("heading", { name: "Train staff" }),
    ).toBeVisible();
    await page.getByLabel(/^Planned start/).fill("2026-10-19");
    await page.getByLabel(/^Duration/).fill("3");
    await page.getByRole("button", { name: "Save schedule" }).click();
    await expect
      .poll(() => puts[`/api/plans/${PID}/tasks/c4/schedule`])
      .toMatchObject({
        start_date: "2026-10-19",
        duration_days: 3,
        constraint_kind: null,
        deadline: null,
      });
    await expect(page.getByTestId("moved")).toContainText("Saved. Moved:");
    await expect(page.getByTestId("moved")).toContainText("Build");
  });

  test("a hard constraint will not save without its date and a reason", async ({
    page,
  }) => {
    const puts = await stub(page);
    await page.goto(`/en-001/plans/${PID}/critical-path`, {
      waitUntil: "networkidle",
    });
    await page.getByRole("button", { name: "Train staff" }).click();
    await page.getByLabel(/^Constraint(?! date)/).selectOption("mso");
    await expect(page.getByLabel(/^Constraint date/)).toBeVisible();
    await expect(page.getByLabel(/^Reason/)).toBeVisible();
    await page.getByLabel(/^Constraint date/).fill("2026-10-20");
    await page.getByRole("button", { name: "Save schedule" }).click();
    expect(Object.keys(puts)).toHaveLength(0);
    await page.getByLabel(/^Reason/).fill("Contract date");
    await page.getByRole("button", { name: "Save schedule" }).click();
    await expect
      .poll(() => puts[`/api/plans/${PID}/tasks/c4/schedule`])
      .toMatchObject({
        constraint_kind: "mso",
        constraint_date: "2026-10-20",
        reason: "Contract date",
      });
  });

  test("an estimate range with the optimistic above the likely is refused before it is sent", async ({
    page,
  }) => {
    const puts = await stub(page);
    await page.goto(`/en-001/plans/${PID}/critical-path`, {
      waitUntil: "networkidle",
    });
    await page.getByRole("button", { name: "Train staff" }).click();
    await page.getByLabel(/^Optimistic/).fill("9");
    await page.getByLabel(/^Pessimistic/).fill("2");
    await page.getByRole("button", { name: "Save range" }).click();
    await expect(page.getByRole("alert").first()).toBeVisible();
    expect(Object.keys(puts)).toHaveLength(0);
  });

  test("says when nothing is scheduled, and when the schedule cannot be loaded", async ({
    page,
  }) => {
    await stub(page, {
      ...CPM,
      nodes: [],
      edges: [],
      unscheduled: [],
      critical_chains: [],
      project_finish: null,
    });
    await page.goto(`/en-001/plans/${PID}/critical-path`, {
      waitUntil: "networkidle",
    });
    await expect(page.getByText("Nothing is scheduled yet.")).toBeVisible();
    await page.unrouteAll();
    await page.route(`**/api/plans/${PID}/**`, (r) =>
      r.fulfill({ status: 500, json: { error: "boom" } }),
    );
    await page.goto(`/en-001/plans/${PID}/critical-path`, {
      waitUntil: "networkidle",
    });
    await expect(page.getByRole("alert")).toBeVisible();
  });

  test("passes axe with the schedule shown as a table", async ({ page }) => {
    await stub(page);
    await page.goto(`/en-001/plans/${PID}/critical-path`, {
      waitUntil: "networkidle",
    });
    await page.getByRole("button", { name: "Table", exact: true }).click();
    await expect(page.getByTestId("cpm-table")).toBeVisible();
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
