// NC-1 slice 12 part (e) (NC-8, BR-4): a plan's lessons learned and what refers to a record, in a real browser over a
// stubbed service. The plan page links here; the rules are said first (a lesson is never changed, only withdrawn);
// each lesson shows its category as a word and a shape, its occasion, who recorded it and the record it is about;
// recording checks every rule first and sends what was typed, trimmed, with no blank detail; withdrawing asks first and
// Cancel sends nothing; what refers to a record lists the meeting items and lessons with a count; each refusal is the
// catalogue's sentence; the service's English is never shown. Welsh; axe with the form open.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle, stubPlansApi } from "./fixtures";

const L1 = {
  pid: "l1",
  plan_pid: PID,
  occasion: "g2_definition",
  category: "went_badly",
  summary: "Suppliers need a second source.",
  detail: "Two quotes at least.",
  recorded_by: "person:pmo-2",
  link_kind: "risk",
  link_pid: "r1",
  created_at: "2026-11-10T09:00:00Z",
};
const L2 = {
  ...L1,
  pid: "l2",
  occasion: "closure",
  category: "went_well",
  summary: "Weekly demos kept the sponsor close.",
  detail: null,
  recorded_by: null,
  link_kind: null,
  link_pid: null,
};

async function stub(
  page: Page,
  opts: { refuse?: Record<string, number> } = {},
) {
  const lessons = [structuredClone(L1), structuredClone(L2)];
  const sent: Array<{ method: string; path: string; body: unknown }> = [];
  await stubPlansApi(page);
  await page.route(
    /\/api\/plans\/[^/]+\/(lessons|references|tasks|risks|milestones|issues)(\/.*)?(\?.*)?$/,
    async (route) => {
      const req = route.request();
      const url = new URL(req.url());
      const path = url.pathname.replace(/^\/api\/proxy/, "");
      const parts = path.split("/").slice(4);
      if (req.method() === "GET") {
        if (parts[0] === "tasks")
          return route.fulfill({
            json: {
              tasks: [{ pid: "t1", title: "Order the servers" }],
              counts: {},
            },
          });
        if (parts[0] === "risks")
          return route.fulfill({
            json: [{ pid: "r1", title: "Supplier insolvency" }],
          });
        if (parts[0] === "milestones") return route.fulfill({ json: [] });
        if (parts[0] === "issues")
          return route.fulfill({
            json: { plan_pid: PID, issues: [], total: 0, open: 0 },
          });
        if (parts[0] === "references") {
          if (url.searchParams.get("pid") !== "r1")
            return route.fulfill({
              status: 404,
              json: { code: "not_found", detail: "service text" },
            });
          return route.fulfill({
            json: {
              target: { kind: "risk", pid: "r1", title: "Supplier insolvency" },
              meetings: [
                {
                  meeting_pid: "m1",
                  meeting_title: "Gate 2 review",
                  meeting_status: "planned",
                  scheduled_on: "2026-11-24",
                  part: "agenda",
                  outcome_kind: null,
                  text: "Budget",
                },
              ],
              lessons: [
                {
                  lesson_pid: "l1",
                  occasion: "g2_definition",
                  category: "went_badly",
                  summary: L1.summary,
                },
              ],
              count: 2,
            },
          });
        }
        return route.fulfill({ json: lessons });
      }
      const body = req.postDataJSON();
      sent.push({ method: req.method(), path, body });
      const action = parts.length === 1 ? "record" : "withdraw";
      const refuse = opts.refuse?.[action];
      if (refuse)
        return route.fulfill({
          status: refuse,
          json: {
            code: refuse === 404 ? "not_found" : "unprocessable",
            detail: "service text",
          },
        });
      if (action === "record") {
        const row = {
          ...L2,
          ...body,
          pid: "l9",
          link_kind: null,
          link_pid: null,
          detail: body.detail ?? null,
        };
        lessons.push(row);
        return route.fulfill({ json: row });
      }
      const at = lessons.findIndex((l) => l.pid === parts[1]);
      lessons.splice(at, 1);
      return route.fulfill({ json: { withdrawn: parts[1] } });
    },
  );
  return sent;
}

