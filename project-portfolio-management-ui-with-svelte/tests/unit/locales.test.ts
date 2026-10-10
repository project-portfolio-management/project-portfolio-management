import { describe, it, expect } from "vitest";
import {
  LOCALES,
  isUnlocalized,
  localeFromAcceptLanguage,
  localeRedirect,
  localeOfPath,
  localizePath,
  normalizeLocale,
  splitPath,
} from "../../src/lib/locales";

describe("splitPath", () => {
  it("reads a locale prefix", () => {
    expect(splitPath("/cy-001/plans/1")).toEqual({
      locale: "cy-001",
      rest: "/plans/1",
    });
    expect(splitPath("/en-001")).toEqual({
      locale: "en-001",
      rest: "/",
    });
    expect(splitPath("/en-001/")).toMatchObject({
      locale: "en-001",
      rest: "/",
    });
  });
  it("does not treat a two-letter code as a locale: there is no alias", () => {
    for (const path of ["/en/plans", "/cy", "/de/tour", "/zh/"]) {
      expect(splitPath(path).locale, path).toBeNull();
      expect(localeOfPath(path), path).toBeNull();
    }
    expect(splitPath("/en/plans").rest).toBe("/en/plans");
  });
  it("leaves an unprefixed or unknown path alone", () => {
    expect(splitPath("/plans")).toEqual({
      locale: null,
      rest: "/plans",
    });
    expect(splitPath("/xx-yy/plans").rest).toBe("/xx-yy/plans");
    expect(splitPath("/").rest).toBe("/");
    // `en-gb` is not supported: it is not a prefix, so it is just a path.
    expect(splitPath("/en-gb/plans").locale).toBeNull();
  });
  it("does not mistake a path that merely starts with a language", () => {
    expect(splitPath("/english").rest).toBe("/english");
    expect(localeOfPath("/envelope")).toBeNull();
  });
});

describe("localizePath", () => {
  it("prefixes app paths, keeping query and fragment", () => {
    expect(localizePath("/plans", "cy-001")).toBe("/cy-001/plans");
    expect(localizePath("/", "en-001")).toBe("/en-001");
    expect(localizePath("/plans?x=1#a", "de-de")).toBe("/de-de/plans?x=1#a");
    expect(localizePath("/?/signout", "en-001")).toBe("/en-001?/signout");
  });
  it("replaces an existing prefix rather than stacking", () => {
    expect(localizePath("/en-001/plans/2", "cy-001")).toBe("/cy-001/plans/2");
    expect(localizePath(localizePath("/plans", "cy-001"), "cy-001")).toBe(
      "/cy-001/plans",
    );
  });
  it("leaves external, fragment and unlocalized paths alone", () => {
    expect(localizePath("https://example.com/x", "cy-001")).toBe(
      "https://example.com/x",
    );
    expect(localizePath("//cdn.example/x", "cy-001")).toBe("//cdn.example/x");
    expect(localizePath("#top", "cy-001")).toBe("#top");
    expect(localizePath("/api/plans", "cy-001")).toBe("/api/plans");
    expect(localizePath("/assets/themes/", "cy-001")).toBe("/assets/themes/");
  });
});

describe("isUnlocalized", () => {
  it("covers the API, framework and static paths", () => {
    for (const p of [
      "/api/proxy/x",
      "/_app/immutable/a.js",
      "/assets/a.css",
      "/robots.txt",
      "/favicon.svg",
    ]) {
      expect(isUnlocalized(p), p).toBe(true);
    }
    for (const p of ["/", "/plans", "/apiary"])
      expect(isUnlocalized(p), p).toBe(false);
  });
});

describe("localeFromAcceptLanguage", () => {
  it("picks the first supported tag, by quality", () => {
    expect(localeFromAcceptLanguage("cy-GB,cy;q=0.9,en;q=0.8")).toBe("cy-001");
    expect(localeFromAcceptLanguage("ja,fr;q=0.8,en;q=0.5")).toBe("fr-001");
    expect(localeFromAcceptLanguage("en;q=0.5,de;q=0.9")).toBe("de-001");
    expect(localeFromAcceptLanguage("zh-TW")).toBe("zh-001");
  });
  it("ignores wildcards, q=0 and nothing-supported", () => {
    expect(localeFromAcceptLanguage("*")).toBeNull();
    expect(localeFromAcceptLanguage("cy;q=0,ja")).toBeNull();
    expect(localeFromAcceptLanguage("")).toBeNull();
    expect(localeFromAcceptLanguage(null)).toBeNull();
  });
});

describe("localeRedirect", () => {
  const u = (pathname: string, search = "", hash = "") => ({
    pathname,
    search,
    hash,
  });
  it("gives a bare path the preferred locale, keeping query and fragment", () => {
    expect(localeRedirect(u("/plans", "?a=1", "#x"), "cy-001")).toEqual({
      status: 307,
      location: "/cy-001/plans?a=1#x",
    });
    expect(localeRedirect(u("/"), "de-de")).toEqual({
      status: 307,
      location: "/de-de",
    });
  });
  it("gives a two-letter path the reader's locale, like any bare path (no forwarding to -001)", () => {
    expect(localeRedirect(u("/en/plans"), "cy-001")).toEqual({
      status: 307,
      location: "/cy-001/en/plans",
    });
  });
  it("leaves localized and unlocalized URLs alone", () => {
    expect(localeRedirect(u("/cy-001/plans"), "en-001")).toBeNull();
    expect(localeRedirect(u("/api/proxy/x"), "en-001")).toBeNull();
    expect(localeRedirect(u("/assets/a.css"), "en-001")).toBeNull();
  });
});

describe("navigator.language on the root route", () => {
  it("a region or underscore form resolves to the nearest supported locale", () => {
    expect(normalizeLocale("cy_GB")).toBe("cy-001");
    expect(normalizeLocale("cy-GB")).toBe("cy-001");
    expect(normalizeLocale("de_DE")).toBe("de-de");
    expect(normalizeLocale("zh_CN")).toBe("zh-cn");
    expect(normalizeLocale("es-MX")).toBe("es-001");
  });
  it("a language the app does not have resolves to nothing, so the default applies", () => {
    expect(normalizeLocale("ja-JP")).toBeNull();
    expect(normalizeLocale("")).toBeNull();
    expect(normalizeLocale(undefined)).toBeNull();
  });
  it("the root path gains the locale prefix", () => {
    const url = { pathname: "/", search: "", hash: "" };
    expect(localeRedirect(url, "cy-001")).toEqual({
      status: 307,
      location: "/cy-001",
    });
    expect(localeRedirect({ ...url, search: "?x=1" }, "de-de")).toEqual({
      status: 307,
      location: "/de-de?x=1",
    });
  });
});
