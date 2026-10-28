// Collaborative review (CQ-24): delegate, accept, give a verdict, withdraw, and the consensus,
// over a small stubbed service that remembers what was done.
import { test, expect, type Page } from "@playwright/test";

const SUBJECT = "22222222-2222-4222-8222-222222222222";
const review = (over: Record<string, unknown>) => ({
  pid: "v1",
  subject_kind: "plan",
  subject_pid: SUBJECT,
  reviewer_ref: "person:aaaa",
  reviewer_scope: "internal",
  expertise: "clinical safety",
  status: "invited",
  due_on: null,
  score: null,
  recommendation: null,
  comment: null,
  invited_by: "person:me",
  responded_at: null,
  submitted_at: null,
  ...over,
});

async function stub(page: Page, rows: Record<string, unknown>[]) {
  const calls: string[] = [];
  const bodies: Record<string, unknown> = {};
  await page.route("**/api/reviews**", async (r) => {
    const req = r.request();
    const path = new URL(req.url()).pathname.replace(/^\/api\/proxy/, "");
    calls.push(`${req.method()} ${path}`);
    if (req.method() !== "GET") bodies[path] = req.postDataJSON();
    if (path === "/api/reviews/consensus") {
      return r.fulfill({
        json: {
          invited: 3,
          submitted: 2,
          outstanding: 1,
          declined: 0,
          mean_score: 72.5,
          recommendations: { advance: 2 },
          majority: "advance",
          external_submitted: 1,
          complete: false,
        },
      });
    }
    if (req.method() === "GET")
      return r.fulfill({
        json: rows,
        headers: { "x-total-count": String(rows.length) },
      });
    if (req.method() === "DELETE") return r.fulfill({ status: 204, body: "" });
    return r.fulfill({ status: 201, json: rows[0] ?? review({}) });
  });
  return { calls, bodies };
}

test.describe("reviews", () => {
  test("lists each invitation with its scope, status and the actions that fit the status", async ({
    page,
  }) => {
    await stub(page, [
      review({}),
      review({ pid: "v2", status: "accepted", reviewer_scope: "external" }),
      review({
        pid: "v3",
        status: "submitted",
        score: 80,
        recommendation: "advance",
      }),
    ]);
    await page.goto("/en-001/reviews", { waitUntil: "networkidle" });
    await expect(
      page.getByRole("heading", { level: 1, name: "Collaborative review" }),
    ).toBeVisible();
    const invited = page
      .getByRole("row", { name: /person:aaaa.*invited/ })
      .first();
    await expect(invited.getByRole("button", { name: "Accept" })).toBeVisible();
    await expect(
      invited.getByRole("button", { name: "Decline" }),
    ).toBeVisible();
    await expect(
      invited.getByRole("button", { name: "Withdraw" }),
    ).toBeVisible();
    const accepted = page.getByRole("row", { name: /external.*accepted/ });
    await expect(
      accepted.getByRole("button", { name: "Advance" }),
    ).toBeVisible();
    await expect(
      accepted.getByRole("button", { name: "Reject" }),
    ).toBeVisible();
    const submitted = page.getByRole("row", { name: /submitted/ });
    await expect(submitted).toContainText("advance · 80");
    await expect(submitted.getByRole("button")).toHaveCount(0);
  });

  test("delegating sends the subject, the reviewer and the scope", async ({
    page,
  }) => {
    const s = await stub(page, []);
    await page.goto("/en-001/reviews", { waitUntil: "networkidle" });
    await expect(page.getByText("No review invitations yet")).toBeVisible();
    await page.getByLabel("Subject pid").fill(SUBJECT);
    await page.getByLabel("reviewer", { exact: true }).fill("person:bbbb");
    await page.getByLabel(/^scope/).selectOption("external");
    await page.getByPlaceholder("e.g. clinical safety").fill("security");
    await page.getByRole("button", { name: "Delegate" }).click();
    await expect
      .poll(() => s.bodies["/api/reviews"])
      .toMatchObject({
        subject_kind: "plan",
        subject_pid: SUBJECT,
        reviewer_ref: "person:bbbb",
        reviewer_scope: "external",
        expertise: "security",
      });
    await expect(page.getByText("Invitation sent")).toBeVisible();
  });

  test("accepting sends the response; a verdict asks for a score and refuses one out of range", async ({
    page,
  }) => {
    const s = await stub(page, [review({ status: "accepted" })]);
    await page.goto("/en-001/reviews", { waitUntil: "networkidle" });
    page.once("dialog", (d) => d.accept("150"));
    await page.getByRole("button", { name: "Advance" }).click();
    await expect(page.getByRole("alert")).toBeVisible();
    expect(s.calls).not.toContain("POST /api/reviews/v1/submit");
    page.once("dialog", (d) => d.accept("85"));
    await page.getByRole("button", { name: "Hold" }).click();
    await expect
      .poll(() => s.bodies["/api/reviews/v1/submit"])
      .toEqual({ score: 85, recommendation: "hold" });
  });

  test("respond and withdraw call the right endpoints", async ({ page }) => {
    const s = await stub(page, [review({})]);
    await page.goto("/en-001/reviews", { waitUntil: "networkidle" });
    await page.getByRole("button", { name: "Accept" }).click();
    await expect
      .poll(() => s.bodies["/api/reviews/v1/respond"])
      .toEqual({ response: "accept" });
    await page.getByRole("button", { name: "Withdraw" }).click();
    await expect.poll(() => s.calls).toContain("DELETE /api/reviews/v1");
  });

  test("shows the consensus for the subject typed in", async ({ page }) => {
    await stub(page, []);
    await page.goto("/en-001/reviews", { waitUntil: "networkidle" });
    await page.getByLabel("Subject pid").fill(SUBJECT);
    await page
      .getByRole("button", { name: "Consensus for this subject" })
      .click();
    await expect(
      page.getByText("2 submitted · 1 outstanding · 0 declined · 1 external"),
    ).toBeVisible();
    await expect(
      page.getByText("mean score: 72.5 · majority: advance"),
    ).toBeVisible();
  });

  test("says so when the list cannot be loaded", async ({ page }) => {
    await page.route("**/api/reviews**", (r) =>
      r.fulfill({ status: 500, json: { error: "boom" } }),
    );
    await page.goto("/en-001/reviews", { waitUntil: "networkidle" });
    await expect(page.getByRole("alert")).toBeVisible();
  });

  test("passes axe with invitations listed", async ({ page }) => {
    await stub(page, [review({}), review({ pid: "v2", status: "accepted" })]);
    await page.goto("/en-001/reviews", { waitUntil: "networkidle" });
    await expect(page.getByRole("row", { name: /invited/ })).toBeVisible();
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
});
