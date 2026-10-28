import { describe, expect, it } from "vitest";
import {
  MAX_RECENT,
  MAX_ROWS,
  move,
  opensPalette,
  parseRecent,
  planItem,
  pushRecent,
  rank,
  rows,
  score,
  type PaletteItem,
} from "../../src/lib/palette-view";

const cmd = (name: string, label: string, href: string): PaletteItem => ({
  id: `cmd:${name}`,
  kind: "command",
  label,
  href,
});

describe("score", () => {
  it("orders the whole label, a prefix, a word start, an inside match and letters in order", () => {
    expect(score("plan", "Plan")).toBe(4);
    expect(score("pla", "Plan")).toBe(3);
    expect(score("rep", "Website replatform")).toBe(2);
    expect(score("lat", "Website replatform")).toBe(1);
    expect(score("wsr", "Website replatform")).toBe(0.5);
    expect(score("zzz", "Website replatform")).toBe(0);
  });
  it("ignores case and accents", () => {
    expect(score("CAFE", "Café plan")).toBe(3);
    expect(score("cymru", "Cymru")).toBe(4);
  });
  it("treats an empty query as matching everything equally", () => {
    expect(score("  ", "Anything")).toBe(1);
  });
});

describe("rank", () => {
  const items = [
    cmd("a", "My work", "/me"),
    cmd("b", "New plan", "/plans/new"),
    planItem({ pid: "1", name: "Apollo" }),
  ];
  it("drops non-matches and puts the best first", () => {
    expect(rank("new", items).map((i) => i.label)).toEqual(["New plan"]);
    expect(rank("p", items).map((i) => i.label)).toEqual([
      "New plan",
      "Apollo",
    ]);
  });
  it("keeps the given order for equal scores and for an empty query", () => {
    expect(rank("", items).map((i) => i.label)).toEqual([
      "My work",
      "New plan",
      "Apollo",
    ]);
  });
  it("shows at most MAX_ROWS", () => {
    const many = Array.from({ length: 40 }, (_, i) =>
      planItem({ pid: String(i), name: `Plan ${i}` }),
    );
    expect(rank("plan", many)).toHaveLength(MAX_ROWS);
  });
});

describe("rows", () => {
  const commands = [
    cmd("me", "My work", "/me"),
    cmd("new", "New plan", "/plans/new"),
  ];
  const apollo = planItem({ pid: "1", name: "Apollo" });
  it("shows recents then commands when nothing is typed, once each", () => {
    const recent = pushRecent([], apollo);
    expect(rows("", commands, [], recent).map((i) => i.label)).toEqual([
      "Apollo",
      "My work",
      "New plan",
    ]);
    const again = pushRecent(recent, commands[0]!);
    expect(rows("", commands, [], again).map((i) => i.id)).toEqual([
      "cmd:me",
      "plan:1",
      "cmd:new",
    ]);
  });
  it("shows commands and found plans for a query, best first", () => {
    expect(rows("apo", commands, [apollo], []).map((i) => i.label)).toEqual([
      "Apollo",
    ]);
    expect(rows("new", commands, [apollo], []).map((i) => i.label)).toEqual([
      "New plan",
    ]);
  });
});

describe("recent items", () => {
  it("puts the newest first, once, and keeps MAX_RECENT", () => {
    let list: PaletteItem[] = [];
    for (let i = 0; i < 8; i++)
      list = pushRecent(list, planItem({ pid: String(i), name: `P${i}` }));
    expect(list).toHaveLength(MAX_RECENT);
    expect(list[0]!.id).toBe("plan:7");
    list = pushRecent(list, planItem({ pid: "5", name: "P5" }));
    expect(list.map((i) => i.id).filter((id) => id === "plan:5")).toHaveLength(
      1,
    );
    expect(list[0]!.id).toBe("plan:5");
    expect(list.every((i) => i.kind === "recent")).toBe(true);
  });
  it("reads back what it wrote and drops anything malformed or off-site", () => {
    const list = pushRecent([], planItem({ pid: "1", name: "Apollo" }));
    expect(parseRecent(JSON.stringify(list))).toEqual(list);
    expect(parseRecent(null)).toEqual([]);
    expect(parseRecent("not json")).toEqual([]);
    expect(parseRecent('{"a":1}')).toEqual([]);
    const bad = [
      { id: "x", label: "Evil", href: "//evil.example/" },
      { id: "y", label: "Evil", href: "https://evil.example/" },
      { id: "w", label: "Evil", href: "/\\evil.example/" },
      { id: 3, label: "n", href: "/ok" },
      { id: "z", label: "Fine", href: "/plans/z" },
    ];
    expect(parseRecent(JSON.stringify(bad)).map((i) => i.id)).toEqual(["z"]);
  });
});

describe("keys", () => {
  it("opens on Ctrl-K or Cmd-K only", () => {
    const e = (
      key: string,
      ctrlKey = false,
      metaKey = false,
      altKey = false,
    ) => ({ key, ctrlKey, metaKey, altKey });
    expect(opensPalette(e("k", true))).toBe(true);
    expect(opensPalette(e("K", false, true))).toBe(true);
    expect(opensPalette(e("k"))).toBe(false);
    expect(opensPalette(e("j", true))).toBe(false);
    expect(opensPalette(e("k", true, false, true))).toBe(false);
  });
  it("moves with the arrows, wrapping, and is -1 with no rows", () => {
    expect(move(-1, "ArrowDown", 3)).toBe(0);
    expect(move(2, "ArrowDown", 3)).toBe(0);
    expect(move(0, "ArrowUp", 3)).toBe(2);
    expect(move(-1, "ArrowUp", 3)).toBe(2);
    expect(move(1, "ArrowUp", 3)).toBe(0);
    expect(move(0, "ArrowDown", 0)).toBe(-1);
  });
});
