// NC-1 slice 2 (IR-4): a plan's change requests page in a real browser, over a stubbed service. A change request is
// drafted (a cost with no currency is refused before any call), edited while a draft (its what-if edits go back unchanged),
// submitted to a named approver (the requester as approver is refused on the page), put under review, decided (a
// short reason is refused on the page) and withdrawn; a 409 says so in a catalogue sentence, not the service's text.
// The register links here. Welsh reads. The page passes axe.

import { test, expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle } from "./fixtures";

const EDITS = [{ op: "shift", task: "t1", days: 2 }];

const cr = (over: Record<string, unknown>) => ({
  pid: "c1",
  plan_pid: PID,
  title: "Move go-live",
  description: "Two weeks later",
  reason: "Supplier slipped",
  kind: "schedule",
  impact: {
    schedule_business_days: 10,
    cost_minor: 150000,
    cost_currency: "GBP",
    summary: null,
    has_forecast_edits: true,
  },
  edits: EDITS,
  status: "draft",
  requested_by: "alice",
  requested_on: "2026-11-01",
  age_calendar_days: 9,
  approver_ref: null,
  decided_by: null,
  decided_on: null,
  decision_reason: null,
  implemented_on: null,
  baseline_pid: null,
  ...over,
});

const NEXT: Record<string, Record<string, string>> = {
  submit: { draft: "submitted" },
  review: { submitted: "under_review" },
  decision: { submitted: "decided", under_review: "decided" },
  withdraw: {
    draft: "withdrawn",
    submitted: "withdrawn",
    under_review: "withdrawn",
  },
};

async function stub(page: Page) {
  let rows = [
    cr({}),
    cr({
      pid: "c2",
      title: "Drop the report",
      kind: "scope",
      status: "approved",
      approver_ref: "person:bob",
      decided_by: "bob",
      decided_on: "2026-11-05",
      decision_reason: "Nobody reads it any more",
      edits: null,
      impact: {
        schedule_business_days: null,
        cost_minor: null,
        cost_currency: null,
        summary: null,
        has_forecast_edits: false,
      },
    }),
  ];
  const sent: { method: string; url: string; body: unknown }[] = [];
  await page.route(`**/api/plans/${PID}/change-requests**`, async (route) => {
    const req = route.request();
    const path = new URL(req.url()).pathname;
    const body = req.postDataJSON?.() ?? null;
    if (req.method() === "GET") {
      return route.fulfill({
        json: { plan_pid: PID, change_requests: rows, total: rows.length },
      });
    }
    sent.push({ method: req.method(), url: path, body });
    const m = /change-requests\/(c\d)\/(\w+)$/.exec(path);
    if (m) {
      const [, id, step] = m;
      const row = rows.find((r) => r.pid === id)!;
      let to = NEXT[step!]?.[row.status as string];
      if (!to)
        return route.fulfill({
          status: 409,
          json: {
            error: "conflict",
            description: `a change request that is ${row.status} cannot do that`,
          },
        });
      if (to === "decided")
        to =
          (body as { decision: string }).decision === "approve"
            ? "approved"
            : "rejected";
      const extra =
        step === "submit"
          ? { approver_ref: (body as { approver_ref: string }).approver_ref }
          : step === "decision"
            ? {
                decided_by: "bob",
                decided_on: "2026-11-10",
                decision_reason: (body as { reason: string }).reason,
              }
            : {};
      rows = rows.map((r) =>
        r.pid === id ? { ...r, status: to, ...extra } : r,
      );
      return route.fulfill({ json: rows.find((r) => r.pid === id) });
    }
    if (req.method() === "POST") {
      const made = cr({ pid: "c3", edits: null, ...(body as object) });
      rows = [made, ...rows];
      return route.fulfill({ json: made });
    }
    if (req.method() === "PUT") {
      rows = rows.map((r) =>
        r.pid === "c1" ? cr({ ...(body as object) }) : r,
      );
      return route.fulfill({ json: rows.find((r) => r.pid === "c1") });
    }
    return route.fulfill({ status: 404, json: {} });
  });
  return sent;
}

