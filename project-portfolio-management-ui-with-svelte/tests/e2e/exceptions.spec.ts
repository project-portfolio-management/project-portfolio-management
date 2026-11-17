// NC-1 slice 11 (DP-3): the delivery exceptions register, in a real browser over a stubbed service. Exceptions are
// listed with the plan's name, the worst value as a word and a symbol, a breach and what is still needed; filters ask
// the service; completing sends only what changed; a decision of ten characters or more closes one; one a threshold
// opened is refused in the catalogue's words. Welsh; axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle, stubPlansApi } from "./fixtures";

const E1 = "66666666-6666-4666-8666-666666666661";
const E2 = "66666666-6666-4666-8666-666666666662";

const exc = (pid: string, over: Record<string, unknown> = {}) => ({
  pid,
  plan_pid: PID,
  status: "open",
  opened_at: "2026-11-14T09:00:00Z",
  opened_value: "amber_red",
  worst_value: "red",
  worsened_count: 1,
  cause: "Supplier late",
  decision_needed: null,
  owner_ref: null,
  owner_role: null,
  decide_by: null,
  completed_at: null,
  complete: false,
  missing: ["decision_needed", "owner", "decide_by"],
  breached: true,
  window_hours: 48,
  hours_open: 72,
  closed_at: null,
  decision: null,
  decided_by: null,
  ...over,
});

async function stubExceptions(
  page: Page,
  opts: { decideStatus?: number } = {},
) {
  const rows = [
    exc(E1),
    exc(E2, {
      breached: false,
      worsened_count: 0,
      worst_value: "amber_red",
      status: "closed",
      decision: "Paused the release.",
      closed_at: "2026-11-15T09:00:00Z",
    }),
  ];
  const sent: Array<{ method: string; url: string; body: unknown }> = [];
  await stubPlansApi(page);
  await page.route(/\/api\/exceptions/, async (route) => {
    const req = route.request();
    const url = new URL(req.url());
    const path = url.pathname.replace(/^\/api\/proxy/, "");
    if (req.method() === "GET") {
      sent.push({ method: "GET", url: `${path}${url.search}`, body: null });
      const status = url.searchParams.get("status");
      const breached = url.searchParams.get("breached");
      return route.fulfill({
        json: {
          window_hours: 48,
          exceptions: rows.filter(
            (r) =>
              (!status || r.status === status) &&
              (breached === null || String(r.breached) === breached),
          ),
        },
      });
    }
    const body = req.postDataJSON();
    sent.push({ method: req.method(), url: path, body });
    const row = rows.find((r) => path.includes(r.pid as string))!;
    if (path.endsWith("/decision")) {
      if (opts.decideStatus)
        return route.fulfill({
          status: opts.decideStatus,
          json: { error: "x", description: "service text" },
        });
      Object.assign(row, {
        status: "closed",
        decision: body.decision,
        closed_at: "2026-11-17T10:00:00Z",
      });
      return route.fulfill({ json: row });
    }
    Object.assign(row, body);
    return route.fulfill({ json: row });
  });
  return sent;
}

test("the register lists open exceptions with the plan, the worst value, the breach and what is still needed", async ({
  page,
}) => {
  const sent = await stubExceptions(page);
  await page.goto("/en-001/exceptions");
  await settle(page);
  await expect(
    page.getByRole("heading", { level: 1, name: "Exceptions" }),
  ).toBeVisible();
  const item = page.getByRole("article", { name: "Website replatform" });
  await expect(
    item.getByRole("link", { name: "Website replatform" }),
  ).toHaveAttribute("href", new RegExp(`/plans/${PID}/confidence$`));
  await expect(item.getByText("Worst confidence: red")).toBeVisible();
  await expect(item.getByText("Worsened 1 time since it opened")).toBeVisible();
  await expect(
    item.getByText("Breached: not completed within 48 hours of opening."),
  ).toBeVisible();
  await expect(
    item.getByText("Still needed: Decision needed, Owner, Decide by."),
  ).toBeVisible();
  await expect(page.getByRole("article")).toHaveCount(1);

  // The filters ask the service.
  await page.getByLabel("Show").selectOption("");
  await expect(page.getByRole("article")).toHaveCount(2);
  await page.getByLabel("Breached only").check();
  await expect(page.getByRole("article")).toHaveCount(1);
  expect(sent.filter((s) => s.method === "GET").map((s) => s.url)).toEqual([
    "/api/exceptions?status=open",
    "/api/exceptions",
    "/api/exceptions?breached=true",
  ]);
});

