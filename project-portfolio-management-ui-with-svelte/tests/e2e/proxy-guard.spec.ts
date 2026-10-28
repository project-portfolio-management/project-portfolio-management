// CQ-5: the BFF proxy refuses a state-changing request from another site
// before it calls the service, so this runs against the real proxy route with
// no service behind it.

import { test, expect } from "@playwright/test";

test("a cross-site write to the proxy is refused with 403", async ({
  request,
}) => {
  const res = await request.post("/api/proxy/api/plans", {
    headers: {
      Origin: "https://evil.example",
      "Content-Type": "application/json",
    },
    data: { name: "x" },
  });
  expect(res.status()).toBe(403);
  const body = await res.json();
  expect(body.error).toBe("forbidden");
  expect(body.description).toMatch(/Origin/);
});

test("a write marked cross-site by Sec-Fetch-Site is refused", async ({
  request,
}) => {
  const res = await request.delete(
    "/api/proxy/api/plans/00000000-0000-0000-0000-000000000000",
    {
      headers: { "Sec-Fetch-Site": "cross-site" },
    },
  );
  expect(res.status()).toBe(403);
});
