// The cutover go/no-go screen (EX-6), over stubbed API answers.
import { test, expect } from "@playwright/test";

const PID = "11111111-1111-4111-8111-111111111111";
const OPEN = {
  pid: "d1", plan_pid: PID, title: "Signalling cutover", version: 2, supersedes_pid: null, status: "open",
  published_on: "2026-10-01", opened_on: "2026-10-05", outcome: null, decided_on: null, decision_reason: null,
  criteria: [
    { n: 1, criterion: "Rollback rehearsed twice", met: true, evidence: "Rehearsal log 4 Oct", marked_on: "2026-10-05" },
    { n: 2, criterion: "Safety case signed off", met: false, evidence: null, marked_on: null },
  ],
  cosignatures: { required: ["safety", "operations"], signed: [{ role: "safety", signed_on: "2026-10-05" }], missing: ["operations"] },
  go_would_be_refused_because: ["criterion 2 is not met", "operations has not co-signed"],
};
const DECIDED = { ...OPEN, criteria: [{ n: 1, criterion: "Rollback ready", met: false, evidence: null, marked_on: null }], pid: "d0", title: "Earlier cutover", version: 1, status: "decided", outcome: "delay_for_safety", decided_on: "2026-09-20T10:00:00Z", decision_reason: "Rollback not ready", go_would_be_refused_because: [] };

test.describe("cutover go/no-go", () => {
  let marked = "";
  let decided = "";
  let cosigned = "";
  let drafted = "";
  test.beforeEach(async ({ page }) => {
    marked = decided = cosigned = drafted = "";
    await page.route(`**/api/plans/${PID}/cutover-decisions`, (r) => {
      if (r.request().method() === "POST") { drafted = r.request().postData() ?? ""; return r.fulfill({ json: {} }); }
      return r.fulfill({ json: { decisions: [OPEN, DECIDED], required_cosigners: ["safety", "operations"] } });
    });
    await page.route(`**/api/plans/${PID}/cutover-decisions/d1/criteria/2/mark`, (r) => { marked = r.request().postData() ?? ""; return r.fulfill({ json: {} }); });
    await page.route(`**/api/plans/${PID}/cutover-decisions/d1/cosign`, (r) => { cosigned = r.request().postData() ?? ""; return r.fulfill({ json: {} }); });
    await page.route(`**/api/plans/${PID}/cutover-decisions/d1/decide`, (r) => { decided = r.request().postData() ?? ""; return r.fulfill({ json: {} }); });
  });

  test("shows criteria, evidence, co-signatures and why go would be refused, and takes the decision", async ({ page }) => {
    await page.goto(`/en-001/plans/${PID}/cutover`, { waitUntil: "networkidle" });
    await expect(page.getByRole("heading", { level: 1, name: "Cutover go/no-go" })).toBeVisible();
    await expect(page.getByRole("row", { name: /Rollback rehearsed twice.*met.*Rehearsal log 4 Oct/ })).toContainText("●");
    await expect(page.getByRole("row", { name: /Safety case signed off.*not met/ })).toContainText("✖");
    await expect(page.getByText("Signed: safety. Still needed: operations.").first()).toBeVisible();
    await expect(page.getByText("operations has not co-signed").first()).toBeVisible();
    await expect(page.getByText("Decided on 2026-09-20: delay for safety. Rollback not ready")).toBeVisible();
    await page.getByRole("row", { name: /Safety case signed off/ }).getByLabel("Evidence").fill("Signed 6 Oct");
    await page.getByRole("button", { name: "Mark met: 2" }).click();
    await expect.poll(() => marked).toBe('{"met":true,"evidence":"Signed 6 Oct"}');
    await page.getByLabel("Role to sign as").fill("operations");
    await page.getByRole("button", { name: "Co-sign" }).click();
    await expect.poll(() => cosigned).toBe('{"role":"operations"}');
    await page.getByLabel("Outcome").selectOption("no_go");
    await page.getByLabel("Reason", { exact: true }).fill("Criterion 2 not met");
    await page.getByRole("button", { name: "Record the decision" }).click();
    await expect.poll(() => decided).toBe('{"outcome":"no_go","reason":"Criterion 2 not met"}');
  });

  test("drafts a decision from lines of criteria", async ({ page }) => {
    await page.goto(`/en-001/plans/${PID}/cutover`, { waitUntil: "networkidle" });
    await page.getByLabel("Title").fill("New cutover");
    await page.getByLabel(/Stop criteria, one on each line/).fill("Rollback rehearsed\nSafety case signed");
    await page.getByRole("button", { name: "Draft the decision" }).click();
    await expect.poll(() => drafted).toBe('{"title":"New cutover","criteria":["Rollback rehearsed","Safety case signed"]}');
  });

  test("reads in Welsh", async ({ page }) => {
    await page.goto(`/cy-001/plans/${PID}/cutover`, { waitUntil: "networkidle" });
    await expect(page.getByRole("heading", { level: 1, name: /Mynd\/peidio mynd/ })).toBeVisible();
  });

  for (const locale of ["en-001", "cy-001", "zh-cn"]) {
    test(`passes axe in ${locale}`, async ({ page }) => {
      await page.goto(`/${locale}/plans/${PID}/cutover`, { waitUntil: "networkidle" });
      const { default: AxeBuilder } = await import("@axe-core/playwright");
      const r = await new AxeBuilder({ page }).withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"]).analyze();
      expect(r.violations.filter((v) => v.impact === "serious" || v.impact === "critical").map((v) => `${v.id}: ${v.nodes.map((n) => n.target.join(" ")).join(" | ")}`)).toEqual([]);
    });
  }
});