test("the plan page links here; the rules are said first; each lesson is shown with no service text", async ({
  page,
}) => {
  await stub(page);
  await page.goto(`/en-001/plans/${PID}`);
  await settle(page);
  await page.getByRole("link", { name: "Lessons learned" }).click();
  await expect(
    page.getByRole("heading", { level: 1, name: "Lessons learned" }),
  ).toBeVisible();
  await expect(page.getByTestId("lessons-rules")).toContainText(
    "It is never changed: it can only be withdrawn, and a withdrawn lesson is no longer shown (it stays in the audit trail).",
  );
  const list = page.getByRole("list", {
    name: "The plan's lessons, oldest first",
  });
  const items = list.getByRole("listitem");
  await expect(items.nth(0)).toContainText(
    "✗ Went badly · G2 definition · recorded on 10 Nov 2026 by person:pmo-2",
  );
  await expect(items.nth(0)).toContainText("Two quotes at least.");
  await expect(items.nth(0)).toContainText("about Risk: Supplier insolvency");
  await expect(items.nth(1)).toContainText(
    "✓ Went well · Closure · recorded on 10 Nov 2026",
  );
  await expect(page.getByText("2 lessons.")).toBeVisible();
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("recording a lesson checks every rule first, then sends what was typed, trimmed, with no blank detail", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto(`/en-001/plans/${PID}/lessons`);
  await settle(page);
  const region = page.getByRole("region", { name: "Record a lesson" });
  await region.getByRole("button", { name: "Record the lesson" }).click();
  await expect(region.getByRole("alert").getByRole("listitem")).toHaveText([
    "Choose a gate or closure.",
    "Choose went well, went badly or recommendation.",
    "Write a summary of up to 300 characters.",
  ]);
  await region
    .getByLabel("Detail (optional, up to 1024 bytes)")
    .fill("é".repeat(513));
  await region.getByLabel("When").selectOption({ label: "Closure" });
  await region.getByRole("radio", { name: "Recommendation" }).check();
  await region
    .getByLabel("Summary (up to 300 characters)")
    .fill("  Book the room early. ");
  await region.getByRole("button", { name: "Record the lesson" }).click();
  await expect(region.getByRole("alert")).toHaveText(
    "The detail is longer than 1024 bytes.",
  );
  expect(sent).toEqual([]);
  await region.getByLabel("Detail (optional, up to 1024 bytes)").fill("   ");
  await expect(
    region.getByLabel("The record it is about (optional)"),
  ).toBeEnabled();
  await region
    .getByLabel("The record it is about (optional)")
    .selectOption({ label: "Order the servers" });
  await region.getByRole("button", { name: "Record the lesson" }).click();
  await expect(page.getByTestId("page-notice")).toHaveText(
    "Recorded: Book the room early.",
  );
  expect(sent).toEqual([
    {
      method: "POST",
      path: `/api/plans/${PID}/lessons`,
      body: {
        occasion: "closure",
        category: "recommendation",
        summary: "Book the room early.",
        link: { kind: "task", pid: "t1" },
      },
    },
  ]);
  await expect(page.getByText("3 lessons.")).toBeVisible();
});

test("withdrawing a lesson asks first; Cancel sends nothing; a lesson already withdrawn is the catalogue's sentence", async ({
  page,
}) => {
  const sent = await stub(page, { refuse: { withdraw: 404 } });
  await page.goto(`/en-001/plans/${PID}/lessons`);
  await settle(page);
  const withdraw = page.getByRole("button", {
    name: "Withdraw the lesson Suppliers need a second source.",
  });
  await withdraw.click();
  const ask = page.getByRole("alertdialog");
  await expect(ask).toContainText(
    "Withdraw the lesson “Suppliers need a second source.”? It will no longer be shown",
  );
  await ask.getByRole("button", { name: "Cancel" }).click();
  expect(sent).toEqual([]);
  await withdraw.click();
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  await expect(page.getByTestId("lessons-refusal")).toHaveText(
    "Refused: that lesson has already been withdrawn. The list was read again.",
  );
  expect(sent.map((s) => s.path)).toEqual([
    `/api/plans/${PID}/lessons/l1/withdraw`,
  ]);
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("withdrawing a lesson takes it off the list", async ({ page }) => {
  await stub(page);
  await page.goto(`/en-001/plans/${PID}/lessons`);
  await settle(page);
  await page
    .getByRole("button", {
      name: "Withdraw the lesson Weekly demos kept the sponsor close.",
    })
    .click();
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  await expect(page.getByTestId("page-notice")).toHaveText(
    "Withdrawn: Weekly demos kept the sponsor close.",
  );
  await expect(page.getByText("1 lesson.")).toBeVisible();
});

test("what refers to a record lists the meeting items and lessons with a count; a record gone is the catalogue's sentence", async ({
  page,
}) => {
  await stub(page);
  await page.goto(`/en-001/plans/${PID}/lessons`);
  await settle(page);
  const region = page.getByRole("region", { name: "What refers to a record" });
  const show = region.getByRole("button", { name: "Show what refers to it" });
  await show.click();
  await expect(region.getByRole("alert")).toHaveText("Choose a record first.");
  await region
    .getByRole("combobox")
    .selectOption({ label: "Supplier insolvency" });
  await show.click();
  const refs = region.getByTestId("refs");
  await expect(refs).toContainText(
    "Risk: Supplier insolvency: 2 items refer to it.",
  );
  await expect(
    refs.getByRole("list", { name: "Meeting items" }).getByRole("listitem"),
  ).toHaveText("Gate 2 review (○ planned, 24 Nov 2026): Agenda: Budget");
  await expect(
    refs
      .getByRole("list", { name: "Lessons that point at it" })
      .getByRole("listitem"),
  ).toHaveText("✗ Went badly · G2 definition: Suppliers need a second source.");
  await region
    .getByRole("combobox")
    .selectOption({ label: "Order the servers" });
  await show.click();
  await expect(region.getByRole("alert")).toHaveText(
    "That record is no longer a record of this plan, or you cannot read it.",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("a refused lesson is the catalogue's sentence", async ({ page }) => {
  await stub(page, { refuse: { record: 422 } });
  await page.goto(`/en-001/plans/${PID}/lessons`);
  await settle(page);
  const region = page.getByRole("region", { name: "Record a lesson" });
  await region.getByLabel("When").selectOption({ label: "Closure" });
  await region.getByRole("radio", { name: "Went well" }).check();
  await region
    .getByLabel("Summary (up to 300 characters)")
    .fill("Demos helped.");
  await region.getByRole("button", { name: "Record the lesson" }).click();
  await expect(page.getByTestId("lessons-refusal")).toHaveText(
    "Refused: the record chosen is no longer a record of this plan. Nothing was recorded; choose it again.",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("the page reads in Welsh and passes axe with the references shown", async ({
  page,
}) => {
  await stub(page);
  await page.goto(`/cy-001/plans/${PID}/lessons`);
  await settle(page);
  await expect(
    page.getByRole("heading", { level: 1, name: "Gwersi a ddysgwyd" }),
  ).toBeVisible();
  const region = page.locator("section").nth(2);
  await region.locator("select").selectOption({ label: "Supplier insolvency" });
  await page
    .getByRole("button", { name: "Dangos yr hyn sy'n cyfeirio ato" })
    .click();
  await expect(page.getByTestId("refs")).toBeVisible();
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
