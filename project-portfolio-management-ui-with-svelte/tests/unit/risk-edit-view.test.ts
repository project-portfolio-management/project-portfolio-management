// NC-1 slice 12 part (c): changing a risk on the governance page. The rules mirror the service's `update_risk`
// (`controllers/governance.rs`) and `risk_exposure` (`domain/src/governance.rs`); the service still decides. Only the
// fields that changed are sent, and nothing for no change.

import { describe, expect, it } from "vitest";
import { ApiError } from "../../src/lib/api/client";
import type { Risk } from "../../src/lib/api/ppm-types";
import {
  MITIGATION_MAX_BYTES,
  formFromRisk,
  riskBody,
  riskProblems,
  riskRefusalKey,
  statusChoices,
} from "../../src/lib/risk-edit-view";

const risk = (over: Partial<Risk> = {}): Risk => ({
  pid: "r1",
  title: "Supplier fails",
  status: "open",
  probability: 3,
  impact: 4,
  exposure: 12,
  owner_ref: null,
  mitigation: "Second supplier",
  review_date: "2026-12-01",
  escalated_at: null,
  ...over,
});

describe("the statuses offered", () => {
  it("leaves materialized to Escalate unless the risk already is, and keeps an unknown one shown", () => {
    expect(statusChoices("open")).toEqual(["open", "mitigating", "closed"]);
    expect(statusChoices("materialized")).toEqual([
      "open",
      "mitigating",
      "closed",
      "materialized",
    ]);
    expect(statusChoices("watching")).toEqual([
      "watching",
      "open",
      "mitigating",
      "closed",
    ]);
  });
});

describe("riskBody", () => {
  it("sends nothing when nothing changed", () => {
    expect(riskBody(formFromRisk(risk()), risk())).toBeNull();
    // A number input binds a number once typed in: the same number is no change.
    expect(
      riskBody(
        { ...formFromRisk(risk()), probability: "3", impact: 4 },
        risk(),
      ),
    ).toBeNull();
    expect(
      riskBody(
        { ...formFromRisk(risk()), mitigation: " Second supplier " },
        risk(),
      ),
    ).toBeNull();
  });

  it("sends only the fields that changed", () => {
    const f = { ...formFromRisk(risk()), status: "mitigating", impact: 2 };
    expect(riskBody(f, risk())).toEqual({
      body: { status: "mitigating", impact: 2 },
    });
    expect(
      riskBody(
        {
          ...formFromRisk(risk()),
          probability: 5,
          mitigation: "  Escrow  ",
          reviewDate: "2027-01-10",
        },
        risk(),
      ),
    ).toEqual({
      body: { probability: 5, mitigation: "Escrow", review_date: "2027-01-10" },
    });
    expect(
      riskBody(
        {
          ...formFromRisk(risk({ review_date: null })),
          reviewDate: "2027-02-01",
        },
        risk({ review_date: null }),
      ),
    ).toEqual({ body: { review_date: "2027-02-01" } });
  });

  it("says every problem before sending", () => {
    const bad = {
      ...formFromRisk(risk()),
      probability: 0,
      mitigation: "é".repeat(MITIGATION_MAX_BYTES / 2 + 1),
      reviewDate: "",
    };
    expect(riskBody(bad, risk())).toEqual({
      problems: [
        "p.risk_edit.problem_score",
        "p.risk_edit.problem_mitigation",
        "p.risk_edit.problem_review_cleared",
      ],
    });
    for (const v of ["", "6", "2.5", -1])
      expect(
        riskProblems({ ...formFromRisk(risk()), impact: v }, risk()),
        String(v),
      ).toEqual(["p.risk_edit.problem_score"]);
    expect(
      riskProblems({ ...formFromRisk(risk()), status: "nonsense" }, risk()),
    ).toEqual(["p.risk_edit.problem_status"]);
    expect(
      riskProblems({ ...formFromRisk(risk()), reviewDate: "1 Dec" }, risk()),
    ).toEqual(["p.risk_edit.problem_review_date"]);
    // Bytes, as the service counts them: 1024 ASCII characters pass, 513 two-byte ones do not.
    expect(
      riskProblems(
        {
          ...formFromRisk(risk()),
          mitigation: "a".repeat(MITIGATION_MAX_BYTES),
        },
        risk(),
      ),
    ).toEqual([]);
    // An empty review date on a risk that has none is no problem.
    expect(
      riskProblems(
        formFromRisk(risk({ review_date: null })),
        risk({ review_date: null }),
      ),
    ).toEqual([]);
  });
});

describe("riskRefusalKey", () => {
  it("says a 404 and a 422 in the catalogue's words, and leaves the rest to the general rule", () => {
    expect(riskRefusalKey(new ApiError(404, "service text"))).toBe(
      "p.risk_edit.refused_gone",
    );
    expect(riskRefusalKey(new ApiError(422, "service text"))).toBe(
      "p.risk_edit.refused_invalid",
    );
    expect(riskRefusalKey(new ApiError(403, "service text"))).toBeNull();
    expect(riskRefusalKey(new Error("offline"))).toBeNull();
  });
});
