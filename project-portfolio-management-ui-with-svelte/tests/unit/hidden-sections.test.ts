import { describe, expect, it } from "vitest";
import {
  MAX_HIDDEN,
  applyHidden,
  parseHidden,
  parsePrefs,
  serialQueue,
  withHidden,
} from "../../src/lib/preferences";

const nav = (...hrefs: string[]) =>
  hrefs.map((href) => ({ href, label: href }));

describe("parseHidden", () => {
  it("keeps in-app paths once each and drops everything else", () => {
    expect(parseHidden(["/okrs", "/okrs", "/kpis"])).toEqual([
      "/okrs",
      "/kpis",
    ]);
    expect(
      parseHidden([
        "//evil.example",
        "/\\evil.example",
        "https://x",
        "okrs",
        3,
        null,
        "/",
      ]),
    ).toBeUndefined();
    expect(parseHidden("/okrs")).toBeUndefined();
    expect(parseHidden([])).toBeUndefined();
  });
  it("takes at most MAX_HIDDEN", () => {
    const many = Array.from({ length: MAX_HIDDEN + 20 }, (_, i) => `/s${i}`);
    expect(parseHidden(many)).toHaveLength(MAX_HIDDEN);
  });
});

describe("parsePrefs with hidden sections", () => {
  it("round-trips them and ignores a bad list without touching the other choices", () => {
    expect(parsePrefs({ simple: true, hidden: ["/okrs"] })).toEqual({
      simple: true,
      hidden: ["/okrs"],
    });
    expect(
      parsePrefs(JSON.stringify({ dyslexia: true, hidden: "nope" })),
    ).toEqual({ dyslexia: true });
  });
});

describe("withHidden", () => {
  it("hides and shows a section, and stores nothing once none is hidden", () => {
    let p = withHidden({}, "/okrs", true);
    expect(p).toEqual({ hidden: ["/okrs"] });
    p = withHidden(p, "/kpis", true);
    expect(p.hidden).toEqual(["/okrs", "/kpis"]);
    p = withHidden(p, "/okrs", false);
    expect(p.hidden).toEqual(["/kpis"]);
    p = withHidden(p, "/kpis", false);
    expect("hidden" in p).toBe(false);
  });
  it("keeps the other choices and never hides the way home", () => {
    expect(
      withHidden({ simple: false, dyslexia: true }, "/okrs", true),
    ).toEqual({ simple: false, dyslexia: true, hidden: ["/okrs"] });
    expect(withHidden({}, "/", true)).toEqual({});
  });
});

describe("applyHidden", () => {
  const items = nav("/", "/okrs", "/kpis");
  it("changes nothing for a person who never hid anything", () => {
    expect(applyHidden(items, {})).toEqual(items);
  });
  it("removes only what the person hid, and never the way home", () => {
    expect(
      applyHidden(items, { hidden: ["/okrs"] }).map((i) => i.href),
    ).toEqual(["/", "/kpis"]);
    expect(applyHidden(items, { hidden: ["/"] }).map((i) => i.href)).toEqual([
      "/",
      "/okrs",
      "/kpis",
    ]);
  });
  it("lets a section added later reach everyone who did not hide it, including someone who hid another", () => {
    const later = nav("/", "/okrs", "/kpis", "/new-section");
    expect(applyHidden(later, {}).map((i) => i.href)).toContain("/new-section");
    expect(
      applyHidden(later, { hidden: ["/okrs"] }).map((i) => i.href),
    ).toEqual(["/", "/kpis", "/new-section"]);
  });
});

describe("serialQueue", () => {
  it("runs tasks one at a time in the order asked, and a failure does not stop the next", async () => {
    const run = serialQueue();
    const log: string[] = [];
    let active = 0;
    let overlapped = false;
    const task =
      (name: string, ms: number, fail = false) =>
      () =>
        new Promise<string>((resolve, reject) => {
          active++;
          if (active > 1) overlapped = true;
          log.push(`start ${name}`);
          setTimeout(() => {
            active--;
            log.push(`end ${name}`);
            if (fail) reject(new Error(name));
            else resolve(name);
          }, ms);
        });
    const results = await Promise.allSettled([
      run(task("a", 30)),
      run(task("b", 5, true)),
      run(task("c", 1)),
    ]);
    expect(overlapped).toBe(false);
    expect(log).toEqual([
      "start a",
      "end a",
      "start b",
      "end b",
      "start c",
      "end c",
    ]);
    expect(results.map((r) => r.status)).toEqual([
      "fulfilled",
      "rejected",
      "fulfilled",
    ]);
    expect((results[2] as PromiseFulfilledResult<string>).value).toBe("c");
  });
});
