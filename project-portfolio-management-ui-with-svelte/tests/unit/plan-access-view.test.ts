import { describe, expect, it } from "vitest";
import {
  accessChange,
  grantChange,
  grantState,
  isUserId,
  membersText,
  parseMembers,
  sortedGrants,
  type GuestGrant,
} from "../../src/lib/plan-access-view";

const U1 = "3f2c6b7e-0000-4000-8000-000000000001";
const U2 = "3f2c6b7e-0000-4000-8000-000000000002";

describe("a plan's members (RM-8)", () => {
  it("reads user ids a line or a comma each, once, in lower case", () => {
    expect(isUserId(U1)).toBe(true);
    expect(isUserId("alice")).toBe(false);
    expect(parseMembers(` ${U1.toUpperCase()}\n\n${U2}, ${U1}`)).toEqual({
      members: [U1, U2],
    });
    expect(parseMembers("alice")).toEqual({
      problems: ["p.plan_access.member_bad"],
    });
    const many = Array.from(
      { length: 201 },
      (_, n) => `3f2c6b7e-0000-4000-8000-${String(n).padStart(12, "0")}`,
    );
    expect(parseMembers(many.join("\n"))).toEqual({
      problems: ["p.plan_access.members_many"],
    });
    expect(membersText([U1, U2])).toBe(`${U1}\n${U2}`);
  });

  it("sends the whole setting only when it changed", () => {
    const now = { private: true, members: [U1, U2] };
    expect(accessChange(true, `${U2}\n${U1}`, now)).toBeNull();
    expect(accessChange(false, `${U2}\n${U1}`, now)).toEqual({
      body: { private: false, members: [U2, U1] },
    });
    expect(accessChange(true, U1, now)).toEqual({
      body: { private: true, members: [U1] },
    });
    expect(accessChange(true, "x", now)).toEqual({
      problems: ["p.plan_access.member_bad"],
    });
  });
});

describe("guest grants (PL-7)", () => {
  const now = new Date("2026-11-27T12:00:00Z");
  it("grants a user id read or comment access, with an end in the future or none", () => {
    expect(grantChange(` ${U1} `, "read", "", now)).toEqual({
      body: { guest_sub: U1, access: "read" },
    });
    expect(grantChange(U1, "comment", "2026-12-01T09:00:00Z", now)).toEqual({
      body: {
        guest_sub: U1,
        access: "comment",
        expires_at: "2026-12-01T09:00:00.000Z",
      },
    });
    expect(grantChange("bob", "write", "2026-11-01T09:00:00Z", now)).toEqual({
      problems: [
        "p.plan_access.guest_bad",
        "p.plan_access.access_bad",
        "p.plan_access.expires_past",
      ],
    });
    expect(grantChange(U1, "read", "soon", now)).toEqual({
      problems: ["p.plan_access.expires_bad"],
    });
  });

  it("says each grant's state and lists the live first", () => {
    const g = (
      pid: string,
      live: boolean,
      revoked_at: string | null,
      expires_at: string | null,
    ): GuestGrant => ({
      pid,
      guest_sub: U1,
      access: "read",
      expires_at,
      granted_by: null,
      revoked_at,
      live,
    });
    const list = [
      g("a", false, "2026-11-20T00:00:00Z", null),
      g("b", true, null, null),
      g("c", false, null, "2026-11-01T00:00:00Z"),
      g("d", true, null, "2026-12-01T00:00:00Z"),
    ];
    expect(list.map(grantState)).toEqual([
      "revoked",
      "live",
      "expired",
      "live",
    ]);
    expect(sortedGrants(list).map((x) => x.pid)).toEqual(["d", "b", "c", "a"]);
  });
});
