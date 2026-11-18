// NC-1 slice 12 part (g), NC-7: choosing a portfolio. The service's rules said first: the frontier's currency, its caps
// spread (2 to 50, ten not sent) or named (one per line in the reader's number format, 1 to 50, each not below zero),
// at most 50 plans to include; a generated draft's name (needed here, at most 1024 bytes), its cap and currency (a cap
// needs its currency); a frontier point saved with the same cap, currency and plans; each reason a plan is left out in
// words; the chart's geometry; and each refusal's sentence, a rollback's 409 as a scenario no longer committed.
import { describe, expect, it } from "vitest";
import { ApiError } from "../../src/lib/api/client";
import { PAGES_EN } from "../../src/lib/i18n-pages";
import {
  FRONTIER_BOX,
  REASON_KEY,
  bestPoint,
  choiceRows,
  frontierLine,
  frontierPlot,
  frontierQuery,
  frontierRefusalKey,
  generateBody,
  pointBody,
  reasonKey,
  rollbackRefusalKey,
} from "../../src/lib/frontier-view";
import type { FrontierPoint } from "../../src/lib/api/ppm-types";

const fform = (over = {}) => ({
  currency: "gbp",
  how: "steps" as "steps" | "caps",
  steps: null as number | null | undefined,
  caps: "",
  mustInclude: [] as string[],
  ...over,
});
const gform = (over = {}) => ({
  name: "Spring",
  cap: "",
  currency: "",
  mustInclude: [] as string[],
  ...over,
});
const point = (over: Partial<FrontierPoint> = {}): FrontierPoint => ({
  budget_cap_minor: 0,
  value: 0,
  cost_minor: 0,
  included: 0,
  dominated: false,
  rationale: [],
  ...over,
});

describe("the frontier's query", () => {
  it("sends the currency upper-cased, and leaves out the ten steps the service assumes", () => {
    expect(frontierQuery(fform(), "en")).toEqual({
      query: { currency: "GBP" },
    });
    expect(frontierQuery(fform({ steps: 10 }), "en")).toEqual({
      query: { currency: "GBP" },
    });
    expect(frontierQuery(fform({ steps: 5 }), "en")).toEqual({
      query: { currency: "GBP", steps: 5 },
    });
  });
  it("reads named caps one per line in the reader's number format, in minor units", () => {
    expect(
      frontierQuery(fform({ how: "caps", caps: "0\n1.500,50\n\n2000" }), "de"),
    ).toEqual({ query: { currency: "GBP", caps: [0, 150050, 200000] } });
  });
  it("sends the plans every point must include", () => {
    expect(frontierQuery(fform({ mustInclude: ["p1", "p2"] }), "en")).toEqual({
      query: { currency: "GBP", mustInclude: ["p1", "p2"] },
    });
  });
  it("names every problem at once", () => {
    expect(frontierQuery(fform({ currency: "pounds", steps: 1 }))).toEqual({
      problems: ["p.frontier.problem_currency", "p.frontier.problem_steps"],
    });
    expect(frontierQuery(fform({ steps: 51 }))).toEqual({
      problems: ["p.frontier.problem_steps"],
    });
    expect(frontierQuery(fform({ how: "caps", caps: "  \n" }))).toEqual({
      problems: ["p.frontier.problem_caps_none"],
    });
    expect(frontierQuery(fform({ how: "caps", caps: "10\n-1" }), "en")).toEqual(
      { problems: ["p.frontier.problem_caps_amount"] },
    );
    expect(
      frontierQuery(
        fform({ how: "caps", caps: Array(51).fill("1").join("\n") }),
      ),
    ).toEqual({ problems: ["p.frontier.problem_caps_many"] });
    expect(frontierQuery(fform({ mustInclude: Array(51).fill("p") }))).toEqual({
      problems: ["p.frontier.problem_must_include"],
    });
  });
});

