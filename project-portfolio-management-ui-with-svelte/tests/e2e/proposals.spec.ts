// NC-1 slice 11 part (m) (EX-13): the named trade-off on a new ask, in a real browser over a stubbed service. A new
// proposal is tagged as an external ask from the sponsor, with or without its trade-off; the board shows which
// proposals are external asks and their trade-off, or that none is named; the trade-off is named and changed while the
// proposal is undecided (nothing sent for the same text); approving an ask without one is refused before anything is
// sent, and the service's 422 is the catalogue's sentence. No service text; Welsh; axe with the forms open.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { settle } from "./fixtures";

const row = (over: Record<string, unknown>) => ({
  title: "Untitled",
  summary: null,
  kind_target: "projects",
  sponsor_ref: null,
  strategic_rationale: null,
  requested_minor: null,
  currency: null,
  status: "draft",
  promoted_plan_pid: null,
  external_ask: false,
  tradeoff: null,
  ...over,
});

async function stub(
  page: Page,
  opts: {
    approveStatus?: number;
    approveCode?: string;
    draft?: boolean;
    editStatus?: number;
    editCode?: string;
    history?: number;
  } = {},
) {
  const proposals = [
    ...(opts.draft
      ? [
          row({
            pid: "d1",
            title: "Kiosk refresh",
            summary: "Lobby kiosks",
            status: "draft",
            sponsor_ref: "person:00000000-0000-4000-8000-000000000001",
            strategic_rationale: "service text",
            requested_minor: 250000,
            currency: "GBP",
          }),
        ]
      : []),
    row({
      pid: "a1",
      title: "Sponsor dashboard",
      status: "in_review",
      external_ask: true,
      tradeoff: null,
    }),
    row({
      pid: "a2",
      title: "Partner portal",
      status: "submitted",
      external_ask: true,
      tradeoff: "Delays the CRM upgrade by a quarter",
    }),
    row({ pid: "a3", title: "Office move", status: "in_review" }),
    row({
      pid: "a4",
      title: "Old ask",
      status: "approved",
      external_ask: true,
      tradeoff: "Pauses the intranet refresh",
    }),
  ];
  const sent: Array<{ method: string; path: string; body: unknown }> = [];
  await page.route(/\/api\/proposals/, async (route) => {
    const req = route.request();
    const url = new URL(req.url());
    const path = url.pathname.replace(/^\/api\/proxy/, "");
    const parts = path.split("/");
    if (req.method() === "GET") {
      if (path === "/api/proposals") return route.fulfill({ json: proposals });
      if (path === "/api/proposals/forecast") {
        sent.push({ method: "GET", path: `${path}${url.search}`, body: null });
        const history = opts.history ?? 12;
        return route.fulfill({
          json: {
            as_of: "2026-11-18T10:00:00Z",
            note: "service text",
            history_window: {
              from: "2026-08-26T10:00:00Z",
              to: "2026-11-18T10:00:00Z",
              periods: history,
              period_days: 7,
            },
            arrivals_per_period: [3, 1, 2],
            approvals_per_period: [1, 0, 2],
            forecast: {
              periods: 4,
              days: 28,
              trials: 10000,
              history_periods: history,
              median_items: history < 6 ? null : 5,
              at_least_items: history < 6 ? null : 1,
              at_most_items: history < 6 ? null : 8,
              reason: history < 6 ? "service text" : null,
              note: "service text",
            },
          },
        });
      }
      if (parts.length === 4) {
        const one = proposals.find((x) => x.pid === parts[3]);
        return one
          ? route.fulfill({ json: one })
          : route.fulfill({
              status: 404,
              json: { code: "not_found", detail: "service text" },
            });
      }
      return route.fulfill({ json: [] });
    }
    const body = req.postDataJSON();
    sent.push({ method: req.method(), path, body });
    const pid = parts[3] ?? "";
    const p = proposals.find((x) => x.pid === pid);
    if (req.method() === "PUT" && parts.length === 4 && p) {
      if (opts.editStatus) {
        if (opts.editCode === "unprocessable") p.status = "submitted";
        return route.fulfill({
          status: opts.editStatus,
          json: { code: opts.editCode ?? "validation", detail: "service text" },
        });
      }
      Object.assign(p, {
        summary: null,
        requested_minor: null,
        currency: null,
        tradeoff: null,
        ...body,
      });
      return route.fulfill({ json: p });
    }
    if (path === "/api/proposals") {
      const id = `n${proposals.length}`;
      proposals.unshift(row({ pid: id, ...body }));
      return route.fulfill({ json: { pid: id, status: "draft" } });
    }
    if (path.endsWith("/tradeoff") && p) {
      p.tradeoff = body.tradeoff;
      return route.fulfill({ json: p });
    }
    if (path.endsWith("/approve") && p) {
      if (opts.approveStatus)
        return route.fulfill({
          status: opts.approveStatus,
          json: {
            code: opts.approveCode ?? "unprocessable",
            detail: "service text",
          },
        });
      p.status = "approved";
      return route.fulfill({ json: p });
    }
    return route.fulfill({ json: p ?? {} });
  });
  return sent;
}

