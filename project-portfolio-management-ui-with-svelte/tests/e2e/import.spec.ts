// T-28n in a real browser: a CSV is previewed (nothing written), the same button then
// imports, and what was left out is listed. One primary button throughout.

import { test, expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { PID, primaryButtons } from "./fixtures";

async function audit(page: Page) {
  const r = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa"])
    .analyze();
  expect(
    r.violations
      .filter((v) => v.impact === "serious" || v.impact === "critical")
      .map((v) => v.id),
  ).toEqual([]);
}

const REPORT = {
  dry_run: true,
  source: "jira",
  items_in_file: 3,
  tasks_created: 3,
  tasks_updated: 0,
  links_in_file: 1,
  links_created: 0,
  links_already_present: 0,
  note: "Due dates come across as deadlines.",
  unmapped: [{ what: "column", key: null, reason: "sprint: not imported" }],
};

test("preview first, then import; the report lists what was left out", async ({
  page,
}) => {
  const calls: {
    dry: string | null;
    body: string | null;
    type: string | undefined;
  }[] = [];
  await page.route(`**/api/plans/${PID}/import/jira**`, (r) => {
    const dry = new URL(r.request().url()).searchParams.get("dry_run");
    calls.push({
      dry,
      body: r.request().postData(),
      type: r.request().headers()["content-type"],
    });
    return r.fulfill({
      json:
        dry === "false"
          ? { ...REPORT, dry_run: false, links_created: 1 }
          : REPORT,
    });
  });
  await page.goto(`/en-001/plans/${PID}/import`);
  await expect(
    page.getByRole("button", { name: "Preview the import" }),
  ).toBeDisabled();
  await page
    .getByLabel("CSV file")
    .setInputFiles({
      name: "jira.csv",
      mimeType: "text/csv",
      buffer: Buffer.from("Issue key,Summary\nP-1,One\n"),
    });
  expect(await primaryButtons(page)).toEqual(["Preview the import"]);
  await page.getByRole("button", { name: "Preview the import" }).click();
  await expect(page.getByRole("status")).toContainText(
    "Would create 3 tasks and update 0; 1 dependency.",
  );
  await expect(page.getByText("Columns not imported (1)")).toBeVisible();
  expect(calls).toHaveLength(1);
  expect(calls[0]).toMatchObject({
    dry: "true",
    body: "Issue key,Summary\nP-1,One\n",
    type: "text/csv",
  });
  expect(await primaryButtons(page)).toEqual(["Import 3 tasks"]);
  await audit(page);
  await page.getByRole("button", { name: "Import 3 tasks" }).click();
  await expect(page.getByRole("status")).toContainText(
    "Created 3 tasks and updated 0; 1 dependency.",
  );
  expect(calls[1]?.dry).toBe("false");
});

test("a refusal is shown, and changing tracker clears the file", async ({
  page,
}) => {
  await page.route(`**/api/plans/${PID}/import/asana**`, (r) =>
    r.fulfill({
      status: 422,
      json: {
        error: "unprocessable_entity",
        description:
          "this does not look like that export: missing column(s) task id",
      },
    }),
  );
  await page.goto(`/en-001/plans/${PID}/import`);
  await page.getByLabel("Tracker").selectOption("asana");
  await page
    .getByLabel("CSV file")
    .setInputFiles({
      name: "x.csv",
      mimeType: "text/csv",
      buffer: Buffer.from("a,b\n"),
    });
  await page.getByRole("button", { name: "Preview the import" }).click();
  await expect(page.getByRole("alert")).toContainText(
    "does not look like that export",
  );
  await page.getByLabel("Tracker").selectOption("jira");
  await expect(
    page.getByRole("button", { name: "Preview the import" }),
  ).toBeDisabled();
});
