# Portfolio for iOS and iPadOS

A native iPhone and iPad app over the portfolio service, for someone away from a desk who needs to look a plan up and move
a task on. It calls the service directly, signs in with any OIDC provider through the system browser (PKCE S256), and
draws its screens with Lily Design System for Swift.

Status: **first slice, 0.1.0, unreleased.** Not in the App Store, not signed for a device, not run against a live service.
What it must do is [`../spec/mobile-apps.md`](../spec/mobile-apps.md) (MA-1 to MA-7); what is particular to iOS is
[`spec/index.md`](spec/index.md); what is and is not done is [`CHANGELOG.md`](CHANGELOG.md).

## Screens

| Screen | Service calls |
| --- | --- |
| Server: the service address and the sign-in settings, with a readiness check | `GET /_health/ready` |
| Sign in and sign out (`ASWebAuthenticationSession`); **Continue without signing in** for a loopback address only | the provider's discovery, authorization, token and end-session endpoints |
| Plans, with search | `GET /api/plans`, `GET /api/plans/search?q=` |
| Plan: fields, child plans, tasks | `GET /api/plans/{pid}`, `GET /api/plans?parent={pid}`, `GET /api/plans/{pid}/tasks` |
| Move a task (a sheet of statuses) | `PATCH /api/plans/{pid}/tasks/{tid}` |
| New plan: name, kind, status, code, start and target dates | `POST /api/plans` |

## Requirements

- Xcode 27 (tested with 27.2, Swift 6.4) and an iOS 18 or later simulator runtime (tested on iOS 27.2).
- Lily Design System for Swift, fetched into the repository's `vendor/` by `scripts/fetch-lily.sh` (run once from the
  repository root). To use another Lily checkout, set `LILY_DESIGN_SYSTEM_DIR` to the root of that monorepo checkout before
  starting Xcode or `xcodebuild`.

## Build, run and test

From this directory:

```sh
# The package: API, sign-in, stub and view-model tests, on macOS (no simulator)
cd PortfolioKit && swift test && cd ..

# The Keychain test too (writes to and cleans up the login keychain of whoever runs it)
cd PortfolioKit && PORTFOLIO_KEYCHAIN_TESTS=1 swift test && cd ..

# Pick a simulator
xcrun simctl list devices available

# Build the app
xcodebuild -project PortfolioMobile.xcodeproj -scheme PortfolioMobile \
  -destination 'platform=iOS Simulator,name=iPhone 18 Pro' -derivedDataPath build/DerivedData build

# The UI tests: the flows and the accessibility audit, against the stub (about four minutes)
xcodebuild -project PortfolioMobile.xcodeproj -scheme PortfolioMobile \
  -destination 'platform=iOS Simulator,name=iPhone 18 Pro' -derivedDataPath build/DerivedData test
```

To run it: open `PortfolioMobile.xcodeproj`, choose a simulator and run. On first launch it asks for the service address
and the sign-in settings (issuer, client id, redirect URI). The provider needs a public client with PKCE whose redirect URI
is `io.github.projectportfoliomanagement.mobile:/oauth2redirect` (or another path under that scheme), and the service needs
`PROJECT_PORTFOLIO_MANAGEMENT_OIDC_ISSUER` set to the same issuer. A service on the Mac with enforcement off can be used at
`http://127.0.0.1:5150` with **Continue without signing in**.

To try the app with no service at all, add the launch argument `-stub` to the scheme's Run action (Debug builds only): it
then runs on an in-memory stand-in with fixture plans and tasks.

## Layout

- `PortfolioKit/`: a Swift package with all the code. `PortfolioAPI` (wire shapes, requests, errors), `PortfolioAuth`
  (OIDC, PKCE, token storage), `PortfolioStub` (the stand-in service), `PortfolioUI` (screens, view models, the String
  Catalog).
- `PortfolioMobile.xcodeproj`, `PortfolioMobile/`, `PortfolioMobile-Info.plist`: the app shell.
- `PortfolioMobileUITests/`: UI flows and the accessibility audit.

Agent guide: [`AGENTS.md`](AGENTS.md).
