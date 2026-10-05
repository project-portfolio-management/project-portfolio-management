// PJ-9 in a real browser: stage an edit, preview it (nothing is saved), commit
// exactly that with the preview's etag, see a stale plan refused, or discard.

import { test, expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { PID, primaryButtons } from "./fixtures";

async function audit(page: Page) {
  const r = await new AxeBuilder({ page }).withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa"]).analyze();
  expect(r.violations.filter((v) => v.impact === "serious" || v.impact === "critical").map((v) => v.id)).toEqual([]);
}

const node = (pid: string, title: string) => ({
  pid, title, status: null, early_start: "2026-10-05", early_finish: "2026-10-07", late_start: "2026-10-05",
  late_finish: "2026-10-07", total_float: 0, free_float: 0, critical: true, near_critical: false,
  planned_start: "2026-10-05", duration_days: 3, pushed_days: 0, constraint: null,
});
const CPM = {
  plan_pid: PID, calendar: "plan", calendar_conflicts: [], project_finish: "2026-10-07",
  nodes: [node("t1", "Build"), node("t2", "Test")],
  edges: [{ pid: "l1", predecessor_pid: "t1", successor_pid: "t2", kind: "fs", lag_days: 0 }],
  unscheduled: [], ignored_edges: [], critical_chains: [], chains_truncated: false,
};
const dates = (s: string, f: string, fl: number) => ({ early_start: s, early_finish: f, late_finish: f, total_float: fl, critical: true });
const PREVIEW = {
  plan_pid: PID, plan_etag: 'W/"abc"', result_etag: 'W/"def"', changes: [{ item: "task", pid: "t1" }],
  finish: { before: "2026-10-07", after: "2026-10-12" },
  tasks: [{ pid: "t1", title: "Build", before: dates("2026-10-05", "2026-10-07", 0), after: dates("2026-10-05", "2026-10-12", 0) }],
  moved: [], critical_path: { before: ["t1"], after: ["t1"], added: [], removed: [] },
  assignments_recomputed: [{ task_pid: "t1", derived: "work", exact: true, units_percent: 100, work_minutes: 2880 }],
  cost_impact: { available: true, labour: [{ currency: "GBP", before_minor: 120000, after_minor: 240000, delta_minor: 120000 }], unrated_minutes_before: 0, unrated_minutes_after: 0 },
  masked: false, note: "",
};

async function stub(page: Page, commit: (route: import("@playwright/test").Route) => Promise<void>) {
  await page.route(`**/api/plans/${PID}/cpm**`, (r) => r.fulfill({ json: CPM }));
  await page.route(`**/api/plans/${PID}/sandbox/preview`, (r) => r.fulfill({ json: PREVIEW }));
  await page.route(`**/api/plans/${PID}/sandbox/commit`, commit);
}

async function stageDuration(page: Page) {
  await page.getByRole("combobox", { name: /^Task/ }).selectOption("t1");
  await page.getByLabel("Working days").fill("6");
  await page.getByRole("button", { name: "Stage edit" }).click();
}

test("stage, preview, then commit sends exactly the staged edits with the preview's etag", async ({ page }) => {
  let sent: { headers: Record<string, string>; body: unknown } | null = null;
  await stub(page, async (r) => {
    sent = { headers: r.request().headers(), body: r.request().postDataJSON() };
    await r.fulfill({ json: { committed: true, audit_entries: 1, finish: { before: "2026-10-07", after: "2026-10-12" } } });
  });
  await page.goto(`/en-001/plans/${PID}/what-if`);
  await stageDuration(page);
  await expect(page.getByTestId("staged")).toContainText("Set “Build” to 6 working days");
  await expect(page.getByTestId("preview")).toHaveCount(0);
  await expect.poll(() => primaryButtons(page), { message: "with edits staged, previewing is the primary action" }).toEqual(["Preview"]);
  await page.getByRole("button", { name: "Preview" }).click();
  const pv = page.getByTestId("preview");
  await expect(pv).toContainText("The finish moves from 2026-10-07 to 2026-10-12.");
  await expect(pv).toContainText("+1,200.00 GBP");
  await expect(pv).toContainText("100% of the day for 48 h of work");
  await expect.poll(() => primaryButtons(page), { message: "once previewed, committing is the primary action" }).toEqual(["Commit these changes"]);
  await audit(page);
  await page.getByLabel("Reason").fill("try it");
  await page.getByRole("button", { name: "Commit these changes" }).click();
  await expect(page.getByRole("status")).toContainText("Committed: 1 change recorded.");
  expect(sent!.headers["if-match"]).toBe('W/"abc"');
  expect(sent!.body).toEqual({ edits: [{ op: "duration", task_pid: "t1", duration_days: 6 }], reason: "try it" });
  await expect(page.getByTestId("staged")).toHaveCount(0);
});

test("a changed staged list drops the preview, so you never commit what you did not see", async ({ page }) => {
  await stub(page, (r) => r.fulfill({ json: {} }));
  await page.goto(`/en-001/plans/${PID}/what-if`);
  await stageDuration(page);
  await page.getByRole("button", { name: "Preview" }).click();
  await expect(page.getByTestId("preview")).toBeVisible();
  await page.getByRole("combobox", { name: /^Task/ }).selectOption("t2");
  await page.getByLabel("Working days").fill("1");
  await page.getByRole("button", { name: "Stage edit" }).click();
  await expect(page.getByTestId("preview")).toHaveCount(0);
  await page.getByRole("button", { name: "Remove edit 2" }).click();
  await expect(page.getByTestId("staged").locator("li")).toHaveCount(1);
});

test("a stale plan is refused with a plain message and the preview is dropped", async ({ page }) => {
  await stub(page, (r) => r.fulfill({ status: 409, json: { error: "stale_preview", description: "x" } }));
  await page.goto(`/en-001/plans/${PID}/what-if`);
  await stageDuration(page);
  await page.getByRole("button", { name: "Preview" }).click();
  await page.getByRole("button", { name: "Commit these changes" }).click();
  await expect(page.getByRole("status")).toContainText("The plan changed since this preview, so nothing was applied.");
  await expect(page.getByTestId("preview")).toHaveCount(0);
  await expect(page.getByTestId("staged")).toBeVisible(); // the edits are kept, to preview again
});

test("discard clears everything and sends nothing", async ({ page }) => {
  let commits = 0;
  await stub(page, async (r) => { commits++; await r.fulfill({ json: {} }); });
  await page.goto(`/en-001/plans/${PID}/what-if`);
  await stageDuration(page);
  await page.getByRole("button", { name: "Preview" }).click();
  await page.getByRole("button", { name: "Discard" }).first().click();
  await expect(page.getByRole("status")).toContainText("Nothing was changed.");
  await expect(page.getByTestId("staged")).toHaveCount(0);
  await expect(page.getByTestId("preview")).toHaveCount(0);
  expect(commits).toBe(0);
});

test("a bad edit is stopped in the form with a sentence", async ({ page }) => {
  await stub(page, (r) => r.fulfill({ json: {} }));
  await page.goto(`/en-001/plans/${PID}/what-if`);
  await page.getByRole("button", { name: "Stage edit" }).click();
  await expect(page.getByRole("status")).toContainText("Choose a task.");
  await audit(page);
});
