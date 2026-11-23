# Changelog

All notable changes to this project are documented in this file. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and this project follows
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

> See also: [spec/index.md](./spec/index.md), [../spec/mobile-apps.md](../spec/mobile-apps.md), [README.md](./README.md),
> [AGENTS.md](./AGENTS.md).

## [0.1.0] — Unreleased

### Added — the first slice (MA-1 to MA-6, 2026-11-23)

- The iOS and iPadOS app: the server screen with a readiness check, sign-in and sign-out through
  `ASWebAuthenticationSession` (OIDC discovery, authorization code with PKCE S256, `state` check, refresh near expiry,
  RP-initiated logout), **Continue without signing in** for a loopback address, the plans list with search, the plan
  screen (fields, child plans, tasks), moving a task, and the new-plan form.
- `PortfolioKit`, a Swift package with `PortfolioAPI`, `PortfolioAuth`, `PortfolioStub` and `PortfolioUI`; the app project
  `PortfolioMobile.xcodeproj` over it, with an `Info.plist` URL scheme for the redirect and a String Catalog (English).
- Tokens in the Keychain (`kSecAttrAccessibleAfterFirstUnlockThisDeviceOnly`); the server settings in user defaults.
- Every MA-3 row: `401` signs out, `403` shows the service's rule, `404`, `409` on create, `422` on the form (Lily's
  `ErrorSummary` and the field's error, from `errors` when sent, else `detail` split on `"; "`), `422` on a move (the
  service's reason in a `LilyAlert`, the task left as it was), any other answer with its HTTP status, and unreachable with
  **Try again**.
- Tests: 102 Swift Testing tests in the package on macOS (requests, decoding including `{"Custom": …}` and unknown fields,
  each MA-3 row, PKCE with the RFC 7636 appendix B vector, discovery and issuer checks, the redirect, refresh and
  sign-out, the copy catalog, the view models, the user-defaults store), one more Keychain test behind
  `PORTFOLIO_KEYCHAIN_TESTS=1`; and 8 UI tests on the iOS Simulator against the stub: list, plan, move, create (with a
  `422` and a `409`), search, the server and sign-in screens, and `performAccessibilityAudit` over every screen at the
  default and the largest accessibility text size.
- The audit's workarounds (iOS 27.2): no `if` inside the sign-in and new-plan forms around a row that is shown (the audit
  reported such rows' Dynamic Type as unsupported), the sheets' Cancel as an icon button (the text button failed contrast
  and Dynamic Type in the toolbar), and wrapping address fields (a long redirect URI was clipped). No audit issue is
  ignored beyond the two contrast cases Lily's own examples ignore.

### Not built

- Everything MA-7 lists: editing or deleting a plan, goals, issues, comments, the board as columns, sprints,
  notifications, offline storage, push, locales other than English, iPad layouts beyond the system's adaptation, store
  packaging and signing, and a CI job on a macOS runner.
- Paging: each list shows the first page the service returns (`Link: rel="next"` is not followed).
- A blocked reason on a move into **Blocked** (the service accepts one; the app sends none).
- An app icon. A distinct error for a search index that is unavailable (`503` is shown as unreachable).

### Not verified

- No run against a real portfolio service or a real OIDC provider: the sign-in, refresh and sign-out flows are tested only
  against stub providers in unit tests, and the browser step (`ASWebAuthenticationSession`) is not exercised by any test.
- No run on a physical iPhone or iPad, with VoiceOver, or on an iOS 18 runtime (only iOS 27.2 in the Simulator).
- The Keychain store's test runs only on macOS when asked for; the iOS Keychain is not exercised by a test.
- The accessibility audit on iPad, in dark mode, and at text sizes between the two tested.
- That a provider accepts the end-session request's `post_logout_redirect_uri`.
