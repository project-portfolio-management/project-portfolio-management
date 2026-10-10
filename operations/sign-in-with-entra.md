# Sign in with Microsoft Entra ID

People can sign in with their organization's Microsoft Entra ID account through
OpenID Connect (OIDC). This guide says which routes exist, which one is tested,
how to set each side up, and what the test proves.

## Choose a route

```text
Route A (tested end to end): Keycloak brokers Entra
  browser -> front-end -> Keycloak realm -> Microsoft Entra ID
                |              |
                |              +--> issues the token --> API verifies it (Keycloak verifier)
                +--> sends the token as the bearer

Route B: the front-end signs in with Entra directly
  browser -> front-end -> Microsoft Entra ID       (the front-end half works and is tested)
  The API does not verify Entra's own access tokens, so on its own this does not
  give API access. See "Why not Entra directly".

Route C: the authentication service federates Entra  (the "Sign in with SSO" button)
  Configured in that service, not in this repository; documented there, not tested here.
```

**Use route A.** The API's token check for non-PASETO tokens is the Keycloak
verifier: signature against the realm's keys, issuer `{url}/realms/{realm}`,
required audience, verified e-mail, and an explicit role map to access
attributes. Entra becomes Keycloak's upstream identity provider; Keycloak is what
the API trusts.

## Route A, step by step

### 1. Register the application in Microsoft Entra

In the Entra admin center: **Applications, App registrations, New registration**.

| Setting | Value |
|---|---|
| Supported account types | **Single tenant** (your organization only) |
| Redirect URI (type **Web**) | `https://<keycloak-host>/realms/<realm>/broker/entra/endpoint` |
| Certificates and secrets | a client secret (or a certificate) for Keycloak to use; note its expiry |
| API permissions | Microsoft Graph delegated `openid`, `profile`, `email` (and `offline_access` if Keycloak should refresh) |
| Token configuration | add the optional claims `email` (and `given_name`, `family_name` if you want names filled in) |
| App roles | create roles such as `Portfolio.Editor` and `Portfolio.Admin` (allowed member types: Users/Groups) |
| Enterprise application, Users and groups | assign the people (or groups) to the roles; turn on **Assignment required** so unassigned people cannot sign in at all |

Note the **Directory (tenant) ID** and **Application (client) ID**. Use the
tenant-specific authority; do not use `common` or `organizations`, whose
discovery documents carry a `{tenantid}` template and whose tokens name
varying issuers.

> **Groups instead of app roles.** Group membership appears in the token as
> `groups` (object ids). A user in more than about 200 groups gets a "groups
> overage" pointer instead of the list, which silently drops roles. Prefer app
> roles for access decisions.

### 2. Configure Keycloak

Create a realm (here `mxi`) and add the Entra identity provider. A complete,
tested example is
[`tests/fixtures/keycloak/entra-broker-realm.json`](../project-portfolio-management-service-with-rust/tests/fixtures/keycloak/entra-broker-realm.json).

**Identity provider** (alias `entra`, provider **OpenID Connect v1.0**):

| Setting | Value |
|---|---|
| Discovery / authorization URL | `https://login.microsoftonline.com/<tenant-id>/oauth2/v2.0/authorize` |
| Token URL | `https://login.microsoftonline.com/<tenant-id>/oauth2/v2.0/token` |
| JWKS URL, **Validate signatures** on, **Use JWKS URL** on | `https://login.microsoftonline.com/<tenant-id>/discovery/v2.0/keys` |
| Issuer | `https://login.microsoftonline.com/<tenant-id>/v2.0` (Keycloak then rejects an ID token from any other issuer) |
| Client ID, client secret, client authentication | from step 1; **client secret sent as post** |
| Scopes | `openid profile email` |
| **Use PKCE**, method | on, `S256` |
| **Trust email** | **on** (see below) |
| Disable user info | on (the ID token carries what is needed) |
| Sync mode | **force**, so roles follow Entra on every sign-in |

**Trust email.** Entra's ID token has no `email_verified` claim, and the API
requires a verified e-mail whenever a token carries one. Turning on *Trust email*
tells Keycloak to treat Entra's address as verified. That is correct for a
**single-tenant** registration whose directory you control; do not turn it on for
a multi-tenant or personal-account registration.

**Role mappers** (identity provider, Mappers, type *Claim to Role*, sync mode
force): `claim: roles`, `claim value: Portfolio.Editor`, `role: editor`; and
`Portfolio.Admin` to `root`. These are the only route from an Entra role to a
Keycloak role: an Entra role with no mapper grants nothing.

**Client** (what the front-end and the API see): a **public** client `mxi-web`
with the standard flow, **PKCE `S256` required**, valid redirect URI
`https://<app-host>/signin/oidc/callback`, and an **audience mapper** so access
tokens carry `aud` = `mxi-api`. (The API requires at least one accepted audience.)

### 3. Configure the API

The container image is built with the `keycloak` feature. Set:

| Variable | Value |
|---|---|
| `PROJECT_PORTFOLIO_MANAGEMENT_KEYCLOAK_URL` | `https://<keycloak-host>` (https only; loopback excepted) |
| `PROJECT_PORTFOLIO_MANAGEMENT_KEYCLOAK_REALM` | `mxi` |
| `PROJECT_PORTFOLIO_MANAGEMENT_KEYCLOAK_AUDIENCES` | `mxi-api` (comma-separated; must not be empty) |
| `PROJECT_PORTFOLIO_MANAGEMENT_KEYCLOAK_ROLE_MAP` | `{"editor":{"access":["write"]},"root":{"access":["admin"]}}` |

With this, `editor` can read and write, `root` can also delete and merge, and a
signed-in person with no mapped role can read only (the default policy). Login
stays enforced; a service with enforcement on and no verifier refuses to start.

