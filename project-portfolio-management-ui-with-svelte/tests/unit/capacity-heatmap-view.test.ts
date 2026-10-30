// FB-6: the pool link check and the capacity heatmap's words, as the service sends them.

import { afterEach, describe, expect, it } from "vitest";
import { i18n } from "../../src/lib/i18n.svelte";
import {
  ageLabel,
  cellView,
  fte,
  heatmapRows,
  isPeerPoolRef,
  linkRefusal,
  monthLabel,
  type CapacityHeatmap,
  type HeatmapCell,
} from "../../src/lib/capacity-heatmap-view";

const base: HeatmapCell = {
  month: "2026-11",
  state: "enough",
  word: "enough",
  shape: "circle",
  colour: "green",
  demand_fte: 2,
  own_demand_fte: 2,
  partner_demand_fte: 0,
  supply_fte: 3.5,
  own_supply_fte: 3.5,
  partner_supply_committed_fte: null,
  partner_supply_delivered_fte: null,
  gap_fte: 1.5,
  partner_delivery_short: false,
  partners: 0,
  supply_source: "here",
};

const linked = (
  over: Partial<HeatmapCell>,
  peer: NonNullable<HeatmapCell["peer_supply"]>,
): HeatmapCell => ({
  ...base,
  supply_source: "wpm",
  entered_here_supply_fte: 4,
  peer_supply: peer,
  ...over,
});

afterEach(() => i18n.set("en-001"));

describe("isPeerPoolRef (FB-1)", () => {
  it("takes a lower-case hyphenated UUID, trimmed, as the service does", () => {
    expect(isPeerPoolRef("3f2a8c1e-0b4d-4e6f-9a7b-1c2d3e4f5a6b")).toBe(true);
    expect(isPeerPoolRef("  3f2a8c1e-0b4d-4e6f-9a7b-1c2d3e4f5a6b ")).toBe(true);
  });
  it("refuses upper case, no hyphens, braces, a name and empty text", () => {
    for (const bad of [
      "3F2A8C1E-0B4D-4E6F-9A7B-1C2D3E4F5A6B",
      "3f2a8c1e0b4d4e6f9a7b1c2d3e4f5a6b",
      "{3f2a8c1e-0b4d-4e6f-9a7b-1c2d3e4f5a6b}",
      "Nursing",
      "",
      "3f2a8c1e-0b4d-4e6f-9a7b-1c2d3e4f5a6",
    ])
      expect(isPeerPoolRef(bad), bad).toBe(false);
  });
  it("tells a pool linked elsewhere (409) from an id refused (422)", () => {
    expect(linkRefusal(409)).toBe("taken");
    expect(linkRefusal(422)).toBe("invalid");
    expect(linkRefusal(404)).toBe("missing");
    expect(linkRefusal(500)).toBe("other");
  });
});

describe("cellView (FB-2, FB-6)", () => {
  it("an unlinked cell says its supply was entered here, with the service's word and shape", () => {
    const v = cellView(base, "en-GB");
    expect(v).toMatchObject({
      symbol: "●",
      tone: "green",
      word: "enough",
      demand: "2",
      supply: "3.5",
      gap: "1.5",
      source: "Supply entered here",
      linked: false,
      peerText: null,
      enteredHere: null,
    });
  });

  it("a live linked cell names the workforce service and shows what was entered here beside it, not added", () => {
    const v = cellView(
      linked(
        { supply_fte: 3.5 },
        {
          source: "wpm",
          status: "live",
          supply_fte: 3.5,
          as_of: "2026-10-30T09:00:00Z",
          age_seconds: 10,
          reason: null,
        },
      ),
      "en-GB",
    );
    expect(v.source).toBe("Supply from the workforce service");
    expect(v.supply).toBe("3.5");
    expect(v.peerStatus).toBe("live");
    expect(v.peerText).toMatch(/^Live, taken 30 Oct 2026/);
    expect(v.enteredHere).toBe("Entered here: 4 (shown beside, not added)");
  });

  it("a last-received figure carries its age, time and why it is not live", () => {
    const v = cellView(
      linked(
        {},
        {
          source: "wpm",
          status: "last_received",
          supply_fte: 3.5,
          as_of: "2026-10-30T09:00:00Z",
          age_seconds: 7200,
          reason: "WPM could not be reached",
        },
      ),
      "en-GB",
    );
    expect(v.peerText).toMatch(
      /^Last received 2 hours ago \(taken 30 Oct 2026.*\); not live now: WPM could not be reached$/,
    );
  });

  it("hidden and unknown months are unknown, never zero, and the entered-here figure is not used", () => {
    for (const status of ["hidden", "unknown"] as const) {
      const v = cellView(
        linked(
          {
            state: "supply_unknown",
            word: "supply unknown",
            shape: "question mark",
            colour: "grey",
            supply_fte: null,
            gap_fte: null,
          },
          {
            source: "wpm",
            status,
            supply_fte: null,
            as_of: null,
            age_seconds: null,
            reason: status === "unknown" ? "no token" : null,
          },
        ),
        "en-GB",
      );
      expect(v.supply).toBe("unknown");
      expect(v.supplyKnown).toBe(false);
      expect(v.gap).toBeNull();
      expect(v.symbol).toBe("?");
      expect(v.word).toBe("supply unknown");
      expect(v.enteredHere).toContain("4");
      expect(v.peerText).toBe(
        status === "hidden"
          ? "Hidden: the workforce service hides a month made up of fewer workers than its floor."
          : "Unknown: no token",
      );
    }
  });

  it("an unlinked pool with no supply says none recorded, not zero; an unknown colour falls back to grey", () => {
    const v = cellView(
      {
        ...base,
        state: "no_supply_recorded",
        shape: "dash",
        colour: "purple",
        supply_fte: null,
        gap_fte: null,
      },
      "en-GB",
    );
    expect(v.supply).toBe("none recorded");
    expect(v.symbol).toBe("–");
    expect(v.tone).toBe("grey");
  });
});

describe("formatters and rows", () => {
  it("formats figures, months and ages in the reader's locale", () => {
    expect(fte(1234.5, "en-GB")).toBe("1,234.5");
    expect(fte(1234.5, "de-DE")).toBe("1.234,5");
    expect(monthLabel("2026-11", "en-GB")).toBe("Nov 2026");
    expect(monthLabel("not a month", "en-GB")).toBe("not a month");
    expect(ageLabel(30, "en")).toBe("30 seconds ago");
    expect(ageLabel(300, "en")).toBe("5 minutes ago");
    expect(ageLabel(3 * 86400, "en")).toBe("3 days ago");
  });

  it("the long table names the no-pool row in the reader's language and keeps the service's order", () => {
    const h: CapacityHeatmap = {
      months: ["2026-11"],
      partner_names_shown: false,
      note: "",
      pools: [
        { pool_pid: "p1", name: "Nursing", peer_pool_ref: null, cells: [base] },
        {
          pool_pid: null,
          name: "no pool assigned",
          peer_pool_ref: null,
          cells: [base],
        },
      ],
    };
    i18n.set("cy-001");
    const rows = heatmapRows(h, "cy");
    expect(rows.map((r) => r.pool)).toEqual([
      "Nursing",
      "Dim cronfa wedi'i neilltuo",
    ]);
    expect(rows[0]?.source).toBe("Cyflenwad a gofnodwyd yma");
    expect(rows[0]?.key).toBe("p1:2026-11");
  });
});
