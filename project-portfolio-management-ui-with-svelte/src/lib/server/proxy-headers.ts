// The response headers the BFF proxy passes back to the browser (CQ-4).
//
// The proxy used to copy only `content-type`, so paging totals, entity tags,
// retry hints, download names and the handling mark never reached the page.
// This is an allow-list: hop-by-hop headers, `set-cookie` (the service never
// sets the browser's cookies) and anything else unnamed stay behind, and the
// security headers come from `hooks.server.ts`, not from upstream.

/** Response headers relayed from the service, lower case. */
export const RELAYED_RESPONSE_HEADERS = [
  "content-type",
  "content-disposition",
  "cache-control",
  "etag",
  "last-modified",
  "location",
  "retry-after",
  "link",
  "x-total-count",
  "x-limit",
  "x-offset",
  "x-handling-mark",
  "ratelimit-limit",
  "ratelimit-remaining",
  "ratelimit-reset",
] as const;

/** The headers of an upstream response that the browser may see. */
export function relayResponseHeaders(upstream: Headers): Headers {
  const out = new Headers();
  for (const name of RELAYED_RESPONSE_HEADERS) {
    const value = upstream.get(name);
    if (value !== null) out.set(name, value);
  }
  return out;
}
