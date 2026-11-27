// The custom fields page's rules (RM-1; NC-1 slice 12 part n).

import { describe, expect, it } from "vitest";
import {
  EMPTY_DEF,
  MAX_SHORT,
  declareRefusalKey,
  defBody,
  keyTaken,
  inputOf,
  kindApplies,
  MAX_TEXT,
  searchValue,
  shownValue,
  validKey,
  valueChanges,
  valueOf,
  type ValueDef,
} from "../../src/lib/custom-field-view";

describe("validKey", () => {
  it("takes the service's lowercase identifiers only", () => {
    expect(validKey("cost_centre")).toBe(true);
    expect(validKey("a1")).toBe(true);
    for (const bad of [
      "",
      "Cost",
      "1a",
      "a-b",
      "a b",
      "x".repeat(MAX_SHORT + 1),
    ])
      expect(validKey(bad)).toBe(false);
  });
});

describe("defBody", () => {
  it("sends a plan text field with its kinds, trimmed, and no options", () => {
    expect(
      defBody({
        ...EMPTY_DEF,
        key: " cost_centre ",
        label: " Cost centre ",
        kinds: "Project, , Programme",
        options: "a",
      }),
    ).toEqual({
      body: {
        key: "cost_centre",
        label: "Cost centre",
        type: "text",
        applies_to: "plan",
        kinds: ["Project", "Programme"],
        required: false,
        options: [],
      },
    });
  });
  it("drops kinds from a task field and reads options one per line for a list", () => {
    const r = defBody({
      ...EMPTY_DEF,
      key: "size",
      label: "Size",
      field_type: "list",
      applies_to: "task",
      kinds: "Project",
      options: "S\n M \n\nL",
      required: true,
    });
    expect(r).toEqual({
      body: {
        key: "size",
        label: "Size",
        type: "list",
        applies_to: "task",
        kinds: [],
        required: true,
        options: ["S", "M", "L"],
      },
    });
  });
  it("reports every problem at once", () => {
    expect(
      defBody({
        ...EMPTY_DEF,
        key: "Bad",
        label: " ",
        field_type: "multi_list",
      }),
    ).toEqual({
      problems: [
        "p.custom_fields.key_bad",
        "p.custom_fields.label_bad",
        "p.custom_fields.options_none",
      ],
    });
    expect(
      defBody({
        ...EMPTY_DEF,
        key: "k",
        label: "K",
        field_type: "list",
        options: "a\na",
      }),
    ).toEqual({
      problems: ["p.custom_fields.options_twice"],
    });
    expect(
      defBody({
        ...EMPTY_DEF,
        key: "k",
        label: "K",
        field_type: "list",
        options: "x".repeat(MAX_SHORT + 1),
      }),
    ).toEqual({
      problems: ["p.custom_fields.options_long"],
    });
    expect(
      defBody({ ...EMPTY_DEF, key: "k", label: "x".repeat(MAX_SHORT + 1) }),
    ).toEqual({
      problems: ["p.custom_fields.label_bad"],
    });
  });
});

describe("keyTaken, searchValue and declareRefusalKey", () => {
  it("a key is taken per subject type", () => {
    const defs = [{ key: "size", applies_to: "task" }];
    expect(keyTaken(" size ", "task", defs)).toBe(true);
    expect(keyTaken("size", "plan", defs)).toBe(false);
  });
  it("writes a search value for its type", () => {
    expect(searchValue("integer", " 42 ")).toEqual({ value: "42" });
    expect(searchValue("integer", "4.2")).toEqual({
      problem: "p.custom_fields.value_integer",
    });
    expect(searchValue("date", "2027-02-30")).toEqual({
      problem: "p.custom_fields.value_date",
    });
    expect(searchValue("date", "2027-02-28")).toEqual({ value: "2027-02-28" });
    expect(searchValue("bool", "true")).toEqual({ value: "true" });
    expect(searchValue("text", "  ")).toEqual({
      problem: "p.custom_fields.value_required",
    });
  });
  it("says a key taken meanwhile in its own words", () => {
    expect(declareRefusalKey(409)).toBe("p.custom_fields.key_taken");
    expect(declareRefusalKey(422)).toBe("p.custom_fields.could_not_declare");
  });
});

