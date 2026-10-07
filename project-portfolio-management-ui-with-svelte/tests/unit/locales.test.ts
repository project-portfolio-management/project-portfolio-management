import { describe, it, expect } from "vitest";
import {
  LOCALES,
  LOCALE_ALIASES,
  isUnlocalized,
  localeFromAcceptLanguage,
  localeRedirect,
  localeOfPath,
  localizePath,
  normaliseLocale,
  splitPath,
} from "../../src/lib/locales";

describe("locale aliases", () => {
  it("maps a bare language code to its -001 locale", () => {
    expect(LOCALE_ALIASES.en).toBe("en-001");
    expect(LOCALE_ALIASES.cy).toBe("cy-001");
    expect(LOCALE_ALIASES.ar).toBe("ar-001");
  });
  it("maps de and zh to their -001 locales, not the regional copies", () => {
    expect(LOCALE_ALIASES.de).toBe("de-001");
    expect(LOCALE_ALIASES.zh).toBe("zh-001");
  });
  it("never shadows a real locale", () => {
    for (const alias of Object.keys(LOCALE_ALIASES)) {
      expect((LOCALES as readonly string[]).includes(alias)).toBe(false);
    }
  });
});

describe("splitPath", () => {
  it("reads a locale prefix", () => {
    expect(splitPath("/cy-001/plans/1")).toEqual({
      locale: "cy-001",
      alias: null,
      rest: "/plans/1",
    });
    expect(splitPath("/en-001")).toEqual({
      locale: "en-001",
      alias: null,
      rest: "/",
    });
    expect(splitPath("/en-001/")).toMatchObject({
      locale: "en-001",
      rest: "/",
    });
  });
  it("reads an alias prefix", () => {
    expect(splitPath("/en/plans")).toEqual({
      locale: null,
      alias: "en-001",
      rest: "/plans",
    });
    expect(splitPath("/cy")).toMatchObject({ alias: "cy-001", rest: "/" });
  });
  it("leaves an unprefixed or unknown path alone", () => {
    expect(splitPath("/plans")).toEqual({
      locale: null,
      alias: null,
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
    expect(localizePath("/en/plans", "cy-001")).toBe("/cy-001/plans");
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
  it("sends an alias to its locale permanently, ignoring the preference", () => {
    expect(localeRedirect(u("/en/plans"), "cy-001")).toEqual({
      status: 301,
      location: "/en-001/plans",
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
    expect(normaliseLocale("cy_GB")).toBe("cy-001");
    expect(normaliseLocale("cy-GB")).toBe("cy-001");
    expect(normaliseLocale("de_DE")).toBe("de-de");
    expect(normaliseLocale("zh_CN")).toBe("zh-cn");
    expect(normaliseLocale("es-MX")).toBe("es-001");
  });
  it("a language the app does not have resolves to nothing, so the default applies", () => {
    expect(normaliseLocale("ja-JP")).toBeNull();
    expect(normaliseLocale("")).toBeNull();
    expect(normaliseLocale(undefined)).toBeNull();
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
