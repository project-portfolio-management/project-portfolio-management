// Demonstration mode, the parts that need no environment (so they are unit-tested): the one address
// the demonstration accepts and the identity it signs in.

import type { IdTokenClaims } from "./server/oidc";

/** The one address the demonstration accepts. */
export const DEMO_EMAIL = "example@example.com";

/** The access token of a demonstration session. The proxy forwards no token for it. */
export const DEMO_TOKEN = "demo";

/** Whether `email` is the demonstration address (case and surrounding space ignored). */
export function isDemoEmail(email: string): boolean {
  return email.trim().toLowerCase() === DEMO_EMAIL;
}

/** The identity claims of the demonstration person. */
export function demoClaims(nowSeconds: number): IdTokenClaims {
  return {
    iss: "demo",
    sub: "demo-user",
    aud: "demo",
    exp: nowSeconds + 8 * 3600,
    iat: nowSeconds,
    name: "Example Person",
    email: DEMO_EMAIL,
    roles: [],
  };
}
