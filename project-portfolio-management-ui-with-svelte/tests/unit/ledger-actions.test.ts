// NC-1 slice 11 (EX-12): the funding ledger's writes: recording an allocation (amount in the reader's format, minor
// units sent), the allocations a line may draw on, the checks on a cost code and a finance export, and each refusal.
import { describe, expect, it } from "vitest";
import { ApiError } from "../../src/lib/api/client";
import type { BudgetBoard } from "../../src/lib/api/ppm-types";
import type { FundingAllocation } from "../../src/lib/api/ppm";
import {
  allocationBody,
  allocationChoices,
  costCodeProblem,
  exportProblem,
  ledgerRefusalKey,
} from "../../src/lib/ledger-actions";

const form = {
  source: "s1",
  amount: "1,234.50",
  currency: "gbp",
  allocatedOn: "2026-11-01",
  reference: "  PO-77 ",
};

describe("recording an allocation", () => {
  it("sends minor units of an upper-cased currency, a trimmed reference, nothing blank", () => {
    expect(allocationBody(form, "2026-11-17", "en-GB")).toEqual({
      body: {
        funding_source_pid: "s1",
        amount_minor: 123450,
        currency: "GBP",
        allocated_on: "2026-11-01",
        reference: "PO-77",
      },
    });
    expect(
      allocationBody({ ...form, reference: " " }, "2026-11-17", "en-GB"),
    ).toEqual({
      body: {
        funding_source_pid: "s1",
        amount_minor: 123450,
        currency: "GBP",
        allocated_on: "2026-11-01",
      },
    });
  });
  it("reads the amount in the reader's own format", () => {
    const r = allocationBody(
      { ...form, amount: "1.234,50", currency: "EUR" },
      "2026-11-17",
      "de-DE",
    );
    expect("body" in r && r.body.amount_minor).toBe(123450);
  });
  it("says every problem at once", () => {
    expect(
      allocationBody(
        {
          source: "",
          amount: "0",
          currency: "pounds",
          allocatedOn: "2026-11-18",
          reference: "x".repeat(121),
        },
        "2026-11-17",
        "en-GB",
      ),
    ).toEqual({
      problems: [
        "p.ledger.problem_source",
        "p.ledger.problem_currency",
        "p.ledger.problem_amount",
        "p.ledger.problem_date_future",
        "p.ledger.problem_reference",
      ],
    });
  });
});

describe("a line's allocations", () => {
  const line = (over: Partial<BudgetBoard["lines"][number]>) =>
    ({
      pid: "l1",
      category: "capex",
      description: "Servers",
      currency: "GBP",
      planned_minor: 1,
      actual_minor: 0,
      ...over,
    }) as BudgetBoard["lines"][number];
  const alloc = (pid: string, source: string, currency: string) =>
    ({ pid, funding_source_pid: source, currency }) as FundingAllocation;
  const all = [
    alloc("a1", "s1", "GBP"),
    alloc("a2", "s1", "EUR"),
    alloc("a3", "s2", "GBP"),
  ];
  it("are those of its own source and currency only", () => {
    expect(
      allocationChoices(line({ funding_source_pid: "s1" }), all).map(
        (a) => a.pid,
      ),
    ).toEqual(["a1"]);
  });
  it("are none until the line has a funding source", () => {
    expect(allocationChoices(line({ funding_source_pid: null }), all)).toEqual(
      [],
    );
  });
});

describe("cost codes and exports", () => {
  it("need a code of 1 to 60 characters and a line", () => {
    expect(costCodeProblem("  ", "l1")).toBe("p.ledger.problem_code");
    expect(costCodeProblem("x".repeat(61), "l1")).toBe("p.ledger.problem_code");
    expect(costCodeProblem("CC-100", "")).toBe("p.ledger.problem_code_line");
    expect(costCodeProblem("CC-100", "l1")).toBeNull();
  });
  it("check an export's header and size before sending it", () => {
    expect(exportProblem(" \n ")).toBe("p.ledger.problem_export_empty");
    expect(exportProblem("code,amount\nA,1")).toBe(
      "p.ledger.problem_export_header",
    );
    expect(
      exportProblem('\n"Cost_Code", amount_minor ,currency\nA,100,GBP'),
    ).toBeNull();
    expect(
      exportProblem(
        `cost_code,amount_minor,currency\n${"A,1,GBP\n".repeat(130_000)}`,
      ),
    ).toBe("p.ledger.problem_export_large");
  });
  it("read a 422 on a line or an export in the catalogue's words", () => {
    expect(ledgerRefusalKey("line", new ApiError(422, "x"))).toBe(
      "p.ledger.refused_line",
    );
    expect(ledgerRefusalKey("reconcile", new ApiError(422, "x"))).toBe(
      "p.ledger.refused_export",
    );
    expect(ledgerRefusalKey("allocation", new ApiError(422, "x"))).toBeNull();
    expect(ledgerRefusalKey("line", new ApiError(403, "x"))).toBeNull();
  });
});
