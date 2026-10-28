import { describe, it, expect } from "vitest";
import {
  crossSiteRefusal,
  forwardedRequestHeaders,
} from "../../src/lib/server/proxy-guard";

const APP = "https://ppm.example.org";

describe("forwardedRequestHeaders (CQ-5)", () => {
  it("never forwards the browser's bearer, cookie or hop-by-hop headers", () => {
    const out = forwardedRequestHeaders(
      new Headers({
        Authorization: "Bearer chosen-by-a-script",
        Cookie: "__Host-mxi_session=x",
        Host: "ppm.example.org",
        Connection: "keep-alive",
        "Content-Length": "2",
        "Content-Type": "application/json",
        "Idempotency-Key": "k1",
      }),
    );
    expect(out.get("authorization")).toBeNull();
    expect(out.get("cookie")).toBeNull();
    expect(out.get("host")).toBeNull();
    expect(out.get("connection")).toBeNull();
    expect(out.get("content-length")).toBeNull();
    expect(out.get("content-type")).toBe("application/json");
    expect(out.get("idempotency-key")).toBe("k1");
  });
});

describe("crossSiteRefusal (CQ-5)", () => {
  it("lets every read through, whatever its origin", () => {
    for (const m of ["GET", "HEAD", "OPTIONS", "get"]) {
      expect(
        crossSiteRefusal(
          m,
          new Headers({ Origin: "https://evil.example" }),
          APP,
        ),
      ).toBeNull();
    }
  });

  it("lets a same-origin write through", () => {
    expect(
      crossSiteRefusal("POST", new Headers({ Origin: APP }), APP),
    ).toBeNull();
    expect(
      crossSiteRefusal(
        "DELETE",
        new Headers({ "Sec-Fetch-Site": "same-origin" }),
        APP,
      ),
    ).toBeNull();
  });

  it("refuses a write from another origin", () => {
    for (const m of ["POST", "PUT", "PATCH", "DELETE"]) {
      expect(
        crossSiteRefusal(
          m,
          new Headers({ Origin: "https://evil.example" }),
          APP,
        ),
      ).toMatch(/Origin/);
    }
    // Same host, other scheme or port, is another origin.
    expect(
      crossSiteRefusal(
        "POST",
        new Headers({ Origin: "http://ppm.example.org" }),
        APP,
      ),
    ).not.toBeNull();
    expect(
      crossSiteRefusal("POST", new Headers({ Origin: "null" }), APP),
    ).not.toBeNull();
  });

  it("falls back to Sec-Fetch-Site when there is no Origin", () => {
    expect(
      crossSiteRefusal(
        "POST",
        new Headers({ "Sec-Fetch-Site": "cross-site" }),
        APP,
      ),
    ).toMatch(/cross-site/);
    expect(
      crossSiteRefusal(
        "POST",
        new Headers({ "Sec-Fetch-Site": "same-site" }),
        APP,
      ),
    ).not.toBeNull();
    expect(
      crossSiteRefusal("POST", new Headers({ "Sec-Fetch-Site": "none" }), APP),
    ).toBeNull();
  });

  it("lets a write with neither header through (not a browser on another site)", () => {
    expect(crossSiteRefusal("POST", new Headers(), APP)).toBeNull();
  });
});
