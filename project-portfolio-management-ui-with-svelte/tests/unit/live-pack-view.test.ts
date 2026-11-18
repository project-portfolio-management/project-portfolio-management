// NC-1 slice 12 part (f) (DP-7, DP-8, DP-9, IR-6): the pack's programme readings read live on `/portfolio-map`. The
// window and actual-cost source asked, checked first; each live answer drawn by the pack's own renderer from the
// answer alone; the issues and change requests with every further figure the summary carries, in the catalogue's
// words (never the service's `statement` or `reason`); and nothing drawn for an answer that has not loaded.
import { describe, expect, it } from "vitest";
import { PAGES_EN } from "../../src/lib/i18n-pages";
import { PAGES_CY } from "../../src/lib/i18n-pages.cy";
import {
  AC_KEY,
  AC_SOURCES,
  WINDOW_DEFAULT,
  acParam,
  issuesUnavailable,
  liveSections,
  windowParam,
  windowProblem,
} from "../../src/lib/live-pack-view";
import type { Deps } from "../../src/lib/pack-view";

function deps(cat: Record<string, string>, locale = "en-GB"): Deps {
  const fill = (s: string, v?: Record<string, string | number>) =>
    s.replace(/\{(\w+)\}/g, (_, k: string) => String(v?.[k] ?? `{${k}}`));
  return {
    locale,
    t: (key, values) => {
      const s = cat[key];
      if (s === undefined) throw new Error(`missing catalogue key ${key}`);
      return fill(s, values);
    },
    label: (group, value) =>
      cat[`l.enum.${group}.${String(value)}`] ?? String(value),
  };
}
const EN = deps(PAGES_EN as unknown as Record<string, string>);
const CY = deps(PAGES_CY as unknown as Record<string, string>, "cy-GB");

const ISSUES = {
  available: true,
  reason: null,
  window_days: 30,
  as_of: "2026-11-18",
  issues: {
    total: 9,
    open: 4,
    open_by_severity: { high: 3, low: 1, med: 0 },
    overdue: 2,
    raised_in_window: 5,
    resolved_in_window: 1,
    oldest_open: {
      plan_pid: "p1",
      title: "Supplier late",
      age_calendar_days: 40,
    },
    statement: "service text",
  },
  change_requests: {
    total: 6,
    by_status: { submitted: 2, approved: 1 },
    awaiting_decision: 2,
    approved_not_implemented: 1,
    decided_in_window: 3,
    oldest_open: {
      plan_pid: "p1",
      title: "Move go-live",
      age_calendar_days: 12,
    },
    statement: "service text",
  },
};

describe("what the live readings ask", () => {
  it("counts issues over 1 to 366 whole days, empty for the service's 30, which is not sent", () => {
    expect(windowProblem(null)).toBeNull();
    expect(windowProblem(1)).toBeNull();
    expect(windowProblem(366)).toBeNull();
    for (const bad of [0, 367, 1.5, -3, Number.NaN])
      expect(windowProblem(bad)).toBe("p.live_pack.problem_window");
    expect(windowParam(null)).toBeUndefined();
    expect(windowParam(WINDOW_DEFAULT)).toBeUndefined();
    expect(windowParam(90)).toBe(90);
  });
  it("offers the service's three actual-cost sources, the default not sent", () => {
    expect(AC_SOURCES).toEqual(["budget_lines", "labour", "both"]);
    for (const a of AC_SOURCES) expect(PAGES_EN[AC_KEY[a]]).toBeTruthy();
    expect(acParam("budget_lines")).toBeUndefined();
    expect(acParam("labour")).toBe("labour");
    expect(acParam("both")).toBe("both");
  });
});

