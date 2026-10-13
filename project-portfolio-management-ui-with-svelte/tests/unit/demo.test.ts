// Demonstration mode: the one address it accepts, and the identity it signs in.

import { describe, expect, it } from "vitest";
import { DEMO_EMAIL, DEMO_SUB, demoClaims, isDemoEmail } from "../../src/lib/demo";

describe("demonstration sign-in", () => {
  it("accepts example@example.com, ignoring case and surrounding space", () => {
    expect(DEMO_EMAIL).toBe("example@example.com");
    expect(isDemoEmail("example@example.com")).toBe(true);
    expect(isDemoEmail("  Example@Example.COM ")).toBe(true);
  });

  it("accepts nothing else", () => {
    for (const other of ["", "someone@example.com", "example@example.org", "example@example.com.evil.test", "xexample@example.com"]) {
      expect(isDemoEmail(other), other).toBe(false);
    }
  });

  it("signs in a named person with no roles", () => {
    const c = demoClaims(1_000);
    expect(c.email).toBe(DEMO_EMAIL);
    expect(c.name).toBe("Example Person");
    expect(c.roles).toEqual([]);
    expect(c.exp).toBeGreaterThan(c.iat ?? 0);
    expect(c.sub).toBe(DEMO_SUB);
  });
});
