// BR-15: paste rows copied from a spreadsheet into the task list. Three rows, one with a date nobody can read: the
// preview flags it and leaves it out, the service's dry run is asked about the other two, and only those two are
// created. axe passes with the preview open.

import { test, expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle, stubPlansApi } from "./fixtures";

const REPORT = (dry: boolean) => ({
  dry_run: dry,
  source: "asana",
  items_in_file: 2,
  tasks_created: 2,
  tasks_updated: 0,
  links_in_file: 0,
  links_created: 0,
  links_already_present: 0,
  unmapped: [],
  note: "",
});

async function stub(page: Page) {
  await stubPlansApi(page);
  const base = `**/api/plans/${PID}`;
  await page.route(`${base}/tasks**`, (r) =>
    r.fulfill({ json: { tasks: [], total: 0, truncated: false } }),
  );
  await page.route(`${base}/sprints**`, (r) => r.fulfill({ json: [] }));
  await page.route(`${base}/standup**`, (r) =>
    r.fulfill({
      json: { tasks_created: [], tasks_moved: [], blocked_now: [] },
    }),
  );
  await page.route(`${base}/velocity**`, (r) =>
    r.fulfill({ json: { sprints: [] } }),
  );
  const sent: { dry: boolean; body: string }[] = [];
  await page.route(`${base}/import/asana**`, (r) => {
    const dry =
      new URL(r.request().url()).searchParams.get("dry_run") === "true";
    sent.push({ dry, body: r.request().postData() ?? "" });
    return r.fulfill({ json: REPORT(dry) });
  });
  return sent;
}

const ROWS =
  "Write plan\t2026-10-05\tfirst draft\nBook room\t2026-10-06\nSend invites\t5 Oct";

test("pasted rows are previewed, the bad one is left out, and only the good ones are created", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto(`/en-001/plans/${PID}/board`, { waitUntil: "networkidle" });
  await page
    .getByText("Paste rows from a spreadsheet", { exact: true })
    .click();

  await page.getByLabel("Pasted rows").fill(ROWS);
  await expect(page.getByText("2 ready, 1 left out")).toBeVisible();
  const table = page.getByTestId("paste-rows");
  await expect(table.getByText("Write plan")).toBeVisible();
  await expect(
    table.getByText(/“5 Oct” is not a date; write it as yyyy-mm-dd/),
  ).toBeVisible();

  // Check with the service: it is sent the two good rows only, and as a dry run.
  await page.getByRole("button", { name: "Check with the service" }).click();
  await expect(page.getByTestId("paste-report")).toContainText(
    "Would create 2 tasks",
  );
  await expect.poll(() => sent.length).toBe(1);
  expect(sent[0]!.dry).toBe(true);
  expect(sent[0]!.body).toContain("Write plan");
  expect(sent[0]!.body).toContain("Book room");
  expect(sent[0]!.body).not.toContain("Send invites");
  expect(sent[0]!.body.trim().split("\n")).toHaveLength(3); // header and two rows

  await settle(page);
  const results = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
    .analyze();
  const bad = results.violations.filter(
    (v) => v.impact === "serious" || v.impact === "critical",
  );
  expect(bad.map((v) => `${v.id}: ${v.help}`)).toEqual([]);

  // Create: the same two rows, not a dry run; the box is emptied and the report says it is done.
  await page.getByRole("button", { name: "Create 2 tasks" }).click();
  await expect(page.getByTestId("paste-report")).toContainText("Done");
  await expect.poll(() => sent.length).toBe(2);
  expect(sent[1]!.dry).toBe(false);
  expect(sent[1]!.body).toBe(sent[0]!.body);
  await expect(page.getByLabel("Pasted rows")).toHaveValue("");
});

test("changing the paste starts again, and a paste with nothing good cannot be sent", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto(`/en-001/plans/${PID}/board`, { waitUntil: "networkidle" });
  await page
    .getByText("Paste rows from a spreadsheet", { exact: true })
    .click();
  const box = page.getByLabel("Pasted rows");
  await box.fill("\t2026-10-05");
  await expect(page.getByText("0 ready, 1 left out")).toBeVisible();
  await expect(
    page.getByRole("button", { name: "Check with the service" }),
  ).toBeDisabled();

  await box.fill("One task");
  await page.getByRole("button", { name: "Check with the service" }).click();
  await expect(page.getByTestId("paste-report")).toBeVisible();
  await box.fill("Another task");
  await expect(page.getByTestId("paste-report")).toBeHidden();
  await expect.poll(() => sent.length).toBe(1);
});
