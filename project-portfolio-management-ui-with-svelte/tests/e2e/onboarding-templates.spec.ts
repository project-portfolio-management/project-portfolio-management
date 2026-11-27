// NC-1 slice 12 part n in a real browser: `/onboarding-templates` shows each kind's checklist (custom or built-in, the
// built-in said to be the service's English), changes one with every problem said first with its item and nothing sent
// when nothing changed, reorders items, and goes back to the built-in one after a question; 403 in its own words.
// Welsh; axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { settle, stubPlansApi } from "./fixtures";

async function stub(page: Page, opts: { putStatus?: number } = {}) {
  const sent: Array<{ method: string; path: string; body: unknown }> = [];
  const builtin = [
    { title: "Confirm the role", offset_days: -7 },
    { title: "Book their time", offset_days: 0 },
  ];
  let custom: Record<string, { title: string; offset_days: number }[]> = {
    plan_closeout: [{ title: "Archive the documents", offset_days: 14 }],
  };
  const answer = () => ({
    templates: [
      "person_join",
      "person_leave",
      "plan_intake",
      "plan_closeout",
    ].map((kind) => ({
      kind,
      custom: kind in custom,
      items: custom[kind] ?? builtin,
    })),
  });
  await stubPlansApi(page);
  await page.route(/\/api\/onboarding\/templates(\/.*)?$/, async (route) => {
    const req = route.request();
    const path = new URL(req.url()).pathname.replace(/^\/api\/proxy/, "");
    const method = req.method();
    if (method === "GET") return route.fulfill({ json: answer() });
    sent.push({ method, path, body: req.postDataJSON() ?? null });
    if (opts.putStatus)
      return route.fulfill({
        status: opts.putStatus,
        json: { code: "forbidden", detail: "service text" },
      });
    const kind = path.split("/").at(-1)!;
    if (method === "DELETE") {
      const { [kind]: _gone, ...rest } = custom;
      custom = rest;
      return route.fulfill({ json: {} });
    }
    const b = req.postDataJSON() as {
      items: { title: string; offset_days: number }[];
    };
    custom = { ...custom, [kind]: b.items };
    return route.fulfill({ json: { kind, custom: true, items: b.items } });
  });
  return sent;
}

test("each kind's checklist; a custom one goes back to the built-in after a question", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto("/en-001/onboarding-templates");
  await settle(page);
  const join = page.getByTestId("ot-person_join");
  await expect(join.getByRole("heading")).toHaveText("Someone joins this plan");
  await expect(join).toContainText(
    "the built-in checklist; its items are the service's, in English.",
  );
  await expect(join.getByRole("listitem").nth(0)).toHaveText(
    "Confirm the role (days before the date: 7)",
  );
  await expect(join.getByRole("listitem").nth(1)).toHaveText(
    "Book their time (on the date)",
  );
  await expect(
    join.getByRole("button", { name: "Go back to the built-in checklist" }),
  ).toHaveCount(0);
  const close = page.getByTestId("ot-plan_closeout");
  await expect(close).toContainText("This kind uses its own checklist.");
  await expect(close.getByRole("listitem")).toHaveText([
    "Archive the documents (days after the date: 14)",
  ]);
  const ask = page.getByRole("alertdialog");
  await close
    .getByRole("button", { name: "Go back to the built-in checklist" })
    .click();
  await expect(ask).toContainText("cases already started keep their items");
  await ask.getByRole("button", { name: "Cancel" }).click();
  expect(sent).toEqual([]);
  await close
    .getByRole("button", { name: "Go back to the built-in checklist" })
    .click();
  await ask.getByRole("button", { name: "OK" }).click();
  await expect(page.getByTestId("ot-notice")).toHaveText(
    "“Close this plan out” uses the built-in checklist again.",
  );
  await expect(close).toContainText("the built-in checklist");
  expect(sent).toEqual([
    {
      method: "DELETE",
      path: "/api/onboarding/templates/plan_closeout",
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

test("a change says every problem with its item first, sends nothing unchanged, and keeps the order given", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto("/en-001/onboarding-templates");
  await settle(page);
  const join = page.getByTestId("ot-person_join");
  await join.getByRole("button", { name: "Change" }).click();
  await join.getByRole("button", { name: "Save the checklist" }).click();
  expect(sent).toEqual([]);
  await expect(join.getByRole("button", { name: "Change" })).toBeVisible();
  await join.getByRole("button", { name: "Change" }).click();
  await join.getByRole("button", { name: "Add an item" }).click();
  await join
    .getByLabel("Item 2: days from the date (−365 to 365; negative is before)")
    .fill("400");
  await join.getByRole("button", { name: "Save the checklist" }).click();
  await expect(
    page.getByTestId("ot-problems").getByRole("listitem"),
  ).toHaveText([
    "Item 2: the days must be a whole number from −365 to 365.",
    "Item 3: a title is required, up to 200 characters.",
  ]);
  expect(sent).toEqual([]);
  await join
    .getByLabel("Item 2: days from the date (−365 to 365; negative is before)")
    .fill("1");
  await join
    .getByLabel("Item 3: what to do (up to 200 characters)")
    .fill(" Meet the team ");
  await join
    .getByLabel("Item 3: days from the date (−365 to 365; negative is before)")
    .fill("-1");
  await join.getByRole("button", { name: "Move item 3 up" }).click();
  await join.getByRole("button", { name: "Remove item 1" }).click();
  await join.getByRole("button", { name: "Save the checklist" }).click();
  await expect(page.getByTestId("ot-notice")).toHaveText(
    "Saved the checklist for “Someone joins this plan”.",
  );
  expect(sent).toEqual([
    {
      method: "PUT",
      path: "/api/onboarding/templates/person_join",
      body: {
        items: [
          { title: "Meet the team", offset_days: -1 },
          { title: "Book their time", offset_days: 1 },
        ],
      },
    },
  ]);
  await expect(join).toContainText("This kind uses its own checklist.");
});

test("a change refused for want of the permission is said in the page's words", async ({
  page,
}) => {
  await stub(page, { putStatus: 403 });
  await page.goto("/en-001/onboarding-templates");
  await settle(page);
  const join = page.getByTestId("ot-person_join");
  await join.getByRole("button", { name: "Change" }).click();
  await join
    .getByLabel("Item 1: what to do (up to 200 characters)")
    .fill("Confirm the role and date");
  await join.getByRole("button", { name: "Save the checklist" }).click();
  await expect(page.getByTestId("ot-error")).toHaveText(
    "Changing a checklist needs the permission to manage people.",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
});

test("reads in Welsh", async ({ page }) => {
  await stub(page);
  await page.goto("/cy-001/onboarding-templates");
  await expect(
    page.getByRole("heading", { level: 1, name: "Templedi rhestrau gwirio" }),
  ).toBeVisible();
});
