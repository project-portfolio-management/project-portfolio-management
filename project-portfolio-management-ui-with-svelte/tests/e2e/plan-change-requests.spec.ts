// NC-1 slice 2 (IR-4): a plan's change requests page in a real browser, over a stubbed service. A change request is
// drafted (a cost with no currency is refused before any call), edited while a draft (its what-if edits go back unchanged),
// submitted to a named approver (the requester as approver is refused on the page), put under review, decided (a
// short reason is refused on the page) and withdrawn; a 409 says so in a catalogue sentence, not the service's text.
// The register links here. Welsh reads. The page passes axe. NC-1 slice 12 part (c): the steps read the one request;
// an approved change is marked implemented; the issues and risks it answers are chosen (an issue answering another
// request is refused on the page, nothing is sent for no change); the forecast of its what-if edits is labelled one and
// the service's English is never shown; a baseline is taken after a question, once; each refusal is the catalogue's.

import { test, expect, type Page } from "./test";
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
  implement: { approved: "implemented" },
  submit: { draft: "submitted" },
  review: { submitted: "under_review" },
  decision: { submitted: "decided", under_review: "decided" },
  withdraw: {
    draft: "withdrawn",
    submitted: "withdrawn",
    under_review: "withdrawn",
  },
};

const ISSUES = [
  { pid: "i1", title: "Racks late", change_request_pid: null },
  { pid: "i2", title: "Power feed", change_request_pid: "c1" },
  { pid: "i3", title: "Licence lapsed", change_request_pid: "c9" },
];
const RISKS = [
  {
    pid: "r1",
    title: "Supplier fails",
    status: "open",
    probability: 3,
    impact: 4,
    exposure: 12,
  },
  {
    pid: "r2",
    title: "Floor too weak",
    status: "open",
    probability: 2,
    impact: 5,
    exposure: 10,
  },
];
const PREVIEW = {
  change_request_pid: "c1",
  label: "service text",
  note: "service text",
  stated_impact: {
    schedule_business_days: 10,
    cost_minor: 150000,
    cost_currency: "GBP",
  },
  forecast: {
    plan_pid: PID,
    plan_etag: "e1",
    result_etag: "e2",
    changes: [],
    finish: { before: "2026-12-01", after: "2026-12-15" },
    tasks: [
      { pid: "t1", title: "Order the racks", before: null, after: null },
      { pid: "t2", title: "Fit the racks", before: null, after: null },
    ],
    moved: [
      { pid: "t1", moved: "later", start_days: 1, finish_days: 1 },
      { pid: "t2", moved: "later", start_days: 10, finish_days: 10 },
    ],
    critical_path: { before: [], after: [], added: [], removed: [] },
    assignments_recomputed: [],
    cost_impact: { available: false, reason: "no_finance_access" },
    masked: true,
    note: "service text",
  },
};