test("the board says which proposals are external asks and their trade-off, and the rule before approving", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto("/en-001/proposals");
  await settle(page);
  await expect(
    page.getByText(
      "An external ask from the sponsor must name what it displaces (a trade-off of 10 or more characters) before it can be approved.",
    ),
  ).toBeVisible();
  const table = page.getByRole("table", { name: "Title" });
  await expect(
    table.getByRole("row", { name: /Sponsor dashboard/ }),
  ).toContainText("⚠ External ask. No trade-off named yet.");
  await expect(
    table.getByRole("row", { name: /Partner portal/ }),
  ).toContainText(
    "External ask. Trade-off: Delays the CRM upgrade by a quarter",
  );
  await expect(table.getByRole("row", { name: /Office move/ })).toContainText(
    "Not an external ask",
  );
  // A decided ask's trade-off can no longer change, so it is not offered.
  await expect(
    page.getByRole("button", { name: /trade-off of Old ask/ }),
  ).toHaveCount(0);

  await page.getByRole("button", { name: "Approve Sponsor dashboard" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "Sponsor dashboard is an external ask: name its trade-off (10 or more characters) before approving it.",
  );
  expect(sent).toEqual([]);
  // An ordinary proposal is approved as before.
  await page.getByRole("button", { name: "Approve Office move" }).click();
  await expect
    .poll(() => sent)
    .toEqual([{ method: "POST", path: "/api/proposals/a3/approve", body: {} }]);
});

test("a trade-off is named with its rule said first, nothing sent for the same text, and then the ask approved", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto("/en-001/proposals");
  await settle(page);
  await page
    .getByRole("button", { name: "Name the trade-off of Sponsor dashboard" })
    .click();
  const form = page.getByRole("region", {
    name: "The trade-off of Sponsor dashboard",
  });
  await expect(
    form.getByText(
      "Name what this ask displaces, in 10 to 2000 characters. It can be named or changed until the proposal is approved or rejected, and each change is recorded.",
    ),
  ).toBeVisible();
  const text = form.getByLabel("What it displaces (10 to 2000 characters)");
  const save = form.getByRole("button", { name: "Save the trade-off" });
  await text.fill("  Too short ");
  await save.click();
  await expect(form.getByRole("alert")).toHaveText(
    "A trade-off is 10 to 2000 characters.",
  );
  expect(sent).toEqual([]);
  await text.fill("  Moves the data platform work to next year  ");
  await save.click();
  await expect(form.getByRole("status")).toHaveText(
    "Trade-off saved for Sponsor dashboard.",
  );
  await save.click();
  await expect(form.getByRole("status")).toHaveText(
    "Nothing changed, so nothing was sent.",
  );
  await expect(
    page
      .getByRole("table", { name: "Title" })
      .getByRole("row", { name: /Sponsor dashboard/ }),
  ).toContainText(
    "External ask. Trade-off: Moves the data platform work to next year",
  );
  await page.getByRole("button", { name: "Approve Sponsor dashboard" }).click();
  await expect.poll(() => sent.length).toBe(2);
  expect(sent).toEqual([
    {
      method: "PUT",
      path: "/api/proposals/a1/tradeoff",
      body: { tradeoff: "Moves the data platform work to next year" },
    },
    { method: "POST", path: "/api/proposals/a1/approve", body: {} },
  ]);

  // An ask that already names one is changed with the same form.
  await page
    .getByRole("button", { name: "Change the trade-off of Partner portal" })
    .click();
  await expect(
    page
      .getByRole("region", { name: "The trade-off of Partner portal" })
      .getByLabel("What it displaces (10 to 2000 characters)"),
  ).toHaveValue("Delays the CRM upgrade by a quarter");
});

