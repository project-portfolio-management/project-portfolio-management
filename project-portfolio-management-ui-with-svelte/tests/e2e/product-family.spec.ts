// RE-5 and RE-7 (NC-1 slice 12 part l) in a real browser: `/plans/[pid]/family` shows a product family's products, its
// colour with how it reads under white and black text, sets the colour and its names in other languages with only what
// changed sent and every problem said first, and removes the colour after a question. Welsh; axe.

import { test, expect, type Page } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle, stubPlansApi } from "./fixtures";

const navy = {
  with_white_text: 8.04,
  with_black_text: 2.61,
  white_text_meets_aa: true,
  black_text_meets_aa: false,
  better_text: "white",
};

async function stub(page: Page, coloured = true) {
  const sent: unknown[] = [];
  let fam: Record<string, unknown> = {
    plan_pid: PID,
    name: "Money",
    colour: coloured ? "#325083" : null,
    contrast: coloured ? navy : null,
    translations: coloured ? { cy: "Arian" } : {},
    products: 3,
  };
  await stubPlansApi(page);
  await page.route(/\/api\/plans\/[^/]+\/roadmap-family$/, async (route) => {
    const r = route.request();
    if (r.method() === "PUT") {
      const b = r.postDataJSON() as {
        colour: string | null;
        translations?: Record<string, string>;
      };
      sent.push(b);
      fam =
        b.colour === null
          ? { ...fam, colour: null, contrast: null, translations: {} }
          : {
              ...fam,
              colour: b.colour,
              contrast: navy,
              translations: b.translations ?? fam.translations,
            };
    }
    return route.fulfill({ json: fam });
  });
  return sent;
}

test("a family's products and colour, how it reads; a change sends only what changed", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto(`/en-001/plans/${PID}/family`);
  await settle(page);
  await expect(page.getByTestId("family-products")).toHaveText(
    "products in the family: 3",
  );
  await expect(page.getByTestId("family-colour")).toHaveText("colour: #325083");
  await expect(page.getByTestId("family-contrast")).toContainText(
    "with white text: 8.04 to 1 — meets AA for body text",
  );
  await expect(page.getByTestId("family-contrast")).toContainText(
    "with black text: 2.61 to 1 — falls short of AA",
  );
  await expect(page.getByTestId("family-contrast")).toContainText(
    "White text reads better on it.",
  );
  const a = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    a.violations
      .filter((v) => v.impact === "serious" || v.impact === "critical")
      .map((v) => v.id),
  ).toEqual([]);

  const form = page.getByRole("form", { name: "Its colour and names" });
  await form.getByRole("button", { name: "Save the colour" }).click();
  await expect(page.getByTestId("family-notice")).toHaveText(
    "Nothing changed, so nothing was sent.",
  );
  await form.getByLabel(/^Colour/).fill("navy");
  await form.getByRole("button", { name: "Save the colour" }).click();
  await expect(page.getByTestId("family-problems")).toContainText(
    "The colour is # and six hexadecimal digits",
  );
  expect(sent).toEqual([]);
  await form.getByLabel(/^Colour/).fill(" #00A3E0 ");
  await form
    .getByLabel(/^Its name in other languages/)
    .fill("cy: Arian\nfr: Argent");
  await form.getByRole("button", { name: "Save the colour" }).click();
  await expect(page.getByTestId("family-notice")).toHaveText(
    "Saved the family.",
  );
  expect(sent).toEqual([
    { colour: "#00a3e0", translations: { cy: "Arian", fr: "Argent" } },
  ]);
});

test("removing the colour asks first and takes the names with it", async ({
  page,
}) => {
  const sent = await stub(page);
  await page.goto(`/en-001/plans/${PID}/family`);
  await settle(page);
  const ask = page.getByRole("alertdialog");
  await page.getByRole("button", { name: "Remove the colour" }).click();
  await expect(ask).toContainText("Its names in other languages go with it.");
  await ask.getByRole("button", { name: "Cancel" }).click();
  expect(sent).toEqual([]);
  await page.getByRole("button", { name: "Remove the colour" }).click();
  await ask.getByRole("button", { name: "OK" }).click();
  await expect(page.getByTestId("family-notice")).toHaveText(
    "Removed the colour and its translations.",
  );
  await expect(page.getByTestId("family-no-colour")).toBeVisible();
  expect(sent).toEqual([{ colour: null }]);
});

test("a family with no colour says so; reads in Welsh", async ({ page }) => {
  await stub(page, false);
  await page.goto(`/en-001/plans/${PID}/family`);
  await settle(page);
  await expect(page.getByTestId("family-no-colour")).toBeVisible();
  await expect(
    page.getByRole("button", { name: "Remove the colour" }),
  ).toHaveCount(0);
  await page.goto(`/cy-001/plans/${PID}/family`);
  await expect(
    page.getByRole("heading", { level: 1, name: "Teulu cynhyrchion" }),
  ).toBeVisible();
});
