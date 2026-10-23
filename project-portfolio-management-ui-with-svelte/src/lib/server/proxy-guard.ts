// What the BFF proxy accepts from the browser (CQ-5).
//
// - The browser's own `Authorization` is never forwarded: the proxy injects
//   the session's bearer, and without a session the service must see none, so
//   a page script cannot reach the service with a token of its choosing.
// - A state-changing request from another site is refused. The session cookie
//   is SameSite=Lax, which already keeps it off cross-site POSTs; this checks
//   `Origin` (and, where a browser sends no `Origin`, `Sec-Fetch-Site`) so the
//   defence does not rest on the cookie attribute alone.

/** Request headers never forwarded to the service. */
const DROPPED_REQUEST_HEADERS = [
  "cookie",
  "host",
  "connection",
  "content-length",
  "authorization",
];

/** The browser's request headers, as the proxy forwards them (before it adds its own). */
export function forwardedRequestHeaders(browser: Headers): Headers {
  const out = new Headers(browser);
  for (const name of DROPPED_REQUEST_HEADERS) out.delete(name);
  return out;
}

/** Whether the method only reads. */
function safeMethod(method: string): boolean {
  return ["GET", "HEAD", "OPTIONS"].includes(method.toUpperCase());
}

/**
 * Why a request must be refused as cross-site, or `null` when it may pass.
 * `appOrigin` is the origin the app is served from (SvelteKit's `url.origin`,
 * which honours `ORIGIN` behind a reverse proxy).
 */
export function crossSiteRefusal(
  method: string,
  headers: Headers,
  appOrigin: string,
): string | null {
  if (safeMethod(method)) return null;
  const origin = headers.get("origin");
  if (origin !== null) {
    return origin === appOrigin
      ? null
      : "cross-site request refused: Origin does not match";
  }
  const site = headers.get("sec-fetch-site");
  if (site !== null && site !== "same-origin" && site !== "none") {
    return "cross-site request refused: Sec-Fetch-Site is " + site;
  }
  // Neither header: not a browser request made by another site (browsers send
  // one of them on every cross-site write), so the session cookie rules apply.
  return null;
}
