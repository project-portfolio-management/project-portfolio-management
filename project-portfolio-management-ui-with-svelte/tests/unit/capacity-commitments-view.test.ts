// FB-6 (completed) and FB-5: the capacity commitment form's checks and the words for what happened to the demand
// sent to the workforce service.

import { afterEach, describe, expect, it } from "vitest";
import { i18n } from "../../src/lib/i18n.svelte";
import {
  readForm,
  recentSends,
  sendOutcome,
  sentFigure,
  type CapacityCommitment,
  type CommitmentForm,
  type PeerDemandSend,
} from "../../src/lib/capacity-commitments-view";

const POOL = "11111111-1111-4111-8111-111111111111";
const PLAN = "22222222-2222-4222-8222-222222222222";

const form = (over: Partial<CommitmentForm> = {}): CommitmentForm => ({
  pool: POOL,
  plan: PLAN,
  organization: "",
  kind: "demand",
  month: "2026-11",
  committed: "1.5",
  delivered: "",
  note: "",
  ...over,
});

const send = (over: Partial<PeerDemandSend>): PeerDemandSend => ({
  outcome: "sent",
  detail: "WPM took the figure",
  fte_centi: 150,
  peer_event_id: "ppm:x",
  ...over,
});

afterEach(() => {
  i18n.set("en-001");
});

describe("readForm", () => {
  it("builds the exact body, leaving out what was not given", () => {
    expect(readForm(form(), "en-001")).toEqual({
      body: {
        pool_pid: POOL,
        plan_pid: PLAN,
        kind: "demand",
        month: "2026-11",
        committed_fte: 1.5,
      },
      problems: [],
    });
    expect(
      readForm(
        form({
          organization: " Acme ",
          delivered: "0.5",
          note: " agreed ",
          kind: "supply",
        }),
        "en-001",
      ).body,
    ).toEqual({
      pool_pid: POOL,
      plan_pid: PLAN,
      kind: "supply",
      month: "2026-11",
      committed_fte: 1.5,
      organization: "Acme",
      delivered_fte: 0.5,
      note: "agreed",
    });
  });

  it("reads figures in the reader's own number format", () => {
    expect(
      readForm(form({ committed: "1,5" }), "de-de").body?.committed_fte,
    ).toBe(1.5);
    expect(
      readForm(form({ committed: "12,5" }), "de-de").body?.committed_fte,
    ).toBe(12.5);
    expect(
      readForm(form({ committed: "1,000.5" }), "en-001").problems,
      "over 1000 in English grouping",
    ).toEqual(["committed"]);
    expect(readForm(form({ committed: "1 234,5" }), "fr-fr").problems).toEqual([
      "committed",
    ]);
    expect(
      readForm(form({ committed: "0" }), "en-001").body?.committed_fte,
    ).toBe(0);
  });

  it("names every wrong field and builds no body", () => {
    const r = readForm(
      {
        pool: "",
        plan: "",
        organization: "",
        kind: "both",
        month: "2026-13",
        committed: "",
        delivered: "x",
        note: "",
      },
      "en-001",
    );
    expect(r.body).toBeNull();
    expect(r.problems).toEqual([
      "pool",
      "plan",
      "kind",
      "month",
      "committed",
      "delivered",
    ]);
    expect(readForm(form({ committed: "1000.5" }), "en-001").problems).toEqual([
      "committed",
    ]);
    expect(readForm(form({ committed: "-1" }), "en-001").problems).toEqual([
      "committed",
    ]);
  });
});

