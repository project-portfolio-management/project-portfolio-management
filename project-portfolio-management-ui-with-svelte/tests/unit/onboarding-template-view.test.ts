// The checklist templates page's rules (NC-1 slice 12 part n).

import { describe, expect, it } from "vitest";
import {
  MAX_ITEMS,
  TITLE_MAX,
  offsetKey,
  rowsOf,
  sameItems,
  templateBody,
} from "../../src/lib/onboarding-template-view";

describe("templateBody", () => {
  it("trims titles and reads offsets as whole days", () => {
    expect(
      templateBody([
        { title: " Book time ", offset: "-3" },
        { title: "Review", offset: "+14" },
      ]),
    ).toEqual({
      items: [
        { title: "Book time", offset_days: -3 },
        { title: "Review", offset_days: 14 },
      ],
    });
  });
  it("reports every problem with the item it is about", () => {
    expect(
      templateBody([
        { title: " ", offset: "0" },
        { title: "x".repeat(TITLE_MAX + 1), offset: "1.5" },
        { title: "ok", offset: "366" },
      ]),
    ).toEqual({
      problems: [
        { key: "p.onboarding_templates.title_bad", n: 1 },
        { key: "p.onboarding_templates.title_bad", n: 2 },
        { key: "p.onboarding_templates.offset_bad", n: 2 },
        { key: "p.onboarding_templates.offset_bad", n: 3 },
      ],
    });
  });
  it("needs 1 to 50 items", () => {
    expect(templateBody([])).toEqual({
      problems: [{ key: "p.onboarding_templates.count_bad" }],
    });
    const many = Array.from({ length: MAX_ITEMS + 1 }, () => ({
      title: "a",
      offset: "0",
    }));
    expect(templateBody(many)).toEqual({
      problems: [{ key: "p.onboarding_templates.count_bad" }],
    });
    expect(templateBody(many.slice(1))).toHaveProperty("items");
  });
});

describe("rowsOf, sameItems and offsetKey", () => {
  it("round-trips a checklist", () => {
    const items = [{ title: "a", offset_days: -7 }];
    const r = templateBody(rowsOf(items));
    expect("items" in r && sameItems(r.items, items)).toBe(true);
    expect(sameItems(items, [{ title: "a", offset_days: -6 }])).toBe(false);
    expect(sameItems(items, [])).toBe(false);
  });
  it("reads an offset as before, on or after the day", () => {
    expect(offsetKey(-7)).toEqual({
      key: "p.onboarding_templates.before",
      n: 7,
    });
    expect(offsetKey(0)).toEqual({
      key: "p.onboarding_templates.on_the_day",
      n: 0,
    });
    expect(offsetKey(3)).toEqual({ key: "p.onboarding_templates.after", n: 3 });
  });
});
