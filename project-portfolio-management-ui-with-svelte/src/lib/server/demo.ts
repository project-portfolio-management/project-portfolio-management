// Demonstration mode: a public try-out with fictitious data, signed in as one fixed address.
//
// Off unless `PPM_DEMO_MODE=true`. When on, the sign-in page accepts only `example@example.com`
// (no email is sent and nothing is checked against an identity provider), and starts a session in
// this server's own store. It is for a demonstration stack whose API runs with login off and holds
// only fictitious data (see `operations/demo.md`); it must never be switched on in front of real data.

import * as env from "$app/env/private";

export { DEMO_EMAIL, DEMO_SUB, isDemoEmail, demoClaims } from "../demo";
export { mintDemoToken } from "./demo-token";

/** Whether demonstration mode is on. */
export function demoEnabled(): boolean {
  return (env.PPM_DEMO_MODE ?? "").trim().toLowerCase() === "true";
}