describe("a generated draft", () => {
  it("needs a name, sent trimmed; a cap needs its currency", () => {
    expect(generateBody(gform({ name: "  " }))).toEqual({
      problems: ["p.frontier.problem_name"],
    });
    expect(generateBody(gform({ name: " Spring " }))).toEqual({
      body: { name: "Spring" },
    });
    expect(generateBody(gform({ cap: "100" }))).toEqual({
      problems: ["p.frontier.problem_cap_currency"],
    });
  });
  it("counts the name in bytes, as the service does", () => {
    expect(generateBody(gform({ name: "é".repeat(513) }))).toEqual({
      problems: ["p.frontier.problem_name_long"],
    });
    expect("body" in generateBody(gform({ name: "é".repeat(512) }))).toBe(true);
  });
  it("sends the cap in minor units with its currency, and a currency alone so each plan is priced", () => {
    expect(
      generateBody(
        gform({ cap: "1,000.50", currency: "eur", mustInclude: ["p1"] }),
        "en",
      ),
    ).toEqual({
      body: {
        name: "Spring",
        budget_cap_minor: 100050,
        currency: "EUR",
        must_include: ["p1"],
      },
    });
    expect(generateBody(gform({ currency: "gbp" }))).toEqual({
      body: { name: "Spring", currency: "GBP" },
    });
    expect(generateBody(gform({ cap: "-5", currency: "GBP" }), "en")).toEqual({
      problems: ["p.frontier.problem_cap_amount"],
    });
    expect(generateBody(gform({ currency: "pound" }))).toEqual({
      problems: ["p.frontier.problem_currency"],
    });
  });
  it("saves a frontier point with the same cap, currency and plans", () => {
    expect(
      pointBody(
        "Frontier at £10.00",
        point({ budget_cap_minor: 1000 }),
        "GBP",
        ["p1"],
      ),
    ).toEqual({
      name: "Frontier at £10.00",
      budget_cap_minor: 1000,
      currency: "GBP",
      must_include: ["p1"],
    });
    expect(pointBody("x", point(), "GBP", [])).toEqual({
      name: "x",
      budget_cap_minor: 0,
      currency: "GBP",
    });
  });
});

describe("a choice", () => {
  it("says every reason a plan is left out, and a general one for a reason it does not know", () => {
    expect(Object.keys(REASON_KEY)).toEqual([
      "no_score",
      "foreign_currency",
      "over_cap",
      "must_include_conflict",
    ]);
    for (const k of Object.values(REASON_KEY)) expect(PAGES_EN[k]).toBeTruthy();
    expect(reasonKey("weather")).toBe("p.frontier.reason_other");
    expect(reasonKey(null)).toBe("p.frontier.reason_other");
  });
  it("lists the chosen first, then by name, a plan with no name last", () => {
    const rows = choiceRows(
      [
        { pid: "x", included: false, reason: "must_include_conflict" },
        { pid: "b", included: false, reason: "over_cap", score: 3 },
        { pid: "a", included: true, score: 5, cost_minor: 10 },
        { pid: "c", included: false, reason: "no_score" },
      ],
      new Map([
        ["a", "Alpha"],
        ["b", "Beta"],
        ["c", "Cedar"],
      ]),
    );
    expect(rows.map((r) => [r.pid, r.name])).toEqual([
      ["a", "Alpha"],
      ["b", "Beta"],
      ["c", "Cedar"],
      ["x", null],
    ]);
  });
});

describe("the chart", () => {
  const pts = [
    point({ budget_cap_minor: 0, value: 0, cost_minor: 0 }),
    point({ budget_cap_minor: 100, value: 10, cost_minor: 80 }),
    point({
      budget_cap_minor: 200,
      value: 10,
      cost_minor: 80,
      dominated: true,
    }),
    point({ budget_cap_minor: 300, value: 20, cost_minor: 160 }),
  ];
  it("places cost across and value up, from zero to the largest", () => {
    const placed = frontierPlot(pts);
    const b = FRONTIER_BOX;
    expect(placed[0]).toMatchObject({ x: b.left, y: b.height - b.bottom });
    expect(placed[3]).toMatchObject({ x: b.width - b.right, y: b.top });
  });
  it("draws the line through the points that buy more value, cheapest first", () => {
    const line = frontierLine(frontierPlot(pts)).split(" ");
    expect(line).toHaveLength(3);
  });
  it("names the point with the most value, the cheapest of equals", () => {
    expect(bestPoint(pts)?.budget_cap_minor).toBe(300);
    expect(bestPoint(pts.slice(0, 3))?.budget_cap_minor).toBe(100);
    expect(bestPoint([])).toBeNull();
  });
});

describe("a refusal", () => {
  it("is the catalogue's sentence, never the service's text", () => {
    const e422 = new ApiError(422, "service text", { code: "unprocessable" });
    expect(frontierRefusalKey("frontier", e422)).toBe(
      "p.frontier.refused_frontier",
    );
    expect(frontierRefusalKey("generate", e422)).toBe(
      "p.frontier.refused_generate",
    );
    expect(
      frontierRefusalKey("frontier", new ApiError(500, "service text")),
    ).toBeNull();
    expect(rollbackRefusalKey(new ApiError(409, "service text"))).toBe(
      "p.frontier.refused_rollback_status",
    );
    expect(rollbackRefusalKey(new ApiError(404, "service text"))).toBe(
      "p.frontier.refused_rollback_gone",
    );
    expect(rollbackRefusalKey(new Error("offline"))).toBeNull();
  });
});
