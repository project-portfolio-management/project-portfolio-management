// The roadmap categories page's rules (RE-7; NC-1 slice 12 part l).

import { describe, expect, it } from "vitest";
import {
  NAME_MAX,
  categoryChange,
  isLanguageCode,
  nameTaken,
  parseTranslations,
  shownName,
  translationsText,
  type RoadmapCategory,
} from "../../src/lib/roadmap-category-view";

const c: RoadmapCategory = {
  pid: "c1",
  name: "Patients",
  translations: { cy: "Cleifion" },
  active: true,
};

describe("translations", () => {
  it("takes the service's language codes", () => {
    for (const ok of ["cy", "zh-cn", "pt-br", "es-419"])
      expect(isLanguageCode(ok)).toBe(true);
    for (const bad of ["Welsh", "c", "zh_cn", "cy-"])
      expect(isLanguageCode(bad)).toBe(false);
  });
  it("reads one code: text to a line, and writes them back by code", () => {
    expect(parseTranslations(" CY : Cleifion \n\nfr: Patients ")).toEqual({
      translations: { cy: "Cleifion", fr: "Patients" },
    });
    expect(translationsText({ fr: "Patients", cy: "Cleifion" })).toBe(
      "cy: Cleifion\nfr: Patients",
    );
  });
  it("says every problem once", () => {
    expect(parseTranslations("Welsh: Cleifion\ncy:\ncy: a\ncy: b")).toEqual({
      problems: [
        "p.roadmap_categories.code_bad",
        "p.roadmap_categories.text_bad",
        "p.roadmap_categories.code_twice",
      ],
    });
  });
});

describe("categoryChange", () => {
  it("adds a category with its name and translations", () => {
    expect(categoryChange(" Staff ", "cy: Staff", null)).toEqual({
      body: { name: "Staff", translations: { cy: "Staff" } },
    });
  });
  it("sends only what changed, and nothing for no change", () => {
    expect(
      categoryChange("Patients", translationsText(c.translations), c),
    ).toBeNull();
    expect(categoryChange("Patients and public", "cy: Cleifion", c)).toEqual({
      body: { name: "Patients and public" },
    });
    expect(categoryChange("Patients", "", c)).toEqual({
      body: { translations: {} },
    });
  });
  it("refuses a blank or long name", () => {
    expect(categoryChange(" ", "", null)).toEqual({
      problems: ["p.roadmap_categories.name_bad"],
    });
    expect(categoryChange("x".repeat(NAME_MAX + 1), "", null)).toEqual({
      problems: ["p.roadmap_categories.name_bad"],
    });
  });
});

describe("nameTaken and shownName", () => {
  it("finds a name taken ignoring case, but not by the category itself", () => {
    expect(nameTaken(" patients ", [c])).toBe(true);
    expect(nameTaken("Patients", [c], "c1")).toBe(false);
  });
  it("shows the reader's translation, else its language's, else the name", () => {
    expect(shownName(c, "cy-001")).toBe("Cleifion");
    expect(
      shownName({ ...c, translations: { "zh-cn": "患者" } }, "zh-CN"),
    ).toBe("患者");
    expect(shownName(c, "en-001")).toBe("Patients");
  });
});
