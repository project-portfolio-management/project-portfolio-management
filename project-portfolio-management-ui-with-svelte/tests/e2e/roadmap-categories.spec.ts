// RE-7 (NC-1 slice 12 part l) in a real browser: `/roadmap-categories` lists the categories (a reader's translation
// shown), adds one with its translations, refuses a name taken and a bad language code before sending, changes one with
// only what changed sent, and retires one after a question, or restores it. Welsh; axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { settle } from "./fixtures";

async function stub(page: Page, opts: { putStatus?: number } = {}) {
  const sent: Array<{ method: string; path: string; body: unknown }> = [];
  let list = [
    {
      pid: "c1",
      name: "Patients",
      translations: { cy: "Cleifion" },
      active: true,
    },
    { pid: "c2", name: "Staff", translations: {}, active: false },
  ];
  await page.route(/\/api\/roadmap-categories(\/.*)?$/, async (route) => {
    const r = route.request();
    const path = new URL(r.url()).pathname.replace(/^\/api\/proxy/, "");
    if (r.method() === "GET")
      return route.fulfill({ json: { categories: list } });
    const body = r.postDataJSON() as Record<string, unknown>;
    sent.push({ method: r.method(), path, body });
    if (opts.putStatus)
      return route.fulfill({
        status: opts.putStatus,
        json: { code: "conflict", detail: "service text" },
      });
    if (r.method() === "POST") {
      const c = {
        pid: "c3",
        name: body.name as string,
        translations: (body.translations as Record<string, string>) ?? {},
        active: true,
      };
      list = [...list, c];
      return route.fulfill({ json: c });
    }
    const id = path.split("/").at(-1)!;
    list = list.map((c) => (c.pid === id ? { ...c, ...(body as object) } : c));
    return route.fulfill({ json: list.find((c) => c.pid === id) });
  });
  return sent;
}

test("the categories; one added with its translations, a taken name and a bad code refused first", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto("/en-001/roadmap-categories");
  await settle(page);
  const items = page.getByTestId("categories").getByRole("listitem");
  await expect(items.nth(0)).toContainText("Patients");
  await expect(items.nth(0)).toContainText("translations: cy: Cleifion");
  await expect(items.nth(1)).toContainText("retired");
  const a = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    a.violations
      .filter((v) => v.impact === "serious" || v.impact === "critical")
      .map((v) => v.id),
  ).toEqual([]);

  const add = page.getByRole("form", { name: "Add a category" });
  await add.getByLabel(/^Name/).fill("patients");
  await add.getByRole("button", { name: "Add the category" }).click();
  await expect(page.getByTestId("rc-error")).toHaveText(
    "A category with that name exists already.",
  );
  await add.getByLabel(/^Name/).fill("Public");
  await add.getByLabel(/^Its name in other languages/).fill("Welsh: Cyhoedd");
  await add.getByRole("button", { name: "Add the category" }).click();
  await expect(page.getByTestId("rc-problems")).toHaveText(
    "Each translation starts with a language code such as cy or zh-cn, then a colon.",
  );
  expect(sent).toEqual([]);
  await add.getByLabel(/^Its name in other languages/).fill("cy: Cyhoedd");
  await add.getByRole("button", { name: "Add the category" }).click();
  await expect(page.getByTestId("rc-notice")).toHaveText("Added “Public”.");
  expect(sent).toEqual([
    {
      method: "POST",
      path: "/api/roadmap-categories",
      body: { name: "Public", translations: { cy: "Cyhoedd" } },
    },
  ]);
});

test("a change sends only what changed; retired after a question and restored", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto("/en-001/roadmap-categories");
  await settle(page);
  const first = page.getByTestId("categories").getByRole("listitem").nth(0);
  await first.getByRole("button", { name: "Change" }).click();
  const form = page.getByRole("form", { name: "Change “Patients”" });
  await form.getByRole("button", { name: "Save the category" }).click();
  await expect(form).toHaveCount(0);
  expect(sent).toEqual([]);
  await first.getByRole("button", { name: "Change" }).click();
  await form.getByLabel(/^Name/).fill("Patients and public");
  await form.getByRole("button", { name: "Save the category" }).click();
  await expect(page.getByTestId("rc-notice")).toHaveText(
    "Saved “Patients and public”.",
  );
  expect(sent.at(-1)).toEqual({
    method: "PUT",
    path: "/api/roadmap-categories/c1",
    body: { name: "Patients and public" },
  });

  const ask = page.getByRole("alertdialog");
  await first.getByRole("button", { name: "Retire" }).click();
  await expect(ask).toContainText("a roadmap that has it keeps it");
  await ask.getByRole("button", { name: "OK" }).click();
  await expect(page.getByTestId("rc-notice")).toHaveText(
    "Retired “Patients and public”.",
  );
  expect(sent.at(-1)).toEqual({
    method: "PUT",
    path: "/api/roadmap-categories/c1",
    body: { active: false },
  });
  await page
    .getByTestId("categories")
    .getByRole("listitem")
    .nth(1)
    .getByRole("button", { name: "Restore" })
    .click();
  expect(sent.at(-1)).toEqual({
    method: "PUT",
    path: "/api/roadmap-categories/c2",
    body: { active: true },
  });
});

test("a refusal meanwhile is said in the page's words; a Welsh reader sees the Welsh name", async ({
  page,
}) => {
  await stub(page, { putStatus: 409 });
  await page.goto("/en-001/roadmap-categories");
  await settle(page);
  const first = page.getByTestId("categories").getByRole("listitem").nth(0);
  await first.getByRole("button", { name: "Change" }).click();
  await page
    .getByRole("form", { name: "Change “Patients”" })
    .getByLabel(/^Name/)
    .fill("Clients");
  await page
    .getByRole("form", { name: "Change “Patients”" })
    .getByRole("button", { name: "Save the category" })
    .click();
  await expect(page.getByTestId("rc-error")).toHaveText(
    "A category with that name exists already.",
  );
  await expect(page.getByText("service text")).toHaveCount(0);
  await page.goto("/cy-001/roadmap-categories");
  await expect(
    page.getByRole("heading", { level: 1, name: "Categorïau mapiau ffordd" }),
  ).toBeVisible();
  await expect(
    page.getByTestId("categories").getByRole("listitem").nth(0),
  ).toContainText("Cleifion (Patients)");
});
