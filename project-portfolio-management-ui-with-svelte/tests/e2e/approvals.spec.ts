// PL-1 (NC-1 slice 12 part n) in a real browser: `/approvals` lists what waits on the reader and what the reader
// asked for; a request is approved, or rejected only with a reason; a pending one the reader asked for is withdrawn
// after a question; a new one is asked with every problem said first; a request settled meanwhile (409) is said in the
// page's words. Welsh; axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle, stubPlansApi } from "./fixtures";

const ME = "3f2c6b7e-0000-4000-8000-000000000001";
const ANA = "3f2c6b7e-0000-4000-8000-000000000002";
const req = (pid: string, over: Record<string, unknown>) => ({
  pid,
  subject_kind: "plan",
  subject_pid: PID,
  requested_by: ANA,
  approver_ref: `person:${ME}`,
  message: null,
  status: "pending",
  decision_comment: null,
  decided_by: null,
  decided_at: null,
  created_at: "2026-11-26T10:00:00Z",
  process_execution_pid: null,
  process_step: null,
  ...over,
});

async function stub(page: Page, opts: { decideStatus?: number } = {}) {
  const sent: Array<{ path: string; body: unknown }> = [];
  let waiting = [req("r1", { message: "Please sign off the budget" })];
  let asked = [
    req("r2", {
      requested_by: ME,
      approver_ref: `person:${ANA}`,
      subject_kind: "risk",
      subject_pid: ANA,
    }),
    req("r3", {
      requested_by: ME,
      approver_ref: `person:${ANA}`,
      status: "rejected",
      decision_comment: "Over budget",
      decided_at: "2026-11-26T12:00:00Z",
    }),
  ];
  await stubPlansApi(page);
  await page.route(/\/api\/approvals(\/.*)?(\?.*)?$/, async (route) => {
    const r = route.request();
    const url = new URL(r.url());
    const path = url.pathname.replace(/^\/api\/proxy/, "");
    if (r.method() === "GET")
      return route.fulfill({
        json: url.searchParams.get("mine") === "approver" ? waiting : asked,
      });
    sent.push({ path, body: r.postDataJSON() ?? null });
    if (path.endsWith("/decide")) {
      if (opts.decideStatus)
        return route.fulfill({
          status: opts.decideStatus,
          json: { code: "conflict", detail: "service text" },
        });
      waiting = [];
      return route.fulfill({ json: {} });
    }
    if (path.endsWith("/withdraw")) {
      asked = asked.map((a) =>
        a.pid === "r2" ? { ...a, status: "withdrawn" } : a,
      );
      return route.fulfill({ json: asked[0] });
    }
    const b = r.postDataJSON() as Record<string, unknown>;
    asked = [req("r4", { ...b, requested_by: ME }), ...asked];
    return route.fulfill({ json: asked[0] });
  });
  return sent;
}

test("what waits is approved; a rejection needs a reason; a pending request is withdrawn after a question", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto("/en-001/approvals");
  await settle(page);
  const waiting = page.getByTestId("waiting").getByRole("listitem");
  await expect(waiting).toHaveCount(1);
  await expect(waiting.nth(0)).toContainText("Please sign off the budget");
  await expect(waiting.nth(0).getByRole("link", { name: PID })).toHaveAttribute(
    "href",
    `/en-001/plans/${PID}`,
  );
  const asked = page.getByTestId("asked").getByRole("listitem");
  await expect(asked.nth(0)).toContainText("Waiting");
  await expect(asked.nth(1)).toContainText("Rejected");
  await expect(asked.nth(1)).toContainText("Their comment: Over budget");
  await expect(
    asked.nth(1).getByRole("button", { name: "Withdraw" }),
  ).toHaveCount(0);
  const a = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    a.violations
      .filter((v) => v.impact === "serious" || v.impact === "critical")
      .map((v) => v.id),
  ).toEqual([]);

  await waiting.nth(0).getByRole("button", { name: "Reject" }).click();
  await expect(page.getByTestId("approvals-error")).toHaveText(
    "A rejection must say why: write a comment.",
  );
  expect(sent).toEqual([]);
  await waiting
    .nth(0)
    .getByLabel(/^Comment/)
    .fill(" Not this quarter ");
  await waiting.nth(0).getByRole("button", { name: "Reject" }).click();
  await expect(page.getByTestId("approvals-notice")).toHaveText(
    "Recorded your rejection.",
  );
  expect(sent).toEqual([
    {
      path: "/api/approvals/r1/decide",
      body: { decision: "rejected", comment: "Not this quarter" },
    },
  ]);
  await expect(page.getByTestId("waiting-none")).toBeVisible();

  const ask = page.getByRole("alertdialog");
  await asked.nth(0).getByRole("button", { name: "Withdraw" }).click();
  await expect(ask).toContainText("Withdraw this request?");
  await ask.getByRole("button", { name: "OK" }).click();
  await expect(page.getByTestId("approvals-notice")).toHaveText(
    "Withdrew the request.",
  );
  expect(sent.at(-1)).toEqual({
    path: "/api/approvals/r2/withdraw",
    body: null,
  });
  await expect(asked.nth(0)).toContainText("Withdrawn");
});

test("asking says every problem first and sends a clean request", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto("/en-001/approvals");
  await settle(page);
  const form = page.getByRole("form", { name: "Ask for an approval" });
  await form.getByLabel("Its id (a UUID)").fill("abc");
  await form.getByLabel(/^Who is asked/).fill("person:ana");
  await form.getByRole("button", { name: "Ask for the approval" }).click();
  await expect(
    page.getByTestId("ask-problems").getByRole("listitem"),
  ).toHaveText([
    "The subject's id must be a UUID.",
    "The person asked must be person: or worker: followed by a UUID.",
  ]);
  expect(sent).toEqual([]);
  await form
    .getByRole("combobox", { name: "What it is about" })
    .selectOption("task");
  await form.getByLabel("Its id (a UUID)").fill(` ${PID} `);
  await form.getByLabel(/^Who is asked/).fill(`worker:${ANA}`);
  await form.getByRole("button", { name: "Ask for the approval" }).click();
  await expect(page.getByTestId("approvals-notice")).toHaveText(
    `Asked worker:${ANA} to approve.`,
  );
  expect(sent).toEqual([
    {
      path: "/api/approvals",
      body: {
        subject_kind: "task",
        subject_pid: PID,
        approver_ref: `worker:${ANA}`,
        message: null,
      },
    },
  ]);
});

test("a request decided meanwhile is said in the page's words, and the lists are read again", async ({
  page,
}) => {
  await stub(page, { decideStatus: 409 });
  await page.goto("/en-001/approvals");
  await settle(page);
  await page
    .getByTestId("waiting")
    .getByRole("button", { name: "Approve" })
    .click();
  await expect(page.getByTestId("approvals-error")).toHaveText(
    /^This request was already decided or withdrawn/,
  );
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("reads in Welsh", async ({ page }) => {
  await stub(page);
  await page.goto("/cy-001/approvals");
  await expect(
    page.getByRole("heading", { level: 1, name: "Cymeradwyaethau" }),
  ).toBeVisible();
});
