// NC-1 slice 12 part (g), EX-13: the intake's forecast and editing a draft. The service's rules said first: the
// forecast's three bounds (each default left out), why it has no figures (too little history, in words); a draft's
// title, summary, amount and trade-off, every problem at once; the fields not shown sent back as stored; nothing sent
// for no change; only a draft edited; and each refusal's sentence, the two 422 codes told apart.
import { describe, expect, it } from "vitest";
import { ApiError } from "../../src/lib/api/client";
import type { Proposal } from "../../src/lib/api/ppm-types";
import { minorToInput } from "../../src/lib/money";
import {
  forecastAbsentKey,
  forecastQuery,
  forecastRefusalKey,
  proposalEditBody,
  proposalEditFormOf,
  proposalEditRefusalKey,
} from "../../src/lib/intake-view";

const draft = (over: Partial<Proposal> = {}): Proposal => ({
  pid: "a1",
  title: "Sponsor dashboard",
  summary: "For the board",
  kind_target: "projects",
  sponsor_ref: "person:00000000-0000-0000-0000-000000000001",
  strategic_rationale: "Growth",
  requested_minor: 150000,
  currency: "GBP",
  status: "draft",
  promoted_plan_pid: null,
  external_ask: false,
  tradeoff: null,
  ...over,
});
const toInput = (m: number, c: string) => minorToInput(m, c, "en");

describe("the forecast's query", () => {
  it("leaves out every default, and sends what differs", () => {
    expect(
      forecastQuery({
        periods: null,
        periodDays: undefined,
        historyPeriods: null,
      }),
    ).toEqual({ query: {} });
    expect(
      forecastQuery({ periods: 4, periodDays: 7, historyPeriods: 12 }),
    ).toEqual({ query: {} });
    expect(
      forecastQuery({ periods: 8, periodDays: 14, historyPeriods: 26 }),
    ).toEqual({ query: { periods: 8, periodDays: 14, historyPeriods: 26 } });
  });
  it("names every bound broken at once", () => {
    expect(
      forecastQuery({ periods: 0, periodDays: 91, historyPeriods: 261 }),
    ).toEqual({
      problems: [
        "p.intake.problem_periods",
        "p.intake.problem_period_days",
        "p.intake.problem_history",
      ],
    });
    expect(
      forecastQuery({ periods: 520, periodDays: 1.5, historyPeriods: 1 }),
    ).toEqual({ problems: ["p.intake.problem_period_days"] });
  });
  it("says too little history in words, never the service's reason", () => {
    expect(forecastAbsentKey(5)).toBe("p.intake.absent_history");
    expect(forecastAbsentKey(6)).toBe("p.intake.absent_other");
    expect(forecastRefusalKey(new ApiError(422, "service text"))).toBe(
      "p.intake.refused_forecast",
    );
    expect(forecastRefusalKey(new ApiError(500, "service text"))).toBeNull();
  });
});

describe("editing a draft", () => {
  it("fills the form from the stored proposal, the amount in the reader's format", () => {
    expect(proposalEditFormOf(draft(), toInput)).toEqual({
      title: "Sponsor dashboard",
      summary: "For the board",
      kindTarget: "projects",
      requested: "1500.00",
      currency: "GBP",
      externalAsk: false,
      tradeoff: "",
    });
  });
  it("sends nothing when nothing changed", () => {
    const p = draft();
    expect(
      proposalEditBody(p, proposalEditFormOf(p, toInput), "en"),
    ).toBeNull();
    const bare = draft({
      summary: null,
      sponsor_ref: null,
      strategic_rationale: null,
      requested_minor: null,
      currency: null,
    });
    expect(
      proposalEditBody(bare, proposalEditFormOf(bare, toInput), "en"),
    ).toBeNull();
  });
  it("tags it as an external ask, sending every field and the ones not shown as stored", () => {
    const p = draft();
    const form = {
      ...proposalEditFormOf(p, toInput),
      externalAsk: true,
      tradeoff: " Delays the CRM upgrade ",
    };
    expect(proposalEditBody(p, form, "en")).toEqual({
      body: {
        title: "Sponsor dashboard",
        kind_target: "projects",
        summary: "For the board",
        sponsor_ref: "person:00000000-0000-0000-0000-000000000001",
        strategic_rationale: "Growth",
        requested_minor: 150000,
        currency: "GBP",
        external_ask: true,
        tradeoff: "Delays the CRM upgrade",
      },
    });
  });
  it("clears a blank summary and an amount, and keeps a trade-off when the tag is taken off", () => {
    const p = draft({ external_ask: true, tradeoff: "Pauses the intranet" });
    const form = {
      ...proposalEditFormOf(p, toInput),
      summary: " ",
      requested: "",
      externalAsk: false,
    };
    expect(proposalEditBody(p, form, "en")).toEqual({
      body: {
        title: "Sponsor dashboard",
        kind_target: "projects",
        sponsor_ref: "person:00000000-0000-0000-0000-000000000001",
        strategic_rationale: "Growth",
        external_ask: false,
        tradeoff: "Pauses the intranet",
      },
    });
  });
  it("names every problem at once", () => {
    const p = draft();
    const form = {
      ...proposalEditFormOf(p, toInput),
      title: " ",
      summary: "é".repeat(513),
      requested: "12",
      currency: "pounds",
      externalAsk: true,
      tradeoff: "short",
    };
    expect(proposalEditBody(p, form, "en")).toEqual({
      problems: [
        "p.proposals.problem_title",
        "p.intake.problem_summary_long",
        "p.proposals.problem_tradeoff",
        "p.money.amount_or_currency",
      ],
    });
    expect(
      proposalEditBody(
        p,
        { ...proposalEditFormOf(p, toInput), title: "x".repeat(1025) },
        "en",
      ),
    ).toEqual({ problems: ["p.intake.problem_title_long"] });
  });
  it("edits only a draft", () => {
    const p = draft({ status: "submitted" });
    expect(proposalEditBody(p, proposalEditFormOf(p, toInput), "en")).toEqual({
      problems: ["p.intake.problem_not_draft"],
    });
  });
  it("tells the two 422s apart by their code, and says a 404 as gone", () => {
    const coded = (code: string) =>
      new ApiError(422, "service text", { code, detail: "service text" });
    expect(proposalEditRefusalKey(coded("unprocessable"))).toBe(
      "p.intake.refused_not_draft",
    );
    expect(proposalEditRefusalKey(coded("validation"))).toBe(
      "p.intake.refused_edit",
    );
    expect(proposalEditRefusalKey(new ApiError(404, "service text"))).toBe(
      "p.proposals.refused_gone",
    );
    expect(
      proposalEditRefusalKey(new ApiError(500, "service text")),
    ).toBeNull();
  });
});
