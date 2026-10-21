// FR-52 and LT-8 in a real browser: a plan's ways of working and governance framework (in force, followed, unfollowed,
// written for the plan) and its links to external tools. The service is stubbed; the pages pass the axe audit.

import { test, expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle } from "./fixtures";

async function audit(page: Page) {
  await settle(page);
  const r = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(r.violations.filter((v) => v.impact === "serious" || v.impact === "critical").map((v) => v.id + ": " + v.nodes.map((n) => n.target.join(" ") + " " + (n.any[0]?.message ?? "")).join(" | "))).toEqual([]);
}

const scrum = { pid: "w1", name: "Scrum, two weeks", description: null, scope: "organization", owner_plan_pid: null, plans_using: 2, methodology: "scrum", cadence_days: 14 };
const kanban = { pid: "w2", name: "Kanban flow", description: null, scope: "organization", owner_plan_pid: null, plans_using: 0, methodology: "kanban", wip_limit: 4 };

test("a plan can follow a shared way of working and stop following it", async ({ page }) => {
  let followed: unknown = null;
  let sent: unknown = "unset";
  await page.route("**/api/ways-of-working", (r) => r.fulfill({ json: [scrum, kanban] }));
  await page.route(`**/api/plans/${PID}/ways-of-working`, (r) => {
    if (r.request().method() === "PUT") {
      sent = r.request().postDataJSON();
      followed = (sent as { framework_pid: string | null }).framework_pid === "w2" ? kanban : null;
      return r.fulfill({ json: { plan_pid: PID, source: followed ? "own" : "inherited", from_plan_pid: followed ? null : "p-prog", ways_of_working: followed ?? scrum, reason: null } });
    }
    return r.fulfill({ json: { plan_pid: PID, source: "inherited", from_plan_pid: "p-prog", ways_of_working: scrum, reason: null } });
  });
  await page.goto(`/en-001/plans/${PID}/ways-of-working`);
  await expect(page.getByRole("heading", { name: "Ways of working" })).toBeVisible();
  await expect(page.getByText("Scrum, two weeks")).toBeVisible();
  await expect(page.getByText("inherited from a plan above")).toBeVisible();
  await expect(page.getByText("14 days")).toBeVisible();
  await audit(page);

  await page.getByLabel("Shared or existing").selectOption("w2");
  await page.getByRole("button", { name: "Follow this" }).click();
  expect(sent).toEqual({ framework_pid: "w2" });
  await expect(page.getByRole("status")).toContainText("Now following it.");
  await expect(page.getByText("set on this plan")).toBeVisible();

  await page.getByRole("button", { name: "Stop following (inherit again)" }).click();
  expect(sent).toEqual({ framework_pid: null });
  await expect(page.getByText("inherited from a plan above")).toBeVisible();
});

test("a governance framework is written for the plan with an escalation path, and a bad path is refused in words", async ({ page }) => {
  let created: Record<string, unknown> | null = null;
  const written = () => ({ pid: "g1", name: "Board governance", description: null, scope: "plan", owner_plan_pid: PID, plans_using: 1, decision_model: "steering_committee", ...created });
  await page.route("**/api/governance-frameworks", (r) => {
    if (r.request().method() === "POST") {
      created = r.request().postDataJSON();
      return r.fulfill({ json: written() });
    }
    return r.fulfill({ json: created ? [written()] : [] });
  });
  await page.route(`**/api/plans/${PID}/governance-framework`, (r) =>
    r.fulfill({ json: created ? { plan_pid: PID, source: "own", from_plan_pid: null, governance_framework: written(), reason: null } : { plan_pid: PID, source: "none", from_plan_pid: null, governance_framework: null, reason: "neither this plan nor any plan above it declares a governance framework" } }),
  );
  await page.goto(`/en-001/plans/${PID}/governance-framework`);
  await expect(page.getByText("neither this plan nor any plan above it declares")).toBeVisible();
  await page.getByRole("button", { name: "Write a new one for this plan" }).click();
  await page.getByLabel("Name").fill("Board governance");
  await page.getByRole("button", { name: "Add a level" }).click();
  await page.getByLabel("Who hears it").fill("Sponsor");
  await page.getByRole("button", { name: "Save" }).click();
  await expect(page.getByRole("alert")).toContainText("Each escalation level");
  expect(created).toBeNull();
  await page.getByLabel("What sends it there").fill("A red tolerance");
  await page.getByRole("button", { name: "Save" }).click();
  await expect(page.getByRole("status")).toContainText("Saved.");
  expect(created).toMatchObject({ name: "Board governance", plan_pid: PID, escalation_path: [{ level: 1, body: "Sponsor", trigger: "A red tolerance" }] });
  await expect(page.getByText("1: Sponsor")).toBeVisible();
  await audit(page);
});

