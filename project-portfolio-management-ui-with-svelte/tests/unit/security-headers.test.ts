import { describe, it, expect } from "vitest";
import {
  SECURITY_HEADERS,
  HSTS_VALUE,
  applySecurityHeaders,
  isHttps,
} from "../../src/lib/security-headers";

describe("isHttps", () => {
  it("detects an https URL", () => {
    expect(isHttps(new URL("https://x.test/"), null)).toBe(true);
  });
  it("trusts X-Forwarded-Proto: https (first hop)", () => {
    expect(isHttps(new URL("http://x.test/"), "https")).toBe(true);
    expect(isHttps(new URL("http://x.test/"), "HTTPS, http")).toBe(true);
  });
  it("is false for plain http", () => {
    expect(isHttps(new URL("http://x.test/"), null)).toBe(false);
    expect(isHttps(new URL("http://x.test/"), "http")).toBe(false);
  });
});

describe("applySecurityHeaders", () => {
  it("sets the full set, with HSTS, over HTTPS", () => {
    const h = applySecurityHeaders(new Headers(), true);
    expect(h.get("strict-transport-security")).toBe(HSTS_VALUE);
    expect(HSTS_VALUE).toBe("max-age=31536000; includeSubDomains");
    for (const [k, v] of Object.entries(SECURITY_HEADERS)) {
      expect(h.get(k)).toBe(v);
    }
    expect(h.get("x-content-type-options")).toBe("nosniff");
    expect(h.get("x-frame-options")).toBe("DENY");
    expect(h.get("referrer-policy")).toBe("strict-origin-when-cross-origin");
    expect(h.get("cross-origin-opener-policy")).toBe("same-origin");
    expect(h.get("cross-origin-resource-policy")).toBe("same-origin");
  });
  it("omits HSTS over plain HTTP", () => {
    const h = applySecurityHeaders(new Headers(), false);
    expect(h.has("strict-transport-security")).toBe(false);
    expect(h.get("x-content-type-options")).toBe("nosniff");
  });
  it("denies the sensitive browser features", () => {
    const p = SECURITY_HEADERS["Permissions-Policy"];
    for (const f of ["camera", "microphone", "geolocation", "payment", "usb"]) {
      expect(p).toContain(`${f}=()`);
    }
  });
  it("does not clobber headers a route already set", () => {
    const h = new Headers({
      "X-Frame-Options": "SAMEORIGIN",
      "Strict-Transport-Security": "max-age=1",
    });
    applySecurityHeaders(h, true);
    expect(h.get("x-frame-options")).toBe("SAMEORIGIN");
    expect(h.get("strict-transport-security")).toBe("max-age=1");
    expect(h.get("referrer-policy")).toBe("strict-origin-when-cross-origin");
  });
});