async function stub(
  page: Page,
  refuse: Partial<Record<"links" | "baseline" | "preview", number>> = {},
) {
  const links: Record<string, { issue_pids: string[]; risk_pids: string[] }> = {
    c1: { issue_pids: ["i2"], risk_pids: [] },
    c2: { issue_pids: [], risk_pids: ["r1"] },
  };
  const withLinks = (r: Record<string, unknown>) => ({
    ...r,
    ...(links[r.pid as string] ?? { issue_pids: [], risk_pids: [] }),
  });
  await page.route(`**/api/plans/${PID}/issues`, (route) =>
    route.fulfill({
      json: {
        plan_pid: PID,
        issues: ISSUES,
        total: 3,
        open: 3,
        overdue: 0,
        as_of: "2026-11-18",
      },
    }),
  );
  await page.route(`**/api/plans/${PID}/risks`, (route) =>
    route.fulfill({ json: RISKS }),
  );
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
    const one = /change-requests\/(c\d)$/.exec(path);
    if (req.method() === "GET" && one) {
      const row = rows.find((r) => r.pid === one[1]);
      return row
        ? route.fulfill({ json: withLinks(row) })
        : route.fulfill({
            status: 404,
            json: { code: "not_found", detail: "service text" },
          });
    }
    if (req.method() === "GET" && path.endsWith("/preview")) {
      const row = rows.find((r) => path.includes(`/${r.pid}/`))!;
      if (refuse.preview || !row.edits)
        return route.fulfill({
          status: 422,
          json: { code: "unprocessable", detail: "service text" },
        });
      return route.fulfill({ json: PREVIEW });
    }
    if (req.method() === "GET") {
      return route.fulfill({
        json: { plan_pid: PID, change_requests: rows, total: rows.length },
      });
    }
    sent.push({ method: req.method(), url: path, body });
    const extraStep = /change-requests\/(c\d)\/(links|baseline)$/.exec(path);
    if (extraStep) {
      const [, id, what] = extraStep;
      const row = rows.find((r) => r.pid === id)!;
      if (refuse[what as "links" | "baseline"])
        return route.fulfill({
          status: refuse[what as "links" | "baseline"],
          json: { code: "conflict", detail: "service text" },
        });
      if (what === "links") {
        links[id!] = body as { issue_pids: string[]; risk_pids: string[] };
        return route.fulfill({ json: withLinks(row) });
      }
      rows = rows.map((r) => (r.pid === id ? { ...r, baseline_pid: "b1" } : r));
      return route.fulfill({
        json: {
          change_request_pid: id,
          baseline_pid: "b1",
          baseline_version: 4,
        },
      });
    }
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

test("an approved change is marked implemented, and a baseline is taken after a question, once", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto(`/en-001/plans/${PID}/change-requests`, {
    waitUntil: "networkidle",
  });
  await page.getByRole("button", { name: "Steps: Drop the report" }).click();
  const baseline = page.getByRole("region", {
    name: "A baseline because of this change",
  });
  await expect(baseline).toContainText(
    "Taking one is a person's choice; nothing is re-baselined automatically. One baseline per change, its reason naming the request, and it cannot be undone.",
  );
  const ask = page.getByRole("alertdialog");
  await baseline.getByRole("button", { name: "Take a baseline" }).click();
  await expect(ask).toContainText(
    "Take a schedule baseline because of “Drop the report”?",
  );
  await ask.getByRole("button", { name: "Cancel" }).click();
  expect(sent).toHaveLength(0);
  await baseline.getByRole("button", { name: "Take a baseline" }).click();
  await ask.getByRole("button", { name: "OK" }).click();
  await status(page, "Baseline version 4 was taken because of this change.");
  await expect(baseline.getByTestId("cr-baseline")).toHaveText(
    "A schedule baseline has been taken because of this change.",
  );
  await expect(
    baseline.getByRole("button", { name: "Take a baseline" }),
  ).toHaveCount(0);

  await page.getByRole("button", { name: "Mark as implemented" }).click();
  await status(page, "The change was marked implemented.");
  await expect(page.getByText("It is now implemented.")).toBeVisible();
  await expect(
    page.getByRole("button", { name: "Mark as implemented" }),
  ).toHaveCount(0);
  // A closed request is not relinked.
  await expect(
    page.getByText("A closed request cannot be relinked."),
  ).toBeVisible();
  expect(
    sent.map(
      (x) => `${x.method} ${x.url.replace(`/api/proxy/api/plans/${PID}`, "")}`,
    ),
  ).toEqual([
    "POST /change-requests/c2/baseline",
    "POST /change-requests/c2/implement",
  ]);
});

test("the issues and risks it answers: rules first, nothing for no change, the whole sets sent", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto(`/en-001/plans/${PID}/change-requests`, {
    waitUntil: "networkidle",
  });
  await page.getByRole("button", { name: "Steps: Move go-live" }).click();
  const links = page.getByRole("region", {
    name: "Issues and risks this change answers",
  });
  await expect(links.getByTestId("cr-answers")).toHaveText("Issue: Power feed");
  await expect(links).toContainText(
    "An issue answers one change request at most; up to 200 issues and 200 risks.",
  );
  const save = links.getByRole("button", { name: "Save what it answers" });
  await expect(save).toBeEnabled();
  await expect(links.getByLabel("Power feed")).toBeChecked();
  // One already answered by another request cannot be ticked.
  await expect(
    links.getByLabel("Licence lapsed (answers another change request)"),
  ).toBeDisabled();
  await save.click();
  await status(page, "Nothing changed, so nothing was sent.");
  expect(sent).toHaveLength(0);
  await links.getByLabel("Racks late").check();
  await links.getByLabel("Floor too weak").check();
  await save.click();
  await status(page, "The issues and risks this change answers were saved.");
  expect(sent).toEqual([
    {
      method: "PUT",
      url: `/api/proxy/api/plans/${PID}/change-requests/c1/links`,
      body: { issue_pids: ["i1", "i2"], risk_pids: ["r2"] },
    },
  ]);
  await expect(
    links.getByTestId("cr-answers").getByRole("listitem"),
  ).toHaveText([
    "Issue: Racks late",
    "Issue: Power feed",
    "Risk: Floor too weak",
  ]);
});

