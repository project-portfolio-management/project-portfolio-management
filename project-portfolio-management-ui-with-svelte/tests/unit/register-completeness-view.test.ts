// Register completeness (EX-1; NC-1 slice 12 part d): the field names, the order, the state words and the switch.

import { describe, expect, it } from "vitest";
import {
  REGISTER_FIELDS,
  counts,
  fieldLabel,
  fieldList,
  orderProgrammes,
  stateOf,
  switchKey,
} from "../../src/lib/register-completeness-view";
import type { RegisterProgramme } from "../../src/lib/api/ppm-types";

const prog = (over: Partial<RegisterProgramme>): RegisterProgramme => ({
  plan_pid: "p1",
  name: "Apollo",
  complete: false,
  score_pct: 50,
  present: 5,
  applicable: 10,
  missing: ["sro"],
  not_applicable: [],
  ...over,
});

describe("register completeness words", () => {
  it("names every field the service checks in the catalogue's words, never by its key", () => {
    for (const f of REGISTER_FIELDS) {
      const label = fieldLabel(f);
      expect(label).not.toMatch(/^p\./);
      expect(label).not.toMatch(/_/);
    }
    expect(fieldLabel("sro")).toBe("senior responsible owner");
    expect(fieldLabel("funding_source")).toBe("funding source");
  });

  it("says 'another field' for a key it does not know", () => {
    expect(fieldLabel("service text")).toBe("another field");
  });

  it("joins a list of fields, and says none for an empty one", () => {
    expect(fieldList(["sro", "top_risks"])).toBe(
      "senior responsible owner, top risks",
    );
    expect(fieldList([])).toBe("none");
  });

  it("gives each state a word and a shape", () => {
    expect(stateOf({ complete: true })).toEqual({
      symbol: "✓",
      text: "complete",
    });
    expect(stateOf({ complete: false })).toEqual({
      symbol: "▲",
      text: "incomplete",
    });
  });

  it("says which way the switch is", () => {
    expect(switchKey(true)).toBe("p.register_completeness.switch_on");
    expect(switchKey(false)).toBe("p.register_completeness.switch_off");
  });
});

describe("register completeness order and counts", () => {
  it("lists the incomplete first, the least complete first, then by name", () => {
    const rows = [
      prog({ plan_pid: "a", name: "Zeta", complete: true, score_pct: 100 }),
      prog({ plan_pid: "b", name: "Beta", score_pct: 80 }),
      prog({ plan_pid: "c", name: "Alpha", score_pct: 80 }),
      prog({ plan_pid: "d", name: "Delta", score_pct: 20 }),
      prog({ plan_pid: "e", name: "Able", complete: true, score_pct: 100 }),
    ];
    expect(orderProgrammes(rows).map((r) => r.name)).toEqual([
      "Delta",
      "Alpha",
      "Beta",
      "Able",
      "Zeta",
    ]);
    // The input is not changed.
    expect(rows[0]!.name).toBe("Zeta");
  });

  it("counts the complete programmes out of all of them", () => {
    expect(
      counts({
        programmes: [
          prog({ complete: true }),
          prog({}),
          prog({ complete: true }),
        ],
      }),
    ).toEqual({ complete: 2, total: 3 });
    expect(counts({ programmes: [] })).toEqual({ complete: 0, total: 0 });
  });
});