test("a new proposal is tagged as an external ask, with or without its trade-off", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto("/en-001/proposals");
  await settle(page);
  const form = page.getByRole("form", { name: "New proposal" });
  const open = form.getByRole("button", { name: "Open proposal" });
  await open.click();
  await expect(page.getByRole("alert")).toHaveText("A proposal needs a title.");
  await form.getByLabel("Proposal title").fill("Board reporting app");
  await form.getByLabel("An external ask from the sponsor").check();
  const tradeoff = form.getByLabel(/^What it displaces \(the trade-off/);
  await tradeoff.fill("short");
  await open.click();
  await expect(page.getByRole("alert")).toHaveText(
    "A trade-off is 10 to 2000 characters.",
  );
  expect(sent).toEqual([]);
  await tradeoff.fill(" Holds back the billing rewrite ");
  await open.click();
  await expect.poll(() => sent.length).toBe(1);
  // The form is cleared once the proposal is opened and the board read again.
  await expect(form.getByLabel("Proposal title")).toHaveValue("");
  await form.getByLabel("Proposal title").fill("Sponsor survey");
  await form.getByLabel("An external ask from the sponsor").check();
  await open.click();
  await expect.poll(() => sent.length).toBe(2);
  expect(sent).toEqual([
    {
      method: "POST",
      path: "/api/proposals",
      body: {
        title: "Board reporting app",
        kind_target: "projects",
        external_ask: true,
        tradeoff: "Holds back the billing rewrite",
      },
    },
    {
      method: "POST",
      path: "/api/proposals",
      body: {
        title: "Sponsor survey",
        kind_target: "projects",
        external_ask: true,
      },
    },
  ]);
});

for (const [code, sentence] of [
  [
    "proposal_status_refused",
    "Refused: only a proposal in review can be approved.",
  ],
  [
    "tradeoff_required",
    "Refused: an external ask needs a named trade-off of 10 or more characters before it can be approved.",
  ],
] as const) {
  test(`a refused approval (${code}) is its own sentence, never the service's`, async ({
    page,
  }) => {
    await stub(page, { approveStatus: 422, approveCode: code });
    await page.goto("/en-001/proposals");
    await settle(page);
    await page.getByRole("button", { name: "Approve Office move" }).click();
    await expect(page.getByRole("alert")).toHaveText(sentence);
    await expect(page.getByText("service text")).toHaveCount(0);
  });
}

test("the board reads in Welsh and passes axe with its forms open", async ({
  page,
}) => {
  await stub(page);
  await page.goto("/cy-001/proposals");
  await settle(page);
  await expect(
    page.getByText("Cais allanol. Dim cyfaddawd wedi'i enwi eto."),
  ).toBeVisible();
  await page
    .getByRole("checkbox", { name: "Cais allanol gan y noddwr" })
    .check();
  await page
    .getByRole("button", {
      name: "Enwi'r cyfaddawd ar gyfer Sponsor dashboard",
    })
    .click();
  await expect(
    page.getByRole("region", { name: "Cyfaddawd Sponsor dashboard" }),
  ).toBeVisible();
  await settle(page);
  const results = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    results.violations.filter(
      (v) => v.impact === "serious" || v.impact === "critical",
    ),
  ).toEqual([]);
});

