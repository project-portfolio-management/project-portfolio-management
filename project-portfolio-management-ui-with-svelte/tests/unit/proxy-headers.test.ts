import { describe, it, expect } from "vitest";
import {
  RELAYED_RESPONSE_HEADERS,
  relayResponseHeaders,
} from "../../src/lib/server/proxy-headers";

describe("relayResponseHeaders (CQ-4)", () => {
  it("passes the paging, caching, download and marking headers through", () => {
    const upstream = new Headers({
      "Content-Type": "application/json",
      "X-Total-Count": "1234",
      "X-Limit": "100",
      "X-Offset": "200",
      ETag: '"abc"',
      "Retry-After": "30",
      Location: "/api/plans/1",
      "Content-Disposition": 'attachment; filename="plans.csv"',
      "X-Handling-Mark": "OFFICIAL-SENSITIVE",
    });
    const out = relayResponseHeaders(upstream);
    expect(out.get("x-total-count")).toBe("1234");
    expect(out.get("x-limit")).toBe("100");
    expect(out.get("x-offset")).toBe("200");
    expect(out.get("etag")).toBe('"abc"');
    expect(out.get("retry-after")).toBe("30");
    expect(out.get("location")).toBe("/api/plans/1");
    expect(out.get("content-disposition")).toBe(
      'attachment; filename="plans.csv"',
    );
    expect(out.get("x-handling-mark")).toBe("OFFICIAL-SENSITIVE");
    expect(out.get("content-type")).toBe("application/json");
  });

  it("drops cookies, hop-by-hop and unnamed headers", () => {
    const upstream = new Headers({
      "Set-Cookie": "sid=x",
      Connection: "keep-alive",
      "Transfer-Encoding": "chunked",
      "Content-Encoding": "gzip",
      "Content-Length": "10",
      "Content-Security-Policy": "default-src *",
      "X-Something-Else": "1",
    });
    expect([...relayResponseHeaders(upstream).keys()]).toEqual([]);
  });

  it("names only lower-case headers, once each", () => {
    for (const h of RELAYED_RESPONSE_HEADERS) expect(h).toBe(h.toLowerCase());
    expect(new Set(RELAYED_RESPONSE_HEADERS).size).toBe(
      RELAYED_RESPONSE_HEADERS.length,
    );
  });
});
