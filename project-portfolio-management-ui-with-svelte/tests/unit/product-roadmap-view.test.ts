// A product roadmap's rules (RE-1, RE-3; NC-1 slice 12 part l).

import { describe, expect, it } from "vitest";
import {
  TAG_MAX,
  TITLE_MAX,
  groups,
  isSlug,
  translationChange,
  translationInput,
  translationSummary,
  fieldKey,
  publishSet,
  valueWords,
  itemChange,
  itemFormOf,
  roadmapChange,
  roadmapFormOf,
  step,
  tagsOf,
  type RoadmapItem,
} from "../../src/lib/product-roadmap-view";

const r = {
  title: "Payments",
  slug: "payments",
  vision: "Pay in a tap",
  service_description: null,
  category_pid: null,
};
const item = (
  pid: string,
  placement: RoadmapItem["placement"],
  position: number,
  over: Partial<RoadmapItem> = {},
): RoadmapItem => ({
  pid,
  public_id: `R-${pid}`,
  title: `Item ${pid}`,
  description: null,
  expected_outcome: null,
  phase: null,
  tags: [],
  placement,
  position,
  ...over,
});

describe("isSlug and tagsOf", () => {
  it("takes the service's slugs only", () => {
    expect(isSlug("payments-2")).toBe(true);
    for (const bad of [
      "",
      "Pay",
      "-pay",
      "pay-",
      "pa--y",
      "pay me",
      "x".repeat(65),
    ])
      expect(isSlug(bad)).toBe(false);
  });
  it("reads tags from one line", () => {
    expect(tagsOf(" cards, , wallets ")).toEqual(["cards", "wallets"]);
  });
});

describe("roadmapChange", () => {
  it("makes a roadmap from a title alone, the slug left to the service", () => {
    expect(
      roadmapChange({ ...roadmapFormOf(null), title: " Payments " }, null),
    ).toEqual({ body: { title: "Payments" } });
  });
  it("sends only what changed, an emptied text as empty, and nothing for no change", () => {
    expect(roadmapChange(roadmapFormOf(r), r)).toBeNull();
    expect(
      roadmapChange(
        {
          ...roadmapFormOf(r),
          vision: " ",
          service_description: "Card payments",
        },
        r,
      ),
    ).toEqual({
      body: { vision: "", service_description: "Card payments" },
    });
  });
  it("says every problem first", () => {
    expect(
      roadmapChange({ ...roadmapFormOf(r), title: " ", slug: "Bad Slug" }, r),
    ).toEqual({
      problems: ["p.product_roadmap.title_bad", "p.product_roadmap.slug_bad"],
    });
    expect(
      roadmapChange(
        { ...roadmapFormOf(r), title: "x".repeat(TITLE_MAX + 1) },
        r,
      ),
    ).toEqual({
      problems: ["p.product_roadmap.title_bad"],
    });
  });
});

describe("itemChange", () => {
  it("adds an item with what is filled in and its placement", () => {
    expect(
      itemChange(
        { ...itemFormOf(null), title: " Tap to pay ", tags: "cards, mobile" },
        null,
        "next",
      ),
    ).toEqual({
      body: {
        title: "Tap to pay",
        tags: ["cards", "mobile"],
        placement: "next",
      },
    });
  });
  it("changes only what changed, the tags as the whole list", () => {
    const i = item("a", "now", 0, { tags: ["cards"], phase: "Beta" });
    expect(itemChange(itemFormOf(i), i)).toBeNull();
    expect(
      itemChange({ ...itemFormOf(i), phase: "", tags: "cards, wallets" }, i),
    ).toEqual({
      body: { phase: "", tags: ["cards", "wallets"] },
    });
  });
  it("refuses a repeated or a long tag", () => {
    expect(
      itemChange(
        { ...itemFormOf(null), title: "a", tags: "Cards, cards" },
        null,
      ),
    ).toEqual({ problems: ["p.product_roadmap.tag_twice"] });
    expect(
      itemChange(
        { ...itemFormOf(null), title: "a", tags: "x".repeat(TAG_MAX + 1) },
        null,
      ),
    ).toEqual({
      problems: ["p.product_roadmap.tag_long"],
    });
  });
});

describe("groups and step", () => {
  const list = [
    item("c", null, 0),
    item("b", "now", 1),
    item("a", "now", 0),
    item("d", "later", 0),
  ];
  it("puts the placements in page order and the unplaced last", () => {
    expect(
      groups(list).map((g) => [g.placement, g.items.map((i) => i.pid)]),
    ).toEqual([
      ["now", ["a", "b"]],
      ["later", ["d"]],
      [null, ["c"]],
    ]);
  });
  it("moves one place within the placement, and not past either end", () => {
    expect(step(list, list[2]!, 1)).toEqual({ placement: "now", index: 1 });
    expect(step(list, list[2]!, -1)).toBeNull();
    expect(step(list, list[1]!, 1)).toBeNull();
  });
});

