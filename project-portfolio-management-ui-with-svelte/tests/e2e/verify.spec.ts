// The sign-in link page (CQ-24). The exchange happens in the server's load, so there is no browser
// request to stub: a link with no token is refused at once, and one that cannot be exchanged (the
// test run has no sign-in service) shows a page message, never a 500.
import { test, expect } from "@playwright/test";

test.describe("verify", () => {
  test("a link with no token says so and offers a new one", async ({ page }) => {
    await page.goto("/en-001/verify", { waitUntil: "networkidle" });
    await expect(page.getByRole("heading", { level: 1, name: "Sign-in link" })).toBeVisible();
    await expect(page.getByRole("alert")).toHaveText("This sign-in link is missing its token.");
    await expect(page.getByRole("link", { name: "Request a new link" })).toHaveAttribute("href", /\/signin$/);
  });

  test("a token that cannot be exchanged shows a message on the page", async ({ page }) => {
    const res = await page.goto("/en-001/verify?token=not-a-real-token", { waitUntil: "networkidle" });
    expect(res?.status()).toBe(200);
    await expect(page.getByRole("alert")).toHaveText(
      /We could not reach the sign-in service\. Please try again in a moment\.|This sign-in link is invalid or has expired\./,
    );
  });

  test("reads in Welsh", async ({ page }) => {
    await page.goto("/cy-001/verify", { waitUntil: "networkidle" });
    await expect(page.getByRole("heading", { level: 1 })).not.toHaveText("Sign-in link");
  });

  test("passes axe", async ({ page }) => {
    await page.goto("/en-001/verify", { waitUntil: "networkidle" });
    const { default: AxeBuilder } = await import("@axe-core/playwright");
    const r = await new AxeBuilder({ page }).withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"]).analyze();
    expect(r.violations.filter((v) => v.impact === "serious" || v.impact === "critical").map((v) => `${v.id}: ${v.nodes.map((n) => n.target.join(" ")).join(" | ")}`)).toEqual([]);
  });
});