const vd = (over: Partial<ValueDef> = {}): ValueDef => ({
  key: "f",
  label: "F",
  field_type: "text",
  required: false,
  options: [],
  kinds: [],
  ...over,
});
const none = { text: "", many: [] };

describe("one plan's values", () => {
  it("a field limited to kinds applies to those kinds only, ignoring case", () => {
    expect(kindApplies([], null)).toBe(true);
    expect(kindApplies(["Project"], "project")).toBe(true);
    expect(kindApplies(["Project"], "Program")).toBe(false);
    expect(kindApplies(["Project"], null)).toBe(false);
  });
  it("reads each type from its input and refuses a wrong one", () => {
    expect(
      valueOf(vd({ field_type: "integer" }), { ...none, text: " 7 " }),
    ).toEqual({ value: 7 });
    expect(
      valueOf(vd({ field_type: "integer" }), { ...none, text: "7.5" }),
    ).toEqual({ problem: "p.custom_fields.v_integer" });
    expect(
      valueOf(vd({ field_type: "bool" }), { ...none, text: "false" }),
    ).toEqual({ value: false });
    expect(
      valueOf(vd({ field_type: "date" }), { ...none, text: "2027-02-30" }),
    ).toEqual({ problem: "p.custom_fields.v_date" });
    expect(
      valueOf(vd({ field_type: "list", options: ["S"] }), {
        ...none,
        text: "M",
      }),
    ).toEqual({ problem: "p.custom_fields.v_choice" });
    expect(
      valueOf(vd({ field_type: "multi_list", options: ["A", "B", "C"] }), {
        text: "",
        many: ["C", "A"],
      }),
    ).toEqual({ value: ["A", "C"] });
    expect(valueOf(vd(), { ...none, text: "x".repeat(MAX_TEXT + 1) })).toEqual({
      problem: "p.custom_fields.v_too_long",
    });
    expect(valueOf(vd(), { ...none, text: " keep spacing " })).toEqual({
      value: " keep spacing ",
    });
    expect(valueOf(vd(), { ...none, text: "  " })).toEqual({ value: null });
    expect(valueOf(vd({ required: true }), none)).toEqual({
      problem: "p.custom_fields.v_required",
    });
  });
  it("round-trips a stored value through its input", () => {
    const multi = vd({ field_type: "multi_list", options: ["A", "B"] });
    expect(valueOf(multi, inputOf(multi, ["B"]))).toEqual({ value: ["B"] });
    const flag = vd({ field_type: "bool" });
    expect(valueOf(flag, inputOf(flag, true))).toEqual({ value: true });
    expect(inputOf(vd(), null)).toEqual(none);
  });
  it("sends only what changed, a cleared value as null, and names the field of each problem", () => {
    const defs = [
      vd({ key: "a", label: "A" }),
      vd({ key: "b", label: "B", field_type: "integer" }),
      vd({ key: "c", label: "C" }),
    ];
    const stored = {
      a: { value: "same", label: "A", definition: "live" as const },
      c: { value: "old", label: "C", definition: "live" as const },
    };
    expect(
      valueChanges(
        defs,
        { a: { ...none, text: "same" }, b: { ...none, text: "3" }, c: none },
        stored,
      ),
    ).toEqual({
      values: { b: 3, c: null },
    });
    expect(
      valueChanges(
        [...defs, vd({ key: "d", label: "D", required: true })],
        { b: { ...none, text: "x" } },
        stored,
      ),
    ).toEqual({
      problems: [
        { key: "p.custom_fields.v_integer", label: "B" },
        { key: "p.custom_fields.v_required", label: "D" },
      ],
    });
  });
  it("shows a stored value in words", () => {
    expect(shownValue(true, "Yes", "No")).toBe("Yes");
    expect(shownValue(["A", "B"], "Yes", "No")).toBe("A, B");
    expect(shownValue(4, "Yes", "No")).toBe("4");
  });
});
