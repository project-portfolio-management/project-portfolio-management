// A product family's rules (RE-5, RE-7; NC-1 slice 12 part l).

import { describe, expect, it } from "vitest";
import { colourOf, familyChange } from "../../src/lib/family-view";

describe("colourOf", () => {
  it("keeps any six-digit colour in lower case, as the service does", () => {
    expect(colourOf(" #00A3E0 ")).toBe("#00a3e0");
    for (const bad of ["325083", "#fff", "#12345g", "red", ""])
      expect(colourOf(bad)).toBeNull();
  });
});

describe("familyChange", () => {
  const f = { colour: "#325083", translations: { cy: "Arian" } };
  it("sends nothing when nothing changed, the colour alone when only it changed", () => {
    expect(familyChange("#325083", "cy: Arian", f)).toBeNull();
    expect(familyChange("#00A3E0", "cy: Arian", f)).toEqual({
      body: { colour: "#00a3e0" },
    });
  });
  it("sends the whole set of translations when they changed", () => {
    expect(familyChange("#325083", "cy: Arian\nfr: Argent", f)).toEqual({
      body: { colour: "#325083", translations: { cy: "Arian", fr: "Argent" } },
    });
    expect(
      familyChange("#325083", "", { colour: null, translations: {} }),
    ).toEqual({ body: { colour: "#325083" } });
  });
  it("says a bad colour and a bad translation first", () => {
    expect(familyChange("navy", "Welsh: Arian", f)).toEqual({
      problems: [
        "p.roadmap_family.colour_bad",
        "p.roadmap_categories.code_bad",
      ],
    });
  });
});
