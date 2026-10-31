// WW-1.1 (NFR-19): every page renders fully in every supported locale, not only in Welsh. Each real page route
// (enumerated from `src/routes`) is loaded in English and then in each other locale over the same stubbed API; a
// line of visible text identical to the English, that is not data, a name or a symbol, is untranslated. Welsh has its
// own, older spec (`welsh.spec.ts`); it is run here too, so the two cannot drift apart.

import { test, expect } from "@playwright/test";
import { ROUTES, stubPlansApi } from "./fixtures";
import { LOCALES, LOCALE_LABELS } from "../../src/lib/locales";

/** Lines that are the same in every language by design: product, data and fixture names, units, codes. */
const NAMES = new Set([
  "PPM",
  "Website replatform",
  "WEB-42",
  "Kanban",
  "Gantt",
  "CSV",
  "CSV TSV Excel",
  "GBP",
  "PERT",
  "RACI",
  "Main X",
  "Jira",
  "Trello",
  "Asana",
  "Linear",
  "Monday",
  "ClickUp",
  "Azure DevOps",
  "GitHub",
  "platform",
  "unhandled in stub",
  "404: unhandled in stub",
  // The locale picker names each language in itself.
  ...Object.values(LOCALE_LABELS),
]);

/** German words the same as the English ("Active" in "Status: Active" is the stub's data). */
const GERMAN = [
  "← Plan",
  "Board",
  "Budget",
  "Chat",
  "Code: WEB-42",
  "Code",
  "Compliance",
  "Dashboard",
  "Design",
  "Detail",
  "Engineering",
  "Filter:",
  "Governance",
  "Links",
  "Name",
  "Portfolio",
  "Portfolios",
  "Sprint",
  "Sprints",
  "Status: Active",
  "Status",
  "Ticket",
  "Tracker",
];

/**
 * Lines that are, correctly, the same word in a language that shares the Latin alphabet with English. A machine
 * draft; a reviewer may change them (governance/translation-reviews/).
 */
const SAME: Partial<Record<string, ReadonlySet<string>>> = {
  "cy-001": new Set(["Project", "Active", "band", "capex", "opex"]),
  "de-001": new Set(GERMAN),
  "de-de": new Set(GERMAN),
  "es-001": new Set([
    "← Plan",
    "Actor",
    "Auditor",
    "Chat",
    "idea",
    "Ideas",
    "plan",
    "Sprint",
    "Sprints",
    "Ticket",
    "Triangular",
  ]),
  "fr-001": new Set([
    "← Documents",
    "← Plan",
    "action",
    "Action",
    "Actions",
    "Allocations",
    "Budget",
    "Description",
    "Distribution",
    "Document",
    "Documents",
    "expertise",
    "Expertise",
    "Main X · Plans",
    "MAIN X · PLANS",
    "plan",
    "Plans",
    "Programme",
    "Programmes",
    "question",
    "Sprint",
    "Sprints",
    "Ticket",
  ]),
};

async function lines(
  page: import("@playwright/test").Page,
  path: string,
): Promise<string[]> {
  await page.goto(path, { waitUntil: "networkidle" });
  await expect(page.locator("h1").first()).toBeVisible();
  const text = await page.locator("body").innerText();
  return text
    .split("\n")
    .map((l) => l.replace(/[⁦-⁩‎‏]/g, "").trim())
    .filter(
      (l) =>
        /[A-Za-z]{4,}/.test(l) &&
        !/[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-/.test(l),
    );
}

test.describe("every page renders in every locale (WW-1.1, NFR-19)", () => {
  test.beforeEach(async ({ page }) => {
    await stubPlansApi(page);
  });

  for (const route of ROUTES) {
    test(`${route} is translated in every locale`, async ({ page }) => {
      test.setTimeout(180_000);
      const suffix = route === "/" ? "" : route;
      const en = new Set(await lines(page, `/en-001${suffix}`));
      const untranslated: string[] = [];
      for (const locale of LOCALES.filter((l) => l !== "en-001")) {
        const shown = await lines(page, `/${locale}${suffix}`);
        await expect(page.locator("html")).toHaveAttribute("lang", locale);
        const same = SAME[locale] ?? new Set<string>();
        for (const l of shown)
          if (en.has(l) && !NAMES.has(l) && !same.has(l))
            untranslated.push(`${locale}: ${l}`);
      }
      expect(untranslated).toEqual([]);
    });
  }
});
