// NC-1 slice 12 part (b) (NC-4, BR-2): status updates on a cadence with their evidence. The service's rules said
// first, what each action sends (nothing for the same cadence), where a plan stands read from the service's fields,
// each source as the page shows it, and each refusal's sentence.
import { describe, expect, it } from "vitest";
import { ApiError } from "../../src/lib/api/client";
import {
  CLAIMS_MAX,
  NARRATIVE_MAX,
  cadenceBody,
  isMetricKey,
  sourceView,
  standingOf,
  statusRefusalKey,
  updateBody,
} from "../../src/lib/status-view";

describe("the cadence", () => {
  it("is a whole number of days from 1 to 92, read as a number from a number input", () => {
    expect(cadenceBody(null, 7)).toEqual({ body: { cadence_days: 7 } });
    expect(cadenceBody(null, " 92 ")).toEqual({ body: { cadence_days: 92 } });
    expect(cadenceBody(null, 1)).toEqual({ body: { cadence_days: 1 } });
    for (const bad of [0, 93, 1.5, "", "seven", -3])
      expect(cadenceBody(null, bad)).toEqual({
        problem: "p.status.problem_cadence",
      });
  });
  it("is not sent again when it is the cadence already set (that would restart the clock)", () => {
    expect(cadenceBody(14, 14)).toBeNull();
    expect(cadenceBody(14, "14")).toBeNull();
    expect(cadenceBody(14, 7)).toEqual({ body: { cadence_days: 7 } });
  });
});

describe("an update", () => {
  const form = (over: Partial<Parameters<typeof updateBody>[0]> = {}) => ({
    status: "at_risk" as const,
    narrative: "  Supplier late; recovery plan agreed  ",
    claims: [],
    ...over,
  });
  it("sends the call and the trimmed narrative, and no claims when there are none", () => {
    expect(updateBody(form())).toEqual({
      body: {
        status: "at_risk",
        narrative: "Supplier late; recovery plan agreed",
      },
    });
  });
  it("needs a call and a narrative of up to 4000 characters, each said at once", () => {
    expect(updateBody(form({ status: "", narrative: "  " }))).toEqual({
      problems: [
        { key: "p.status.problem_call", claim: 0 },
        { key: "p.status.problem_narrative", claim: 0 },
      ],
    });
    expect(
      updateBody(form({ narrative: "x".repeat(NARRATIVE_MAX + 1) })),
    ).toEqual({ problems: [{ key: "p.status.problem_narrative", claim: 0 }] });
  });
  it("sends claims trimmed with their sources once each, and leaves out an empty one", () => {
    expect(
      updateBody(
        form({
          claims: [
            {
              text: " Two tasks are blocked ",
              sources: ["metric:blocked", "task:t1", "metric:blocked"],
            },
            { text: "  ", sources: [] },
          ],
        }),
      ),
    ).toEqual({
      body: {
        status: "at_risk",
        narrative: "Supplier late; recovery plan agreed",
        claims: [
          {
            text: "Two tasks are blocked",
            sources: ["metric:blocked", "task:t1"],
          },
        ],
      },
    });
  });
  it("names the claim a problem is on: a claim with sources needs its sentence, at most 500 characters and 5 sources", () => {
    expect(
      updateBody(
        form({
          claims: [
            { text: "Fine", sources: [] },
            { text: "", sources: ["metric:overdue"] },
            {
              text: "x".repeat(501),
              sources: ["a:1", "b:2", "c:3", "d:4", "e:5", "f:6"],
            },
          ],
        }),
      ),
    ).toEqual({
      problems: [
        { key: "p.status.problem_claim_text", claim: 2 },
        { key: "p.status.problem_claim_text", claim: 3 },
        { key: "p.status.problem_sources", claim: 3 },
      ],
    });
  });
  it("carries at most 10 claims", () => {
    const claims = Array.from({ length: CLAIMS_MAX + 1 }, (_, i) => ({
      text: `Claim ${i}`,
      sources: [],
    }));
    expect(updateBody(form({ claims }))).toEqual({
      problems: [{ key: "p.status.problem_claims", claim: 0 }],
    });
  });
});

describe("where a plan stands", () => {
  it("is read from the service's fields: overdue by days, due today, or the next date", () => {
    expect(
      standingOf({
        due_on: "2026-11-10",
        due: true,
        overdue: true,
        days_overdue: 8,
      }),
    ).toEqual({ is: "overdue", date: "2026-11-10", days: 8 });
    expect(
      standingOf({
        due_on: "2026-11-18",
        due: true,
        overdue: false,
        days_overdue: 0,
      }),
    ).toEqual({ is: "due", date: "2026-11-18", days: 0 });
    expect(
      standingOf({
        due_on: "2026-11-25",
        due: false,
        overdue: false,
        days_overdue: 0,
      }),
    ).toEqual({ is: "not_due", date: "2026-11-25", days: 0 });
  });
});

describe("a source", () => {
  it("is a record's kind and its own title", () => {
    expect(
      sourceView({
        source: "task:t1",
        state: "ok",
        kind: "task",
        pid: "t1",
        title: "Order the servers",
      }),
    ).toEqual({ as: "record", kind: "task", title: "Order the servers" });
    expect(
      sourceView({
        source: "change_request:c1",
        state: "ok",
        kind: "change_request",
        pid: "c1",
        title: "Move go-live",
      }),
    ).toEqual({ as: "record", kind: "other", title: "Move go-live" });
  });
  it("is a number's value, unknown as null and never zero, without the service's definition", () => {
    expect(
      sourceView({
        source: "metric:overdue",
        state: "ok",
        kind: "metric",
        key: "overdue",
        value: 3,
        known: true,
        definition: "service text",
      }),
    ).toEqual({ as: "metric", key: "overdue", value: 3 });
    expect(
      sourceView({
        source: "metric:due_soon",
        state: "ok",
        kind: "metric",
        key: "due_soon",
        value: null,
        known: false,
      }),
    ).toEqual({ as: "metric", key: "due_soon", value: null });
  });
  it("is unavailable when gone, or a number this page has no words for", () => {
    expect(sourceView({ source: "task:t9", state: "unavailable" })).toEqual({
      as: "unavailable",
    });
    expect(
      sourceView({
        source: "metric:score",
        state: "ok",
        kind: "metric",
        key: "score",
        value: 1,
        known: true,
      }),
    ).toEqual({ as: "unavailable" });
    expect(isMetricKey("no_due_date")).toBe(true);
    expect(isMetricKey("score")).toBe(false);
  });
});

describe("a refusal", () => {
  it("is the catalogue's sentence for each action", () => {
    expect(statusRefusalKey("cadence", new ApiError(422, "x"))).toBe(
      "p.status.problem_cadence",
    );
    expect(statusRefusalKey("update", new ApiError(422, "x"))).toBe(
      "p.status.refused_update",
    );
    expect(statusRefusalKey("update", new ApiError(404, "x"))).toBe(
      "p.status.refused_gone",
    );
  });
  it("falls back to the generic sentence otherwise", () => {
    expect(statusRefusalKey("update", new ApiError(403, "x"))).toBeNull();
    expect(statusRefusalKey("cadence", new ApiError(500, "x"))).toBeNull();
    expect(statusRefusalKey("cadence", new Error("x"))).toBeNull();
  });
});
