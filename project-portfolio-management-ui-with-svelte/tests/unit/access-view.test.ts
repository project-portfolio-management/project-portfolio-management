// NC-1 slice 7 (RG-9, RG-10): the checks on the access review's filters and on a break-glass request, the bodies, a
// grant's state, and the refusal sentences. The rules mirror the service's `src/access_review.rs`,
// `src/break_glass.rs` and their controllers; the service still decides. Where the page is stricter than the service
// (a person to tell must have a UUID), the test says so.

import { describe, expect, it } from "vitest";
import {
  GRANT_SYMBOL,
  MAX_MINUTES,
  MAX_REASON,
  MIN_REASON,
  accessRefusalKey,
  grantState,
  isPersonRef,
  isPlanId,
  openBody,
  openProblems,
  reasonLength,
  reviewProblem,
  reviewQuery,
} from "../../src/lib/access-view";

const PLAN = "11111111-1111-4111-8111-111111111111";
const ERIN = "person:6f1c2a3b-0000-4000-8000-00000000000e";
const REASON =
  "Incident 4412: the plan owner is unreachable and the release is blocked";

const good = {
  plan: PLAN,
  reason: REASON,
  minutes: "45",
  notify: ERIN,
  acknowledged: true,
};

describe("the access review's filters", () => {
  it("takes a blank plan as every plan and a blank age as the service's default", () => {
    expect(reviewProblem("", "")).toBeNull();
    expect(reviewQuery("", "")).toEqual({});
    expect(reviewQuery(` ${PLAN.toUpperCase()} `, " 90 ")).toEqual({
      plan: PLAN,
      older_than_days: 90,
    });
    expect(reviewQuery("", "0")).toEqual({ older_than_days: 0 });
  });

  it("refuses what the service would: a plan that is not an id, a negative or fractional age", () => {
    expect(reviewProblem("Apollo", "365")).toBe("plan_id");
    expect(reviewProblem(PLAN, "-1")).toBe("days");
    expect(reviewProblem(PLAN, "1.5")).toBe("days");
    expect(reviewProblem(PLAN, "a year")).toBe("days");
    expect(reviewProblem(PLAN, "365")).toBeNull();
  });
});

describe("a break-glass request", () => {
  it("has no problems when it is what the service takes", () => {
    expect(openProblems(good)).toEqual([]);
    expect(
      openProblems({
        ...good,
        notify: "worker:6F1C2A3B-0000-4000-8000-00000000000E",
      }),
    ).toEqual([]);
    expect(openProblems({ ...good, minutes: "1" })).toEqual([]);
    expect(openProblems({ ...good, minutes: String(MAX_MINUTES) })).toEqual([]);
  });

  it("needs a reason of at least 20 characters once trimmed, counted as the service counts them", () => {
    expect(openProblems({ ...good, reason: "because" })).toEqual([
      "reason_short",
    ]);
    expect(openProblems({ ...good, reason: " ".repeat(60) })).toEqual([
      "reason_short",
    ]);
    // Nineteen characters with spaces around is still short; twenty is enough.
    expect(
      openProblems({ ...good, reason: `  ${"x".repeat(MIN_REASON - 1)}  ` }),
    ).toEqual(["reason_short"]);
    expect(openProblems({ ...good, reason: "x".repeat(MIN_REASON) })).toEqual(
      [],
    );
    // An emoji is one character to the service (a Unicode scalar value), not two.
    expect(reasonLength("🔥".repeat(MIN_REASON))).toBe(MIN_REASON);
    expect(
      openProblems({ ...good, reason: "x".repeat(MAX_REASON + 1) }),
    ).toEqual(["reason_long"]);
  });

  it("needs a whole number of minutes from 1 to 240", () => {
    for (const m of ["0", "241", "-5", "1.5", "", "an hour", "1000"])
      expect(openProblems({ ...good, minutes: m }), m).toEqual(["minutes"]);
  });

  it("needs a person to tell as a person: or worker: reference with an id, which is stricter than the service", () => {
    // The service takes any text after the prefix, but it can deliver a notice and read the person's choices only
    // for an id (`recipient_sub`), so the page asks for one.
    expect(isPersonRef(ERIN)).toBe(true);
    for (const n of [
      "erin",
      "person:",
      "person:erin",
      `person: ${ERIN.slice(7)}`,
      `${ERIN} extra`,
      "team:6f1c2a3b-0000-4000-8000-00000000000e",
    ])
      expect(openProblems({ ...good, notify: n }), n).toEqual(["notify_ref"]);
  });

  it("needs a plan id and the acknowledgement, and lists every problem at once", () => {
    expect(isPlanId(` ${PLAN} `)).toBe(true);
    expect(
      openProblems({
        plan: "Apollo",
        reason: "",
        minutes: "0",
        notify: "",
        acknowledged: false,
      }),
    ).toEqual([
      "plan_id",
      "reason_short",
      "minutes",
      "notify_ref",
      "acknowledged",
    ]);
  });

  it("sends the trimmed fields and always the minutes, so the service's default is never leaned on", () => {
    expect(
      openBody({
        plan: ` ${PLAN.toUpperCase()} `,
        reason: `  ${REASON}  `,
        minutes: " 45 ",
        notify: ` ${ERIN} `,
      }),
    ).toEqual({
      plan_pid: PLAN,
      reason: REASON,
      minutes: 45,
      notify_ref: ERIN,
    });
  });
});

describe("a grant's state", () => {
  it("follows the service's live flag, then says whether it was ended early or ran out", () => {
    expect(grantState({ live: true, revoked_at: null })).toBe("live");
    expect(
      grantState({ live: false, revoked_at: "2026-11-10T10:00:00Z" }),
    ).toBe("revoked");
    expect(grantState({ live: false, revoked_at: null })).toBe("expired");
    expect(new Set(Object.values(GRANT_SYMBOL)).size).toBe(3);
  });
});

describe("refusals", () => {
  it("say a sentence for each step the service refuses", () => {
    expect(accessRefusalKey("review", 403)).toBe(
      "p.access.refused_review_forbidden",
    );
    expect(accessRefusalKey("review", 422)).toBe(
      "p.access.refused_review_invalid",
    );
    expect(accessRefusalKey("review", 400)).toBe(
      "p.access.refused_review_invalid",
    );
    expect(accessRefusalKey("list", 401)).toBe("p.access.refused_sign_in");
    expect(accessRefusalKey("open", 401)).toBe("p.access.refused_sign_in");
    expect(accessRefusalKey("open", 403)).toBe(
      "p.access.refused_open_forbidden",
    );
    expect(accessRefusalKey("open", 404)).toBe(
      "p.access.refused_open_not_found",
    );
    expect(accessRefusalKey("open", 409)).toBe(
      "p.access.refused_open_conflict",
    );
    expect(accessRefusalKey("open", 422)).toBe("p.access.refused_open_invalid");
    expect(accessRefusalKey("open", 429)).toBe("p.access.refused_open_limit");
    expect(accessRefusalKey("revoke", 404)).toBe(
      "p.access.refused_revoke_not_found",
    );
  });

  it("leave anything else to the general rule", () => {
    expect(accessRefusalKey("open", 500)).toBeNull();
    expect(accessRefusalKey("list", 403)).toBeNull();
    expect(accessRefusalKey("revoke", 403)).toBeNull();
  });
});