describe("sendOutcome", () => {
  it("says what happened in plain words, a zero as the withdrawal", () => {
    expect(sendOutcome(send({}), "en-001")).toBe(
      "The workforce service took 1.5 full-time equivalent.",
    );
    expect(sendOutcome(send({ fte_centi: 0 }), "en-001")).toBe(
      "The workforce service withdrew its figure for this programme, pool and month.",
    );
    expect(
      sendOutcome(send({ outcome: "started", fte_centi: 200 }), "en-001"),
    ).toContain("Sending 2 full-time equivalent");
    expect(
      sendOutcome(send({ outcome: "started", fte_centi: 0 }), "en-001"),
    ).toContain("Withdrawing");
    expect(sendOutcome(send({ outcome: "duplicate" }), "en-001")).toContain(
      "already had",
    );
    expect(sendOutcome(send({ outcome: "refused" }), "en-001")).toContain(
      "refused",
    );
    expect(sendOutcome(send({ outcome: "unreachable" }), "en-001")).toContain(
      "could not be reached",
    );
    expect(
      sendOutcome(
        send({ outcome: "not_configured", fte_centi: null }),
        "en-001",
      ),
    ).toContain("Not sent");
    expect(
      sendOutcome(
        send({ outcome: "not_sent_no_token", fte_centi: null }),
        "en-001",
      ),
    ).toContain("sign-in");
    expect(
      sendOutcome(
        send({ outcome: "not_sent_too_large", fte_centi: null }),
        "en-001",
      ),
    ).toContain("at most");
    expect(
      sendOutcome(
        send({ outcome: "not_sent_zero", fte_centi: null }),
        "en-001",
      ),
    ).toContain("kept");
    expect(sendOutcome(send({ outcome: "something_new" }), "en-001")).toContain(
      "something_new",
    );
  });

  it("tells a commitment that is never sent from one with nothing recorded yet", () => {
    expect(sendOutcome(undefined, "en-001")).toContain(
      "not sent to the workforce service",
    );
    expect(sendOutcome(null, "en-001")).toContain("Nothing has been sent");
  });

  it("never shows the service's own detail text", () => {
    expect(
      sendOutcome(
        send({ outcome: "refused", detail: "WPM answered 403" }),
        "en-001",
      ),
    ).not.toContain("403");
  });

  it("reads in Welsh", () => {
    i18n.set("cy-001");
    expect(sendOutcome(send({ fte_centi: 0 }), "cy-001")).not.toContain(
      "withdrew",
    );
    expect(sendOutcome(send({ fte_centi: 0 }), "cy-001")).toContain(
      "gwasanaeth gweithlu",
    );
  });
});

describe("sentFigure and recentSends", () => {
  it("writes the figure, the withdrawal or nothing", () => {
    expect(sentFigure(send({ fte_centi: 75 }), "en-001")).toBe(
      "0.75 full-time equivalent",
    );
    expect(sentFigure(send({ fte_centi: 75 }), "de-de")).toBe(
      "0,75 full-time equivalent",
    );
    expect(sentFigure(send({ fte_centi: 0 }), "en-001")).toBe(
      "a withdrawal (zero)",
    );
    expect(sentFigure(send({ fte_centi: null }), "en-001")).toBe("nothing");
  });

  it("lists one send per pool, programme and month, newest first, demand only", () => {
    const c = (over: Partial<CapacityCommitment>): CapacityCommitment => ({
      pid: crypto.randomUUID(),
      pool_pid: POOL,
      plan_pid: PLAN,
      organization: null,
      kind: "demand",
      month: "2026-11",
      committed_fte: 1,
      delivered_fte: null,
      note: null,
      ...over,
    });
    const older = send({ at: "2026-10-30T09:00:00Z", fte_centi: 100 });
    const newer = send({ at: "2026-10-30T10:00:00Z", fte_centi: 0 });
    const rows = recentSends(
      [
        c({ peer_demand_send: older }),
        c({ organization: "Acme", peer_demand_send: older }),
        c({ month: "2026-12", peer_demand_send: newer }),
        c({ month: "2027-01", peer_demand_send: null }),
        c({ kind: "supply", month: "2027-02" }),
      ],
      () => "Nursing",
      () => "Care",
      20,
      "en-001",
    );
    expect(rows.map((r) => [r.key.split("|")[2], r.sent])).toEqual([
      ["2026-12", "a withdrawal (zero)"],
      ["2026-11", "1 full-time equivalent"],
    ]);
    expect(rows[0]).toMatchObject({ pool: "Nursing", programme: "Care" });
    expect(
      recentSends([c({ peer_demand_send: older })], String, String, 0),
    ).toEqual([]);
  });
});
