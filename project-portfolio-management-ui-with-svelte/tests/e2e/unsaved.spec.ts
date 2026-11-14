// RE-13: what was typed into the plan form outlives an ended session, and leaving with changes not saved asks first.
import { test, expect } from "./test";
import { PID, stubPlansApi } from "./fixtures";

test("a save refused because the session ended keeps what was typed and offers sign-in in a new tab", async ({
  page,
}) => {
  await stubPlansApi(page);
  let answer = 401;
  await page.route("**/api/proxy/api/plans", async (route) => {
    if (route.request().method() !== "POST") return route.fallback();
    if (answer === 401)
      return route.fulfill({ status: 401, json: { error: "unauthorized" } });
    return route.fulfill({ json: { pid: PID, name: "Clinic move" } });
  });
  await page.goto("/en-001/plans/new", { waitUntil: "networkidle" });
  const name = page.locator("main form input[type=text]").first();
  await name.fill("Clinic move");
  await page.locator("main form button[type=submit]").click();

  const alert = page.locator("main [role=alert]");
  await expect(alert).toContainText(
    "Your session has ended. What you typed is still here",
  );
  const signIn = alert.getByRole("link", {
    name: "Sign in again (opens a new tab)",
  });
  await expect(signIn).toHaveAttribute("target", "_blank");
  await expect(signIn).toHaveAttribute("href", "/en-001/signin");
  await expect(name).toHaveValue("Clinic move");
  await expect(page).toHaveURL(/\/plans\/new$/);

  // Signed in again elsewhere, the same form saves.
  answer = 200;
  await page.locator("main form button[type=submit]").click();
  await expect(alert).toHaveCount(0);
});

test("leaving the form with changes not saved asks first, and staying keeps them", async ({
  page,
}) => {
  await stubPlansApi(page);
  await page.goto("/en-001/plans/new", { waitUntil: "networkidle" });
  const name = page.locator("main form input[type=text]").first();
  await name.fill("Half-written plan");
  const asked: string[] = [];
  page.once("dialog", async (d) => {
    asked.push(d.message());
    await d.dismiss();
  });
  await page.evaluate(() => {
    const a = document.querySelector<HTMLAnchorElement>('a[href$="/plans"]');
    a?.click();
  });
  await expect
    .poll(() => asked)
    .toEqual(["You have changes that are not saved. Leave this page anyway?"]);
  await expect(page).toHaveURL(/\/plans\/new$/);
  await expect(name).toHaveValue("Half-written plan");
});