// NC-1 slice 12 part (e) (NC-8): a proposal's lessons from plans whose title is like its own.
test("a proposal shows the lessons of similar plans, with how they were matched said first and no service text", async ({
  page,
}) => {
  await stub(page);
  await page.route(/\/api\/proposals\/[^/]+\/lessons$/, (route) => {
    const pid = new URL(route.request().url()).pathname.split("/").at(-2);
    if (pid === "a3")
      return route.fulfill({
        status: 404,
        json: { code: "not_found", detail: "service text" },
      });
    if (pid === "a2") return route.fulfill({ json: [] });
    return route.fulfill({
      json: [
        {
          plan_pid: "p7",
          plan_name: "Sponsor dashboards 2025",
          match: {
            score: 0.874,
            confidence: "Medium",
            breakdown: { name_score: 0.912, goals_score: null },
          },
          lessons: [
            {
              pid: "l1",
              plan_pid: "p7",
              occasion: "closure",
              category: "recommendation",
              summary: "Agree the data feeds first.",
              detail: "service text",
              recorded_by: null,
              link_kind: null,
              link_pid: null,
              created_at: "2025-06-01T00:00:00Z",
            },
          ],
        },
      ],
    });
  });
  await page.goto("/en-001/proposals");
  await settle(page);
  await page
    .getByRole("button", { name: "Lessons from plans like Sponsor dashboard" })
    .click();
  const shown = page.getByRole("region", {
    name: "Lessons from plans like Sponsor dashboard",
  });
  await expect(shown).toContainText(
    "Matched on the title only, against every plan with lessons, closed or not, best match first.",
  );
  await expect(
    shown.getByRole("link", { name: "Sponsor dashboards 2025" }),
  ).toHaveAttribute("href", "/en-001/plans/p7");
  await expect(shown).toContainText(
    "match 87%, medium confidence. Titles 91% alike.",
  );
  await expect(shown.getByRole("listitem").getByRole("listitem")).toHaveText(
    "→ Recommendation · Closure: Agree the data feeds first.",
  );
  await page
    .getByRole("button", { name: "Lessons from plans like Partner portal" })
    .click();
  await expect(
    page.getByRole("region", {
      name: "Lessons from plans like Partner portal",
    }),
  ).toContainText("No similar plan has lessons.");
  await page
    .getByRole("button", { name: "Lessons from plans like Office move" })
    .click();
  await expect(page.getByRole("alert")).toHaveText(
    "That proposal no longer exists, or you cannot read it.",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
});

// NC-1 slice 12 part (g) (EX-13): a draft read as stored and edited, which is where an existing proposal is tagged as
// an external ask; and the intake's forecast.
test("a draft is read as stored, nothing is sent for no change, every problem is said first, and it is tagged as an external ask", async ({
  page,
}) => {
  const sent = await stub(page, { draft: true });
  await page.goto("/en-001/proposals");
  await settle(page);
  await expect(
    page.getByRole("button", { name: "Edit the draft Sponsor dashboard" }),
  ).toHaveCount(0);
  await page
    .getByRole("button", { name: "Edit the draft Kiosk refresh" })
    .click();
  const d = page.getByRole("region", { name: "Edit Kiosk refresh" });
  await expect(d).toContainText(
    "Only a draft can be edited, which is also where a proposal is tagged as an external ask.",
  );
  await expect(d.getByLabel("Requested (major units)")).toHaveValue("2500.00");
  await d.getByRole("button", { name: "Save the draft" }).click();
  await expect(d.getByRole("status")).toHaveText(
    "Nothing has changed, so nothing was sent.",
  );
  await d.getByLabel("Proposal title").fill(" ");
  await d
    .getByRole("checkbox", { name: "An external ask from the sponsor" })
    .check();
  await d.getByRole("textbox", { name: /What it displaces/ }).fill("short");
  await d.getByRole("button", { name: "Save the draft" }).click();
  await expect(d.getByRole("alert")).toHaveText(
    "A proposal needs a title. A trade-off is 10 to 2000 characters.",
  );
  expect(sent).toEqual([]);
  await d.getByLabel("Proposal title").fill("Kiosk refresh");
  await d
    .getByRole("textbox", { name: /What it displaces/ })
    .fill(" Delays the signage work ");
  await d.getByRole("button", { name: "Save the draft" }).click();
  await expect(d.getByRole("status")).toHaveText("Kiosk refresh saved.");
  expect(sent).toEqual([
    {
      method: "PUT",
      path: "/api/proposals/d1",
      body: {
        title: "Kiosk refresh",
        kind_target: "projects",
        summary: "Lobby kiosks",
        sponsor_ref: "person:00000000-0000-4000-8000-000000000001",
        strategic_rationale: "service text",
        requested_minor: 250000,
        currency: "GBP",
        external_ask: true,
        tradeoff: "Delays the signage work",
      },
    },
  ]);
  await expect(
    page.getByText("External ask. Trade-off: Delays the signage work"),
  ).toBeVisible();
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("an edit refused because the proposal moved on, or a field, is the catalogue's sentence", async ({
  page,
}) => {
  await stub(page, { draft: true, editStatus: 422, editCode: "unprocessable" });
  await page.goto("/en-001/proposals");
  await settle(page);
  await page
    .getByRole("button", { name: "Edit the draft Kiosk refresh" })
    .click();
  const d = page.getByRole("region", { name: "Edit Kiosk refresh" });
  await d.getByLabel("Summary").fill("Lobby and ward kiosks");
  await d.getByRole("button", { name: "Save the draft" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "The edit was refused: the proposal is no longer a draft. The list has been read again.",
  );
  await expect(
    page.getByRole("button", { name: "Edit the draft Kiosk refresh" }),
  ).toHaveCount(0);
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("the forecast: its bounds checked first, the defaults not sent, the figures and their history in the page's words", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto("/en-001/proposals");
  await settle(page);
  const f = page.getByRole("region", { name: "How many will be approved" });
  await expect(f).toContainText(
    "A forecast needs at least 6 periods of history.",
  );
  await f.getByLabel("Periods ahead (1 to 520; empty for 4)").fill("0");
  await f.getByLabel("Days in a period (1 to 90; empty for 7)").fill("91");
  await f.getByRole("button", { name: "Forecast" }).click();
  await expect(f.getByRole("alert")).toHaveText(
    "The periods ahead must be a whole number from 1 to 520. A period must be a whole number of days from 1 to 90.",
  );
  expect(sent).toEqual([]);
  await f.getByLabel("Periods ahead (1 to 520; empty for 4)").fill("4");
  await f.getByLabel("Days in a period (1 to 90; empty for 7)").fill("");
  await f.getByLabel("Periods of history (1 to 260; empty for 12)").fill("26");
  await f.getByRole("button", { name: "Forecast" }).click();
  await expect(f).toContainText(
    "In the next 4 periods (28 days), at least 1 proposal · with 85% confidence: the figure to plan on",
  );
  await expect(f).toContainText("Typically 5 proposals");
  await expect(f).toContainText("At most 8 proposals");
  await expect(f).toContainText("Approved per period, oldest first: 1, 0, 2");
  await expect(f).toContainText("they do not feed the forecast): 3, 1, 2");
  expect(sent).toEqual([
    {
      method: "GET",
      path: "/api/proposals/forecast?history_periods=26",
      body: null,
    },
  ]);
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("a forecast with too little history says so in the page's words", async ({
  page,
}) => {
  await stub(page, { history: 3 });
  await page.goto("/en-001/proposals");
  await settle(page);
  const f = page.getByRole("region", { name: "How many will be approved" });
  await f.getByRole("button", { name: "Forecast" }).click();
  await expect(f).toContainText(
    "No forecast: it needs at least 6 periods of history, and there are 3.",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("editing a draft and the forecast read in Welsh and pass axe", async ({
  page,
}) => {
  await stub(page, { draft: true });
  await page.goto("/cy-001/proposals");
  await settle(page);
  await page
    .getByRole("button", { name: "Golygu'r drafft Kiosk refresh" })
    .click();
  await expect(
    page.getByRole("region", { name: "Golygu Kiosk refresh" }),
  ).toContainText("Dim ond drafft y gellir ei olygu");
  const f = page.getByRole("region", {
    name: "Faint fydd yn cael eu cymeradwyo",
  });
  await f.getByRole("button", { name: "Rhagweld" }).click();
  await expect(f).toContainText("gyda hyder o 85%");
  await settle(page);
  const results = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    results.violations.filter(
      (v) => v.impact === "serious" || v.impact === "critical",
    ),
  ).toEqual([]);
});
