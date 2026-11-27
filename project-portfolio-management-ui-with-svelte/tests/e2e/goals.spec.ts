// FR-12 and CF-2 (NC-1 slice 12 part n) in a real browser: `/plans/[pid]/goals` lists a plan's goals in order with
// their target date and status; a goal is added, changed or removed (after a question) with every limit checked
// first, and each change sends the list's ETag as If-Match; a list changed meanwhile (412) is said in the page's
// words and read again. Welsh; axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle, stubPlansApi } from "./fixtures";

type Goal = {
  title: string;
  description?: string | null;
  target_date?: string | null;
  status?: unknown;
};

async function stub(page: Page, opts: { staleOnce?: boolean } = {}) {
  const sent: Array<{
    method: string;
    path: string;
    ifMatch: string | null;
    body: unknown;
  }> = [];
  let goals: Goal[] = [
    {
      title: "Ship the portal",
      description: "Public launch",
      target_date: "2027-03-31",
      status: "InProgress",
    },
    { title: "Retire the old system", status: { Custom: "Paused" } },
  ];
  let version = 1;
  let stale = opts.staleOnce ?? false;
  const answer = () => ({
    json: goals.map((g, index) => ({ index, ...g })),
    headers: { etag: `W/"v${version}"` },
  });
  await stubPlansApi(page);
  await page.route(/\/api\/plans\/[^/]+\/goals(\/.*)?$/, async (route) => {
    const req = route.request();
    const path = new URL(req.url()).pathname.replace(/^\/api\/proxy/, "");
    const method = req.method();
    if (method === "GET") return route.fulfill(answer());
    sent.push({
      method,
      path,
      ifMatch: req.headers()["if-match"] ?? null,
      body: req.postDataJSON() ?? null,
    });
    if (method === "POST") {
      goals = [...goals, req.postDataJSON() as Goal];
    } else {
      if (stale) {
        // Someone else removed the first goal meanwhile.
        stale = false;
        goals = goals.slice(1);
        version += 1;
        return route.fulfill({
          status: 412,
          json: { code: "precondition_failed", detail: "service text" },
        });
      }
      if (req.headers()["if-match"] !== `W/"v${version}"`)
        return route.fulfill({
          status: 412,
          json: { code: "precondition_failed" },
        });
      const i = Number(path.split("/").at(-1));
      goals =
        method === "DELETE"
          ? goals.filter((_, k) => k !== i)
          : goals.map((g, k) => (k === i ? (req.postDataJSON() as Goal) : g));
    }
    version += 1;
    return route.fulfill(answer());
  });
  return sent;
}

test("every goal in order; removal after a question, with the list's tag", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto(`/en-001/plans/${PID}/goals`);
  await settle(page);
  const items = page.getByTestId("goals").getByRole("listitem");
  await expect(items).toHaveCount(2);
  await expect(items.nth(0)).toContainText("Ship the portal");
  await expect(items.nth(0)).toContainText("Public launch");
  await expect(items.nth(0)).toContainText("Target date:");
  await expect(items.nth(0)).toContainText("Status: In progress");
  await expect(items.nth(1)).toContainText("Status: Paused");
  const ask = page.getByRole("alertdialog");
  await items.nth(0).getByRole("button", { name: "Remove" }).click();
  await expect(ask).toContainText("Remove the goal “Ship the portal”?");
  await ask.getByRole("button", { name: "Cancel" }).click();
  expect(sent).toEqual([]);
  await items.nth(0).getByRole("button", { name: "Remove" }).click();
  await ask.getByRole("button", { name: "OK" }).click();
  await expect(page.getByTestId("goals-notice")).toHaveText(
    "Removed “Ship the portal”.",
  );
  await expect(items).toHaveCount(1);
  expect(sent).toEqual([
    {
      method: "DELETE",
      path: `/api/plans/${PID}/goals/0`,
      ifMatch: 'W/"v1"',
      body: null,
    },
  ]);
  const a = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    a.violations
      .filter((v) => v.impact === "serious" || v.impact === "critical")
      .map((v) => v.id),
  ).toEqual([]);
});

test("adding and changing check every limit first; nothing is sent for no change; the newest tag is sent", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto(`/en-001/plans/${PID}/goals`);
  await settle(page);
  const add = page.getByRole("form", { name: "Add a goal" });
  await add.getByRole("button", { name: "Add the goal" }).click();
  await expect(page.getByRole("alert")).toHaveText("A title is required.");
  await add.getByLabel("Title (up to 512 characters)").fill(" Cut costs ");
  await add.getByRole("combobox", { name: /^Status/ }).selectOption("Custom");
  await add.getByRole("button", { name: "Add the goal" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "Name the status, or choose another.",
  );
  expect(sent).toEqual([]);
  await add
    .getByRole("combobox", { name: /^Status/ })
    .selectOption("NotStarted");
  await add.getByRole("button", { name: "Add the goal" }).click();
  await expect(page.getByTestId("goals-notice")).toHaveText(
    "Added “Cut costs”.",
  );
  expect(sent.at(-1)).toEqual({
    method: "POST",
    path: `/api/plans/${PID}/goals`,
    ifMatch: null,
    body: {
      title: "Cut costs",
      description: null,
      target_date: null,
      status: "NotStarted",
    },
  });

  const second = page.getByTestId("goals").getByRole("listitem").nth(1);
  await second.getByRole("button", { name: "Change" }).click();
  await second.getByRole("button", { name: "Save the goal" }).click();
  expect(sent).toHaveLength(1);
  await second.getByRole("button", { name: "Change" }).click();
  await second
    .getByRole("combobox", { name: /^Status/ })
    .selectOption("Achieved");
  await second.getByRole("button", { name: "Save the goal" }).click();
  await expect(page.getByTestId("goals-notice")).toHaveText(
    "Saved “Retire the old system”.",
  );
  expect(sent.at(-1)).toEqual({
    method: "PUT",
    path: `/api/plans/${PID}/goals/1`,
    ifMatch: 'W/"v2"',
    body: {
      title: "Retire the old system",
      description: null,
      target_date: null,
      status: "Achieved",
    },
  });
});

test("a list changed meanwhile is refused in the page's words and read again; the plan page links here", async ({
  page,
}) => {
  await stub(page, { staleOnce: true });
  await page.goto(`/en-001/plans/${PID}/goals`);
  await settle(page);
  const items = page.getByTestId("goals").getByRole("listitem");
  await items.nth(1).getByRole("button", { name: "Change" }).click();
  await items
    .nth(1)
    .getByLabel("Title (up to 512 characters)")
    .fill("Retire it");
  await items.nth(1).getByRole("button", { name: "Save the goal" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    /^The goals changed after this page read them/,
  );
  await expect(page.getByText("service text")).toHaveCount(0);
  await expect(items).toHaveCount(1);
  await expect(items.nth(0)).toContainText("Retire the old system");
  await page.goto(`/en-001/plans/${PID}`);
  await expect(
    page.getByRole("link", { name: "Goals" }).first(),
  ).toHaveAttribute("href", `/en-001/plans/${PID}/goals`);
});

test("reads in Welsh", async ({ page }) => {
  await stub(page);
  await page.goto(`/cy-001/plans/${PID}/goals`);
  await expect(
    page.getByRole("heading", { level: 1, name: "Nodau" }),
  ).toBeVisible();
});