test("a refused relink and a refused baseline say the catalogue's sentence", async ({
  page,
}) => {
  await stub(page, { links: 409, baseline: 409 });
  await page.goto(`/en-001/plans/${PID}/change-requests`, {
    waitUntil: "networkidle",
  });
  await page.getByRole("button", { name: "Steps: Drop the report" }).click();
  const links = page.getByRole("region", {
    name: "Issues and risks this change answers",
  });
  await links.getByLabel("Racks late").check();
  await links.getByRole("button", { name: "Save what it answers" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "Refused: the request has closed, or one of the issues now answers another change request. The request and the plan's issues were read again.",
  );
  await page.getByRole("button", { name: "Take a baseline" }).click();
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  await expect(page.getByRole("alert")).toHaveText(
    "Refused: a baseline has already been taken for this change, or it is no longer approved. The request was read again.",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("the forecast is labelled one, in the page's words; a request with no edits says so and asks nothing", async ({
  page,
}) => {
  await stub(page);
  const previews: string[] = [];
  page.on("request", (r) => {
    if (r.url().endsWith("/preview")) previews.push(r.url());
  });
  await page.goto(`/en-001/plans/${PID}/change-requests`, {
    waitUntil: "networkidle",
  });
  await page.getByRole("button", { name: "Steps: Drop the report" }).click();
  const region = page.getByRole("region", { name: "What-if forecast" });
  await expect(region).toHaveText(
    /This request carries no what-if edits, so there is no forecast/,
  );
  await expect(region.getByRole("button")).toHaveCount(0);

  await page.getByRole("button", { name: "Steps: Move go-live" }).click();
  await expect(region).toContainText("Reading it changes nothing");
  await region.getByRole("button", { name: "Show the forecast" }).click();
  const f = page.getByTestId("cr-forecast");
  await expect(f).toContainText("Forecast — nothing was changed");
  await expect(f).toContainText(
    "Plan finish: 1 Dec 2026 now, 15 Dec 2026 with the change.",
  );
  await expect(f).toContainText("2 tasks would finish on a different day.");
  await expect(f.getByRole("listitem")).toHaveText([
    "Fit the racks: finishes 10 business days later",
    "Order the racks: finishes 1 business day later",
  ]);
  await expect(f).toContainText(
    "The labour cost forecast is shown only to people who may see cost rates.",
  );
  await expect(
    page.getByText("Impact stated by the requester: 10 business days"),
  ).toBeVisible();
  await expect(page.getByText("service text")).toHaveCount(0);
  expect(previews).toHaveLength(1);
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

test("a refused forecast says the catalogue's sentence", async ({ page }) => {
  await stub(page, { preview: 422 });
  await page.goto(`/en-001/plans/${PID}/change-requests`, {
    waitUntil: "networkidle",
  });
  await page.getByRole("button", { name: "Steps: Move go-live" }).click();
  await page.getByRole("button", { name: "Show the forecast" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "Refused: this request carries no what-if edits the scheduler can read, so there is no forecast.",
  );
  await expect(page.getByTestId("cr-forecast")).toHaveCount(0);
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("the links, the forecast and the baseline read in Welsh", async ({
  page,
}) => {
  await stub(page);
  await page.goto(`/cy-001/plans/${PID}/change-requests`, {
    waitUntil: "networkidle",
  });
  await page.getByRole("button", { name: "Camau: Drop the report" }).click();
  await expect(
    page.getByRole("heading", {
      name: "Materion a risgiau y mae'r newid hwn yn eu hateb",
    }),
  ).toBeVisible();
  await expect(
    page.getByRole("heading", { name: "Rhagolwg beth-os" }),
  ).toBeVisible();
  await expect(
    page.getByRole("button", { name: "Cymryd llinell sylfaen" }),
  ).toBeVisible();
  await expect(
    page.getByRole("button", { name: "Marcio wedi'i weithredu" }),
  ).toBeVisible();
});
