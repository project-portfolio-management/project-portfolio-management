// RM-1 (NC-1 slice 12 part n) in a real browser: `/custom-fields` lists the live definitions for plans and for tasks,
// declares one with every problem said first and all at once, refuses a key already taken before sending, withdraws
// one after a question, and finds the plans whose value matches. Welsh; axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle, stubPlansApi } from "./fixtures";

const def = (pid: string, over: Record<string, unknown>) => ({
  pid,
  key: "k",
  label: "K",
  field_type: "text",
  applies_to: "plan",
  kinds: [],
  required: false,
  options: [],
  created_at: "2026-11-01T00:00:00Z",
  updated_at: "2026-11-01T00:00:00Z",
  deleted_at: null,
  ...over,
});

async function stub(page: Page, opts: { declareStatus?: number } = {}) {
  const sent: Array<{ method: string; path: string; body: unknown }> = [];
  let defs = [
    def("aaaaaaaa-0000-4000-8000-0000000000c1", {
      key: "cost_centre",
      label: "Cost centre",
      required: true,
      kinds: ["Project"],
    }),
    def("aaaaaaaa-0000-4000-8000-0000000000c2", {
      key: "size",
      label: "Size",
      field_type: "list",
      applies_to: "task",
      options: ["S", "M", "L"],
    }),
  ];
  await stubPlansApi(page);
  await page.route(/\/api\/custom-fields(\/.*)?(\?.*)?$/, async (route) => {
    const req = route.request();
    const url = new URL(req.url());
    const path = url.pathname.replace(/^\/api\/proxy/, "") + url.search;
    const method = req.method();
    if (method !== "GET" || url.pathname.endsWith("/search"))
      sent.push({ method, path, body: req.postDataJSON() ?? null });
    if (url.pathname.endsWith("/search"))
      return route.fulfill({
        json: { count: 1, pids: [PID], truncated: false },
      });
    if (method === "GET") return route.fulfill({ json: defs });
    if (method === "POST") {
      if (opts.declareStatus)
        return route.fulfill({
          status: opts.declareStatus,
          json: { code: "conflict", detail: "service text" },
        });
      const b = req.postDataJSON() as Record<string, unknown>;
      const d = def("aaaaaaaa-0000-4000-8000-0000000000c3", {
        ...b,
        field_type: b.type,
      });
      defs = [...defs, d];
      return route.fulfill({ json: d });
    }
    defs = defs.filter((d) => !url.pathname.endsWith(d.pid));
    return route.fulfill({ json: {} });
  });
  return sent;
}

