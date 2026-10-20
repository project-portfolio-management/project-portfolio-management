import { describe, expect, it } from "vitest";
import {
  OUTCOMES,
  REASONS,
  STATUS_ORDER,
  STATUS_SYMBOL,
  STATUS_TEXT,
  ageText,
  eventLabel,
  isLive,
  levelText,
  noteOk,
  outcomeLabel,
  raiseProblems,
  reasonLabel,
} from "../../src/lib/escalation-view";

const draft = { title: "Cost is over", reason_kind: "tolerance", tolerance_dimension: "cost", level: "", owner_ref: "" };

describe("escalation words and shapes", () => {
  it("gives every state a word and its own shape", () => {
    for (const s of STATUS_ORDER) {
      expect(STATUS_TEXT[s]).toBeTruthy();
      expect(STATUS_SYMBOL[s]).toBeTruthy();
    }
    expect(new Set(STATUS_ORDER.map((s) => STATUS_SYMBOL[s])).size).toBe(STATUS_ORDER.length);
    expect(isLive({ status: "open" }) && isLive({ status: "acknowledged" })).toBe(true);
    expect(isLive({ status: "resolved" }) || isLive({ status: "withdrawn" })).toBe(false);
  });

  it("labels reasons, outcomes and events, and passes an unknown value through", () => {
    for (const r of REASONS) expect(reasonLabel(r)).not.toMatch(/^l\./);
    for (const o of OUTCOMES) expect(outcomeLabel(o)).not.toBe(o);
    expect(outcomeLabel("odd")).toBe("odd");
    expect(eventLabel("level_raised")).toBe("Taken up a level");
    expect(eventLabel("odd")).toBe("odd");
  });

  it("says the level, the age and when it is late in words", () => {
    expect(levelText({ level: 2, escalated_to: "Sponsor" })).toBe("Level 2: Sponsor");
    expect(levelText({ level: 1, escalated_to: null })).toBe("Level 1");
    expect(ageText({ status: "open", age_days: 12, overdue: false })).toBe("Open for 12 days");
    expect(ageText({ status: "open", age_days: 12, overdue: true })).toBe("Open for 12 days — overdue");
    expect(ageText({ status: "resolved", age_days: 3, overdue: false })).toBe("Took 3 days");
  });
});

describe("escalation drafts", () => {
  it("passes a good draft and names each problem in a bad one", () => {
    expect(raiseProblems(draft)).toEqual([]);
    expect(raiseProblems({ ...draft, title: " " })).toHaveLength(1);
    expect(raiseProblems({ ...draft, tolerance_dimension: "" })).toHaveLength(1);
    expect(raiseProblems({ ...draft, reason_kind: "risk", tolerance_dimension: "" })).toEqual([]);
    for (const level of ["0", "10", "1.5", "x"]) expect(raiseProblems({ ...draft, level })).toHaveLength(1);
    for (const level of ["", "1", "9"]) expect(raiseProblems({ ...draft, level })).toEqual([]);
    expect(raiseProblems({ ...draft, owner_ref: "nobody" })).toHaveLength(1);
    expect(raiseProblems({ ...draft, owner_ref: "person:abc" })).toEqual([]);
  });

  it("counts a note or response only from ten characters", () => {
    expect(noteOk("too short")).toBe(false);
    expect(noteOk("   exactly ten ")).toBe(true);
    expect(noteOk("123456789")).toBe(false);
    expect(noteOk("")).toBe(false);
  });
});
