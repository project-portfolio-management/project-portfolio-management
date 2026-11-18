// NC-1 slice 1 (IR-2, IR-3): a plan's issues page in a real browser, over a stubbed service. An issue is
// raised, edited (the whole body is replaced, and closing without a resolution is refused before any call),
// and raised from a risk (a second attempt is a 409 and says so in a catalogue sentence, not the service's
// text). Welsh reads. The page passes axe. NC-1 slice 12 part (c): editing reads the issue as stored and shows where
// it came from; removing asks first (Cancel sends nothing); where only an administrator may remove it that is said
// before anything is sent; a 403 and a 404 say the catalogue's sentence, never the service's text.

import { test, expect, type Page } from "./test";
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

async function stub(
  page: Page,
  opts: {
    options?: unknown;
    refuseDelete?: number;
    stored?: Record<string, unknown>;
  } = {},
) {
  let issues = [issue({})];
  const sent: { method: string; url: string; body: unknown }[] = [];
  let fromRiskCalls = 0;
  await page.route(`**/api/plans/${PID}/issues**`, async (route) => {
    const req = route.request();
    const path = new URL(req.url()).pathname;
    const body = req.postDataJSON?.() ?? null;
    if (req.method() === "GET" && path.endsWith("/issues/options")) {
      return opts.options
        ? route.fulfill({ json: opts.options })
        : route.fulfill({
            status: 404,
            json: { code: "not_found", detail: "service text" },
          });
    }
    const one = /\/issues\/(i\d+)$/.exec(path);
    if (req.method() === "GET" && one) {
      const found = issues.find((i) => i.pid === one[1]);
      return found
        ? route.fulfill({ json: { ...found, ...(opts.stored ?? {}) } })
        : route.fulfill({
            status: 404,
            json: { code: "not_found", detail: "service text" },
          });
    }
    if (req.method() === "DELETE" && one) {
      sent.push({ method: "DELETE", url: path, body: null });
      if (opts.refuseDelete)
        return route.fulfill({
          status: opts.refuseDelete,
          json: {
            code: opts.refuseDelete === 403 ? "forbidden" : "not_found",
            detail: "service text",
          },
        });
      issues = issues.filter((i) => i.pid !== one[1]);
      return route.fulfill({ json: {} });
    }
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

test("editing reads the issue as stored and says where it came from", async ({
  page,
}) => {
  await stub(page, {
    stored: {
      title: "Build server is down (stored)",
      raised_by: "person:ada",
      source_risk_pid: "r1",
      source_task_pid: "t7",
      change_request_pid: "c1",
    },
  });
  await page.goto(`/en-001/plans/${PID}/issues`, { waitUntil: "networkidle" });
  await page
    .getByRole("button", { name: "Edit: Build server is down" })
    .click();
  await expect(page.getByLabel("Title", { exact: true })).toHaveValue(
    "Build server is down (stored)",
  );
  const origin = page.getByTestId("issue-origin");
  await expect(origin).toContainText("Raised on 1 Oct 2026 by person:ada.");
  await expect(origin).toContainText("Raised from the risk “Supplier fails”.");
  await expect(
    origin.getByRole("link", {
      name: "Raised from a blocked task (open the task)",
    }),
  ).toHaveAttribute("href", `/en-001/plans/${PID}/tasks/t7`);
  await expect(
    origin.getByRole("link", { name: /Answered by a change request/ }),
  ).toBeVisible();
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

test("an issue is removed after a question, and Cancel sends nothing", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto(`/en-001/plans/${PID}/issues`, { waitUntil: "networkidle" });
  await expect(
    page.getByText("Removing an issue takes it off every list"),
  ).toBeVisible();
  const ask = page.getByRole("alertdialog");
  await page
    .getByRole("button", { name: "Remove: Build server is down" })
    .click();
  await expect(ask).toContainText("Remove the issue “Build server is down”?");
  await ask.getByRole("button", { name: "Cancel" }).click();
  expect(sent).toHaveLength(0);
  await page
    .getByRole("button", { name: "Remove: Build server is down" })
    .click();
  await ask.getByRole("button", { name: "OK" }).click();
  await expect(
    page
      .getByRole("status")
      .filter({ hasText: "The issue “Build server is down” was removed." }),
  ).toBeVisible();
  expect(sent).toEqual([
    {
      method: "DELETE",
      url: `/api/proxy/api/plans/${PID}/issues/i1`,
      body: null,
    },
  ]);
  await expect(page.getByText("No issues recorded.")).toBeVisible();
});

test("only an administrator may remove an issue under a gating workflow: said first, nothing sent", async ({
  page,
}) => {
  const workflow = {
    states: [
      {
        key: "open",
        label: "Open",
        category: "todo",
        is_initial: true,
        is_terminal: false,
      },
      {
        key: "closed",
        label: "Closed",
        category: "done",
        is_initial: false,
        is_terminal: true,
      },
    ],
    transitions: [{ from: "open", to: "closed", roles: [] }],
  };
  const sent = await stub(page, {
    options: {
      types: [
        {
          key: "blocker",
          label: "Blocker",
          position: 0,
          retired: false,
          workflow,
        },
      ],
      caller_roles: [],
      enforced: false,
      caller_admin: false,
      caller_permissions: [
        "issue_add",
        "issue_edit",
        "issue_move",
        "issue_delete",
      ],
    },
  });
  await page.goto(`/en-001/plans/${PID}/issues`, { waitUntil: "networkidle" });
  await page
    .getByRole("button", { name: "Remove: Build server is down" })
    .click();
  await expect(page.getByRole("alert")).toHaveText(
    "Only an administrator can remove “Build server is down”: its type's workflow controls its moves or fields. Nothing was sent.",
  );
  await expect(page.getByRole("alertdialog")).toHaveCount(0);
  expect(sent).toHaveLength(0);
});

test("a refused removal says the catalogue's sentence; an issue gone is read again", async ({
  page,
}) => {
  const sent = await stub(page, { refuseDelete: 403 });
  await page.goto(`/en-001/plans/${PID}/issues`, { waitUntil: "networkidle" });
  await page
    .getByRole("button", { name: "Remove: Build server is down" })
    .click();
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  await expect(page.getByRole("alert")).toHaveText(
    "Refused: you may not remove issues on this plan. Nothing was removed.",
  );
  expect(sent).toHaveLength(1);
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("an issue gone when editing begins says so", async ({ page }) => {
  await stub(page);
  await page.route(`**/api/plans/${PID}/issues/i1`, (route) =>
    route.fulfill({
      status: 404,
      json: { code: "not_found", detail: "service text" },
    }),
  );
  await page.goto(`/en-001/plans/${PID}/issues`, { waitUntil: "networkidle" });
  await page
    .getByRole("button", { name: "Edit: Build server is down" })
    .click();
  await expect(page.getByRole("alert")).toHaveText(
    "That issue is no longer on this plan; the list was read again.",
  );
  await expect(
    page.getByRole("heading", { name: "Edit the issue" }),
  ).toHaveCount(0);
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("removing reads in Welsh", async ({ page }) => {
  await stub(page);
  await page.goto(`/cy-001/plans/${PID}/issues`, { waitUntil: "networkidle" });
  await expect(
    page.getByRole("button", { name: "Tynnu: Build server is down" }),
  ).toBeVisible();
  await page
    .getByRole("button", { name: "Golygu: Build server is down" })
    .click();
  await expect(page.getByTestId("issue-origin")).toContainText("Codwyd ar");
});