test("every live definition by subject; withdrawn after a question", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto("/en-001/custom-fields");
  await settle(page);
  const plans = page.getByTestId("cf-plan").getByRole("listitem");
  await expect(plans).toHaveCount(1);
  await expect(plans.nth(0)).toContainText("Cost centre");
  await expect(plans.nth(0)).toContainText("cost_centre");
  await expect(plans.nth(0)).toContainText("required");
  await expect(plans.nth(0)).toContainText("only for plans of kind: Project");
  const tasks = page.getByTestId("cf-task").getByRole("listitem");
  await expect(tasks.nth(0)).toContainText("One choice from a list");
  await expect(tasks.nth(0)).toContainText("choices: S, M, L");
  const ask = page.getByRole("alertdialog");
  await tasks.nth(0).getByRole("button", { name: "Withdraw" }).click();
  await expect(ask).toContainText("Withdraw the field “Size”?");
  await ask.getByRole("button", { name: "Cancel" }).click();
  expect(sent).toEqual([]);
  await tasks.nth(0).getByRole("button", { name: "Withdraw" }).click();
  await ask.getByRole("button", { name: "OK" }).click();
  await expect(page.getByTestId("cf-notice")).toHaveText("Withdrew “Size”.");
  await expect(page.getByTestId("cf-none-task")).toBeVisible();
  expect(sent).toEqual([
    {
      method: "DELETE",
      path: "/api/custom-fields/aaaaaaaa-0000-4000-8000-0000000000c2",
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

test("a declaration says every problem first, refuses a taken key, and sends only what applies", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto("/en-001/custom-fields");
  await settle(page);
  const form = page.getByRole("form", { name: "Declare a field" });
  await form.getByLabel(/^Key/).fill("Bad Key");
  await form
    .getByRole("combobox", { name: "Type of value" })
    .selectOption("multi_list");
  await form.getByRole("button", { name: "Declare the field" }).click();
  const problems = page.getByTestId("cf-problems").getByRole("listitem");
  await expect(problems).toHaveCount(3);
  await expect(problems.nth(0)).toContainText(
    "The key must be lowercase letters",
  );
  await expect(problems.nth(2)).toHaveText(
    "A list field needs at least one choice.",
  );
  await form.getByLabel(/^Key/).fill("cost_centre");
  await form.getByLabel(/^Label/).fill("Again");
  await form
    .getByRole("combobox", { name: "Type of value" })
    .selectOption("text");
  await form.getByRole("button", { name: "Declare the field" }).click();
  await expect(page.getByTestId("cf-error")).toHaveText(
    "A field with this key already exists for that kind of subject.",
  );
  expect(sent).toEqual([]);
  await form.getByLabel(/^Key/).fill("phase");
  await form.getByLabel(/^Label/).fill(" Phase ");
  await form.getByLabel(/^Only for these plan kinds/).fill("Project");
  await form.getByRole("combobox", { name: "Applies to" }).selectOption("task");
  await form
    .getByRole("combobox", { name: "Type of value" })
    .selectOption("list");
  await form.getByLabel(/^Choices, one per line/).fill("Discovery\nDelivery");
  await form.getByRole("button", { name: "Declare the field" }).click();
  await expect(page.getByTestId("cf-notice")).toHaveText("Declared “Phase”.");
  expect(sent).toEqual([
    {
      method: "POST",
      path: "/api/custom-fields",
      body: {
        key: "phase",
        label: "Phase",
        type: "list",
        applies_to: "task",
        kinds: [],
        required: false,
        options: ["Discovery", "Delivery"],
      },
    },
  ]);
  await expect(page.getByTestId("cf-task").getByRole("listitem")).toHaveCount(
    2,
  );
});

test("a key taken meanwhile is said in the page's words; a search finds the matching plans", async ({
  page,
}) => {
  const sent = await stub(page, { declareStatus: 409 });
  await page.goto("/en-001/custom-fields");
  await settle(page);
  const form = page.getByRole("form", { name: "Declare a field" });
  await form.getByLabel(/^Key/).fill("owner");
  await form.getByLabel(/^Label/).fill("Owner");
  await form.getByRole("button", { name: "Declare the field" }).click();
  await expect(page.getByTestId("cf-error")).toHaveText(
    "A field with this key already exists for that kind of subject.",
  );
  await expect(page.getByText("service text")).toHaveCount(0);

  const find = page.getByRole("form", { name: "Find by a field's value" });
  await find.getByRole("button", { name: "Find" }).click();
  await expect(find.getByRole("alert")).toHaveText("Choose a field.");
  await find
    .getByRole("combobox", { name: "Field" })
    .selectOption("cost_centre");
  await find.getByRole("button", { name: "Find" }).click();
  await expect(find.getByRole("alert")).toHaveText("Give a value to look for.");
  await find.getByLabel("Value").fill("CC-7");
  await find.getByRole("button", { name: "Find" }).click();
  await expect(page.getByTestId("cf-found")).toContainText(
    "Matching plans you can see: 1",
  );
  await expect(page.getByTestId("cf-found").getByRole("link")).toHaveAttribute(
    "href",
    `/en-001/plans/${PID}`,
  );
  expect(sent.at(-1)).toEqual({
    method: "GET",
    path: "/api/custom-fields/search?applies_to=plan&key=cost_centre&value=CC-7",
    body: null,
  });
});

test("reads in Welsh", async ({ page }) => {
  await stub(page);
  await page.goto("/cy-001/custom-fields");
  await expect(
    page.getByRole("heading", { level: 1, name: "Meysydd personol" }),
  ).toBeVisible();
});
