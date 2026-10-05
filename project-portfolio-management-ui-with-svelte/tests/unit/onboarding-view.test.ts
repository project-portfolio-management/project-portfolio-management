import { describe, expect, it } from "vitest";
import { accessBody, caseBody, closeoutText, handoverBody, handoverText, itemState, progressText } from "../../src/lib/onboarding-view";

const err = (x: unknown) => "error" in (x as object);
const ana = "person:00000000-0000-4000-8000-0000000000a1";

describe("forms", () => {
  it("a person case needs a person; a plan case needs none", () => {
    expect(caseBody("person_leave", ana, "2026-10-12")).toEqual({ kind: "person_leave", subject_ref: ana, effective_date: "2026-10-12" });
    expect(caseBody("plan_closeout", "", "2026-10-12")).toEqual({ kind: "plan_closeout", effective_date: "2026-10-12" });
    expect(err(caseBody("person_join", "ana", "2026-10-12"))).toBe(true);
    expect(err(caseBody("plan_intake", "", "soon"))).toBe(true);
  });
  it("access needs a person, words and a clear action", () => {
    expect(accessBody(ana, " repo write ", "revoked")).toEqual({ person_ref: ana, access: "repo write", action: "revoked" });
    expect(err(accessBody("ana", "x", "granted"))).toBe(true);
    expect(err(accessBody(ana, "", "granted"))).toBe(true);
    expect(err(accessBody(ana, "x", "lent"))).toBe(true);
  });
  it("a handover to nobody is a release; to oneself is refused", () => {
    expect(handoverBody(ana, "")).toEqual({ from_person: ana, to_person: null });
    expect(handoverBody(ana, "worker:b")).toEqual({ from_person: ana, to_person: "worker:b" });
    expect(err(handoverBody(ana, ana))).toBe(true);
    expect(err(handoverBody("", "worker:b"))).toBe(true);
    expect(err(handoverBody(ana, "ben"))).toBe(true);
  });
});

describe("words", () => {
  it("says how far a checklist has got", () => {
    expect(progressText({ total: 6, done: 3, skipped: 1, open: 2, overdue: 2, resolved_bp: 6666 })).toBe("3 of 6 done, 1 skipped, 2 overdue");
    expect(progressText({ total: 0, done: 0, skipped: 0, open: 0, overdue: 0, resolved_bp: null })).toBe("No items");
  });
  it("gives every item state a word and a shape", () => {
    expect(itemState({ resolution: "done", overdue: false })).toEqual({ symbol: "●", word: "Done" });
    expect(itemState({ resolution: null, overdue: true }).word).toBe("Overdue");
    expect(itemState({ resolution: null, overdue: false }).word).toBe("Open");
    expect(itemState({ resolution: "skipped", overdue: false }).word).toBe("Skipped");
  });
  it("says whether a plan can close, and why not", () => {
    expect(closeoutText({ ready: true, findings: [] })).toMatch(/ready to close/);
    expect(closeoutText({ ready: false, findings: [{ code: "open_tasks", count: 2, text: "2 task(s) are not finished" }] })).toBe("Not ready to close: 2 task(s) are not finished.");
  });
  it("describes a handover", () => {
    expect(handoverText("reassign", 3, "person:b")).toBe("3 items would move to person:b.");
    expect(handoverText("release", 1, null)).toMatch(/1 item would be released/);
    expect(handoverText("reassign", 0, "x")).toMatch(/holds nothing/);
  });
});