describe("the live readings, drawn by the pack's renderer", () => {
  it("draws nothing for an answer that has not loaded", () => {
    expect(liveSections({}, EN)).toEqual({});
    expect(
      Object.keys(liveSections({ fever: { programmes: [] } }, EN)),
    ).toEqual(["fever"]);
  });

  it("draws the fever chart, earned value and benefits as the pack's sections", () => {
    const s = liveSections(
      {
        fever: {
          programmes: [
            {
              plan: "Wards",
              point: {
                chain_completed_pct: 40,
                buffer_consumed_pct: 75,
                zone: "red",
                edge: null,
                slip_days: 6,
                buffer_days: 8,
              },
              previous: null,
            },
            {
              plan: "Apps",
              point: null,
              no_fever_chart_point: { code: "no_baseline" },
            },
          ],
          note: "service text",
        },
        evm: {
          programmes: [
            {
              name: "Wards",
              state: "evm",
              currencies: [
                {
                  currency: "GBP",
                  state: "included",
                  totals: {
                    bac_minor: 100000,
                    ev_minor: 40000,
                    ac_minor: 50000,
                  },
                  spi: { bp: 9000 },
                  cpi: { bp: 8000 },
                },
              ],
            },
          ],
        },
        benefits: {
          totals: [
            {
              currency: "GBP",
              recurrence: "one_off",
              classification: "cash_releasing",
              planned_minor: 500000,
              claimed_minor: 100000,
              validated_minor: 50000,
              unvalidated_claim_minor: 50000,
            },
          ],
          note: "service text",
        },
      },
      EN,
    );
    expect(Object.keys(s)).toEqual(["fever", "evm", "benefits"]);
    expect(s.fever!.title).toBe(PAGES_EN["p.director_pack.s_fever"]);
    expect(s.fever!.tables[0]!.rows[0]![0]!.text).toBe("Wards");
    expect(s.fever!.tables[0]!.rows[0]![3]).toMatchObject({ tone: "red" });
    expect(s.fever!.lines.join(" ")).toContain("Apps");
    expect(s.evm!.tables[0]!.rows[0]!.map((c) => c.text)).toContain(
      "£1,000.00",
    );
    expect(s.benefits!.tables[0]!.rows[0]!.map((c) => c.text)).toContain(
      "£5,000.00",
    );
    const all = JSON.stringify(s);
    expect(all).not.toContain("service text");
  });

  it("draws issues and change requests with every figure the summary carries, never its statement", () => {
    const s = liveSections({ issues: ISSUES }, EN);
    expect(Object.keys(s)).toEqual(["issues", "changes"]);
    expect(s.issues!.lines).toEqual([
      "Open issues: 4",
      "Open by severity: high 3, low 1",
      "Overdue issues: 2",
      "In the period: 5 raised, 1 resolved",
      "Oldest open issue: Supplier late (calendar days open: 40)",
    ]);
    expect(s.changes!.lines).toEqual([
      "Change requests awaiting a decision: 2",
      "Approved but not yet implemented: 1",
      "Decided in the period: 3",
      "By status: submitted 2, approved 1",
      "Oldest open change request: Move go-live (calendar days open: 12)",
    ]);
    expect(JSON.stringify(s)).not.toContain("service text");
  });

  it("says in Welsh what it says in English", () => {
    const s = liveSections({ issues: ISSUES }, CY);
    expect(s.issues!.title).toBe(PAGES_CY["p.director_pack.s_issues"]);
    expect(s.issues!.lines[1]).toContain(PAGES_CY["l.enum.severity.high"]);
    expect(s.changes!.lines[0]).not.toBe(
      "Change requests awaiting a decision: 2",
    );
  });

  it("draws no issues for an answer that says none is readable, and says so in its own words", () => {
    const none = {
      available: false,
      reason: "service text",
      window_days: 30,
      issues: {},
      change_requests: {},
    };
    expect(issuesUnavailable(none)).toBe(true);
    expect(issuesUnavailable(ISSUES)).toBe(false);
    expect(issuesUnavailable(null)).toBe(false);
    expect(liveSections({ issues: none }, EN)).toEqual({});
  });
});