const status = (page: Page, text: string) =>
  expect(page.getByRole("status").filter({ hasText: text })).toBeVisible();

test("a change request is drafted, edited, submitted, reviewed and decided", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto(`/en-001/plans/${PID}/change-requests`, {
    waitUntil: "networkidle",
  });
  await expect(
    page.getByRole("heading", { name: "Plan change requests", level: 1 }),
  ).toBeVisible();
  await expect(page.locator("tbody tr")).toHaveCount(2);

  // Draft: a cost with no currency is refused on the page, with no call.
  const form = page.locator("[aria-labelledby=cr-form-h]");
  await form.getByLabel("Title", { exact: true }).fill("Add a supplier");
  await form
    .getByLabel("Description (what is to change)")
    .fill("A second supplier for the racks");
  await form
    .getByLabel("Reason (why the change is needed)")
    .fill("One supplier is a single point of failure");
  await form.getByLabel(/^Cost impact/).fill("2,500.00");
  await page.getByRole("button", { name: "Save the draft" }).click();
  await expect(page.getByRole("alert")).toContainText(
    "A cost impact needs its currency.",
  );
  expect(sent).toHaveLength(0);
  await form.getByLabel(/^Currency of the cost/).fill("eur");
  await page.getByRole("button", { name: "Save the draft" }).click();
  await status(page, "The change request was drafted.");
  await expect(page.locator("tbody tr")).toHaveCount(3);
  expect(sent[0]).toMatchObject({
    method: "POST",
    url: `/api/proxy/api/plans/${PID}/change-requests`,
    body: {
      title: "Add a supplier",
      reason: "One supplier is a single point of failure",
      kind: "other",
      impact_schedule_days: null,
      impact_cost_minor: 250000,
      cost_currency: "EUR",
      edits: null,
    },
  });

  // Edit the draft: every field goes back, the what-if edits with them.
  await page.getByRole("button", { name: "Edit: Move go-live" }).click();
  await expect(
    page.getByRole("heading", { name: "Edit the draft" }),
  ).toBeVisible();
  await form.getByLabel(/^Schedule impact/).fill("12");
  await page.getByRole("button", { name: "Save changes" }).click();
  await status(page, "The draft was saved.");
  expect(sent[1]).toMatchObject({
    method: "PUT",
    url: `/api/proxy/api/plans/${PID}/change-requests/c1`,
    body: {
      title: "Move go-live",
      impact_schedule_days: 12,
      impact_cost_minor: 150000,
      cost_currency: "GBP",
      edits: EDITS,
    },
  });

  // Submit: naming the requester as approver is refused on the page.
  await page.getByRole("button", { name: "Steps: Move go-live" }).click();
  await expect(
    page.getByRole("heading", { name: "Steps for Move go-live" }),
  ).toBeVisible();
  const approver = page.getByLabel(/^Approver \(/);
  await approver.fill("person:alice");
  await page.getByRole("button", { name: "Submit", exact: true }).click();
  await expect(page.getByRole("alert")).toContainText(
    "cannot be the person who asked",
  );
  expect(sent).toHaveLength(2);
  await approver.fill("person:bob");
  await page.getByRole("button", { name: "Submit", exact: true }).click();
  await status(page, "The change request was submitted.");
  expect(sent[2]).toMatchObject({
    url: `/api/proxy/api/plans/${PID}/change-requests/c1/submit`,
    body: { approver_ref: "person:bob" },
  });
  await expect(page.getByText("It is now submitted.")).toBeVisible();

  // Review, then decide: a short reason is refused on the page.
  await page.getByRole("button", { name: "Start the review" }).click();
  await status(page, "The review has started.");
  await expect(page.getByText("It is now under review.")).toBeVisible();
  await expect(
    page.getByRole("button", { name: "Start the review" }),
  ).toHaveCount(0);
  await page
    .locator("[aria-labelledby=cr-decide-h] select")
    .selectOption("reject");
  const reason = page.getByLabel(/^Reason for the decision/);
  await reason.fill("No");
  await page.getByRole("button", { name: "Record decision" }).click();
  await expect(page.getByRole("alert")).toContainText("at least 10 characters");
  expect(sent).toHaveLength(4);
  await reason.fill("The date cannot move this quarter");
  await page.getByRole("button", { name: "Record decision" }).click();
  await status(page, "The decision was recorded.");
  expect(sent[4]).toMatchObject({
    url: `/api/proxy/api/plans/${PID}/change-requests/c1/decision`,
    body: { decision: "reject", reason: "The date cannot move this quarter" },
  });
  await expect(page.getByText("It is now rejected.")).toBeVisible();
  await expect(page.getByText("No further step can be taken")).toBeVisible();
  await expect(
    page.getByText(
      "Decided by bob on 10 Nov 2026: The date cannot move this quarter",
    ),
  ).toBeVisible();
});

