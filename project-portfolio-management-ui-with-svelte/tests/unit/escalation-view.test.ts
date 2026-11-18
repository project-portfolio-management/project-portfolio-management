import { describe, expect, it } from "vitest";
import {
  OUTCOMES,
  REASONS,
  STATUS_ORDER,
  STATUS_SYMBOL,
  STATUS_TEXT,
  ageText,
  automaticText,
  eventLabel,
  isLive,
  levelText,
  noteOk,
  outcomeLabel,
  raiseProblems,
  reasonLabel,
  RESPONSE_MAX,
  acknowledgeBody,
  acknowledgementText,
  isNoRecord,
  recordRefusalKey,
  recordsToRead,
  resolutionProblems,
  resolutionText,
  resolveBody,
} from "../../src/lib/escalation-view";
import { ApiError } from "../../src/lib/api/client";

const draft = {
  title: "Cost is over",
  reason_kind: "tolerance",
  tolerance_dimension: "cost",
  level: "",
  owner_ref: "",
};

describe("escalation words and shapes", () => {
  it("says so when the scheduled task raised it", () => {
    expect(automaticText({ raised_by: "system:tolerance-escalations" })).toBe(
      "Raised automatically",
    );
    expect(automaticText({ raised_by: "u" })).toBe("");
    expect(automaticText({ raised_by: null })).toBe("");
  });

  it("gives every state a word and its own shape", () => {
    for (const s of STATUS_ORDER) {
      expect(STATUS_TEXT[s]).toBeTruthy();
      expect(STATUS_SYMBOL[s]).toBeTruthy();
    }
    expect(new Set(STATUS_ORDER.map((s) => STATUS_SYMBOL[s])).size).toBe(
      STATUS_ORDER.length,
    );
    expect(
      isLive({ status: "open" }) && isLive({ status: "acknowledged" }),
    ).toBe(true);
    expect(
      isLive({ status: "resolved" }) || isLive({ status: "withdrawn" }),
    ).toBe(false);
  });

  it("labels reasons, outcomes and events, and passes an unknown value through", () => {
    for (const r of REASONS) expect(reasonLabel(r)).not.toMatch(/^l\./);
    for (const o of OUTCOMES) expect(outcomeLabel(o)).not.toBe(o);
    expect(outcomeLabel("odd")).toBe("odd");
    expect(eventLabel("level_raised")).toBe("Taken up a level");
    expect(eventLabel("odd")).toBe("odd");
  });

  it("says the level, the age and when it is late in words", () => {
    expect(levelText({ level: 2, escalated_to: "Sponsor" })).toBe(
      "Level 2: Sponsor",
    );
    expect(levelText({ level: 1, escalated_to: null })).toBe("Level 1");
    expect(ageText({ status: "open", age_days: 12, overdue: false })).toBe(
      "Open for 12 days",
    );
    expect(ageText({ status: "open", age_days: 12, overdue: true })).toBe(
      "Open for 12 days — overdue",
    );
    expect(ageText({ status: "resolved", age_days: 3, overdue: false })).toBe(
      "Took 3 days",
    );
  });
});

