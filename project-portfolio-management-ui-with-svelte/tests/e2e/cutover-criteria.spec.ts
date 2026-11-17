// NC-1 slice 11 (EX-6): changing a draft cutover decision's stop criteria, over stubbed API answers. The rules
// (1 to 30 criteria, each 5 to 500 characters) are said before anything is sent; an unchanged list sends nothing; a
// changed one is sent whole, in order; a decision published meanwhile (409) is said in the catalogue's words. The
// new-draft form says the same rules first. Welsh; axe on the open editor.
import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";

const PID = "11111111-1111-4111-8111-111111111111";
const DRAFT = {
  pid: "d9",
  plan_pid: PID,
  title: "Ticketing cutover",
  version: 1,
  supersedes_pid: null,
  status: "draft",
  published_on: null,
  opened_on: null,
  outcome: null,
  decided_on: null,
  decision_reason: null,
  criteria: [
    {
      n: 1,
      criterion: "Rollback rehearsed",
      met: false,
      evidence: null,
      marked_on: null,
    },
    {
      n: 2,
      criterion: "Fares loaded and checked",
      met: false,
      evidence: null,
      marked_on: null,
    },
  ],
  cosignatures: { required: ["safety"], signed: [], missing: ["safety"] },
  go_would_be_refused_because: [],
};

async function stub(page: Page, opts: { status?: number } = {}) {
  const sent: Array<{ method: string; body: unknown }> = [];
  const decision = structuredClone(DRAFT);
  await page.route(`**/api/plans/${PID}/cutover-decisions`, (route) => {
    if (route.request().method() === "GET")
      return route.fulfill({
        json: { decisions: [decision], required_cosigners: ["safety"] },
      });
    sent.push({ method: "POST", body: route.request().postDataJSON() });
    return route.fulfill({ json: decision });
  });
  await page.route(
    `**/api/plans/${PID}/cutover-decisions/d9/criteria`,
    (route) => {
      const body = route.request().postDataJSON();
      sent.push({ method: "PUT", body });
      if (opts.status)
        return route.fulfill({
          status: opts.status,
          json: { error: "conflict", description: "service text" },
        });
      decision.criteria = body.criteria.map((criterion: string, i: number) => ({
        n: i + 1,
        criterion,
        met: false,
        evidence: null,
        marked_on: null,
      }));
      return route.fulfill({ json: decision });
    },
  );
  return sent;
}

test("a draft's criteria are checked first, sent whole and in order, and nothing is sent for no change", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto(`/en-001/plans/${PID}/cutover`);
  const sec = page.getByRole("region", { name: /Ticketing cutover/ });
  await sec.getByRole("button", { name: "Change the stop criteria" }).click();
  const box = sec.getByLabel("Stop criteria, one per line");
  await expect(box).toHaveValue("Rollback rehearsed\nFares loaded and checked");
  await sec.getByRole("button", { name: "Save the stop criteria" }).click();
  await expect(sec.getByRole("status")).toHaveText(
    "Nothing changed, so nothing was sent.",
  );
  await box.fill("Rollback rehearsed\nok");
  await sec.getByRole("button", { name: "Save the stop criteria" }).click();
  await expect(sec.getByRole("alert")).toHaveText(
    "Stop criterion 2 must be 5 to 500 characters.",
  );
  await box.fill("\n  \n");
  await sec.getByRole("button", { name: "Save the stop criteria" }).click();
  await expect(sec.getByRole("alert")).toHaveText(
    "A decision needs 1 to 30 stop criteria.",
  );
  expect(sent).toEqual([]);
  await box.fill(
    "Fares loaded and checked\n  Rollback rehearsed  \n\nStaff briefed on the new gates",
  );
  await sec.getByRole("button", { name: "Save the stop criteria" }).click();
  await expect(sec.getByRole("status")).toHaveText(
    "The stop criteria are saved.",
  );
  await expect(
    sec.getByRole("rowheader", { name: "Staff briefed on the new gates" }),
  ).toBeVisible();
  expect(sent).toEqual([
    {
      method: "PUT",
      body: {
        criteria: [
          "Fares loaded and checked",
          "Rollback rehearsed",
          "Staff briefed on the new gates",
        ],
      },
    },
  ]);
});

test("a decision published meanwhile is said in the catalogue's words", async ({
  page,
}) => {
  await stub(page, { status: 409 });
  await page.goto(`/en-001/plans/${PID}/cutover`);
  const sec = page.getByRole("region", { name: /Ticketing cutover/ });
  await sec.getByRole("button", { name: "Change the stop criteria" }).click();
  await sec
    .getByLabel("Stop criteria, one per line")
    .fill("Rollback rehearsed twice");
  await sec.getByRole("button", { name: "Save the stop criteria" }).click();
  await expect(sec.getByRole("alert")).toHaveText(
    "This decision has been published, so its criteria are frozen: make a new version to change them.",
  );
  await expect(sec.getByRole("alert")).not.toContainText("service text");
});

test("a new draft says the length rule before anything is sent", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto(`/en-001/plans/${PID}/cutover`);
  const sec = page.getByRole("region", { name: "Draft a cutover decision" });
  await sec.getByLabel("Title").fill("Payments cutover");
  await sec
    .getByLabel("Stop criteria, one on each line")
    .fill("Card terminals updated\nno");
  await sec.getByRole("button", { name: "Draft the decision" }).click();
  await expect(sec.getByRole("alert")).toHaveText(
    "Stop criterion 2 must be 5 to 500 characters.",
  );
  expect(sent).toEqual([]);
});

test("the criteria editor reads in Welsh and passes axe", async ({ page }) => {
  await stub(page);
  await page.goto(`/cy-001/plans/${PID}/cutover`);
  await page.getByRole("button", { name: "Newid y meini prawf atal" }).click();
  await expect(
    page.getByLabel("Meini prawf atal, un i bob llinell"),
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
