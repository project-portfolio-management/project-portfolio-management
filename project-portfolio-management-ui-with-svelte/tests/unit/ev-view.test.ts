import { describe, expect, it } from "vitest";
import {
  absentText,
  costSentence,
  earnedScheduleSentence,
  evBars,
  health,
  marker,
  methodSentence,
  ON_TRACK_BAND_BP,
  ratioOrReason,
  ratioText,
  scheduleSentence,
} from "../../src/lib/ev-view";
import type { EvBlock } from "../../src/lib/api/ppm";

const money = (n: number) => `${(n / 100).toFixed(2)}`;
const block = (over: Partial<EvBlock> = {}): EvBlock => ({
  currency: "GBP",
  baseline_version: 1,
  bac_minor: 400_000,
  pv_minor: 250_000,
  ev_minor: 200_000,
  ac_minor: 250_000,
  sv_minor: -50_000,
  cv_minor: -50_000,
  spi: { bp: 8000 },
  cpi: { bp: 8000 },
  earned_schedule_days: 10,
  actual_days: 15,
  spi_t: { bp: 6667 },
  ...over,
});

describe("ratio text", () => {
  it("formats basis points to two decimals with integer arithmetic, half-up", () => {
    expect(ratioText(10_000)).toBe("1.00");
    expect(ratioText(8000)).toBe("0.80");
    expect(ratioText(6667)).toBe("0.67");
    expect(ratioText(12_345)).toBe("1.23");
    expect(ratioText(12_350)).toBe("1.24");
    expect(ratioText(0)).toBe("0.00");
    expect(ratioText(-500)).toBe("-0.05");
    expect(ratioText(null)).toBe("—");
  });
  it("says why a ratio is missing instead of showing zero", () => {
    expect(ratioOrReason({ bp: null, absent: "no_actuals_recorded" })).toBe("no cost has been recorded yet");
    expect(absentText({ bp: null, absent: "no_planned_value_yet" })).toMatch(/nothing was planned/);
    expect(absentText({ bp: null, absent: "no_planned_value" })).toMatch(/plans no value/);
    expect(ratioOrReason({ bp: 9000 })).toBe("0.90");
  });
});

describe("health", () => {
  it("has a stated 5% on-track band and a marker for every state", () => {
    expect(ON_TRACK_BAND_BP).toBe(500);
    expect(health(10_000)).toBe("on_track");
    expect(health(10_500)).toBe("on_track");
    expect(health(10_501)).toBe("ahead");
    expect(health(9_500)).toBe("on_track");
    expect(health(9_499)).toBe("behind");
    expect(health(null)).toBe("unknown");
    const marks = (["ahead", "on_track", "behind", "unknown"] as const).map(marker);
    expect(new Set(marks).size).toBe(4);
  });
});

describe("sentences", () => {
  it("states schedule and cost in words with the amounts", () => {
    expect(scheduleSentence(block(), money)).toBe("Behind schedule: SPI 0.80, 500.00 less value earned than planned by now.");
    expect(costSentence(block(), money)).toBe("Over budget: CPI 0.80, 500.00 more spent than the work earned.");
    const good = block({ spi: { bp: 20_000 }, sv_minor: 100_000, cpi: { bp: 12_000 }, cv_minor: 40_000 });
    expect(scheduleSentence(good, money)).toMatch(/^Ahead of schedule: SPI 2.00, 1000.00 more value earned/);
    expect(costSentence(good, money)).toMatch(/^Under budget: CPI 1.20, 400.00 less spent/);
    expect(costSentence(block({ cpi: { bp: 10_000 }, cv_minor: 0 }), money)).toMatch(/^On budget/);
  });
  it("says so when a ratio cannot be stated", () => {
    expect(scheduleSentence(block({ spi: { bp: null, absent: "no_planned_value_yet" } }), money)).toBe("Schedule: no ratio, nothing was planned to be done yet.");
    expect(costSentence(block({ ac_minor: null, cv_minor: null, cpi: { bp: null, absent: "no_actuals_recorded" } }), money)).toBe("Cost: no ratio, no cost has been recorded yet.");
  });
  it("earned schedule says a finished-late project was late, which SPI cannot", () => {
    const late = block({ spi: { bp: 10_000 }, earned_schedule_days: 20, actual_days: 30, spi_t: { bp: 6667 } });
    expect(scheduleSentence(late, money)).toMatch(/On schedule: SPI 1.00/);
    expect(earnedScheduleSentence(late)).toBe("By earned schedule the work is behind: it has earned in 30 days what the plan expected after 20 days (SPI(t) 0.67).");
    expect(earnedScheduleSentence(block({ earned_schedule_days: 1, actual_days: 1, spi_t: { bp: 10_000 } }))).toContain("1 day what the plan expected after 1 day");
    expect(earnedScheduleSentence(block({ spi_t: { bp: null, absent: "no_planned_value_yet" } }))).toMatch(/no ratio/);
  });
});

describe("evBars", () => {
  it("draws planned, earned and actual against the budget line", () => {
    const bars = evBars(block());
    expect(bars.map((b) => b.key)).toEqual(["pv", "ev", "ac"]);
    expect(bars[0]!.widthPct).toBeCloseTo(62.5);
    expect(bars[1]!.widthPct).toBeCloseTo(50);
    expect(bars[0]!.bacPct).toBe(100);
  });
  it("widens the chart when cost overran the budget, moving the budget line left", () => {
    const bars = evBars(block({ ac_minor: 800_000 }));
    expect(bars[2]!.widthPct).toBe(100);
    expect(bars[0]!.bacPct).toBeCloseTo(50);
  });
  it("draws an unknown cost as nothing and says null, never zero", () => {
    const ac = evBars(block({ ac_minor: null }))[2]!;
    expect(ac.minor).toBeNull();
    expect(ac.widthPct).toBe(0);
  });
  it("survives a zero budget", () => {
    expect(evBars(block({ bac_minor: 0, pv_minor: 0, ev_minor: 0, ac_minor: 0 })).every((b) => b.widthPct === 0)).toBe(true);
  });
});

describe("methodSentence", () => {
  it("states how percent complete was counted, including manual and weightless tasks", () => {
    expect(methodSentence({ default_method: "zero_hundred", weighting: "equal", tasks_counted: 4, manual_tasks: [], unweighted_tasks: [] })).toBe("4 tasks counted, weighted equally; only finished work counts");
    expect(methodSentence({ default_method: "points", weighting: "points", tasks_counted: 1, manual_tasks: ["a"], unweighted_tasks: ["b", "c"] })).toBe(
      "1 task counted, weighted by story points; finished work counts for its story points; 1 counted by a person-entered percent (manual); 2 with no points carry no weight",
    );
    expect(methodSentence({ default_method: "fifty_fifty", weighting: "equal", tasks_counted: 2, manual_tasks: [], unweighted_tasks: [] })).toContain("started work counts half");
  });
});