test("completing sends only what changed, and saving nothing new sends nothing", async ({
  page,
}) => {
  const sent = await stubExceptions(page);
  await page.goto("/en-001/exceptions");
  await settle(page);
  await page
    .getByRole("button", { name: "Complete it: Website replatform" })
    .click();
  await page.getByRole("button", { name: "Save", exact: true }).click();
  await expect(page.getByRole("status")).toHaveText(
    "Nothing changed, so nothing was sent.",
  );
  await page
    .getByRole("button", { name: "Complete it: Website replatform" })
    .click();
  await page.getByLabel("Decision needed").fill("  Fund a second supplier? ");
  await page.getByLabel("Owner (a person reference)").fill("ab");
  await page.getByRole("button", { name: "Save", exact: true }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "A cause, a decision needed or an owner needs at least 3 characters to count.",
  );
  await page.getByLabel("Owner (a person reference)").fill("person:7");
  await page.getByLabel("Decide by").fill("2026-11-20");
  await page.getByRole("button", { name: "Save", exact: true }).click();
  await expect(page.getByRole("status")).toHaveText("Saved.");
  expect(sent.filter((s) => s.method === "PUT")).toEqual([
    {
      method: "PUT",
      url: `/api/exceptions/${E1}`,
      body: {
        decision_needed: "Fund a second supplier?",
        owner_ref: "person:7",
        decide_by: "2026-11-20",
      },
    },
  ]);
});

test("a decision of ten characters or more closes an exception", async ({
  page,
}) => {
  const sent = await stubExceptions(page);
  await page.goto("/en-001/exceptions");
  await settle(page);
  await page
    .getByRole("button", { name: "Record the decision: Website replatform" })
    .click();
  await page.getByLabel("Decision (at least 10 characters)").fill("Pause it");
  await page
    .getByRole("button", { name: "Record the decision and close" })
    .click();
  await expect(page.getByRole("alert")).toHaveText(
    "Write the decision in at least 10 characters.",
  );
  await page
    .getByLabel("Decision (at least 10 characters)")
    .fill("  Fund a second supplier for the data move.  ");
  await page
    .getByRole("button", { name: "Record the decision and close" })
    .click();
  await expect(page.getByRole("status")).toHaveText(
    "Decision recorded; the exception is closed.",
  );
  await expect(page.getByRole("article")).toHaveCount(0);
  expect(sent.filter((s) => s.method === "POST")).toEqual([
    {
      method: "POST",
      url: `/api/exceptions/${E1}/decision`,
      body: { decision: "Fund a second supplier for the data move." },
    },
  ]);
});

test("one a threshold opened is refused in the catalogue's words", async ({
  page,
}) => {
  await stubExceptions(page, { decideStatus: 422 });
  await page.goto("/en-001/exceptions");
  await settle(page);
  await page
    .getByRole("button", { name: "Record the decision: Website replatform" })
    .click();
  await page
    .getByLabel("Decision (at least 10 characters)")
    .fill("Fund a second supplier.");
  await page
    .getByRole("button", { name: "Record the decision and close" })
    .click();
  await expect(page.getByRole("alert")).toHaveText(
    "This exception was opened by a threshold, so it is closed by a pause, reset, stop or recorded override, not a decision here.",
  );
  await expect(page.getByRole("alert")).not.toContainText("service text");
});

test("the register reads in Welsh and passes axe", async ({ page }) => {
  await stubExceptions(page);
  await page.goto("/cy-001/exceptions");
  await settle(page);
  await expect(page.getByRole("heading", { level: 1 })).toHaveText(
    "Eithriadau",
  );
  await expect(
    page.getByText(
      "Dal ei angen: Penderfyniad sydd ei angen, Perchennog, Penderfynu erbyn.",
    ),
  ).toBeVisible();
  const results = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    results.violations.filter(
      (v) => v.impact === "serious" || v.impact === "critical",
    ),
  ).toEqual([]);
});
