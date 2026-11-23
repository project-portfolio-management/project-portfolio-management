# AGENTS.md — Portfolio for iOS and iPadOS

The native iPhone and iPad app over the portfolio service. Read [`../spec/mobile-apps.md`](../spec/mobile-apps.md) (the
contract shared with the Android app) and [`spec/index.md`](spec/index.md) (what is particular to iOS) before changing
anything. Where code and spec disagree, the spec wins: fix the code, or change the spec on purpose in the same commit.

## Ground rules

1. **Spec first.** A change of behaviour is one commit with the spec edit, the code edit and the test edit. A change that
   the Android app must match goes in `../spec/mobile-apps.md`, not here.
2. **Swift 6 language mode, complete strict concurrency, no warnings.** iOS 18 and macOS 15 are the minimums (macOS only so
   that `swift test` runs).
3. **No third-party dependency** other than Lily (`LilyUI`, `LilyThemes`), as a local package.
4. **Every word is in the String Catalog** (`PortfolioKit/Sources/PortfolioUI/Resources/Localizable.xcstrings`), reached
   through the `Copy` enum. Add the case and the catalog entry together; `CopyTests` fails otherwise. English only: add no
   other language and no machine-written translation.
5. **Lily where Lily has the component, the system's control where it has none** (buttons, pickers, toggles, lists,
   search, sheets, navigation). Lily takes `String`s; never pass it a literal.
6. **Tokens only in the Keychain.** The server settings are in user defaults; no plan or task data is stored.
7. **The view does not decide.** Behaviour lives in the view models and `PortfolioAPI` / `PortfolioAuth`, where the unit
   tests reach it on macOS; a view only lays it out.
8. **Say plainly what is not built or not verified**, in `CHANGELOG.md`, every change.

## Layout

| Path | What it is |
| --- | --- |
| `PortfolioKit/Package.swift` | The package; reads `LILY_DESIGN_SYSTEM_DIR` |
| `PortfolioKit/Sources/PortfolioAPI` | `Models.swift` (wire shapes), `Endpoints.swift` (one function per call), `HTTP.swift` (the transport seam), `APIError.swift` (MA-3), `PortfolioClient.swift`, `ServerAddress.swift` (HTTPS and development addresses) |
| `PortfolioKit/Sources/PortfolioAuth` | `PKCE.swift`, `OIDC.swift` (discovery, the authorization request, the redirect, token requests), `AuthSession.swift` (sign in, refresh, sign out), `KeyValueStore.swift` (Keychain, user defaults, memory) |
| `PortfolioKit/Sources/PortfolioStub` | The in-memory service (`StubService`, `StubFixtures`) and `ScriptedTransport` for tests |
| `PortfolioKit/Sources/PortfolioUI` | `AppModel.swift`, `ViewModels.swift`, `Presentation.swift` (labels and MA-3 messages), the screens, `Copy.swift` and the catalog |
| `PortfolioKit/Tests` | Swift Testing suites: requests, decoding, MA-3 rows, PKCE (RFC 7636 appendix B), OIDC, sessions, copy, view models |
| `PortfolioMobile.xcodeproj` | App and UI test targets; synchronized folders, so a new file in `PortfolioMobile/` or `PortfolioMobileUITests/` needs no project edit |
| `PortfolioMobileUITests` | `FlowTests` (list, plan, move, create, search, server and sign-in) and `AccessibilityAuditTests` |

## Gates

Run from this directory before handing a change over:

```sh
cd PortfolioKit && swift build --build-tests 2>&1 | grep -E "warning:|error:" ; swift test && cd ..
xcodebuild -project PortfolioMobile.xcodeproj -scheme PortfolioMobile \
  -destination 'platform=iOS Simulator,name=iPhone 18 Pro' -derivedDataPath build/DerivedData test
```

The first line must print no warning or error from `PortfolioKit` (Lily's own are not ours). There is no CI job for this
app yet (MA-7): the gates are run by hand on a Mac.

## Things that are easy to get wrong

- The task model is `PlanTask`, not `Task`: `Task` is Swift's concurrency type and is used everywhere in the UI.
- Dates are `CalendarDate` (`YYYY-MM-DD`), never `Date`, on the wire. Showing one uses a UTC Gregorian calendar so the day
  never moves; reading a date picker uses the picker's own calendar (`CalendarDate(pickedDate:)`).
- `PlanStatus` is a string or `{"Custom": …}`; an unknown unit string is `.other` and is sent back as a string, not as
  `Custom`. A task status the app does not know is `TaskStatus.other`.
- A `401` is never shown as an error: the view model hands it to `AppModel.handle`, which signs out.
- A failed move must leave the task's status alone: `PlanViewModel.move` changes the row only from the service's answer.
- The issuer check is exact string equality; do not normalise a trailing slash away.
- A development address on iOS is loopback only; `10.0.2.2` is the Android emulator's and is refused here (spec S-4).
- Launch arguments `-stub`, `-stub-server-screen` and `-stub-sign-in-screen` work only in Debug builds.
- Lily's `RecordListItem` and `Tag` join their spoken parts with line breaks; a UI test matches a label with `CONTAINS`.
- A wrapping `TextField(axis: .vertical)` is a text view to XCUITest; find it by identifier with `app.field(_:)`.
- An accessibility audit issue may be ignored only in `AccessibilityAuditTests.isIgnored`, with its reason; the list may
  only shrink.
- Do not modify `../vendor/`; it is fetched, and ignored by git.
