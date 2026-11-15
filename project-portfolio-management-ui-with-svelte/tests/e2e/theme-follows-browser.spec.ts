// The UI uses the browser's light/dark setting (spec, Theming): with no theme the person chose, `prefers-color-scheme`
// picks `light` or `dark`; a theme the person chose in the picker wins over it.
import { expect, test } from "./test";
import { stubPlansApi } from "./fixtures";

const theme = (page: import("@playwright/test").Page) =>
  page.evaluate(() => document.documentElement.dataset.theme ?? "");

for (const colorScheme of ["light", "dark"] as const)
  test(`with no choice, a browser set to ${colorScheme} gets the ${colorScheme} theme`, async ({
    page,
  }) => {
    await stubPlansApi(page);
    await page.emulateMedia({ colorScheme });
    await page.goto("/en-001");
    await expect.poll(() => theme(page)).toBe(colorScheme);
  });

test("a theme the person chose wins over the browser's setting", async ({
  page,
}) => {
  await stubPlansApi(page);
  await page.addInitScript(() =>
    localStorage.setItem("ppm-theme-choice", "cupcake"),
  );
  await page.emulateMedia({ colorScheme: "dark" });
  await page.goto("/en-001");
  await expect.poll(() => theme(page)).toBe("cupcake");
});

test("with no choice, changing the browser's setting changes the theme", async ({
  page,
}) => {
  await stubPlansApi(page);
  await page.emulateMedia({ colorScheme: "light" });
  await page.goto("/en-001");
  await expect.poll(() => theme(page)).toBe("light");
  await page.emulateMedia({ colorScheme: "dark" });
  await expect.poll(() => theme(page), { timeout: 3000 }).toBe("dark");
});

test("a later visit still follows the browser's setting", async ({ page }) => {
  await stubPlansApi(page);
  await page.emulateMedia({ colorScheme: "light" });
  await page.goto("/en-001");
  await expect.poll(() => theme(page)).toBe("light");
  await page.emulateMedia({ colorScheme: "dark" });
  await page.reload();
  await expect.poll(() => theme(page)).toBe("dark");
});

test("a theme picked in the picker is remembered, and picking the browser's own theme follows the browser again", async ({
  page,
}) => {
  await stubPlansApi(page);
  await page.emulateMedia({ colorScheme: "light" });
  await page.goto("/en-001");
  await expect.poll(() => theme(page)).toBe("light");
  const pick = async (name: RegExp) => {
    await page.locator(".theme-picker-button").first().click();
    await page
      .locator(".theme-picker-option", { hasText: name })
      .first()
      .click();
  };
  await pick(/^\s*cupcake\s*$/i);
  await expect.poll(() => theme(page)).toBe("cupcake");
  await page.emulateMedia({ colorScheme: "dark" });
  await page.reload();
  await expect.poll(() => theme(page)).toBe("cupcake");
  // Back to following the browser: pick the theme the browser asks for.
  await pick(/^\s*dark\s*$/i);
  await expect.poll(() => theme(page)).toBe("dark");
  await page.emulateMedia({ colorScheme: "light" });
  await expect.poll(() => theme(page)).toBe("light");
});
