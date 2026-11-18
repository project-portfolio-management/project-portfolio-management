// Saved reports (CQ-24): list, save a definition, run it into a table, and the CSV link. And the time report (NC-1
// slice 12 part h, RM-10): its rules said and checked first, the dates' defaults not sent, the groups in words with
// billable and not billable apart, no entries said, the entries left out counted, the CSV link, a refusal in the
// catalogue's words; Welsh and axe.
import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";

const REPORT = {
  pid: "r1",
  name: "Late projects",
  collection: "projects",
  filters: {},
  fields: ["name", "status"],
};
const RUN = {
  name: "Late projects",
  collection: "projects",
  rows: 2,
  data: [
    { name: "Website replatform", status: "Active" },
    { name: "Data centre exit", status: "On hold" },
  ],
};

test.describe("reports", () => {
  test("lists a saved report and runs it into a table", async ({ page }) => {
    await page.route("**/api/reports", (r) => r.fulfill({ json: [REPORT] }));
    await page.route("**/api/reports/r1/run**", (r) =>
      r.fulfill({ json: RUN }),
    );
    await page.goto("/en-001/reports", { waitUntil: "networkidle" });
    await expect(
      page.getByRole("heading", { level: 1, name: "Reports" }),
    ).toBeVisible();
    await expect(page.getByText("Late projects")).toBeVisible();
    await expect(page.getByRole("link", { name: "CSV" })).toHaveAttribute(
      "href",
      /\/api\/reports\/r1\/run\?format=csv$/,
    );
    await page.getByRole("button", { name: "Run" }).click();
    await expect(page.getByText("2 rows")).toBeVisible();
    await expect(
      page.getByRole("row", { name: /Website replatform\s+Active/ }),
    ).toBeVisible();
    await expect(
      page.getByRole("row", { name: /Data centre exit\s+On hold/ }),
    ).toBeVisible();
  });

  test("saves a definition with the filter and the field list", async ({
    page,
  }) => {
    let posted: Record<string, unknown> | null = null;
    await page.route("**/api/reports", async (r) => {
      if (r.request().method() === "POST") {
        posted = r.request().postDataJSON();
        return r.fulfill({ status: 201, json: { pid: "r2" } });
      }
      return r.fulfill({ json: [] });
    });
    await page.goto("/en-001/reports", { waitUntil: "networkidle" });
    await page.getByPlaceholder("Report name").fill("Replatform");
    await page.getByPlaceholder("name contains…").fill("replatform");
    await page
      .getByPlaceholder("fields (comma separated)")
      .fill("name, status ,");
    await page.getByRole("button", { name: "Save" }).click();
    await expect
      .poll(() => posted)
      .toMatchObject({
        name: "Replatform",
        collection: "projects",
        filters: { name_like: "replatform" },
        fields: ["name", "status"],
      });
  });

  test("says so when a report cannot be run", async ({ page }) => {
    await page.route("**/api/reports", (r) => r.fulfill({ json: [REPORT] }));
    await page.route("**/api/reports/r1/run**", (r) =>
      r.fulfill({ status: 500, json: { error: "boom" } }),
    );
    await page.goto("/en-001/reports", { waitUntil: "networkidle" });
    await page.getByRole("button", { name: "Run" }).click();
    // The time report below has its own alert when its plan list cannot be read.
    await expect(
      page.getByRole("alert").filter({ hasText: "run failed" }),
    ).toBeVisible();
  });

  test("passes axe with a report run and shown", async ({ page }) => {
    await page.route("**/api/reports", (r) => r.fulfill({ json: [REPORT] }));
    await page.route("**/api/reports/r1/run**", (r) =>
      r.fulfill({ json: RUN }),
    );
    await page.goto("/en-001/reports", { waitUntil: "networkidle" });
    await page.getByRole("button", { name: "Run" }).click();
    await expect(
      page.getByRole("row", { name: /Website replatform/ }),
    ).toBeVisible();
    const { default: AxeBuilder } = await import("@axe-core/playwright");
    const r = await new AxeBuilder({ page })
      .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
      .analyze();
    expect(
      r.violations
        .filter((v) => v.impact === "serious" || v.impact === "critical")
        .map(
          (v) =>
            `${v.id}: ${v.nodes.map((n) => n.target.join(" ")).join(" | ")}`,
        ),
    ).toEqual([]);
  });
});

const PLAN_A = "11111111-1111-4111-8111-111111111111";
const ME = "person:6f1c2a3b-0000-4000-8000-000000000009";