describe("escalation drafts", () => {
  it("passes a good draft and names each problem in a bad one", () => {
    expect(raiseProblems(draft)).toEqual([]);
    expect(raiseProblems({ ...draft, title: " " })).toHaveLength(1);
    expect(raiseProblems({ ...draft, tolerance_dimension: "" })).toHaveLength(
      1,
    );
    expect(
      raiseProblems({ ...draft, reason_kind: "risk", tolerance_dimension: "" }),
    ).toEqual([]);
    for (const level of ["0", "10", "1.5", "x"])
      expect(raiseProblems({ ...draft, level })).toHaveLength(1);
    for (const level of ["", "1", "9"])
      expect(raiseProblems({ ...draft, level })).toEqual([]);
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

describe("the acknowledgement and the resolution as records (NC-1 slice 12 part d, CF-4)", () => {
  it("reads only the records a state can have", () => {
    expect(recordsToRead("open")).toEqual({
      acknowledgement: false,
      resolution: false,
    });
    expect(recordsToRead("acknowledged")).toEqual({
      acknowledgement: true,
      resolution: false,
    });
    expect(recordsToRead("resolved")).toEqual({
      acknowledgement: true,
      resolution: true,
    });
    expect(recordsToRead("withdrawn")).toEqual({
      acknowledgement: true,
      resolution: false,
    });
  });

  it("sends a note only when one is written, trimmed", () => {
    expect(acknowledgeBody("")).toEqual({});
    expect(acknowledgeBody("   ")).toEqual({});
    expect(acknowledgeBody("  Seen it ")).toEqual({ note: "Seen it" });
  });

  it("checks a resolution's outcome and its response of 10 to 10,000 characters", () => {
    expect(resolutionProblems("decided", "Funded from contingency")).toEqual(
      [],
    );
    expect(resolutionProblems("maybe", "Funded from contingency")).toEqual([
      "Choose an outcome: decided, guidance given or rejected.",
    ]);
    expect(resolutionProblems("decided", "  too short ")).toEqual([
      "Write a response of 10 to 10,000 characters.",
    ]);
    expect(resolutionProblems("rejected", "x".repeat(RESPONSE_MAX))).toEqual(
      [],
    );
    expect(
      resolutionProblems("rejected", "x".repeat(RESPONSE_MAX + 1)),
    ).toHaveLength(1);
    // Characters, not UTF-16 units: ten emoji are ten characters.
    expect(resolutionProblems("decided", "🙂".repeat(10))).toEqual([]);
    expect(resolutionProblems("", "")).toHaveLength(2);
    expect(resolveBody("decided", "  Funded from contingency ")).toEqual({
      outcome: "decided",
      response: "Funded from contingency",
    });
  });

  it("gives each refusal its sentence by action and status", () => {
    const err = (status: number, code: string) =>
      new ApiError(status, "service text", { code, detail: "service text" });
    expect(recordRefusalKey("acknowledge", err(403, "forbidden"))).toBe(
      "p.escalations.refused_ack_right",
    );
    expect(recordRefusalKey("resolve", err(403, "forbidden"))).toBe(
      "p.escalations.refused_resolve_right",
    );
    expect(recordRefusalKey("acknowledge", err(409, "conflict"))).toBe(
      "p.escalations.refused_ack_state",
    );
    expect(recordRefusalKey("resolve", err(409, "conflict"))).toBe(
      "p.escalations.refused_resolve_state",
    );
    expect(recordRefusalKey("resolve", err(422, "unprocessable"))).toBe(
      "p.escalations.refused_resolve_invalid",
    );
    expect(recordRefusalKey("acknowledge", err(422, "unprocessable"))).toBe(
      null,
    );
    expect(recordRefusalKey("acknowledge", err(404, "not_found"))).toBe(
      "p.escalations.refused_gone",
    );
    expect(recordRefusalKey("resolve", err(500, "internal"))).toBe(null);
    expect(recordRefusalKey("resolve", new Error("offline"))).toBe(null);
  });

  it("reads a 404 on a record as none yet, and nothing else", () => {
    expect(isNoRecord(new ApiError(404, "x"))).toBe(true);
    expect(isNoRecord(new ApiError(403, "x"))).toBe(false);
    expect(isNoRecord(new Error("offline"))).toBe(false);
  });

  it("says who acknowledged and resolved, as references, or that it is not recorded", () => {
    const ack = {
      escalation_pid: "e1",
      plan_pid: "p1",
      level: 2,
      acknowledged_by: "person:ana",
      acknowledged_at: "2026-11-18",
      note: null,
    };
    expect(acknowledgementText(ack)).toBe(
      "Acknowledged at level 2 by person:ana, 18 Nov 2026.",
    );
    expect(acknowledgementText({ ...ack, acknowledged_by: null })).toBe(
      "Acknowledged at level 2 by someone not recorded, 18 Nov 2026.",
    );
    const res = {
      escalation_pid: "e1",
      plan_pid: "p1",
      outcome: "guidance_given",
      response: "Use the contingency",
      resolved_on: "2026-11-18",
      resolved_by: "person:ben",
      resolved_at: null,
    };
    expect(resolutionText(res)).toBe(
      "Resolved 18 Nov 2026 by person:ben: Guidance given.",
    );
    expect(resolutionText({ ...res, outcome: null, resolved_by: null })).toBe(
      "Resolved 18 Nov 2026 by someone not recorded: no outcome recorded.",
    );
  });
});
