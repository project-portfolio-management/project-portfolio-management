// Workflow automation (CQ-24): the rules, the deadline queue and the run log, with the actions that
// change them, over a stubbed service that records what it was asked.
import { test, expect, type Page } from "@playwright/test";

const SUBJECT = "33333333-3333-4333-8333-333333333333";
const RULE = {
  pid: "u1",
  plan_pid: null,
  name: "Assign reviewer on review",
  trigger_kind: "task_moved",
  from_status: "in_progress",
  to_status: "in_review",
  actions: [{ kind: "assign", value: { assignee_ref: "person:x" } }],
  enabled: true,
};
const SCHEDULED = {
  pid: "s1",
  subject_kind: "review",
  subject_pid: SUBJECT,
  action_kind: "notify",
  payload: {},
  due_at: "2026-11-01T09:00:00Z",
  status: "pending",
  source_automation_pid: null,
  created_by: null,
  fired_at: null,
  outcome: null,
};
const RUN = {
  pid: "n1",
  automation_pid: "u1",
  subject_kind: "task",
  subject_pid: SUBJECT,
  outcome: "failed",
  detail: { reason: "no such person" },
  created_at: "2026-10-12T09:00:00Z",
};

async function stub(
  page: Page,
  data: { rules: unknown[]; scheduled: unknown[]; runs: unknown[] },
) {
  const calls: string[] = [];
  const bodies: Record<string, unknown> = {};
  await page.route("**/api/automations**", async (r) => {
    const req = r.request();
    const path = new URL(req.url()).pathname.replace(/^\/api\/proxy/, "");
    calls.push(`${req.method()} ${path}`);
    if (req.method() !== "GET" && req.method() !== "DELETE")
      bodies[path] = req.postDataJSON();
    if (req.method() === "GET") {
      const rows = path === "/api/automations/runs" ? data.runs : data.rules;
      return r.fulfill({
        json: rows,
        headers: { "x-total-count": String(rows.length) },
      });
    }
    if (req.method() === "DELETE") return r.fulfill({ status: 204, body: "" });
    return r.fulfill({ status: 201, json: RULE });
  });
  await page.route("**/api/scheduled-actions**", async (r) => {
    const req = r.request();
    const path = new URL(req.url()).pathname.replace(/^\/api\/proxy/, "");
    calls.push(`${req.method()} ${path}`);
    if (path.endsWith("/sweep"))
      return r.fulfill({
        json: { fired: 2, skipped_already_claimed: 1, capped: true, cap: 100 },
      });
    if (req.method() === "GET")
      return r.fulfill({
        json: data.scheduled,
        headers: { "x-total-count": String(data.scheduled.length) },
      });
    return r.fulfill({ status: 204, body: "" });
  });
  return { calls, bodies };
}

test.describe("automations", () => {
  test("shows the rules, the pending deadlines and every run, failures included", async ({
    page,
  }) => {
    await stub(page, { rules: [RULE], scheduled: [SCHEDULED], runs: [RUN] });
    await page.goto("/en-001/automations", { waitUntil: "networkidle" });
    await expect(
      page.getByRole("heading", { level: 1, name: "Workflow automation" }),
    ).toBeVisible();
    const rule = page.getByRole("row", { name: /Assign reviewer on review/ });
    await expect(rule).toContainText("every plan");
    await expect(rule).toContainText("task_moved (in_progress → in_review)");
    await expect(rule).toContainText("on");
    await expect(
      page.getByRole("row", { name: /review 33333333….*notify/ }),
    ).toContainText("operator");
    const run = page.getByRole("row", { name: /failed/ });
    await expect(run).toContainText("task 33333333…");
    await expect(run).toContainText("no such person");
  });

  test("says so when there is nothing yet", async ({ page }) => {
    await stub(page, { rules: [], scheduled: [], runs: [] });
    await page.goto("/en-001/automations", { waitUntil: "networkidle" });
    await expect(page.getByText("No rules configured.")).toBeVisible();
    await expect(page.getByText("Nothing pending.")).toBeVisible();
    await expect(page.getByText("Nothing has fired yet.")).toBeVisible();
  });

  test("saving a rule sends the trigger and the action's value", async ({
    page,
  }) => {
    const s = await stub(page, { rules: [], scheduled: [], runs: [] });
    await page.goto("/en-001/automations", { waitUntil: "networkidle" });
    await page.getByLabel("name", { exact: true }).fill("Label done work");
    await page.getByLabel(/^from/).selectOption("in_review");
    await page.getByLabel(/^to/).selectOption("done");
    await page.getByLabel(/^action/).selectOption("add_label");
    await page.getByLabel(/^label/).fill("shipped");
    await page.getByRole("button", { name: "Save rule" }).click();
    await expect
      .poll(() => s.bodies["/api/automations"])
      .toMatchObject({
        name: "Label done work",
        trigger_kind: "task_moved",
        from_status: "in_review",
        to_status: "done",
        actions: [{ kind: "add_label", value: { label: "shipped" } }],
      });
    await expect(
      page.getByText("Rule saved — it fires from the next matching move"),
    ).toBeVisible();
  });

  test("disable, delete and cancel call the right endpoints", async ({
    page,
  }) => {
    const s = await stub(page, {
      rules: [RULE],
      scheduled: [SCHEDULED],
      runs: [],
    });
    await page.goto("/en-001/automations", { waitUntil: "networkidle" });
    await page.getByRole("button", { name: "Disable" }).click();
    await expect
      .poll(() => s.calls)
      .toContain("POST /api/automations/u1/disable");
    await page.getByRole("button", { name: "Delete" }).click();
    await expect.poll(() => s.calls).toContain("DELETE /api/automations/u1");
    await page.getByRole("button", { name: "Cancel" }).click();
    await expect
      .poll(() => s.calls)
      .toContain("DELETE /api/scheduled-actions/s1");
  });

  test("a sweep says what it did, and when it hit its cap", async ({
    page,
  }) => {
    await stub(page, { rules: [], scheduled: [], runs: [] });
    await page.goto("/en-001/automations", { waitUntil: "networkidle" });
    await page.getByRole("button", { name: "Sweep due actions" }).click();
    await expect(
      page.getByText(
        "Swept: 2 fired, 1 already claimed (capped at 100 — run again for the rest)",
      ),
    ).toBeVisible();
  });

  test("says so when the page cannot be loaded", async ({ page }) => {
    await page.route("**/api/automations**", (r) =>
      r.fulfill({ status: 500, json: { error: "boom" } }),
    );
    await page.route("**/api/scheduled-actions**", (r) =>
      r.fulfill({ status: 500, json: { error: "boom" } }),
    );
    await page.goto("/en-001/automations", { waitUntil: "networkidle" });
    await expect(page.getByRole("alert")).toBeVisible();
  });

  test("passes axe with rules, deadlines and runs listed", async ({ page }) => {
    await stub(page, { rules: [RULE], scheduled: [SCHEDULED], runs: [RUN] });
    await page.goto("/en-001/automations", { waitUntil: "networkidle" });
    await expect(
      page.getByRole("row", { name: /Assign reviewer/ }),
    ).toBeVisible();
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
