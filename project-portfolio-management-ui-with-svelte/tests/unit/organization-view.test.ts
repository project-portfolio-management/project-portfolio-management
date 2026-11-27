import { describe, expect, it } from "vitest";
import {
  isOrgCode,
  namesText,
  orgChange,
  orgFormOf,
  parseNames,
  shownOrgName,
  type OrgNamed,
} from "../../src/lib/organization-view";

const swansea: OrgNamed = {
  pid: "o1",
  name: "Swansea Bay University Health Board",
  code: "SBUHB",
  national: false,
  short_name: "Swansea Bay",
  translations: {
    cy: {
      name: "Bwrdd Iechyd Prifysgol Bae Abertawe",
      short_name: "Bae Abertawe",
    },
  },
};
const dhcw: OrgNamed = {
  pid: "o2",
  name: "Digital Health and Care Wales",
  code: "DHCW",
  national: true,
  short_name: null,
  translations: {},
};
const list = [swansea, dhcw];

describe("organization names (RE-7)", () => {
  it("takes a code of a letter or digit then letters, digits or hyphens, at most 20", () => {
    for (const c of ["SBUHB", "7a", "a-b-c", "x".repeat(20)])
      expect(isOrgCode(c)).toBe(true);
    for (const c of ["-a", "a b", "a_b", "x".repeat(21), "é"])
      expect(isOrgCode(c)).toBe(false);
  });

  it("writes and reads the names in other languages one to a line", () => {
    expect(namesText(swansea.translations)).toBe(
      "cy: Bwrdd Iechyd Prifysgol Bae Abertawe | Bae Abertawe",
    );
    expect(parseNames(" CY: Enw | Byr \n\nzh-cn: 名称 ")).toEqual({
      translations: {
        cy: { name: "Enw", short_name: "Byr" },
        "zh-cn": { name: "名称" },
      },
    });
    expect(parseNames("cy: A\ncy: B")).toEqual({
      problems: ["p.organizations.tr_code_twice"],
    });
    expect(parseNames("welsh: A\ncy:\nfr: A | " + "x".repeat(51))).toEqual({
      problems: [
        "p.organizations.tr_code_bad",
        "p.organizations.tr_name_bad",
        "p.organizations.tr_short_bad",
      ],
    });
  });

  it("sends nothing for no change, and only what changed", () => {
    const f = orgFormOf(swansea);
    expect(orgChange(f, swansea, list)).toBeNull();
    expect(
      orgChange(
        { ...f, short_name: " ", code: "sbu-hb", translations: "" },
        swansea,
        list,
      ),
    ).toEqual({ body: { short_name: "", code: "sbu-hb", translations: {} } });
  });

  it("says every problem first, a code, a name or the national scope another holds among them", () => {
    const f = orgFormOf(swansea);
    expect(
      orgChange(
        {
          ...f,
          name: " digital health and care wales ",
          code: "dhcw",
          national: true,
        },
        swansea,
        list,
      ),
    ).toEqual({
      problems: [
        "p.organizations.name_taken",
        "p.organizations.code_taken",
        "p.organizations.national_taken",
      ],
    });
    expect(
      orgChange(
        {
          ...f,
          name: "",
          short_name: "x".repeat(51),
          code: "a b",
          translations: "cy",
        },
        swansea,
        list,
      ),
    ).toEqual({
      problems: [
        "p.organizations.name_bad",
        "p.organizations.short_bad",
        "p.organizations.code_bad",
        "p.organizations.tr_name_bad",
      ],
    });
    expect(
      orgChange({ ...orgFormOf(dhcw), national: false }, dhcw, list),
    ).toEqual({
      body: { national: false },
    });
  });

  it("shows the reader's language's name, else the stored one", () => {
    expect(shownOrgName(swansea, "cy-001")).toBe(
      "Bwrdd Iechyd Prifysgol Bae Abertawe",
    );
    expect(shownOrgName(swansea, "en-001")).toBe(swansea.name);
  });
});
