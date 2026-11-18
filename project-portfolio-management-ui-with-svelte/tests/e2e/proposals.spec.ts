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

async function stub(page: Page, opts: { approveStatus?: number } = {}) {
  const proposals = [
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
    const path = new URL(req.url()).pathname.replace(/^\/api\/proxy/, "");
    if (req.method() === "GET") {
      if (path === "/api/proposals") return route.fulfill({ json: proposals });
      return route.fulfill({ json: [] });
    }
    const body = req.postDataJSON();
    sent.push({ method: req.method(), path, body });
    const pid = path.split("/")[3] ?? "";
    const p = proposals.find((x) => x.pid === pid);
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
          json: { code: "unprocessable", detail: "service text" },
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

test("a refused approval is the catalogue's sentence, never the service's", async ({
  page,
}) => {
  await stub(page, { approveStatus: 422 });
  await page.goto("/en-001/proposals");
  await settle(page);
  await page.getByRole("button", { name: "Approve Office move" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "Refused: only a proposal in review can be approved, and an external ask needs a named trade-off of 10 or more characters first.",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
});

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
