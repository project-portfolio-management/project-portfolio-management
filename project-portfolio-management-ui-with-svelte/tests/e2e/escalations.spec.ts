// FR-54 in a real browser: raise an escalation on a plan, work it (acknowledge, take it up a level with a reason,
// resolve), see the register and the link from a red tolerance. The service is stubbed; the pages pass axe.

import { test, expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle } from "./fixtures";

async function audit(page: Page) {
  await settle(page);
  const r = await new AxeBuilder({ page }).withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"]).analyze();
  expect(r.violations.filter((v) => v.impact === "serious" || v.impact === "critical").map((v) => v.id)).toEqual([]);
}

const esc = (over: Record<string, unknown>) => ({
  pid: "e1",
  plan_pid: PID,
  title: "Cost is beyond tolerance",
  description: "The forecast is 15% over.",
  reason_kind: "tolerance",
  tolerance_dimension: "cost",
  level: 1,
  escalated_to: "Programme manager",
  owner_ref: null,
  status: "open",
  raised_by: "u",
  raised_on: "2026-10-10",
  respond_by: null,
  outcome: null,
  response: null,
  resolved_on: null,
  age_days: 10,
  overdue: false,
  ...over,
});

test("an escalation is raised, taken up a level and resolved", async ({ page }) => {
  let current = esc({});
  let live = false;
  const sent: Record<string, unknown> = {};
  const events = () => [
    { event: "raised", level: 1, actor: "u", note: null, at: "2026-10-10T09:00:00Z" },
    ...(current.level === 2 ? [{ event: "level_raised", level: 2, actor: "u", note: "No answer in a week", at: "2026-10-17T09:00:00Z" }] : []),
  ];
  await page.route(`**/api/plans/${PID}/escalations`, async (r) => {
    if (r.request().method() === "POST") {
      sent.raised = r.request().postDataJSON();
      live = true;
      return r.fulfill({ json: current });
    }
    return r.fulfill({ json: { plan_pid: PID, escalations: live ? [current] : [] } });
  });
  await page.route(`**/api/plans/${PID}/escalations/e1`, (r) => r.fulfill({ json: { ...current, events: events() } }));
  await page.route(`**/api/plans/${PID}/escalations/e1/raise-level`, (r) => {
    sent.up = r.request().postDataJSON();
    current = esc({ level: 2, escalated_to: "Sponsor" });
    return r.fulfill({ json: current });
  });
  await page.route(`**/api/plans/${PID}/escalations/e1/resolve`, (r) => {
    sent.resolved = r.request().postDataJSON();
    current = esc({ level: 2, escalated_to: "Sponsor", status: "resolved", outcome: "decided", response: "Funded from contingency", resolved_on: "2026-10-20", age_days: 10 });
    return r.fulfill({ json: current });
  });

  await page.goto(`/en-001/plans/${PID}/escalations?reason=tolerance&dimension=cost`);
  await expect(page.getByText("No escalations yet.")).toBeVisible();
  // The link from a red tolerance pre-fills a tolerance escalation about cost.
  await expect(page.getByLabel("Which tolerance")).toHaveValue("cost");

  // A draft with no title is refused here, in words, before anything is sent.
  await page.getByRole("button", { name: "Raise it" }).click();
  await expect(page.getByRole("alert")).toContainText("Give it a title");
  expect(sent.raised).toBeUndefined();
  await page.getByLabel("Title").fill("Cost is beyond tolerance");
  await page.getByRole("button", { name: "Raise it" }).click();
  expect(sent.raised).toEqual({ title: "Cost is beyond tolerance", reason_kind: "tolerance", tolerance_dimension: "cost" });
  await expect(page.getByRole("status")).toContainText("Raised Cost is beyond tolerance.");
  await expect(page.getByText("open", { exact: false }).first()).toBeVisible();

  await page.getByRole("button", { name: "Show Cost is beyond tolerance" }).click();
  const region = page.getByRole("region", { name: "Show Cost is beyond tolerance" });
  await expect(region).toContainText("The forecast is 15% over.");
  await expect(region).toContainText("Raised, Level 1");
  await audit(page);

  // Taking it up needs a reason of ten characters.
  await region.getByRole("button", { name: "Take it up a level" }).click();
  await expect(page.getByRole("status")).toContainText("Write at least 10 characters.");
  expect(sent.up).toBeUndefined();
  await region.getByLabel("Why it is going up").fill("No answer in a week");
  await region.getByRole("button", { name: "Take it up a level" }).click();
  expect(sent.up).toEqual({ note: "No answer in a week" });
  await expect(page.getByText("Level 2: Sponsor")).toBeVisible();
  await expect(region).toContainText("Taken up a level");

  await region.getByLabel("Response").fill("Funded from contingency");
  await region.getByRole("button", { name: "Resolve" }).click();
  expect(sent.resolved).toEqual({ outcome: "decided", response: "Funded from contingency" });
  await expect(region.getByRole("button", { name: "Resolve" })).toHaveCount(0);
  await expect(region).toContainText("Decided: Funded from contingency");
});

