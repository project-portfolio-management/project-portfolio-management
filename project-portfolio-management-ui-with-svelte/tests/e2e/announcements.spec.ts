// The announcements feed (CQ-24): its content, publishing and retracting, and the reader without
// the publishing permission, over stubbed API answers.
import { test, expect } from "@playwright/test";

const NOW = Date.now();
const iso = (msAgo: number) => new Date(NOW - msAgo).toISOString();
const FEED = [
  { pid: "a1", title: "Planned maintenance", body: "The service restarts at nine.", severity: "urgent", pinned: true, publish_at: iso(3 * 3_600_000), expires_at: null, author_ref: "person:1", state: "live" },
  { pid: "a2", title: "New report pack", body: "The director pack is live.", severity: "info", pinned: false, publish_at: iso(2 * 86_400_000), expires_at: null, author_ref: "person:1", state: "live" },
];

test.describe("announcements", () => {
  test("shows each announcement with its severity as a word, and publishers can retract", async ({ page }) => {
    await page.route("**/api/announcements**", (r) => r.fulfill({ json: { announcements: FEED, total: 2 } }));
    await page.goto("/en-001/announcements", { waitUntil: "networkidle" });
    await expect(page.getByRole("heading", { level: 1, name: "Announcements" })).toBeVisible();
    const items = page.getByTestId("announcements").getByRole("listitem");
    await expect(items).toHaveCount(2);
    await expect(items.first()).toContainText("Planned maintenance");
    await expect(items.first()).toContainText("The service restarts at nine.");
    await expect(items.first()).toContainText("Urgent");
    await expect(items.first()).toContainText("3 h ago");
    await expect(items.nth(1)).toContainText("Info");
    await expect(page.getByRole("button", { name: "Retract New report pack" })).toBeVisible();
  });

  test("publishes a new announcement and reloads the feed", async ({ page }) => {
    let posted: Record<string, unknown> | null = null;
    await page.route("**/api/announcements**", async (r) => {
      if (r.request().method() === "POST") {
        posted = r.request().postDataJSON();
        return r.fulfill({ status: 201, json: { ...FEED[1], pid: "a3", title: "Hello" } });
      }
      return r.fulfill({ json: { announcements: FEED, total: 2 } });
    });
    await page.goto("/en-001/announcements", { waitUntil: "networkidle" });
    await page.getByLabel("Title").fill("Hello");
    await page.getByLabel("Message").fill("A short note.");
    await page.getByLabel("Severity").selectOption("important");
    await page.getByLabel("Pin to the top").check();
    await page.getByRole("button", { name: "Publish", exact: true }).click();
    await expect.poll(() => posted).toMatchObject({ title: "Hello", body: "A short note.", severity: "important", pinned: true });
    await expect(page.getByLabel("Title")).toHaveValue("");
  });

  test("retracts the announcement that was chosen", async ({ page }) => {
    let deleted = "";
    await page.route("**/api/announcements**", async (r) => {
      if (r.request().method() === "DELETE") {
        deleted = new URL(r.request().url()).pathname;
        return r.fulfill({ status: 204, body: "" });
      }
      return r.fulfill({ json: { announcements: FEED, total: 2 } });
    });
    await page.goto("/en-001/announcements", { waitUntil: "networkidle" });
    await page.getByRole("button", { name: "Retract New report pack" }).click();
    await expect.poll(() => deleted).toMatch(/\/api\/announcements\/a2$/);
  });

  test("a reader without the permission sees the live feed and a note, not the form", async ({ page }) => {
    await page.route("**/api/announcements**", (r) => {
      const all = new URL(r.request().url()).searchParams.get("include") === "all";
      return all ? r.fulfill({ status: 403, json: { error: "forbidden" } }) : r.fulfill({ json: { announcements: FEED, total: 2 } });
    });
    await page.goto("/en-001/announcements", { waitUntil: "networkidle" });
    await expect(page.getByTestId("announcements").getByRole("listitem")).toHaveCount(2);
    await expect(page.getByRole("button", { name: /^Retract/ })).toHaveCount(0);
    await expect(page.getByLabel("Title")).toHaveCount(0);
  });

  test("says so when nothing has been announced", async ({ page }) => {
    await page.route("**/api/announcements**", (r) => r.fulfill({ json: { announcements: [], total: 0 } }));
    await page.goto("/en-001/announcements", { waitUntil: "networkidle" });
    await expect(page.getByTestId("no-announcements")).toBeVisible();
  });

  test("passes axe with the feed populated", async ({ page }) => {
    await page.route("**/api/announcements**", (r) => r.fulfill({ json: { announcements: FEED, total: 2 } }));
    await page.goto("/en-001/announcements", { waitUntil: "networkidle" });
    await expect(page.getByTestId("announcements")).toBeVisible();
    const { default: AxeBuilder } = await import("@axe-core/playwright");
    const r = await new AxeBuilder({ page }).withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"]).analyze();
    expect(r.violations.filter((v) => v.impact === "serious" || v.impact === "critical").map((v) => `${v.id}: ${v.nodes.map((n) => n.target.join(" ")).join(" | ")}`)).toEqual([]);
  });
});