test("a draft is withdrawn, and a refused step says so in words", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto(`/en-001/plans/${PID}/change-requests`, {
    waitUntil: "networkidle",
  });
  await page.getByRole("button", { name: "Steps: Move go-live" }).click();
  await page
    .getByRole("button", { name: "Withdraw the change request" })
    .click();
  await status(page, "The change request was withdrawn.");
  expect(sent[0]).toMatchObject({
    method: "POST",
    url: `/api/proxy/api/plans/${PID}/change-requests/c1/withdraw`,
  });
  await expect(page.locator("tbody tr").first()).toContainText("withdrawn");
  // Only a draft has an edit button.
  await expect(page.getByRole("button", { name: /^Edit:/ })).toHaveCount(0);

  // The approved one: the service allows a withdrawal, but this stub has moved it on (409).
  await page.getByRole("button", { name: "Steps: Drop the report" }).click();
  await expect(
    page.getByText("Decided by bob on 5 Nov 2026: Nobody reads it any more"),
  ).toBeVisible();
  await page
    .getByRole("button", { name: "Withdraw the change request" })
    .click();
  await expect(page.getByRole("alert")).toContainText("has moved on");
  await expect(page.getByRole("alert")).not.toContainText("cannot do that");
});

test("the register's rows open the plan's change requests", async ({
  page,
}) => {
  await page.route("**/api/change-requests**", (route) =>
    route.fulfill({
      json: {
        change_requests: [cr({})],
        summary: {
          by_status: { draft: 1 },
          awaiting_decision: 0,
          oldest_open: null,
        },
        as_of: "2026-11-10",
      },
    }),
  );
  await page.goto("/en-001/change-requests", { waitUntil: "networkidle" });
  await expect(
    page.getByRole("link", {
      name: "Move go-live: Open the plan's change requests",
    }),
  ).toHaveAttribute("href", `/en-001/plans/${PID}/change-requests`);
});

test("the page reads in Welsh", async ({ page }) => {
  await stub(page);
  await page.goto(`/cy-001/plans/${PID}/change-requests`, {
    waitUntil: "networkidle",
  });
  await expect(
    page.getByRole("heading", { name: "Ceisiadau newid y cynllun" }),
  ).toBeVisible();
  await expect(
    page.getByRole("heading", { name: "Drafftio cais newid" }),
  ).toBeVisible();
  await page.getByRole("button", { name: "Camau: Move go-live" }).click();
  await expect(
    page.getByRole("heading", { name: "Cyflwyno am benderfyniad" }),
  ).toBeVisible();
});

test("passes the accessibility audit, with a request's steps open", async ({
  page,
}) => {
  await stub(page);
  await page.goto(`/en-001/plans/${PID}/change-requests`, {
    waitUntil: "networkidle",
  });
  await expect(page.locator("h1").first()).toBeVisible();
  await page.getByRole("button", { name: "Steps: Move go-live" }).click();
  await expect(
    page.getByRole("heading", { name: "Submit for a decision" }),
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
