// Demonstration mode (PPM_DEMO_MODE=true, served on 4174): sign in as example@example.com only, then a
// guided tour of every feature. The API is stubbed.
import { test, expect } from "@playwright/test";

test.use({
  baseURL: `http://localhost:${Number(process.env.PW_PORT ?? 4173) + 1}`,
});

const NORTH = "46314b8b-8f65-4185-b6e9-ab5a4d08524c";
const PLANS = [
  { pid: NORTH, name: "Interlocking Replacement, North Line" },
  { pid: "p2", name: "Signalling Renewal" },
];

test.beforeEach(async ({ page }) => {
  await page.route("**/api/proxy/api/plans*", (r) =>
    r.fulfill({ json: PLANS, headers: { "x-total-count": "2" } }),
  );
});

test.describe("demonstration sign-in", () => {
  test("says it is a demonstration and offers the demonstration address", async ({
    page,
  }) => {
    await page.goto("/en-001/signin", { waitUntil: "networkidle" });
    await expect(
      page.getByRole("heading", { level: 2, name: "Demonstration" }),
    ).toBeVisible();
    await expect(
      page.getByText(
        "Sign in with example@example.com to see every feature with fictitious data.",
      ),
    ).toBeVisible();
    await expect(page.getByLabel("Email")).toHaveValue("example@example.com");
    await expect(
      page.getByRole("button", { name: "Sign in to the demonstration" }),
    ).toBeVisible();
  });

  test("refuses any other address", async ({ page }) => {
    await page.goto("/en-001/signin", { waitUntil: "networkidle" });
    await page.getByLabel("Email").fill("someone@else.test");
    await page
      .getByRole("button", { name: "Sign in to the demonstration" })
      .click();
    await expect(page.getByRole("alert")).toContainText(
      "The demonstration accepts only example@example.com.",
    );
    await expect(page).toHaveURL(/\/signin$/);
  });

  test("signs in as example@example.com and lands on the tour with the fictitious-data banner", async ({
    page,
  }) => {
    await page.goto("/en-001/signin", { waitUntil: "networkidle" });
    await page
      .getByRole("button", { name: "Sign in to the demonstration" })
      .click();
    await expect(page).toHaveURL(/\/en-001\/demo$/);
    await expect(
      page.getByRole("heading", { level: 1, name: "Demonstration tour" }),
    ).toBeVisible();
    await expect(page.locator(".demo-banner")).toContainText("fictitious data");
    await expect(page.getByRole("button", { name: "Sign out" })).toBeVisible();
  });
});

test.describe("the dashboard when signed in", () => {
  const GLANCE = {
    as_of: "2026-10-13T00:00:00Z",
    site_tiles: {
      plans: 10,
      proposals_open: 3,
      materialized_risks: 0,
      open_risk_exposure: 234,
      schedule_violations: 0,
      over_allocated_people: 3,
    },
    collections: [
      {
        collection: "Project",
        total: 6,
        rag: { red: 0, amber: 6, green: 0 },
        stages: { g3_delivery: 2 },
      },
    ],
  };
  test.beforeEach(async ({ page }) => {
    await page.route("**/api/proxy/api/at-a-glance", (r) =>
      r.fulfill({ json: GLANCE }),
    );
  });

  test("signed out: the plain title and a Sign in link in the header, no Sign out", async ({
    page,
  }) => {
    await page.goto("/en-001/dashboard", { waitUntil: "networkidle" });
    await expect(
      page.getByRole("heading", { level: 1, name: "Portfolio dashboard" }),
    ).toBeVisible();
    await expect(page.getByRole("link", { name: "Sign in" })).toBeVisible();
    await expect(page.getByRole("button", { name: "Sign out" })).toHaveCount(0);
  });

  test("signed in: the headline is My Home Page, the header offers Sign out and not Sign in, with six tiles", async ({
    page,
  }) => {
    await page.goto("/en-001/signin", { waitUntil: "networkidle" });
    await page
      .getByRole("button", { name: "Sign in to the demonstration" })
      .click();
    await expect(page).toHaveURL(/\/en-001\/demo$/);
    await page.goto("/en-001/dashboard", { waitUntil: "networkidle" });
    await expect(
      page.getByRole("heading", { level: 1, name: "My Home Page" }),
    ).toBeVisible();
    await expect(
      page.getByRole("heading", { level: 1, name: "Portfolio dashboard" }),
    ).toHaveCount(0);
    const header = page.locator("header.topbar");
    await expect(header.getByText("Sign out", { exact: true })).toBeVisible();
    await expect(header.getByText("Sign in", { exact: true })).toHaveCount(0);
    await expect(page.getByTestId("site-tiles").locator(".tile")).toHaveCount(
      6,
    );
  });

  test("signed in, in Welsh", async ({ page }) => {
    await page.goto("/cy-001/signin", { waitUntil: "networkidle" });
    await page
      .getByRole("button", { name: "Mewngofnodi i'r arddangosiad" })
      .click();
    await expect(page).toHaveURL(/\/cy-001\/demo$/);
    await page.goto("/cy-001/dashboard", { waitUntil: "networkidle" });
    await expect(
      page.getByRole("heading", { level: 1, name: "Fy Nhudalen Hafan" }),
    ).toBeVisible();
  });
});

