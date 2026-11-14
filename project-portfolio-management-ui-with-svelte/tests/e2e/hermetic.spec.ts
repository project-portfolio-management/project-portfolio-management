// RE-10: the shared `test` (./test.ts) fails any test whose browser reached an origin no test faked.
import { test, expect, isAllowed } from "./test";

test("the allow rule: the app, the mock identity provider and local schemes only", () => {
  const app = "http://localhost:4173";
  expect(isAllowed(new URL("http://localhost:4173/en-001/plans"), app)).toBe(
    true,
  );
  expect(isAllowed(new URL("http://localhost:4180/authorize"), app)).toBe(true);
  expect(isAllowed(new URL("ws://localhost:4173/socket"), app)).toBe(true);
  expect(isAllowed(new URL("data:text/plain,hi"), app)).toBe(true);
  expect(isAllowed(new URL("https://fonts.googleapis.com/css"), app)).toBe(
    false,
  );
  expect(isAllowed(new URL("http://localhost:4181/api/plans"), app)).toBe(
    false,
  );
  expect(isAllowed(new URL("wss://example.invalid/"), app)).toBe(false);
});

// The app's Content Security Policy already stops its own scripts fetching another origin, so these
// use a navigation, which `connect-src` does not govern and the guard must catch.
test("a request a test fakes at another origin is allowed", async ({
  page,
}) => {
  await page.route("https://cdn.example.invalid/**", (r) =>
    r.fulfill({ body: "faked", contentType: "text/plain" }),
  );
  await page.goto("https://cdn.example.invalid/x");
  await expect(page.locator("body")).toHaveText("faked");
});

// Expected to fail: the fixture's teardown finds the blocked address. If the guard were removed,
// this test would pass and `test.fail` would report it as an unexpected pass.
test.fail(
  "a request to an origin no test faked fails the test",
  async ({ page }) => {
    await page.goto("https://unfaked.example.invalid/x").catch(() => undefined);
  },
);