test("links to Jira and Trello are added, kept on their own domain, and removed", async ({ page }) => {
  let links: unknown[] = [];
  let sent: unknown = null;
  await page.route(`**/api/plans/${PID}/links`, (r) => {
    if (r.request().method() === "POST") {
      sent = r.request().postDataJSON();
      links = [{ pid: "l1", ...(sent as object) }];
      return r.fulfill({ json: links[0] });
    }
    return r.fulfill({ json: links });
  });
  await page.route(`**/api/plans/${PID}/links/l1/delete`, (r) => {
    links = [];
    return r.fulfill({ json: {} });
  });
  await page.goto(`/en-001/plans/${PID}/links`);
  await expect(page.getByText("No links yet.")).toBeVisible();
  await page.getByLabel("Label").fill("Delivery board");
  await page.getByLabel("Address (http or https)").fill("https://eviltrello.com/b/abc");
  await page.getByLabel("Kind").selectOption("tracker");
  await page.getByLabel("External tool").selectOption("trello");
  await page.getByRole("button", { name: "Add the link" }).click();
  await expect(page.getByRole("alert")).toContainText("trello.com");
  expect(sent).toBeNull();

  await page.getByLabel("Address (http or https)").fill("https://trello.com/b/abc/delivery");
  await page.getByRole("button", { name: "Add the link" }).click();
  expect(sent).toEqual({ kind: "tracker", url: "https://trello.com/b/abc/delivery", label: "Delivery board", system: "trello" });
  await expect(page.getByRole("link", { name: "Delivery board" })).toHaveAttribute("target", "_blank");
  await audit(page);
  await page.getByRole("button", { name: "Remove Delivery board" }).click();
  await expect(page.getByText("No links yet.")).toBeVisible();
});

test("a task shows its status and keeps its own links to the issue it is tracked as", async ({ page }) => {
  const TASK = "33333333-3333-4333-8333-333333333333";
  let links: unknown[] = [];
  let sent: unknown = null;
  await page.route(`**/api/plans/${PID}/tasks/${TASK}`, (r) =>
    r.fulfill({ json: { pid: TASK, title: "Fix login", description: null, status: "in_progress", assignee_ref: null, points: null, due_date: "2026-11-01", sprint_pid: null, created_at: "2026-10-01T00:00:00Z", status_changed_at: "2026-10-01T00:00:00Z", done_at: null, blocked_days: null } }),
  );
  await page.route(`**/api/plans/${PID}/tasks/${TASK}/links`, (r) => {
    if (r.request().method() === "POST") {
      sent = r.request().postDataJSON();
      links = [{ pid: "l9", ...(sent as object) }];
      return r.fulfill({ json: links[0] });
    }
    return r.fulfill({ json: links });
  });
  await page.goto(`/en-001/plans/${PID}/tasks/${TASK}`);
  await expect(page.getByRole("heading", { name: "Fix login" })).toBeVisible();
  await expect(page.getByText("In progress")).toBeVisible();
  await page.getByLabel("Label").fill("PPM-7");
  await page.getByLabel("Address (http or https)").fill("https://acme.atlassian.net/browse/PPM-7");
  await page.getByLabel("Kind").selectOption("ticket");
  await page.getByLabel("External tool").selectOption("jira");
  await page.getByLabel("Its identifier there").fill("PPM-7");
  await page.getByRole("button", { name: "Add the link" }).click();
  expect(sent).toEqual({ kind: "ticket", url: "https://acme.atlassian.net/browse/PPM-7", label: "PPM-7", system: "jira", external_id: "PPM-7" });
  await expect(page.getByRole("link", { name: "PPM-7" })).toBeVisible();
  await audit(page);
});

