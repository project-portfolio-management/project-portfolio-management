// The built site: every page is there, axe finds no serious or critical WCAG 2.2 AA violation under the light and
// the dark system preference, the locale switcher opens the same brief in another locale, a right-to-left page
// mirrors without overflowing, and the content security policy blocks nothing the site itself does.
import { test, expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { BRIEF_LOCALES, DOC_PAGES, ROLES } from "../../src/lib/site.ts";

const docRoutes = DOC_PAGES.map((d) => (d.slug ? `/${d.slug}/` : "/"));
const indexRoutes = BRIEF_LOCALES.map((l) => `/${l}/for/`);
const briefRoutes = BRIEF_LOCALES.flatMap((l) => ROLES.map((r) => `/${l}/for/${r}/`));
// Audited: every document, every locale's list, and one brief per locale (a different role each time).
const audited = [
  ...docRoutes,
  ...indexRoutes,
  ...BRIEF_LOCALES.map((l, i) => `/${l}/for/${ROLES[i % ROLES.length]}/`),
];

/** Load a page under a colour scheme and wait until ThemePicker has applied the matching Lily theme. */
async function open(page: Page, route: string, scheme: "light" | "dark") {
  await page.emulateMedia({ colorScheme: scheme });
  await page.goto(route, { waitUntil: "networkidle" });
  await page.waitForFunction(
    (t) =>
      document.documentElement.dataset.theme === t &&
      [...document.styleSheets].some((s) => (s.href ?? "").endsWith(`/assets/themes/${t}.css`)),
    scheme,
  );
}

async function axe(page: Page): Promise<string[]> {
  const results = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
    .analyze();
  return results.violations
    .filter((v) => v.impact === "serious" || v.impact === "critical")
    .map((v) => `${v.id}: ${v.nodes.slice(0, 3).map((n) => n.target.join(" ")).join("; ")}`);
}

test("every page is built and answers 200", async ({ request }) => {
  const all = [...docRoutes, ...indexRoutes, ...briefRoutes];
  expect(all.length).toBe(DOC_PAGES.length + BRIEF_LOCALES.length * (1 + ROLES.length));
  for (const route of all) {
    const res = await request.get(route);
    expect(res.status(), route).toBe(200);
  }
});

for (const route of audited)
  for (const scheme of ["light", "dark"] as const)
    test(`${route} passes axe (${scheme})`, async ({ page }) => {
      const csp: string[] = [];
      page.on("console", (m) => {
        if (/Content Security Policy/i.test(m.text())) csp.push(m.text());
      });
      await open(page, route, scheme);
      await expect(page.locator("h1").first()).toBeVisible();
      expect(await axe(page)).toEqual([]);
      expect(csp).toEqual([]);
    });

test("a brief's html carries its locale; a document is English", async ({ page }) => {
  await page.goto("/cy-gb/for/chief-risk-officer/");
  await expect(page.locator("html")).toHaveAttribute("lang", "cy-GB");
  await expect(page.locator("html")).toHaveAttribute("dir", "ltr");
  await page.goto("/install/");
  await expect(page.locator("html")).toHaveAttribute("lang", "en");
});

test("the locale switcher opens the same brief in the chosen locale", async ({ page }) => {
  await open(page, "/en-gb/for/chief-risk-officer/", "light");
  await page.getByRole("button", { name: "Language of this brief" }).click();
  await page.locator('.locale-picker-option[lang="cy-GB"], .locale-picker-option').filter({ hasText: /Cymraeg|Welsh/ }).first().click();
  await expect(page).toHaveURL(/\/cy-(gb|001)\/for\/chief-risk-officer\/$/);
  await expect(page.locator("html")).toHaveAttribute("lang", /^cy-/);
  await expect(page.locator("h1").first()).toContainText("Risg");
});

test("the locale switcher is only on locale pages", async ({ page }) => {
  await open(page, "/readme/", "light");
  await expect(page.getByRole("button", { name: "Language of this brief" })).toHaveCount(0);
  await expect(page.getByRole("button", { name: "Theme" })).toBeVisible();
});

test("links between the site's own documents stay on the site; others go to the repository", async ({ page }) => {
  await page.goto("/");
  const readme = page.locator("main a", { hasText: "README.md" }).first();
  await expect(readme).toHaveAttribute("href", /readme\/$/);
  const tasks = page.locator("main a", { hasText: "tasks.md" }).first();
  await expect(tasks).toHaveAttribute(
    "href",
    "https://github.com/project-portfolio-management/project-portfolio-management/blob/main/tasks.md",
  );
});

for (const route of ["/", "/readme/", "/en-gb/for/", "/zh-cn/for/chief-executive-officer/"])
  for (const scheme of ["light", "dark"] as const)
    test(`${route} under dir=rtl mirrors without overflow and passes axe (${scheme})`, async ({ page }) => {
      await open(page, route, scheme);
      await page.evaluate(() => {
        document.documentElement.dir = "rtl";
        document.querySelector("header")?.setAttribute("dir", "rtl");
      });
      const overflow = await page.evaluate(
        () => document.documentElement.scrollWidth - document.documentElement.clientWidth,
      );
      expect(overflow).toBeLessThanOrEqual(0);
      // The brand sits at the inline start: on the right under rtl.
      const brand = await page.locator(".brand").boundingBox();
      const width = page.viewportSize()?.width ?? 0;
      expect(brand && brand.x + brand.width).toBeGreaterThan(width / 2);
      expect(await axe(page)).toEqual([]);
    });

test("at phone width no page scrolls sideways", async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  for (const route of [...docRoutes, "/en-gb/for/", "/hi-in/for/chief-technology-officer/"]) {
    await page.goto(route);
    const overflow = await page.evaluate(
      () => document.documentElement.scrollWidth - document.documentElement.clientWidth,
    );
    expect(overflow, route).toBeLessThanOrEqual(0);
  }
});

test("no element carries a style attribute (the CSP allows only SvelteKit's announcer)", async ({ page }) => {
  for (const route of ["/", "/spec/", "/cy-001/for/chief-legal-officer/"]) {
    await open(page, route, "dark");
    const styled = await page.evaluate(() =>
      [...document.querySelectorAll("[style]")]
        .filter((e) => e.id !== "svelte-announcer")
        .map((e) => e.outerHTML.slice(0, 120)),
    );
    expect(styled, route).toEqual([]);
  }
});
