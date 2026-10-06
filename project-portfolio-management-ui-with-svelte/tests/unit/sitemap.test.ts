import { describe, expect, it } from "vitest";
import { LOCALES } from "../../src/lib/locales";
import {
  buildRobots,
  buildSitemap,
  cleanOrigin,
  hreflang,
  pagePaths,
  xmlEscape,
} from "../../src/lib/sitemap";

const FILES = [
  "/src/routes/+page.svelte",
  "/src/routes/ceo/+page.svelte",
  "/src/routes/plans/+page.svelte",
  "/src/routes/plans/[pid]/+page.svelte",
  "/src/routes/plans/[pid]/wbs/+page.svelte",
  "/src/routes/resources/rbs/+page.svelte",
  "/src/routes/signin/+page.svelte",
  "/src/routes/verify/+page.svelte",
  "/src/routes/(app)/okrs/+page.svelte",
  "/src/routes/ceo/+layout.svelte",
];

describe("page paths", () => {
  it("lists static pages only, sorted, once", () => {
    expect(pagePaths([...FILES, ...FILES])).toEqual([
      "/",
      "/ceo",
      "/okrs",
      "/plans",
      "/resources/rbs",
    ]);
  });
  it("never lists a record, a sign-in page or a layout", () => {
    const all = pagePaths(FILES).join(" ");
    expect(all).not.toMatch(/pid|\[|signin|verify|layout/);
  });
});

describe("the sitemap", () => {
  const xml = buildSitemap("https://ppm.example", ["/", "/ceo"]);
  it("has one url per page per locale, each with every alternate and x-default", () => {
    expect(xml.match(/<url>/g)?.length).toBe(2 * LOCALES.length);
    expect(xml.match(/<xhtml:link /g)?.length).toBe(
      2 * LOCALES.length * (LOCALES.length + 1),
    );
    expect(xml).toContain("<loc>https://ppm.example/en-001/ceo</loc>");
    expect(xml).toContain("<loc>https://ppm.example/de-de</loc>");
    expect(xml).toContain(
      'hreflang="x-default" href="https://ppm.example/en-001/ceo"',
    );
    expect(xml).toContain('hreflang="de-DE"');
    expect(xml).toContain('hreflang="zh-CN"');
  });
  it("is well-formed XML with the namespaces", () => {
    expect(xml.startsWith('<?xml version="1.0" encoding="UTF-8"?>')).toBe(true);
    expect(xml).toContain(
      'xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"',
    );
    expect(xml.trimEnd().endsWith("</urlset>")).toBe(true);
  });
  it("escapes what XML cannot hold raw", () => {
    expect(xmlEscape(`a&b<c>"d'`)).toBe("a&amp;b&lt;c&gt;&quot;d&apos;");
    expect(buildSitemap("https://x.example", ["/a&b"])).toContain("/a&amp;b");
  });
  it("is the same every time", () => {
    expect(buildSitemap("https://ppm.example", ["/", "/ceo"])).toBe(xml);
  });
});

describe("helpers", () => {
  it("hreflang is BCP 47", () => {
    expect(hreflang("en-001")).toBe("en");
    expect(hreflang("de-de")).toBe("de-DE");
  });
  it("only an http(s) origin is accepted, reduced to its origin", () => {
    expect(cleanOrigin("https://ppm.example/some/path?x=1")).toBe(
      "https://ppm.example",
    );
    expect(cleanOrigin("javascript:alert(1)")).toBeNull();
    expect(cleanOrigin("")).toBeNull();
    expect(cleanOrigin("not a url")).toBeNull();
  });
  it("robots names the sitemap and keeps crawlers off the API", () => {
    expect(buildRobots("https://ppm.example")).toBe(
      "User-agent: *\nDisallow: /api/\nSitemap: https://ppm.example/sitemap.xml\n",
    );
  });
});
