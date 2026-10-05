import { describe, it, expect } from "vitest";
import {
  PREFERENCES_NAME,
  PREFERENCES_ROUTE,
  SIMPLE_NAV_HREFS,
  effectiveSimple,
  fromSavedViews,
  htmlAttributes,
  parsePrefs,
  splitNav,
  splitPlanLinks,
  toSavedView,
} from "../../src/lib/preferences";
import { KNOWN_NAV_HREFS, landingRouteForView } from "../../src/lib/nav";

describe("simple mode default and choice", () => {
  it("is on by default only for the worker persona, and off for everyone else", () => {
    expect(effectiveSimple({}, "me")).toBe(true);
    expect(effectiveSimple({}, "executive")).toBe(false);
    expect(effectiveSimple({}, null)).toBe(false);
    expect(effectiveSimple({}, undefined)).toBe(false);
  });
  it("an explicit choice always wins over the default", () => {
    expect(effectiveSimple({ simple: false }, "me")).toBe(false);
    expect(effectiveSimple({ simple: true }, "executive")).toBe(true);
    expect(effectiveSimple({ simple: true }, null)).toBe(true);
  });
});

describe("parsePrefs", () => {
  it("keeps only booleans and survives rubbish", () => {
    expect(parsePrefs('{"simple":true,"dyslexia":false}')).toEqual({
      simple: true,
      dyslexia: false,
    });
    expect(parsePrefs({ simple: "yes", dyslexia: 1, other: true })).toEqual({});
    expect(parsePrefs("{not json")).toEqual({});
    expect(parsePrefs(null)).toEqual({});
    expect(parsePrefs(7)).toEqual({});
    expect(parsePrefs("null")).toEqual({});
  });
});

describe("simple mode changes prominence, not capability", () => {
  const items = [
    { href: "/", label: "Home" },
    { href: "/me", label: "My work" },
    { href: "/plans", label: "Plans" },
    { href: "/risk", label: "Risk" },
    { href: "/board", label: "Board" },
    { href: "/capacity", label: "Capacity" },
    { href: "/tour", label: "Tour" },
  ];
  it("shows everything in front when simple mode is off", () => {
    expect(splitNav(items, false)).toEqual({ front: items, more: [] });
  });
  it("moves the advanced destinations behind More, and drops nothing", () => {
    const { front, more } = splitNav(items, true);
    expect(front.map((i) => i.href)).toEqual([
      "/",
      "/me",
      "/plans",
      "/board",
      "/tour",
    ]);
    expect(more.map((i) => i.href)).toEqual(["/risk", "/capacity"]);
    expect([...front, ...more].map((i) => i.href).sort()).toEqual(
      items.map((i) => i.href).sort(),
    );
  });
  it("keeps every simple destination a real nav destination", () => {
    // KNOWN_NAV_HREFS lists landing targets only; the tour is a nav item that is not one.
    const known = [...(KNOWN_NAV_HREFS as readonly string[]), "/tour"];
    for (const h of SIMPLE_NAV_HREFS) expect(known).toContain(h);
  });
  it("splits a plan's workspace links the same way, dropping none", () => {
    const links = [
      "governance",
      "board",
      "schedule",
      "flow",
      "critical-path",
      "raci",
      "schedule-health",
    ] as const;
    const { front, more } = splitPlanLinks(links, true);
    expect(front).toEqual(["governance", "board", "schedule"]);
    expect(more).toEqual(["flow", "critical-path", "raci", "schedule-health"]);
    expect(splitPlanLinks(links, false)).toEqual({
      front: [...links],
      more: [],
    });
  });
});

describe("the worker's first click", () => {
  it("a worker (view=me) lands on My work with no further click", () => {
    expect(landingRouteForView(KNOWN_NAV_HREFS, "me", "/plans")).toBe("/me");
  });
  it("and My work is the first destination in front in simple mode", () => {
    expect(SIMPLE_NAV_HREFS[0]).toBe("/me");
  });
});

describe("saved-view storage", () => {
  it("stores preferences as one saved view of the user's own", () => {
    expect(toSavedView({ simple: true, dyslexia: false })).toEqual({
      route: PREFERENCES_ROUTE,
      name: PREFERENCES_NAME,
      filter: { simple: true, dyslexia: false },
      sort: {},
      columns: [],
    });
  });
  it("reads the newest preferences view and ignores every other saved view", () => {
    const views = [
      { route: "/plans", name: "Mine", filter: { simple: true } },
      {
        route: PREFERENCES_ROUTE,
        name: PREFERENCES_NAME,
        filter: { simple: false },
      },
      {
        route: PREFERENCES_ROUTE,
        name: PREFERENCES_NAME,
        filter: { simple: true, dyslexia: true },
      },
    ];
    expect(fromSavedViews(views)).toEqual({
      prefs: { simple: true, dyslexia: true },
    });
    expect(
      fromSavedViews([{ route: "/plans", name: "x", filter: {} }]),
    ).toBeNull();
    expect(fromSavedViews([])).toBeNull();
  });
  it("expresses the preferences as html attributes", () => {
    expect(htmlAttributes(true, true)).toEqual({
      "data-simple": "on",
      "data-font": "dyslexia",
    });
    expect(htmlAttributes(false, false)).toEqual({
      "data-simple": "off",
      "data-font": "default",
    });
  });
});
