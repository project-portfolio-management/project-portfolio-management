// The RACI matrix (CQ-24): the cells, the gaps named in words, toggling a letter saves the row,
// the one-Accountable rule says so, and a new person column needs a valid reference.
import { test, expect, type Page } from "@playwright/test";

const PID = "55555555-5555-4555-8555-555555555555";
const ANN = "person:aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa";
const BOB = "person:bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb";
const MATRIX = {
  plan_pid: PID,
  people: [ANN, BOB],
  rows: [
    { task_pid: "t1", title: "Design", cells: { [ANN]: "RA" }, findings: [] },
    { task_pid: "t2", title: "Build", cells: { [BOB]: "C" }, findings: ["no_accountable", "no_responsible"] },
  ],
  summary: { tasks: 2, complete: 1, missing_accountable: 1, multiple_accountable: 0, missing_responsible: 1 },
  workload: [
    { person_ref: ANN, responsible: 1, accountable: 1, consulted: 0, informed: 0 },
    { person_ref: BOB, responsible: 0, accountable: 0, consulted: 1, informed: 0 },
  ],
};

async function stub(page: Page, matrix: unknown = MATRIX) {
  const puts: Record<string, unknown> = {};
  await page.route(`**/api/plans/${PID}/**`, async (r) => {
    const req = r.request();
    const path = new URL(req.url()).pathname.replace(/^\/api\/proxy/, "");
    if (req.method() === "PUT") {
      puts[path] = req.postDataJSON();
      return r.fulfill({ json: { task_pid: "t", cells: {}, findings: [] } });
    }
    if (path.endsWith("/raci")) return r.fulfill({ json: matrix });
    return r.fulfill({ status: 404, json: { error: "unhandled in stub" } });
  });
  return puts;
}

test.describe("raci", () => {
  test("shows who holds which letter, the gaps in words and the load by person", async ({ page }) => {
    await stub(page);
    await page.goto(`/en-001/plans/${PID}/raci`, { waitUntil: "networkidle" });
    await expect(page.getByRole("heading", { level: 1, name: "Who does what (RACI matrix)" })).toBeVisible();
    await expect(page.getByText("1 of 2 tasks are complete")).toBeVisible();
    await expect(page.getByText("1 have nobody accountable.")).toBeVisible();
    await expect(page.getByRole("group", { name: "Design: aaaaaaaa… is Responsible and Accountable" })).toBeVisible();
    const accountable = page.getByRole("group", { name: "Design: aaaaaaaa…" }).getByRole("button", { name: "Accountable" });
    await expect(accountable).toHaveAttribute("aria-pressed", "true");
    const build = page.getByRole("row", { name: /^Build/ });
    await expect(build).toContainText("Nobody is accountable");
    await expect(build).toContainText("Nobody is responsible");
    await expect(page.getByRole("row", { name: /^aaaaaaaa…\s+1\s+1\s+0\s+0/ })).toBeVisible();
    await expect(page.getByRole("link", { name: "Export to a spreadsheet" })).toHaveAttribute("href", /\/raci\?format=csv$/);
  });

  test("toggling a letter saves the whole row", async ({ page }) => {
    const puts = await stub(page);
    await page.goto(`/en-001/plans/${PID}/raci`, { waitUntil: "networkidle" });
    await page.getByRole("group", { name: "Build: bbbbbbbb… is Consulted" }).getByRole("button", { name: "Responsible" }).click();
    await expect.poll(() => puts[`/api/plans/${PID}/tasks/t2/raci`]).toEqual({ assignments: [{ person_ref: BOB, roles: "RC" }] });
  });

  test("giving Accountable to someone else says it was moved", async ({ page }) => {
    const puts = await stub(page);
    await page.goto(`/en-001/plans/${PID}/raci`, { waitUntil: "networkidle" });
    await page.getByRole("group", { name: "Design: bbbbbbbb… has no role" }).getByRole("button", { name: "Accountable" }).click();
    await expect(page.getByRole("status")).toHaveText("Accountable moved from aaaaaaaa… to bbbbbbbb…: a task has one.");
    await expect.poll(() => puts[`/api/plans/${PID}/tasks/t1/raci`]).toEqual({
      assignments: [{ person_ref: ANN, roles: "R" }, { person_ref: BOB, roles: "A" }],
    });
  });

  test("a new column needs a person: or worker: reference with a UUID", async ({ page }) => {
    await stub(page);
    await page.goto(`/en-001/plans/${PID}/raci`, { waitUntil: "networkidle" });
    await page.getByLabel("Add a person").fill("ann");
    await page.getByRole("button", { name: "Add column" }).click();
    await expect(page.getByRole("alert")).toHaveText("Use a person: or worker: reference with a UUID.");
    await page.getByLabel("Add a person").fill("worker:cccccccc-cccc-4ccc-8ccc-cccccccccccc");
    await page.getByRole("button", { name: "Add column" }).click();
    await expect(page.getByRole("columnheader", { name: "cccccccc…" })).toBeVisible();
    await expect(page.getByRole("alert")).toHaveCount(0);
  });

  test("a plan with no tasks says so", async ({ page }) => {
    await stub(page, { ...MATRIX, people: [], rows: [], workload: [], summary: { tasks: 0, complete: 0, missing_accountable: 0, multiple_accountable: 0, missing_responsible: 0 } });
    await page.goto(`/en-001/plans/${PID}/raci`, { waitUntil: "networkidle" });
    await expect(page.getByText("This plan has no tasks yet.")).toBeVisible();
  });

  test("says so when the matrix cannot be loaded", async ({ page }) => {
    await page.route(`**/api/plans/${PID}/raci`, (r) => r.fulfill({ status: 500, json: { error: "boom" } }));
    await page.goto(`/en-001/plans/${PID}/raci`, { waitUntil: "networkidle" });
    await expect(page.getByRole("alert")).toBeVisible();
  });

  test("passes axe with the matrix populated", async ({ page }) => {
    await stub(page);
    await page.goto(`/en-001/plans/${PID}/raci`, { waitUntil: "networkidle" });
    await expect(page.getByTestId("raci-matrix")).toBeVisible();
    const { default: AxeBuilder } = await import("@axe-core/playwright");
    const r = await new AxeBuilder({ page }).withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"]).analyze();
    expect(r.violations.filter((v) => v.impact === "serious" || v.impact === "critical").map((v) => `${v.id}: ${v.nodes.map((n) => n.target.join(" ")).join(" | ")}`)).toEqual([]);
  });
});
