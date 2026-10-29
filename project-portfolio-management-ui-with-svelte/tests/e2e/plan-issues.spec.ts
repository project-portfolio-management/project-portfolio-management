// NC-1 slice 1 (IR-2, IR-3): a plan's issues page in a real browser, over a stubbed service. An issue is
// raised, edited (the whole body is replaced, and closing without a resolution is refused before any call),
// and raised from a risk (a second attempt is a 409 and says so in a catalogue sentence, not the service's
// text). Welsh reads. The page passes axe.

import { test, expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle } from "./fixtures";

const issue = (over: Record<string, unknown>) => ({
  pid: "i1",
  plan_pid: PID,
  title: "Build server is down",
  description: null,
  kind: "blocker",
  severity: "high",
  priority: null,
  status: "open",
  owner_ref: "person:ada",
  raised_by: null,
  raised_on: "2026-10-01",
  due_date: null,
  resolution: null,
  resolved_on: null,
  age_calendar_days: 9,
  overdue: false,
  ...over,
});

async function stub(page: Page) {
  let issues = [issue({})];
  const sent: { method: string; url: string; body: unknown }[] = [];
  let fromRiskCalls = 0;
  await page.route(`**/api/plans/${PID}/issues**`, async (route) => {
    const req = route.request();
    const path = new URL(req.url()).pathname;
    const body = req.postDataJSON?.() ?? null;
    if (req.method() === "GET") {
      return route.fulfill({
        json: {
          plan_pid: PID,
          issues,
          total: issues.length,
          open: issues.length,
          overdue: 0,
          as_of: "2026-10-10",
        },
      });
    }
    sent.push({ method: req.method(), url: path, body });
    if (path.endsWith("/from-risk/r1")) {
      fromRiskCalls += 1;
      if (fromRiskCalls > 1)
        return route.fulfill({
          status: 409,
          json: { error: "An open issue already exists for this risk (i9)" },
        });
      issues = [
        issue({ pid: "i9", title: "Supplier fails", kind: "risk" }),
        ...issues,
      ];
      return route.fulfill({ json: issues[0] });
    }
    if (req.method() === "POST") {
      const made = issue({ pid: "i2", ...(body as object) });
      issues = [...issues, made];
      return route.fulfill({ json: made });
    }
    if (req.method() === "PUT") {
      issues = issues.map((i) =>
        i.pid === "i1" ? issue({ pid: "i1", ...(body as object) }) : i,
      );
      return route.fulfill({ json: issues[0] });
    }
    return route.fulfill({ status: 404, json: {} });
  });
  await page.route(`**/api/plans/${PID}/risks`, (route) =>
    route.fulfill({
      json: [
        {
          pid: "r1",
          title: "Supplier fails",
          status: "materialized",
          probability: 4,
          impact: 4,
          exposure: 16,
          owner_ref: null,
          mitigation: null,
          review_date: null,
          escalated_at: null,
        },
      ],
    }),
  );
  return sent;
}

test("an issue is raised, edited and raised from a risk", async ({ page }) => {
  const sent = await stub(page);
  await page.goto(`/en-001/plans/${PID}/issues`, { waitUntil: "networkidle" });
  await expect(
    page.getByRole("heading", { name: "Plan issues" }),
  ).toBeVisible();
  await expect(page.locator("tbody tr")).toHaveCount(1);

  // Raise.
  await page.getByLabel("Title", { exact: true }).fill("Licence expired");
  await page
    .locator(".issue-form label:has-text('Severity') select")
    .selectOption("critical");
  await page.getByRole("button", { name: "Raise issue", exact: true }).click();
  await expect(
    page.getByRole("status").filter({ hasText: "The issue was raised." }),
  ).toBeVisible();
  await expect(page.locator("tbody tr")).toHaveCount(2);
  expect(sent[0]).toMatchObject({
    method: "POST",
    body: {
      title: "Licence expired",
      severity: "critical",
      status: "open",
      kind: "concern",
    },
  });

  // Edit: closing without a resolution is refused on the page, with no call.
  await page
    .getByRole("button", { name: "Edit: Build server is down" })
    .click();
  await expect(
    page.getByRole("heading", { name: "Edit the issue" }),
  ).toBeVisible();
  await page
    .locator(".issue-form label:has-text('Status') select")
    .selectOption("closed");
  await page.getByRole("button", { name: "Save changes" }).click();
  await expect(page.getByRole("alert")).toContainText("needs a resolution");
  expect(sent).toHaveLength(1);
  await page.getByLabel("Resolution").fill("Replaced the disk");
  await page.getByRole("button", { name: "Save changes" }).click();
  await expect(
    page.getByRole("status").filter({ hasText: "The issue was saved." }),
  ).toBeVisible();
  expect(sent[1]).toMatchObject({
    method: "PUT",
    url: `/api/proxy/api/plans/${PID}/issues/i1`,
    body: {
      status: "closed",
      resolution: "Replaced the disk",
      owner_ref: "person:ada",
    },
  });

  // Raise from a risk, then again: 409, said in the catalogue's words.
  await page.locator("[aria-labelledby=from-risk-h] select").selectOption("r1");
  await page
    .getByRole("button", { name: "Raise issue from this risk" })
    .click();
  await expect(
    page.getByRole("status").filter({ hasText: "raised from the risk" }),
  ).toBeVisible();
  await expect(page.locator("tbody tr")).toHaveCount(3);
  await page.locator("[aria-labelledby=from-risk-h] select").selectOption("r1");
  await page
    .getByRole("button", { name: "Raise issue from this risk" })
    .click();
  await expect(page.getByRole("alert")).toBeVisible();
  await expect(page.getByRole("alert")).not.toContainText(
    "An open issue already exists",
  );
});

test("the page reads in Welsh", async ({ page }) => {
  await stub(page);
  await page.goto(`/cy-001/plans/${PID}/issues`, { waitUntil: "networkidle" });
  await expect(
    page.getByRole("heading", { name: "Materion y cynllun" }),
  ).toBeVisible();
  await expect(
    page.getByRole("heading", { name: "Codi mater o risg" }),
  ).toBeVisible();
});

test("passes the accessibility audit", async ({ page }) => {
  await stub(page);
  await page.goto(`/en-001/plans/${PID}/issues`, { waitUntil: "networkidle" });
  await expect(page.locator("h1").first()).toBeVisible();
  await settle(page);
  const r = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    r.violations
      .filter((v) => v.impact === "serious" || v.impact === "critical")
      .map((v) => v.id),
  ).toEqual([]);
});