test.describe("the tour", () => {
  test("groups every feature and links to its screen, with the North Line pages resolved", async ({
    page,
  }) => {
    await page.goto("/en-001/demo", { waitUntil: "networkidle" });
    for (const g of [
      "The portfolio at a glance",
      "Delivering one project (North Line interlocking)",
      "Money",
      "Governance and escalation",
      "Critical chain",
    ]) {
      await expect(
        page.getByRole("heading", { level: 2, name: g }),
      ).toBeVisible();
    }
    await expect(
      page.getByRole("link", { name: "Open: Cutover go/no-go" }),
    ).toHaveAttribute("href", `/en-001/plans/${NORTH}/cutover`);
    await expect(
      page.getByRole("link", { name: "Open: Start queue" }),
    ).toHaveAttribute("href", "/en-001/start-queue");
    await expect(
      page.getByRole("link", { name: "Open: Search" }),
    ).toHaveAttribute("href", "/en-001/search?Signalling");
    expect(await page.locator(".card").count()).toBeGreaterThanOrEqual(35);
  });

  test("hides the North Line links, and says so, when that plan is missing", async ({
    page,
  }) => {
    await page.route("**/api/proxy/api/plans*", (r) =>
      r.fulfill({
        json: [{ pid: "p2", name: "Signalling Renewal" }],
        headers: { "x-total-count": "1" },
      }),
    );
    await page.goto("/en-001/demo", { waitUntil: "networkidle" });
    await expect(
      page
        .getByRole("note")
        .filter({ hasText: "North Line plan was not found" }),
    ).toBeVisible();
    await expect(
      page.getByRole("link", { name: "Open: Cutover go/no-go" }),
    ).toHaveCount(0);
    await expect(
      page.getByRole("link", { name: "Open: Start queue" }),
    ).toBeVisible();
  });

  test("reads in Welsh", async ({ page }) => {
    await page.goto("/cy-001/demo", { waitUntil: "networkidle" });
    await expect(
      page.getByRole("heading", { level: 1, name: "Taith arddangos" }),
    ).toBeVisible();
    await expect(
      page.getByRole("heading", { level: 2, name: "Arian" }),
    ).toBeVisible();
  });

  for (const route of ["signin", "demo"]) {
    for (const locale of ["en-001", "cy-001", "zh-cn"]) {
      test(`${route} passes axe in ${locale}`, async ({ page }) => {
        await page.goto(`/${locale}/${route}`, { waitUntil: "networkidle" });
        const { default: AxeBuilder } = await import("@axe-core/playwright");
        const r = await new AxeBuilder({ page })
          .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
          .analyze();
        expect(
          r.violations
            .filter((v) => v.impact === "serious" || v.impact === "critical")
            .map(
              (v) =>
                `${v.id}: ${v.nodes.map((n) => n.target.join(" ")).join(" | ")}`,
            ),
        ).toEqual([]);
      });
    }
  }
});
