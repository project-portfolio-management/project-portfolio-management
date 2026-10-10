import { describe, expect, it } from "vitest";
import {
  decisionLines,
  escalatedRisks,
  healthSentence,
  milestonesToWatch,
  needsAttention,
  overrunPortfolios,
  ragTotals,
  worstStatus,
} from "../../src/lib/ceo-view";
import { ageText, excerpt, stateLabel } from "../../src/lib/announcement-view";

const port = (
  status: "red" | "amber" | "green",
  rag: [number, number, number],
  extra = {},
) => ({
  portfolio: { pid: "p", name: "Alpha" },
  status,
  rag: { red: rag[0], amber: rag[1], green: rag[2] },
  members: rag[0] + rag[1] + rag[2],
  overdue_milestones: 1,
  escalated_risks: 2,
  open_risk_exposure: 0,
  overrun_currencies: [] as string[],
  days_since_last_update: 0,
  ...extra,
});
const health = (ps: ReturnType<typeof port>[]) =>
  ({ as_of: "x", derivation: "d", portfolios: ps }) as never;

describe("health tile", () => {
  it("adds plans across portfolios and reports the worst status", () => {
    const h = health([port("green", [0, 1, 4]), port("red", [2, 0, 1])]);
    expect(ragTotals(h)).toEqual({ red: 2, amber: 1, green: 5 });
    expect(worstStatus(h)).toBe("red");
    expect(healthSentence(h)).toBe("2 of 8 plans are red, 1 amber, 5 green.");
    expect(escalatedRisks(h)).toBe(4);
  });
  it("says so when there is nothing, rather than reporting all green", () => {
    expect(worstStatus(health([]))).toBeNull();
    expect(healthSentence(health([]))).toMatch(/No portfolios/);
    expect(healthSentence(health([port("green", [0, 0, 0])]))).toMatch(
      /no plans/,
    );
  });
  it("names portfolios over budget", () => {
    const h = health([
      port("amber", [0, 1, 0], { overrun_currencies: ["GBP"] }),
      port("green", [0, 0, 1]),
    ]);
    expect(overrunPortfolios(h)).toEqual(["Alpha"]);
  });
});

const kpi = (name: string, status: string) =>
  ({ kpi: { name }, status }) as never;
describe("needsAttention", () => {
  it("lists red before amber before no data, hides green, and counts the rest", () => {
    const r = needsAttention(
      [
        kpi("g", "green"),
        kpi("b", "amber"),
        kpi("a", "red"),
        kpi("n", "no_data"),
        kpi("c", "red"),
      ],
      3,
    );
    expect(r.shown.map((k) => k.kpi.name)).toEqual(["a", "c", "b"]);
    expect(r.more).toBe(1);
  });
});

const ms = (name: string, due: string, status: string, achieved = false) =>
  ({ name, due, status, achieved }) as never;
describe("milestonesToWatch", () => {
  it("puts late first, then soonest due, and leaves achieved ones out", () => {
    const r = milestonesToWatch(
      [
        ms("c", "2026-12-01", "upcoming"),
        ms("done", "2026-01-01", "achieved", true),
        ms("a", "2026-11-01", "late"),
        ms("b", "2026-10-20", "due_soon"),
      ],
      2,
    );
    expect(r.shown.map((m) => m.name)).toEqual(["a", "b"]);
    expect(r.more).toBe(1);
  });
});

describe("announcement words", () => {
  const now = new Date("2026-10-05T12:00:00Z");
  it("says how long ago, in words", () => {
    expect(ageText("2026-10-05T11:59:40Z", now)).toBe("just now");
    expect(ageText("2026-10-05T11:30:00Z", now)).toBe("30 min ago");
    expect(ageText("2026-10-05T09:00:00Z", now)).toBe("3 h ago");
    expect(ageText("2026-10-04T10:00:00Z", now)).toBe("yesterday");
    expect(ageText("2026-10-01T10:00:00Z", now)).toBe("4 calendar days ago");
    expect(ageText("2026-10-06T10:00:00Z", now)).toBe("scheduled");
  });
  it("cuts an excerpt at a word and only marks a cut", () => {
    expect(excerpt("short text", 40)).toBe("short text");
    expect(excerpt("the quick brown fox jumps over", 15)).toBe("the quick…");
    expect(stateLabel({ state: "live" })).toBe("");
    expect(stateLabel({ state: "expired" })).toBe("Expired");
  });
});

describe("decisionLines", () => {
  it("lists the newest first, words each kind, and counts the rest", () => {
    const d = (
      at: string,
      kind: string,
      decision: string,
      name: string | null,
    ) => ({ at, kind, decision, subject: { pid: "x", name } }) as never;
    const r = decisionLines(
      [
        d("2026-10-01T00:00:00Z", "merge", "done", null),
        d("2026-10-04T00:00:00Z", "gate_review", "Approved", "Apollo"),
        d("2026-10-03T00:00:00Z", "proposal", "Rejected", "Zed"),
      ],
      2,
    );
    expect(r.lines.map((l) => l.text)).toEqual([
      "Gate review: Approved — Apollo",
      "Proposal: Rejected — Zed",
    ]);
    expect(r.lines[0]!.at).toBe("2026-10-04");
    expect(r.more).toBe(1);
  });
});
