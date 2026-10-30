// FB-4: a programme's people impact, as passed through from the workforce service.

import { describe, expect, it } from "vitest";
import { peopleRows } from "../../src/lib/people-impact";

describe("peopleRows (FB-4)", () => {
  it("keeps WPM's hidden counts hidden and its zeros as zeros, with no total", () => {
    const v = peopleRows(
      {
        known: true,
        reason: null,
        pools: [
          { pool_pid: "a", pool: "Nursing", count: 12, hidden: false },
          { pool_pid: "b", pool: null, count: null, hidden: true },
          { pool_pid: "c", pool: "Estates", count: 0, hidden: false },
        ],
        in_no_pool: { count: null, hidden: true },
        end_not_recorded: { count: 3, hidden: false },
      },
      "Not known.",
    );
    expect(v).toEqual({
      known: true,
      rows: [
        { name: "Nursing", kind: "pool", count: 12, hidden: false },
        { name: "b", kind: "pool", count: null, hidden: true },
        { name: "Estates", kind: "pool", count: 0, hidden: false },
        { name: null, kind: "in_no_pool", count: null, hidden: true },
        { name: null, kind: "end_not_recorded", count: 3, hidden: false },
      ],
    });
  });

  it("never shows a figure WPM marks hidden, and a missing count is hidden, not zero", () => {
    const v = peopleRows(
      {
        known: true,
        reason: null,
        pools: [{ pool_pid: "a", pool: "A", count: 7, hidden: true }],
        in_no_pool: { count: null, hidden: false },
      },
      "x",
    );
    expect(v.known && v.rows.map((r) => [r.count, r.hidden])).toEqual([
      [null, true],
      [null, true],
    ]);
  });

  it("an unknown part is WPM's reason, with no rows", () => {
    expect(
      peopleRows(
        {
          known: false,
          reason: "WPM holds no end date",
          pools: null,
          in_no_pool: null,
        },
        "Not known.",
      ),
    ).toEqual({ known: false, reason: "WPM holds no end date" });
    expect(
      peopleRows(
        { known: false, reason: null, pools: null, in_no_pool: null },
        "Not known.",
      ),
    ).toEqual({ known: false, reason: "Not known." });
  });
});
