// The sitemap and robots.txt are served unlocalized, as XML and text, listing only
// fixed pages in every locale.

import { test, expect } from "@playwright/test";

test("sitemap.xml lists static pages per locale and no records or sign-in", async ({
  request,
  baseURL,
}) => {
  const r = await request.get("/sitemap.xml", { maxRedirects: 0 });
  expect(r.status()).toBe(200);
  expect(r.headers()["content-type"]).toContain("application/xml");
  const xml = await r.text();
  expect(xml).toContain(`<loc>${baseURL}/en-001/ceo</loc>`);
  expect(xml).toContain(`<loc>${baseURL}/de-de/plans</loc>`);
  expect(xml).toContain('hreflang="x-default"');
  expect(xml).not.toMatch(/signin|verify|\[pid\]|\/api\//);
});

test("robots.txt names the sitemap", async ({ request, baseURL }) => {
  const r = await request.get("/robots.txt", { maxRedirects: 0 });
  expect(r.status()).toBe(200);
  expect(await r.text()).toContain(`Sitemap: ${baseURL}/sitemap.xml`);
});