test("the ceremonies, definitions and forums of a plan's own framework can be edited and are all sent", async ({ page }) => {
  let saved: Record<string, unknown> | null = null;
  const own = { pid: "w9", name: "Our way", description: null, scope: "plan", owner_plan_pid: PID, plans_using: 1, methodology: "scrum", ceremonies: [{ name: "Standup", cadence: "daily" }], definition_of_done: "Tested" };
  await page.route("**/api/ways-of-working", (r) => r.fulfill({ json: [own] }));
  await page.route("**/api/ways-of-working/w9", (r) => {
    saved = r.request().postDataJSON();
    return r.fulfill({ json: { ...own, ...saved } });
  });
  await page.route(`**/api/plans/${PID}/ways-of-working`, (r) =>
    r.fulfill({ json: { plan_pid: PID, source: "own", from_plan_pid: null, ways_of_working: saved ? { ...own, ...saved } : own, reason: null } }),
  );
  await page.goto(`/en-001/plans/${PID}/ways-of-working`);
  await page.getByRole("button", { name: "Edit", exact: true }).click();
  await expect(page.getByLabel("Definition of done")).toHaveValue("Tested");
  await page.getByLabel("Description").fill("How the platform team works");
  await page.getByLabel("Definition of ready").fill("Has a clear title");
  await page.getByRole("button", { name: "Add a ceremony" }).click();
  await page.getByLabel("Name").nth(2).fill("Retrospective");
  await page.getByLabel("How often").nth(1).fill("every two weeks");
  await audit(page);
  await page.getByRole("button", { name: "Save" }).click();
  await expect(page.getByRole("status")).toContainText("Saved.");
  expect(saved).toMatchObject({
    description: "How the platform team works",
    definition_of_ready: "Has a clear title",
    definition_of_done: "Tested",
    ceremonies: [{ name: "Standup", cadence: "daily", purpose: null }, { name: "Retrospective", cadence: "every two weeks", purpose: null }],
  });
});

test("a shared way of working asks for confirmation before it is changed for other plans, and an unused one can be deleted", async ({ page }) => {
  let saved: unknown = null;
  let removed = false;
  const shared = { pid: "w1", name: "Scrum, two weeks", description: null, scope: "organization", owner_plan_pid: null, plans_using: 3, methodology: "scrum", cadence_days: 14 };
  const unused = { pid: "w3", name: "Old kanban", description: null, scope: "organization", owner_plan_pid: null, plans_using: 0, methodology: "kanban" };
  await page.route("**/api/ways-of-working", (r) => r.fulfill({ json: removed ? [shared] : [shared, unused] }));
  await page.route("**/api/ways-of-working/w1", (r) => {
    saved = r.request().postDataJSON();
    return r.fulfill({ json: { ...shared, ...(saved as object) } });
  });
  await page.route("**/api/ways-of-working/w3/delete", (r) => {
    removed = true;
    return r.fulfill({ json: {} });
  });
  await page.route(`**/api/plans/${PID}/ways-of-working`, (r) =>
    r.fulfill({ json: { plan_pid: PID, source: "own", from_plan_pid: null, ways_of_working: shared, reason: null } }),
  );
  await page.goto(`/en-001/plans/${PID}/ways-of-working`);
  await page.getByRole("button", { name: "Edit", exact: true }).click();
  await page.getByLabel("Iteration length (days)").fill("7");
  await page.getByRole("button", { name: "Save" }).click();
  await expect(page.getByRole("alert")).toContainText("Tick the box to confirm the change for 2 other plan(s).");
  expect(saved).toBeNull();
  await page.getByLabel("This is shared: saving changes it for 2 other plan(s) too.").check();
  await page.getByRole("button", { name: "Save" }).click();
  await expect(page.getByRole("status")).toContainText("Saved.");
  expect(saved).toMatchObject({ cadence_days: 7 });

  await page.getByLabel("Shared or existing").selectOption("w3");
  await page.getByRole("button", { name: "Delete Old kanban (nobody follows it)" }).click();
  await expect(page.getByRole("status")).toContainText("Deleted Old kanban.");
  await audit(page);
});
