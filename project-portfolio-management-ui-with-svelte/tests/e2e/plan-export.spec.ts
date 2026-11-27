// NC-1 (PJ-10): export a plan to Microsoft Project, in a real browser over a stubbed service. The plan page leads to the
// export page; it says what the file carries and leaves out and its limit; the link saves the service's own file (the
// browser downloads it, the page does not draw it). Welsh; axe.

import { test, expect } from "./test";
import AxeBuilder from "@axe-core/playwright";
import { PID, settle, stubPlansApi } from "./fixtures";

const XML =
  '<?xml version="1.0" encoding="UTF-8"?><Project xmlns="http://schemas.microsoft.com/project"></Project>';

test("the plan page leads to the export, which says what the file holds and downloads it", async ({
  page,
}) => {
  await stubPlansApi(page);
  const asked: string[] = [];
  await page.route(`**/api/plans/${PID}/export/mspdi`, (route) => {
    asked.push(route.request().method());
    return route.fulfill({
      body: XML,
      headers: {
        "content-type": "application/xml; charset=utf-8",
        "content-disposition": 'attachment; filename="website-replatform.xml"',
      },
    });
  });
  await page.goto(`/en-001/plans/${PID}`);
  await settle(page);
  await page.getByRole("link", { name: "Export to Microsoft Project" }).click();
  await expect(
    page.getByRole("heading", {
      level: 1,
      name: "Export to Microsoft Project",
    }),
  ).toBeVisible();
  await expect(page.getByText(/People assigned and resources/)).toBeVisible();
  await expect(
    page.getByText("A plan of more than 5,000 tasks cannot be exported."),
  ).toBeVisible();
  await expect(
    page.getByText(/not yet opened in Microsoft Project itself/),
  ).toBeVisible();

  const download = page.waitForEvent("download");
  await page.getByRole("link", { name: "Download the MSPDI file" }).click();
  const file = await download;
  expect(file.suggestedFilename()).toBe("website-replatform.xml");
  expect(asked).toEqual(["GET"]);
  await expect(page).toHaveURL(new RegExp(`/plans/${PID}/export$`));
  await expect(
    page.getByRole("link", {
      name: "Bring an MSPDI file into this plan instead",
    }),
  ).toHaveAttribute("href", new RegExp(`/plans/${PID}/import$`));
});

test("the export page reads in Welsh and passes axe", async ({ page }) => {
  await stubPlansApi(page);
  await page.goto(`/cy-001/plans/${PID}/export`);
  await settle(page);
  await expect(page.getByRole("heading", { level: 1 })).toHaveText(
    "Allforio i Microsoft Project",
  );
  const results = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21aa", "wcag22aa"])
    .analyze();
  expect(
    results.violations.filter(
      (v) => v.impact === "serious" || v.impact === "critical",
    ),
  ).toEqual([]);
});

test("the plan's record downloads as JSON, named by the link (NC-1 slice 12 part k)", async ({
  page,
}) => {
  await stubPlansApi(page);
  await page.route(`**/api/plans/${PID}/export`, (route) =>
    route.fulfill({
      json: { pid: PID, masked: true, plan: { name: "Website replatform" } },
    }),
  );
  await page.goto(`/en-001/plans/${PID}/export`);
  await settle(page);
  await expect(
    page.getByRole("heading", { level: 2, name: "The plan's record (JSON)" }),
  ).toBeVisible();
  await expect(
    page.getByText(/written to the plan's audit trail/),
  ).toBeVisible();
  const link = page.getByRole("link", { name: "Download the plan's record" });
  // A download is not seen by page.route here; the link's address is the check that it asks the right route.
  await expect(link).toHaveAttribute(
    "href",
    new RegExp(`/api/proxy/api/plans/${PID}/export$`),
  );
  const download = page.waitForEvent("download");
  await link.click();
  expect((await download).suggestedFilename()).toBe(`plan-${PID}.json`);
});
