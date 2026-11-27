// NC-13 (NC-1 slice 12 part k) in a real browser: `/mspdi-import` takes up to ten MS Project files, each into a new plan
// or a chosen one; a preview that writes nothing comes first, then the same button imports after a question; each
// project's outcome is a word with the importer's report; resources are listed once and never matched to people; a
// failed project's English error is not shown. Linked from /jobs. Welsh; axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle, stubPlansApi } from "./fixtures";

const XML =
  '<?xml version="1.0"?><Project xmlns="http://schemas.microsoft.com/project"><Name>Alpha</Name></Project>';
const report = (dry: boolean) => ({
  dry_run: dry,
  tasks_in_file: 4,
  tasks_created: dry ? 0 : 4,
  tasks_updated: 0,
  links_in_file: 2,
  links_created: dry ? 0 : 2,
  links_already_present: 0,
  unmapped: [],
  note: "",
});
const answer = (dry: boolean, fail = false) => ({
  dry_run: dry,
  projects: fail
    ? [
        { project: "alpha.xml", outcome: "failed", error: "service text" },
        { project: "beta.xml", outcome: "not_attempted" },
      ]
    : [
        {
          project: "alpha.xml",
          outcome: dry ? "would_import" : "imported",
          plan_pid: dry ? null : "99999999-9999-4999-8999-999999999999",
          plan_created: !dry,
          plan_would_be_created: dry,
          report: report(dry),
        },
        {
          project: "beta.xml",
          outcome: dry ? "would_import" : "imported",
          plan_pid: PID,
          plan_created: false,
          plan_would_be_created: false,
          report: report(dry),
        },
      ],
  failed_at: fail ? 0 : null,
  resource_pool: [
    {
      name: "Ana",
      projects: ["alpha.xml", "beta.xml"],
      assignments: 3,
      note: "service text",
    },
  ],
  cross_project_links: dry
    ? { would_create: 1, unresolved: [{ reason: "service text" }] }
    : fail
      ? { created: 0, reason: "service text", unresolved: [] }
      : { created: 1, already_present: 0, unresolved: [] },
});

async function stub(page: Page, opts: { fail?: boolean } = {}) {
  const sent: Array<{
    dry: string | null;
    body: { projects: Array<Record<string, string>>; parent_pid?: string };
  }> = [];
  await stubPlansApi(page);
  await page.route("**/api/import/mspdi-batch*", (route) => {
    const url = new URL(route.request().url());
    const dry = url.searchParams.get("dry_run");
    sent.push({ dry, body: route.request().postDataJSON() });
    return route.fulfill({
      json: answer(dry === "true", opts.fail && dry === "false"),
    });
  });
  return sent;
}
const files = [
  { name: "alpha.xml", mimeType: "application/xml", buffer: Buffer.from(XML) },
  {
    name: "beta.xml",
    mimeType: "application/xml",
    buffer: Buffer.from(XML.replace("Alpha", "Beta")),
  },
];

test("files are previewed first, then imported after a question", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto("/en-001/mspdi-import");
  await settle(page);
  await expect(
    page.getByRole("heading", {
      level: 1,
      name: "Import several MS Project files",
    }),
  ).toBeVisible();
  await page.getByRole("button", { name: "Preview the import" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "Choose at least one file first.",
  );
  await page
    .getByLabel("Files (MS Project XML, at most ten)")
    .setInputFiles(files);
  const beta = page.getByRole("group", { name: "beta.xml" });
  await beta.getByLabel("Into").selectOption("existing");
  await page.getByRole("button", { name: "Preview the import" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "Choose the plan for “beta.xml” first.",
  );
  expect(sent).toEqual([]);
  await beta.getByRole("combobox", { name: /^Plan/ }).selectOption(PID);
  await page
    .getByRole("group", { name: "alpha.xml" })
    .getByLabel(/New plan's name/)
    .fill(" Alpha rollout ");
  await page.getByRole("button", { name: "Preview the import" }).click();
  const result = page.getByTestId("batch-result");
  await expect(
    result.getByRole("heading", { level: 2, name: "Preview" }),
  ).toBeVisible();
  await expect(result).toContainText("alpha.xml · would be imported");
  await expect(result).toContainText("a new plan would be made");
  await expect(result).toContainText(
    "Links between the files that would be made: 1.",
  );
  await expect(result).toContainText(
    "Links that point outside the files or to a missing task, not made: 1.",
  );
  await expect(result).toContainText("Ana — assignments: 3");
  await expect(page.getByText("service text")).toHaveCount(0);
  expect(sent[0]?.dry).toBe("true");
  expect(
    sent[0]?.body.projects.map((p) => ({ name: p.name, plan_pid: p.plan_pid })),
  ).toEqual([
    { name: "Alpha rollout", plan_pid: undefined },
    { name: undefined, plan_pid: PID },
  ]);
  const ask = page.getByRole("alertdialog");
  await page.getByRole("button", { name: "Import", exact: true }).click();
  await expect(ask).toContainText("This is not one transaction");
  await ask.getByRole("button", { name: "Cancel" }).click();
  expect(sent).toHaveLength(1);
  await page.getByRole("button", { name: "Import", exact: true }).click();
  await ask.getByRole("button", { name: "OK" }).click();
  await expect(
    result.getByRole("heading", { level: 2, name: "Imported" }),
  ).toBeVisible();
  await expect(result).toContainText("a new plan was made");
  await expect(result).toContainText("Links made: 1; already there: 0.");
  expect(sent[1]?.dry).toBe("false");
  const a = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    a.violations
      .filter((v) => v.impact === "serious" || v.impact === "critical")
      .map((v) => v.id),
  ).toEqual([]);
});

test("a failed project is said in the page's words, and the rest are not attempted", async ({
  page,
}) => {
  await stub(page, { fail: true });
  await page.goto("/en-001/mspdi-import");
  await settle(page);
  await page
    .getByLabel("Files (MS Project XML, at most ten)")
    .setInputFiles(files);
  await page.getByRole("button", { name: "Preview the import" }).click();
  await page.getByRole("button", { name: "Import", exact: true }).click();
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  const result = page.getByTestId("batch-result");
  await expect(result).toContainText("alpha.xml · failed");
  await expect(result).toContainText(
    "This project failed, so the ones after it were not attempted.",
  );
  await expect(result).toContainText("beta.xml · not attempted");
  await expect(result).toContainText(
    "No links between the files were made, because a project failed.",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("more than ten files are refused before anything is read; /jobs links here", async ({
  page,
}) => {
  await stub(page);
  await page.goto("/en-001/mspdi-import");
  await settle(page);
  await page.getByLabel("Files (MS Project XML, at most ten)").setInputFiles(
    Array.from({ length: 11 }, (_, i) => ({
      name: `p${i}.xml`,
      mimeType: "application/xml",
      buffer: Buffer.from(XML),
    })),
  );
  await expect(page.getByRole("alert")).toHaveText("Choose at most ten files.");
  await page.route("**/api/plans/bulk-jobs*", (r) => r.fulfill({ json: [] }));
  await page.route("**/api/plans/review-queue*", (r) =>
    r.fulfill({ json: [] }),
  );
  await page.goto("/en-001/jobs");
  await expect(
    page.getByRole("link", { name: "Import several MS Project files" }),
  ).toHaveAttribute("href", "/en-001/mspdi-import");
});

test("reads in Welsh", async ({ page }) => {
  await stub(page);
  await page.goto("/cy-001/mspdi-import");
  await expect(
    page.getByRole("heading", {
      level: 1,
      name: "Mewnforio sawl ffeil MS Project",
    }),
  ).toBeVisible();
});
