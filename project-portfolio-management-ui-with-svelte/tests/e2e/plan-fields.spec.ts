// RM-1 (NC-1 slice 12 part n) in a real browser: `/plans/[pid]/fields` shows the plan fields that apply to this
// plan's kind with an input of each field's type, the required ones still unset said first; a save sends only what
// changed, after every problem is said with its field; a withdrawn field's value is read only. Welsh; axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle, stubPlansApi } from "./fixtures";

const def = (n: number, over: Record<string, unknown>) => ({
  pid: `aaaaaaaa-0000-4000-8000-0000000000d${n}`,
  key: "k",
  label: "K",
  field_type: "text",
  applies_to: "plan",
  kinds: [],
  required: false,
  options: [],
  created_at: "2026-11-01T00:00:00Z",
  ...over,
});

async function stub(page: Page) {
  const sent: unknown[] = [];
  let values: Record<
    string,
    { value: unknown; label: string | null; definition: string }
  > = {
    owner: { value: "Ana", label: "Owner", definition: "live" },
    legacy: { value: true, label: "Legacy flag", definition: "withdrawn" },
  };
  const defs = [
    def(1, { key: "owner", label: "Owner" }),
    def(2, {
      key: "budget",
      label: "Budget",
      field_type: "integer",
      required: true,
    }),
    def(3, {
      key: "tags",
      label: "Tags",
      field_type: "multi_list",
      options: ["Red", "Blue"],
    }),
    def(4, {
      key: "programme_only",
      label: "Programme only",
      kinds: ["Program"],
    }),
    def(5, { key: "size", label: "Size", applies_to: "task" }),
  ];
  const answer = () => ({
    json: {
      subject_pid: PID,
      values,
      missing_required: values.budget ? [] : ["budget"],
    },
  });
  await stubPlansApi(page);
  await page.route(/\/api\/custom-fields(\/values)?(\?.*)?$/, async (route) => {
    const req = route.request();
    const url = new URL(req.url());
    if (url.pathname.endsWith("/custom-fields"))
      return route.fulfill({ json: defs });
    if (req.method() === "GET") return route.fulfill(answer());
    const b = req.postDataJSON() as { values: Record<string, unknown> };
    sent.push(b);
    for (const [k, v] of Object.entries(b.values)) {
      if (v === null) delete values[k];
      else
        values = { ...values, [k]: { value: v, label: k, definition: "live" } };
    }
    return route.fulfill(answer());
  });
  return sent;
}

test("the fields for this plan's kind, the required ones said first; a withdrawn value read only", async ({
  page,
}) => {
  await stub(page);
  await page.goto(`/en-001/plans/${PID}/fields`);
  await settle(page);
  const form = page.getByRole("form", { name: "Values" });
  await expect(page.getByTestId("fv-missing")).toHaveText(
    "Required fields with no value yet: Budget",
  );
  await expect(form.getByLabel("Owner")).toHaveValue("Ana");
  await expect(form.getByLabel("Budget (required)")).toHaveValue("");
  await expect(
    form.getByRole("group", { name: "Tags" }).getByRole("checkbox"),
  ).toHaveCount(2);
  await expect(form.getByText("Programme only")).toHaveCount(0);
  await expect(form.getByText("Size")).toHaveCount(0);
  await expect(page.getByTestId("fv-withdrawn")).toHaveText("Legacy flag: Yes");
  const a = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    a.violations
      .filter((v) => v.impact === "serious" || v.impact === "critical")
      .map((v) => v.id),
  ).toEqual([]);
});

test("a save says every problem first, then sends only what changed", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto(`/en-001/plans/${PID}/fields`);
  await settle(page);
  const form = page.getByRole("form", { name: "Values" });
  await form.getByRole("button", { name: "Save the values" }).click();
  await expect(page.getByTestId("fv-problems")).toHaveText(
    "Budget is required and cannot be left empty.",
  );
  await form.getByLabel("Budget (required)").fill("12.5");
  await form.getByRole("button", { name: "Save the values" }).click();
  await expect(page.getByTestId("fv-problems")).toHaveText(
    "Budget: the value must be a whole number.",
  );
  expect(sent).toEqual([]);
  await form.getByLabel("Budget (required)").fill("1200");
  await form.getByLabel("Owner").fill("");
  await form.getByRole("checkbox", { name: "Blue" }).check();
  await form.getByRole("button", { name: "Save the values" }).click();
  await expect(page.getByTestId("fv-notice")).toHaveText("Saved the values.");
  await expect(page.getByTestId("fv-missing")).toHaveCount(0);
  expect(sent).toEqual([
    {
      applies_to: "plan",
      subject_pid: PID,
      values: { owner: null, budget: 1200, tags: ["Blue"] },
    },
  ]);
  await form.getByRole("button", { name: "Save the values" }).click();
  await expect(page.getByTestId("fv-notice")).toHaveText(
    "Nothing changed, so nothing was sent.",
  );
  expect(sent).toHaveLength(1);
  await page.goto(`/en-001/plans/${PID}`);
  await expect(
    page.getByRole("link", { name: "Custom fields" }).first(),
  ).toHaveAttribute("href", `/en-001/plans/${PID}/fields`);
});

test("reads in Welsh", async ({ page }) => {
  await stub(page);
  await page.goto(`/cy-001/plans/${PID}/fields`);
  await expect(
    page.getByRole("heading", {
      level: 1,
      name: "Meysydd personol y cynllun hwn",
    }),
  ).toBeVisible();
});
