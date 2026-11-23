# iOS and iPadOS app: platform specification

The contract is [`../../spec/mobile-apps.md`](../../spec/mobile-apps.md) (MA-1 to MA-7): the screens, the service calls and
wire shapes, sign-in, errors, design system and accessibility, the Lily dependency, the tests, and what is not built. This
file records only what is particular to iOS and iPadOS. Where the two disagree, the shared contract wins.

Status: **first slice, 0.1.0, unreleased** (2026-11-23). Not in the App Store.

## S-1 Platform

- iOS and iPadOS 18 or later; Swift 6 language mode with complete strict concurrency; SwiftUI.
- Built with Xcode 27 (tested with 27.2 and Swift 6.4). The package's tests also build and run on macOS 15 or later
  (`swift test`), so the API, sign-in and view-model code compiles for macOS too; there is no macOS app.
- No third-party dependency other than Lily Design System for Swift (`LilyUI`, `LilyThemes`).
- Bundle id `io.github.projectportfoliomanagement.mobile`; the UI tests' bundle adds `.uitests`.

## S-2 Layout

| Part | What it is |
| --- | --- |
| `PortfolioKit/` | A Swift package holding all the app's code: `PortfolioAPI` (wire shapes, requests, the `HTTPTransport` seam, MA-3 error mapping, the address rule), `PortfolioAuth` (OIDC discovery, PKCE, `state`, token exchange and refresh, the Keychain and user-defaults stores), `PortfolioStub` (an in-memory stand-in for the service), `PortfolioUI` (SwiftUI screens and their view models, and the String Catalog) |
| `PortfolioMobile.xcodeproj` | The app target (a shell that starts `RootView`) and the UI test target; Xcode 16 synchronized folders and a local package reference to `PortfolioKit` |
| `PortfolioMobile/` | The app's entry point and `InfoPlist.xcstrings` (the display name) |
| `PortfolioMobile-Info.plist` | The keys Xcode does not generate: the redirect URL scheme and `NSAllowsLocalNetworking` |
| `PortfolioMobileUITests/` | The UI flows and the accessibility audit, against the stub |

## S-3 Lily (MA-5)

`PortfolioKit/Package.swift` depends on Lily as a local package: `$LILY_DESIGN_SYSTEM_DIR/lily-design-system-swift` when that
variable is set (it names the root of a Lily monorepo checkout), else
`../../vendor/lily-design-system/lily-design-system-swift` from `PortfolioKit/`. Xcode reads the variable only if it is in
the environment Xcode or `xcodebuild` was started from.

## S-4 Sign-in (MA-2)

- The browser is `ASWebAuthenticationSession`, through SwiftUI's `webAuthenticationSession` environment value, with an
  **ephemeral** browser session: no cookie is shared with Safari, so each sign-in asks the provider again.
- The redirect URI's scheme is the one the session listens for; the default scheme
  `io.github.projectportfoliomanagement.mobile` is also registered in `CFBundleURLTypes`.
- The scope sent is `openid offline_access`. Refresh needs the provider to issue a refresh token; without one the person
  signs in again once the access token is within 60 seconds of expiry.
- The issuer is compared with the discovery document's `issuer` as exact strings (a trailing slash makes them differ).
  The document's `code_challenge_methods_supported`, when present, must include `S256`.
- Sign-out forgets the tokens first, then opens the provider's `end_session_endpoint` in the same kind of browser session
  with `client_id`, `id_token_hint` and `post_logout_redirect_uri` (the redirect URI). A provider that does not accept that
  redirect leaves the sheet open for the person to close; the tokens are already gone.
- **Development address on iOS.** A development address is a loopback one: `localhost`, `*.localhost`, `127.0.0.0/8` or
  `[::1]`. The Android emulator's host `10.0.2.2` is not one on iOS (the iOS Simulator reaches the Mac at `127.0.0.1`), so
  plain `http://10.0.2.2` is refused here and does not offer **Continue without signing in**. This narrows MA-2's list
  on purpose; on iOS that address is an ordinary private address. `NSAllowsLocalNetworking` lets App Transport Security allow those addresses.

## S-5 Storage

The Keychain holds one generic-password item, `tokens` (access, refresh and id token, and the expiry), under the service
`io.github.projectportfoliomanagement.mobile`, `kSecAttrAccessibleAfterFirstUnlockThisDeviceOnly`. User defaults hold the
server settings under the key `io.github.projectportfoliomanagement.mobile.server-settings` (service URL, issuer, client id,
redirect URI, and whether **Continue without signing in** was chosen); they hold no secret. A sign-in under way (its
`state` and verifier) is held in memory only. No plan or task data is stored. `URLSession` runs with an ephemeral
configuration: no cookie store and no URL cache.

## S-6 Strings and accessibility (MA-4)

- Every word is in `PortfolioKit/Sources/PortfolioUI/Resources/Localizable.xcstrings`, looked up by key through the `Copy`
  enum; the catalog has English only. `CopyTests` fails when a key has no English value, when the catalog has a key the
  code does not use, or when a language other than English appears. Words the service sends (a `422` problem, a `403`
  rule, a custom status, a plan's name) are shown as the service wrote them.
- Lily's `Field`, `ErrorSummary`, `SummaryList`, `Tag`, `Card`, `InsetText`, `RecordList` and `LilyAlert` are used; buttons,
  pickers, toggles, date pickers, lists, search, sheets and navigation are the system's.
- A `422` on a move (a WIP limit, a workflow refusal) is shown in a `LilyAlert` on the move sheet with the service's
  reason; any other answer the app does not expect, including a success it cannot read, says so with the HTTP status.
- Lists show the first page the service returns only (plans: its default page; tasks: up to 2,000, with a note when the
  service says more follow). Paging through `Link: rel="next"` is not built.
- A task's status is the words "Status: In review" on its row, a plan's status a Lily `Tag`, and the move sheet marks the
  present status with the words "Current status".
- The UI tests run `performAccessibilityAudit`, on an iOS Simulator, on every screen at the default text size and at
  `UICTContentSizeCategoryAccessibilityXXXL`. Ignored issues are listed in `AccessibilityAuditTests.isIgnored`, each with
  its reason.
- On iOS 27.2 the audit reports a Form row inside a taken `if` branch (or a `ForEach` of one) as not supporting Dynamic
  Type, though it does. The forms therefore keep shown rows out of `if`s: a busy state disables the button rather than
  replacing it, and the sign-in screen chooses one of two whole forms.

## S-7 The stub (MA-6)

In a Debug build the launch argument `-stub` runs the app on `PortfolioStub` (fixture plans and tasks, in memory, at
`http://127.0.0.1:5150`, without signing in); `-stub-server-screen` with it starts at the server screen and
`-stub-sign-in-screen` at the sign-in screen. A Release build
ignores both. The stub answers `422` for a blank name, `409` for a name already used and `404` for an unknown plan or task,
and adds an unknown field to every record.

## S-8 Not built on iOS beyond MA-7

No app icon, no launch screen beyond the generated one, no widget, no Handoff, no state restoration of the navigation
path, no iPad-specific layout (the system's default adaptation only), no macOS or visionOS app.