test("the register shows state as words and shapes and flags the overdue", async ({ page }) => {
  const rows = [
    esc({ pid: "a", title: "Vendor slips", plan_name: "Apollo", overdue: true, age_days: 21 }),
    esc({ pid: "b", title: "Scope change", plan_name: "Gemini", status: "acknowledged", level: 2, escalated_to: "Sponsor" }),
  ];
  await page.route("**/api/escalations**", (r) => {
    const url = new URL(r.request().url());
    const onlyOverdue = url.searchParams.get("overdue") === "true";
    const list = rows.filter((x) => !onlyOverdue || x.overdue);
    return r.fulfill({ json: { escalations: list, counts: { open: 1, acknowledged: 1 }, overdue: 1 } });
  });
  await page.goto("/en-001/escalations");
  const body = page.getByRole("row");
  await expect(body.nth(1)).toContainText("Vendor slips");
  await expect(body.nth(1)).toContainText("overdue");
  await expect(body.nth(2)).toContainText("acknowledged");
  await expect(page.getByRole("link", { name: "Vendor slips" })).toHaveAttribute("href", /\/plans\/.*\/escalations$/);
  await audit(page);
  await page.getByLabel("Show").selectOption("overdue");
  await expect(page.getByRole("link", { name: "Scope change" })).toHaveCount(0);
});

test("a red tolerance with no escalation offers to raise one", async ({ page }) => {
  const dimension = (name: string, over: Record<string, unknown>) => ({
    dimension: name, unit: "u", basis: `How ${name} is measured.`, status: "not_set", limit: null, source: "none", ...over,
  });
  await page.route(`**/api/plans/${PID}/tolerances`, (r) =>
    r.fulfill({
      json: {
        plan_pid: PID, plan_name: "Apollo", overall: "red", escalation_needed: true, as_of: "2026-10-20T10:00:00Z", how_to_read: "x",
        dimensions: [
          dimension("time", { status: "red", limit: 5, measured: 9, usage_percent: 180, source: "own", amber_from_percent: 75, open_escalations: 0, escalation_needed: true }),
          dimension("cost", { status: "red", limit: 500, measured: 900, usage_percent: 180, source: "own", amber_from_percent: 75, open_escalations: 2, escalation_needed: false }),
          dimension("risk", {}), dimension("resources", {}),
        ],
      },
    }),
  );
  await page.goto(`/en-001/plans/${PID}/tolerances`);
  const time = page.getByRole("region", { name: /Time/ });
  await expect(time).toContainText("nobody has escalated it");
  await expect(time.getByRole("link", { name: "Raise an escalation" })).toHaveAttribute("href", /escalations\?reason=tolerance&dimension=time$/);
  const cost = page.getByRole("region", { name: /Cost/ });
  await expect(cost).toContainText("2 open escalations");
  await expect(cost).not.toContainText("nobody has escalated it");
  await audit(page);
});
