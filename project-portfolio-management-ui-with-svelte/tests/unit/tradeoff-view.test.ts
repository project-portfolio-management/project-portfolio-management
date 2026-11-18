// NC-1 slice 11 part (m) (EX-13): the named trade-off on a new ask, the service's rules said first.
import { describe, expect, it } from "vitest";
import { ApiError } from "../../src/lib/api/client";
import {
  approveProblem,
  canNameTradeoff,
  hasNamedTradeoff,
  newAskFields,
  newProposalProblem,
  proposalRefusalKey,
  tradeoffBody,
  tradeoffProblem,
} from "../../src/lib/tradeoff-view";

type Status =
  "draft" | "submitted" | "in_review" | "approved" | "rejected" | "promoted";
const ask = (tradeoff: string | null, status: Status = "in_review") => ({
  external_ask: true,
  tradeoff,
  status,
});

describe("a trade-off", () => {
  it("is named when it has 10 or more characters after trimming", () => {
    expect(hasNamedTradeoff(ask(null))).toBe(false);
    expect(hasNamedTradeoff(ask("  Nine char "))).toBe(false);
    expect(hasNamedTradeoff(ask(" Ten chars! "))).toBe(true);
    // Counted as the service counts characters, not UTF-16 units.
    expect(hasNamedTradeoff(ask("😀".repeat(9)))).toBe(false);
    expect(hasNamedTradeoff(ask("😀".repeat(10)))).toBe(true);
  });
  it("is 10 to 2000 characters", () => {
    expect(tradeoffProblem("short")).toBe("p.proposals.problem_tradeoff");
    expect(tradeoffProblem("x".repeat(2001))).toBe(
      "p.proposals.problem_tradeoff",
    );
    expect(tradeoffProblem("x".repeat(2000))).toBeNull();
  });
  it("can be named only on an external ask not yet decided", () => {
    expect(canNameTradeoff(ask(null, "draft"))).toBe(true);
    expect(canNameTradeoff(ask(null, "submitted"))).toBe(true);
    expect(canNameTradeoff(ask(null, "in_review"))).toBe(true);
    expect(canNameTradeoff(ask(null, "approved"))).toBe(false);
    expect(canNameTradeoff(ask(null, "rejected"))).toBe(false);
    expect(canNameTradeoff({ external_ask: false, status: "draft" })).toBe(
      false,
    );
  });
  it("sends the trimmed text, nothing for the same text, and says why it cannot", () => {
    expect(tradeoffBody(ask(null), " Delays the CRM upgrade ")).toEqual({
      body: { tradeoff: "Delays the CRM upgrade" },
    });
    expect(
      tradeoffBody(ask("Delays the CRM upgrade"), "Delays the CRM upgrade  "),
    ).toBeNull();
    expect(tradeoffBody(ask(null), "short")).toEqual({
      problem: "p.proposals.problem_tradeoff",
    });
    expect(
      tradeoffBody(ask(null, "approved"), "Delays the CRM upgrade"),
    ).toEqual({ problem: "p.proposals.problem_decided" });
    expect(
      tradeoffBody(
        { external_ask: false, tradeoff: null, status: "draft" },
        "Delays the CRM upgrade",
      ),
    ).toEqual({ problem: "p.proposals.problem_not_ask" });
  });
});

describe("a new proposal", () => {
  it("needs a title, and a trade-off named now is 10 to 2000 characters", () => {
    expect(
      newProposalProblem({ title: " ", externalAsk: false, tradeoff: "" }),
    ).toBe("p.proposals.problem_title");
    expect(
      newProposalProblem({ title: "CRM", externalAsk: true, tradeoff: "" }),
    ).toBeNull();
    expect(
      newProposalProblem({
        title: "CRM",
        externalAsk: true,
        tradeoff: "short",
      }),
    ).toBe("p.proposals.problem_tradeoff");
    expect(
      newProposalProblem({
        title: "CRM",
        externalAsk: false,
        tradeoff: "short",
      }),
    ).toBeNull();
  });
  it("sends the ask's fields only for an external ask", () => {
    expect(newAskFields(false, "Delays the CRM upgrade")).toEqual({});
    expect(newAskFields(true, "  ")).toEqual({ external_ask: true });
    expect(newAskFields(true, " Delays the CRM upgrade ")).toEqual({
      external_ask: true,
      tradeoff: "Delays the CRM upgrade",
    });
  });
});

describe("approving", () => {
  it("an external ask needs a named trade-off; an ordinary proposal does not", () => {
    expect(approveProblem(ask(null))).toBe("p.proposals.problem_approve");
    expect(approveProblem(ask("short"))).toBe("p.proposals.problem_approve");
    expect(approveProblem(ask("Delays the CRM upgrade"))).toBeNull();
    expect(approveProblem({ external_ask: false, tradeoff: null })).toBeNull();
  });
  it("is refused in the catalogue's words", () => {
    const e422 = new ApiError(422, "x", { code: "unprocessable" });
    // A 422 with no code this page knows is the generic sentence.
    expect(proposalRefusalKey("approve", e422)).toBeNull();
    const coded = (code: string) =>
      new ApiError(422, "service text", { code, detail: "service text" });
    expect(
      proposalRefusalKey("approve", coded("proposal_status_refused")),
    ).toBe("p.proposals.refused_approve_status");
    expect(proposalRefusalKey("approve", coded("tradeoff_required"))).toBe(
      "p.proposals.refused_tradeoff_required",
    );
    expect(proposalRefusalKey("submit", coded("proposal_status_refused"))).toBe(
      "p.proposals.refused_step",
    );
    expect(proposalRefusalKey("tradeoff", e422)).toBe(
      "p.proposals.refused_tradeoff",
    );
    expect(proposalRefusalKey("create", e422)).toBe(
      "p.proposals.refused_create",
    );
    expect(proposalRefusalKey("submit", e422)).toBeNull();
    expect(proposalRefusalKey("approve", new ApiError(404, "x"))).toBe(
      "p.proposals.refused_gone",
    );
    expect(proposalRefusalKey("approve", new ApiError(403, "x"))).toBeNull();
    expect(proposalRefusalKey("approve", new Error("x"))).toBeNull();
  });
});