### 4. Configure the front-end

| Variable | Value |
|---|---|
| `PUBLIC_OIDC_DIRECT_ENABLED` | `true` |
| `PUBLIC_OIDC_DIRECT_PROVIDER` | `Microsoft Entra ID` (the name on the sign-in button) |
| `OIDC_ISSUER` | `https://<keycloak-host>/realms/mxi` |
| `OIDC_CLIENT_ID` | `mxi-web` |
| `OIDC_IDP_HINT` | `entra` (skips Keycloak's own chooser and goes straight to Entra) |
| `OIDC_SESSION_HOURS` | `8` (hard limit; the access token is refreshed inside it) |
| `OIDC_IDLE_MINUTES` | `30`: sign out after this long without a request (sliding); `0` turns it off. The person lands on the sign-in page with a sentence saying why |
| `OIDC_END_SESSION` | `true` to also end the Keycloak session on sign-out (register the post-logout address in Keycloak first) |
| `OIDC_REDIRECT_URI` | only if the public address differs from what the server sees behind a proxy |

The sign-in page then shows **Sign in with Microsoft Entra ID**. See
[`.env.example`](../project-portfolio-management-ui-with-svelte/.env.example).

## What the front-end does

The server side of the front-end is the OIDC client. It starts the
authorization-code flow with **PKCE (S256)**, a random `state` and `nonce`, and
the exact redirect address; remembers them on the server, bound to the browser by
a short-lived `HttpOnly` cookie; and on the callback requires the `state` to match
**both** the cookie and a pending login (single use), exchanges the code with the
PKCE verifier, and **verifies the ID token itself** (RS256 only; signature
against the provider's keys; issuer, audience, expiry, not-before, nonce; and the
tenant if `OIDC_TENANT_ID` is set). The tokens stay on the server. The browser
holds one random, `HttpOnly`, `Secure`, host-locked cookie. Requests to the API go
through the same-origin proxy, which adds the access token as the bearer and
refreshes it shortly before it expires; if it cannot be refreshed the person is
signed out.

**Limit.** Sessions are held in the front-end process's memory. A restart signs
people out (one click to sign back in), and several front-end instances need
sticky sessions or a shared store.

## Why not Entra directly

Entra can be the front-end's provider (the tested mock stands in for it), but the
API verifies Keycloak tokens, not Entra's. Entra access tokens for an API have a
different issuer form (v1.0 `sts.windows.net` or v2.0, chosen by the app's
`accessTokenAcceptedVersion`), a different audience (`api://...`) and different
role and group claims. Verifying them directly would mean a second verifier. If
you need that, it is a new feature, not a setting.

## What is tested

| Test | Proves | Run |
|---|---|---|
| `tests/e2e/oidc-signin.spec.ts` (7 browser tests, mock Entra-compatible provider and API) | the whole round trip signs the person in with PKCE; tokens never reach the browser; a short-lived token is refreshed; cancelling shows a sentence; a person from another tenant, a forged `state` and a replayed code are refused; sign-out ends the session here and at the provider; the page passes the WCAG 2.2 AA audit | `pnpm exec playwright test tests/e2e/oidc-signin.spec.ts` |
| `tests/unit/oidc.test.ts` (38 tests) | ID-token validation (signature, algorithm confusion, issuer, audience, nonce, expiry, tenant), PKCE, discovery checks, safe return paths, the session store and refresh | `pnpm exec vitest run tests/unit/oidc.test.ts` |
| `tests/keycloak_entra_broker.rs` (real Keycloak in a container, mock Entra) | Entra sign-in **through a real Keycloak** ends in an access token that the real API guard accepts, with Entra's `Portfolio.Editor` role mapped to write access, a person without a role read-only, PKCE enforced on the provider leg, and a tampered token refused | `cargo test --features keycloak --test keycloak_entra_broker -- --ignored` (needs Podman; see the test's header) |

**Not tested against real Entra.** The provider in the tests follows Entra's
documented v2.0 shapes but is not Microsoft's service. Before go-live, run the
checklist below once with a real tenant.

### Go-live checklist (real tenant)

1. Sign in as a person assigned `Portfolio.Editor`: you can read and write.
2. Sign in as a person assigned no role: you can read, not write.
3. Remove the person's assignment in Entra; with *Assignment required* on, sign-in is refused.
4. Sign in with an account from another tenant: refused.
5. Conditional Access and MFA, if you use them, apply at the Entra step.
6. Change a person's role in Entra and sign in again: access follows (sync mode *force*).
7. Sign out; confirm the API refuses the old session; confirm the audit log names the person.

## Troubleshooting

| Symptom | Likely cause |
|---|---|
| Entra shows `AADSTS50011` | the redirect address registered in Entra differs from Keycloak's broker endpoint, character for character |
| `AADSTS700016` | the client id is wrong, or the app is not in this tenant |
| Back at the sign-in page with "Sign-in did not complete" | read the front-end log line `oidc sign-in refused: <reason>` (`state`, `id_token`, `token_endpoint`, `metadata`...); the reason is never shown to the user |
| Signed in but cannot write | no role mapper matched: check the Entra assignment, the `roles` claim in a token, the mapper's claim value, and `KEYCLOAK_ROLE_MAP` |
| API answers `401` after a successful sign-in | audience missing (`aud` must contain a value in `KEYCLOAK_AUDIENCES`), issuer host differs from `KEYCLOAK_URL`, or the e-mail is not verified (turn on *Trust email* for a single-tenant app) |
| Signed out every few minutes | the access token cannot be refreshed: add `offline_access` to the scopes and check the refresh-token lifetime |