describe("publishing (RE-2)", () => {
  const words = {
    yes: "Yes",
    no: "No",
    none: "none",
    placement: (p: string) => p.toUpperCase(),
  };
  it("publishes or withdraws through one change set, the base being what was read", () => {
    expect(publishSet(false, true)).toEqual({
      roadmap: { published: { base: false, value: true } },
    });
    expect(publishSet(true, false)).toEqual({
      roadmap: { published: { base: true, value: false } },
    });
  });
  it("says a value in words", () => {
    expect(valueWords("published", true, words)).toBe("Yes");
    expect(valueWords("placement", "now", words)).toBe("NOW");
    expect(valueWords("tags", ["a", "b"], words)).toBe("a, b");
    expect(valueWords("details", [{ text: "x", level: 0 }], words)).toBe("x");
    expect(valueWords("vision", null, words)).toBe("none");
    expect(valueWords("tags", [], words)).toBe("none");
    expect(fieldKey("vision")).toBe("p.product_roadmap.field.vision");
    expect(fieldKey(null)).toBe("p.product_roadmap.field.item");
  });
});

describe("a roadmap's category (RE-7)", () => {
  const r = {
    title: "Payments",
    slug: "payments",
    vision: null,
    service_description: null,
    category_pid: "c1",
  };
  it("sends a chosen category, null to clear it, and nothing when it is the same", () => {
    expect(roadmapChange(roadmapFormOf(r), r)).toBeNull();
    expect(roadmapChange({ ...roadmapFormOf(r), category: "c2" }, r)).toEqual({
      body: { category_pid: "c2" },
    });
    expect(roadmapChange({ ...roadmapFormOf(r), category: "" }, r)).toEqual({
      body: { category_pid: null },
    });
    expect(
      roadmapChange(
        { ...roadmapFormOf(null), title: "New", category: "c1" },
        null,
      ),
    ).toEqual({
      body: { title: "New", category_pid: "c1" },
    });
  });
});

describe("translations (RE-6)", () => {
  const sources = {
    title: "Tap to pay",
    description: null,
    phase: "Beta",
    tags: ["cards", "mobile"],
  };
  const fields = ["title", "description", "phase", "tags"];
  const shown = {
    title: { value: "Talu â thap" },
    tags: { value: ["cardiau", "symudol"] },
  };
  it("sends only what changed, an emptied one as empty, never a field with no source", () => {
    const inputs = {
      title: "Talu â thap",
      description: "Disgrifiad",
      phase: "Beta",
      tags: "cardiau, symudol",
    };
    expect(translationChange(fields, sources, shown, inputs)).toEqual({
      body: { phase: "Beta" },
    });
    expect(
      translationChange(fields, sources, shown, {
        ...inputs,
        phase: "",
        title: "",
      }),
    ).toEqual({ body: { title: "" } });
    expect(
      translationChange(fields, sources, shown, {
        title: "Talu â thap",
        tags: "cardiau, symudol",
      }),
    ).toBeNull();
  });
  it("sends tags as a list as long as the source's, and says a wrong count or a long text first", () => {
    expect(
      translationChange(fields, sources, shown, {
        title: "Talu â thap",
        tags: "un, dau",
      }),
    ).toEqual({
      body: { tags: ["un", "dau"] },
    });
    expect(
      translationChange(fields, sources, shown, {
        title: "Talu â thap",
        tags: "un",
      }),
    ).toEqual({
      problems: [{ key: "p.product_roadmap.tr_tags_count", field: "tags" }],
    });
    expect(
      translationChange(fields, sources, shown, {
        title: "x".repeat(201),
        tags: "cardiau, symudol",
      }),
    ).toEqual({
      problems: [{ key: "p.product_roadmap.tr_too_long", field: "title" }],
    });
  });
  it("reads a translation into its input, and sums what needs review by language", () => {
    expect(translationInput(["a", "b"])).toBe("a, b");
    expect(translationInput(null)).toBe("");
    expect(
      translationSummary([
        { cy: { title: { value: "a", needs_review: true } } },
        {
          cy: { phase: { value: "b", needs_review: false } },
          fr: { title: { value: "c", needs_review: true } },
        },
        undefined,
      ]),
    ).toEqual([
      { lang: "cy", review: 1 },
      { lang: "fr", review: 1 },
    ]);
  });
});