/** The plan list and the time report; `answer` is what the report answers with, or a status to refuse it. */
async function stubTime(page: Page, answer: "groups" | "empty" | number) {
  const asked: string[] = [];
  await page.route("**/api/reports", (r) => r.fulfill({ json: [] }));
  await page.route(/\/api\/proxy\/api\/plans(\?.*)?$/, (r) =>
    r.fulfill({ json: [{ pid: PLAN_A, name: "Website replatform" }] }),
  );
  await page.route(/\/api\/proxy\/api\/time-reports/, (r) => {
    const url = new URL(r.request().url());
    asked.push(`${url.pathname.replace(/^\/api\/proxy/, "")}${url.search}`);
    if (typeof answer === "number")
      return r.fulfill({
        status: answer,
        json: { code: "unprocessable", detail: "service text" },
      });
    const by = url.searchParams.get("group_by");
    const groups =
      answer === "empty"
        ? []
        : by === "billable"
          ? [
              {
                key: "billable",
                entries: 2,
                split: { billable: 150, non_billable: 0 },
                minutes: 150,
              },
              {
                key: "non_billable",
                entries: 1,
                split: { billable: 0, non_billable: 30 },
                minutes: 30,
              },
            ]
          : [
              {
                key: PLAN_A,
                entries: 2,
                split: { billable: 90, non_billable: 30 },
                minutes: 120,
              },
              {
                key: "22222222-2222-4222-8222-222222222222",
                entries: 1,
                split: { billable: 60, non_billable: 0 },
                minutes: 60,
              },
            ];
    return r.fulfill({
      json: {
        group_by: by,
        from: "2026-08-20",
        to: "2026-11-17",
        groups,
        split:
          answer === "empty"
            ? { billable: 0, non_billable: 0 }
            : { billable: 150, non_billable: 30 },
        total_minutes: answer === "empty" ? null : 180,
        reason: answer === "empty" ? "no_entries" : null,
        omitted_unreadable: answer === "empty" ? 0 : 2,
        truncated: false,
      },
    });
  });
  return asked;
}

test("the time report: checked first, the defaults not sent, groups in words with both halves, and the CSV", async ({
  page,
}) => {
  const asked = await stubTime(page, "groups");
  await page.goto("/en-001/reports", { waitUntil: "networkidle" });
  const s = page.getByTestId("time-report");
  await expect(s).toContainText(
    "with billable and not billable always apart. Leave the dates empty for the last 90 days to today.",
  );
  const form = s.getByRole("form", { name: "Ask for a time report" });
  await expect(form.getByRole("combobox", { name: /^Plan\b/ })).toBeEnabled();
  await form.getByLabel("From").fill("2026-12-01");
  await form.getByLabel("To", { exact: true }).fill("2026-11-01");
  await form.getByLabel("Person (a reference)").fill("alice");
  await form.getByRole("button", { name: "Show the time report" }).click();
  await expect(s.getByRole("alert")).toHaveText(
    "The first day must not be after the last. A person is person: or worker: and an identifier, such as person:6f1c2a3b-0000-4000-8000-000000000009.",
  );
  expect(asked).toEqual([]);
  await form.getByLabel("From").fill("");
  await form.getByLabel("To", { exact: true }).fill("");
  await form.getByLabel("Person (a reference)").fill("");
  await form.getByRole("button", { name: "Show the time report" }).click();
  await expect(s.getByTestId("time-report-total")).toHaveText(
    "In all: 2 hrs 30 mins billable, 30 mins not billable, 3 hrs together.",
  );
  const table = s.getByTestId("time-report-table");
  await expect(
    table.getByRole("row", {
      name: /Website replatform\s+2\s+1 hr 30 mins\s+30 mins\s+2 hrs/,
    }),
  ).toBeVisible();
  await expect(table).toContainText("A plan not named here (22222222)");
  await expect(s).toContainText(
    "2 entries in plans you cannot read are left out.",
  );
  await expect(
    s.getByRole("link", { name: "Download this time report as CSV" }),
  ).toHaveAttribute(
    "href",
    /\/api\/proxy\/api\/time-reports\?group_by=plan&format=csv$/,
  );
  await form.getByLabel("Group by").selectOption({ label: "Billable or not" });
  await form
    .getByRole("combobox", { name: /^Plan\b/ })
    .selectOption({ label: "Website replatform" });
  await form.getByLabel("Person (a reference)").fill(ME);
  await form.getByLabel("From").fill("2026-11-01");
  await form.getByRole("button", { name: "Show the time report" }).click();
  await expect(
    s
      .getByTestId("time-report-table")
      .getByRole("row", { name: /^Not billable/ }),
  ).toBeVisible();
  expect(asked).toEqual([
    "/api/time-reports?group_by=plan",
    `/api/time-reports?group_by=billable&from=2026-11-01&plan=${PLAN_A}&person=person%3A6f1c2a3b-0000-4000-8000-000000000009`,
  ]);
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("a time report with no entries says so, never zero, and a refused one is the catalogue's sentence", async ({
  page,
}) => {
  await stubTime(page, "empty");
  await page.goto("/en-001/reports", { waitUntil: "networkidle" });
  const s = page.getByTestId("time-report");
  await s.getByRole("button", { name: "Show the time report" }).click();
  await expect(s).toContainText(
    "No time was recorded in these plans in that window.",
  );
  await expect(s.getByTestId("time-report-table")).toHaveCount(0);
  await page.unrouteAll();
  await stubTime(page, 422);
  await s.getByRole("button", { name: "Show the time report" }).click();
  await expect(s.getByRole("alert")).toHaveText(
    "The time report was not accepted for those choices.",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("the time report reads in Welsh and passes axe with a report shown", async ({
  page,
}) => {
  await stubTime(page, "groups");
  await page.goto("/cy-001/reports", { waitUntil: "networkidle" });
  const s = page.getByTestId("time-report");
  await s.getByRole("button", { name: "Dangos yr adroddiad amser" }).click();
  await expect(s.getByTestId("time-report-total")).toContainText("Y cyfan:");
  const r = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    r.violations
      .filter((v) => v.impact === "serious" || v.impact === "critical")
      .map((v) => v.id),
  ).toEqual([]);
});
