// NC-12 (NC-1 slice 12 part k) in a real browser: `/plans/[pid]/template` lists the template's changes since the plan
// was last brought up to date; a change is taken by a tick, one to a task the plan edited only with a second tick to
// overwrite; applied after a question; what was applied, overwritten, refused and still waiting is said in the
// catalogue's words. A plan not made from a template says so. Welsh; axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle, stubPlansApi } from "./fixtures";

const TPL = "66666666-6666-4666-8666-666666666666";
const task = (
  key: number,
  title: string,
  description: string | null,
  points: number | null,
) => ({
  key,
  title,
  description,
  points,
  flow_type: null,
  due_offset_days: null,
  component: null,
  parent: null,
});
const DIFF = {
  template_pid: TPL,
  template_name: "Replatform kit",
  synced_version: 1,
  template_version: 2,
  changes: [
    {
      id: "add:3",
      kind: "add",
      title: "Load test",
      from: null,
      to: task(3, "Load test", null, 3),
      locally_edited: false,
    },
    {
      id: "change:1",
      kind: "change",
      title: "Cut over",
      from: task(1, "Cut over", "One night", 5),
      to: task(1, "Cut over", "Two nights", 8),
      locally_edited: true,
    },
    {
      id: "remove:2",
      kind: "remove",
      title: "Fax the bank",
      from: task(2, "Fax the bank", null, null),
      to: null,
      locally_edited: false,
    },
  ],
};

async function stub(page: Page, opts: { status?: number } = {}) {
  const sent: Array<{ path: string; body: unknown }> = [];
  await stubPlansApi(page);
  await page.route(
    /\/api\/plans\/[^/]+\/template-(diff|sync)$/,
    async (route) => {
      const req = route.request();
      const path = new URL(req.url()).pathname.replace(/^\/api\/proxy/, "");
      if (opts.status)
        return route.fulfill({
          status: opts.status,
          json: { code: "not_found", detail: "service text" },
        });
      if (path.endsWith("template-diff"))
        return route.fulfill({
          json: sent.length
            ? {
                ...DIFF,
                changes: DIFF.changes.filter((c) => c.id === "change:1"),
              }
            : DIFF,
        });
      sent.push({ path, body: req.postDataJSON() });
      return route.fulfill({
        json: {
          applied: ["add:3"],
          overwritten_local_edits: [],
          refused: [{ id: "change:1", reason: "service text" }],
          remaining: 1,
        },
      });
    },
  );
  return sent;
}

test("the template's changes are listed, ticked and applied after a question", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto(`/en-001/plans/${PID}/template`);
  await settle(page);
  await expect(
    page.getByRole("heading", { level: 1, name: "Template changes" }),
  ).toBeVisible();
  await expect(
    page.getByRole("link", {
      name: "Made from the template “Replatform kit”.",
    }),
  ).toHaveAttribute("href", `/en-001/plan-templates/${TPL}`);
  await expect(
    page.getByText(
      "Brought up to date with version 1; the template is at version 2.",
    ),
  ).toBeVisible();
  const items = page.getByRole("listitem");
  await expect(items.nth(0)).toContainText("+ Load test · added");
  await expect(items.nth(1)).toContainText("~ Cut over · changed");
  await expect(items.nth(1)).toContainText(
    "Description: One night → Two nights",
  );
  await expect(items.nth(1)).toContainText("Points: 5 → 8");
  await expect(items.nth(1)).toContainText(
    "This plan's own people have edited or removed this task.",
  );
  await expect(items.nth(2)).toContainText("− Fax the bank · removed");
  // Nothing ticked: said, nothing sent.
  await page.getByRole("button", { name: "Apply the chosen changes" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "Tick at least one change first.",
  );
  // The overwrite tick waits for its change to be ticked.
  await expect(
    items.nth(1).getByLabel("Overwrite this plan's edit"),
  ).toBeDisabled();
  await items.nth(0).getByLabel("Take this change").check();
  await items.nth(1).getByLabel("Take this change").check();
  const ask = page.getByRole("alertdialog");
  await page.getByRole("button", { name: "Apply the chosen changes" }).click();
  await ask.getByRole("button", { name: "Cancel" }).click();
  expect(sent).toEqual([]);
  await page.getByRole("button", { name: "Apply the chosen changes" }).click();
  await ask.getByRole("button", { name: "OK" }).click();
  const result = page.getByTestId("sync-result");
  await expect(result).toContainText("Applied: 1.");
  await expect(result).toContainText(
    "“Cut over” was not applied: this plan has edited it; tick “Overwrite this plan's edit” to apply it anyway.",
  );
  await expect(result).toContainText("Still waiting: 1.");
  expect(sent).toEqual([
    {
      path: `/api/plans/${PID}/template-sync`,
      body: { accept: ["add:3", "change:1"] },
    },
  ]);
  await expect(page.getByText("service text")).toHaveCount(0);
  const a = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    a.violations
      .filter((v) => v.impact === "serious" || v.impact === "critical")
      .map((v) => v.id),
  ).toEqual([]);
});

test("an overwrite is sent only for a ticked change the plan edited", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto(`/en-001/plans/${PID}/template`);
  await settle(page);
  const edited = page.getByRole("listitem").nth(1);
  await edited.getByLabel("Take this change").check();
  await edited.getByLabel("Overwrite this plan's edit").check();
  await page.getByRole("button", { name: "Apply the chosen changes" }).click();
  await page
    .getByRole("alertdialog")
    .getByRole("button", { name: "OK" })
    .click();
  await expect(page.getByTestId("sync-result")).toBeVisible();
  expect(sent[0]?.body).toEqual({
    accept: ["change:1"],
    overwrite_local: ["change:1"],
  });
});

test("a plan not made from a template says so; the plan page links to the page", async ({
  page,
}) => {
  await stub(page, { status: 404 });
  await page.goto(`/en-001/plans/${PID}/template`);
  await expect(page.getByTestId("not-from-template")).toHaveText(
    "This plan was not made from a saved template, so there is nothing to bring up to date.",
  );
  await page.goto(`/en-001/plans/${PID}`);
  await expect(
    page.getByRole("link", { name: "Template changes" }).first(),
  ).toHaveAttribute("href", `/en-001/plans/${PID}/template`);
});

test("reads in Welsh", async ({ page }) => {
  await stub(page);
  await page.goto(`/cy-001/plans/${PID}/template`);
  await expect(
    page.getByRole("heading", { level: 1, name: "Newidiadau'r templed" }),
  ).toBeVisible();
});
