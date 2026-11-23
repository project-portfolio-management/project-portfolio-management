# Mobile apps (MA)

Two native mobile apps call the portfolio service directly, as the operator UI's BFF does:

| App | Directory | Stack |
| --- | --- | --- |
| iOS and iPadOS | `project-portfolio-management-mobile-app-with-swift` | Swift 6, SwiftUI, Lily Design System for Swift (`LilyUI`, `LilyThemes`) |
| Android | `project-portfolio-management-mobile-app-with-kotlin` | Kotlin, Jetpack Compose, Lily Design System for Kotlin (`lily-compose`, `lily-compose-themes`) |

This document is the contract both apps build to. Each app's own `spec/index.md` records only what is particular to its
platform, and points here for the rest. Where an app and this document disagree, this document wins: fix the app, or
change this document on purpose in the same commit.

Status: **first slice, early, unreleased** (opened 2026-11-23). Neither app is in a store.

## MA-1 Scope of the first slice

The apps are for someone away from a desk who needs to look a plan up and move a task on. They are not a second operator
UI; the dashboards, governance, scheduling, cost, merge and administration screens stay in the web UI.

| Id | Screen | Service calls |
| --- | --- | --- |
| MA-1.1 | Server: the service's base URL, and the sign-in settings (below) | `GET /_health/ready` (to check the address) |
| MA-1.2 | Sign in and sign out | the identity provider (MA-2) |
| MA-1.3 | Plans: the active plans, with a search box | `GET /api/plans` · `GET /api/plans/search?q=` |
| MA-1.4 | Plan: one plan's fields, its child plans and its tasks | `GET /api/plans/{pid}` · `GET /api/plans?parent={pid}` · `GET /api/plans/{pid}/tasks` |
| MA-1.5 | Move a task to another status | `PATCH /api/plans/{pid}/tasks/{tid}` with `{"status": …}` |
| MA-1.6 | New plan: name (required), kind, status, code, start and target dates | `POST /api/plans` |

The wire shapes are the service's: `Plan` (the matcher type), `PlanRef` (`{pid, name}`), and the task row
(`pid`, `title`, `description`, `status` one of `todo`, `in_progress`, `in_review`, `done`, `blocked`, `assignee_ref`,
`points`, `due_date`, `blocked_days`, …). `PlanStatus` is a unit string or `{"Custom": "…"}`; an app reads both and
shows a custom status by its text. An app ignores a field it does not know, so a newer service does not break it.
Dates are ISO calendar dates (`YYYY-MM-DD`) and are shown in the device's locale, never converted through a time zone.

Every request sends `Accept: application/json` and `Accepts-version: 1.0` (service spec §9).

## MA-2 Sign-in

The service accepts a bearer token from any OIDC provider configured with `PROJECT_PORTFOLIO_MANAGEMENT_OIDC_ISSUER`
(service spec §9.6, WW-1.12). A native app is a public client, so it signs in by the OAuth 2.0 authorization-code flow
with PKCE (`S256`) in the system browser, as RFC 8252 requires: `ASWebAuthenticationSession` on iOS, a Custom Tab on
Android. It never shows a password field of its own, and never uses an embedded web view.

- The settings are the issuer URL, the client id, and the redirect URI (a custom scheme, by default
  `io.github.projectportfoliomanagement.mobile:/oauth2redirect`; the scheme is fixed by the app's manifest or
  `Info.plist`, so only the part after it can change). Endpoints come from the issuer's
  `/.well-known/openid-configuration`, whose `issuer` must equal the one set. The scope asked for is
  `openid offline_access`.
- The app checks `state`, sends the `code_verifier`, and keeps the access and refresh tokens only in the platform's
  protected store (the iOS Keychain, `kSecAttrAccessibleAfterFirstUnlockThisDeviceOnly`; on Android, values encrypted
  with an Android Keystore key). The server settings above are kept in the platform's ordinary preferences; no token,
  plan or task data is written anywhere else. A sign-in under way is held in memory only.
- An access token near expiry is refreshed with the refresh token; when refresh fails the app is signed out.
- Sign out forgets the tokens, and calls the provider's `end_session_endpoint` when it has one.
- A service whose enforcement is off (a development profile) needs no token. The app offers **Continue without signing
  in** only when the server address is a loopback address (`localhost`, `127.0.0.1`, `[::1]`) or the Android
  emulator's host (`10.0.2.2`).

HTTPS is required for the service and the issuer, except for those same development addresses.

## MA-3 Errors

| Answer | What the app does |
| --- | --- |
| `401` | Signs out and asks the person to sign in again |
| `403` | Says the person is not allowed to do this, and shows the rule the service names in the body |
| `404` | Says the plan or task is not there (it may have been deleted or merged) |
| `409` on create | Says a plan like this already exists |
| `422` on a form | Shows every problem the service lists, beside the field it names where it names one, with Lily's `ErrorSummary` |
| `422` on a move (a WIP limit or a workflow refusal) | Shows the service's reason in a `LilyAlert` |
| any other answer (`400`, another `4xx`, a success the app cannot read) | Says it got an answer it did not expect, with the status code |
| no network, timeout, `5xx` | Says the service could not be reached, with a way to try again; never shows a stack trace |

A failed move leaves the task in the status it had.

A plan's `422` today carries its problems joined by `"; "` in `detail`, with no per-field `errors` member, so an app
splits the text and places a problem beside a field only when the message starts with that field's name. An app uses
`errors` first when the service sends it. A per-field `errors` member in the service's `422` body is an open item.

## MA-4 Design system and accessibility

- Lily's components where Lily has one (`Field`, `ErrorSummary`, `SummaryList`, `Tag`, `Card`, `InsetText`,
  `RecordList`, `LilyAlert`), and the platform's native controls where Lily ships none (buttons, pickers, lists,
  navigation; Lily's own rule). The theme is Lily's light or dark, following the system.
- Every word is the app's, from the platform's string resources (a String Catalog on iOS, `strings.xml` on Android).
  The first slice ships English only; other locales are not built, and no machine-written translation is added.
- The text size follows the person's setting; nothing is clipped at the largest accessibility size.
- Tappable parts are at least 44 by 44 points (iOS) and 48 by 48 dp (Android). Status is never shown by colour alone.
- Each app runs the platform's automated accessibility audit over its screens in UI tests on a simulator, emulator or
  device (`performAccessibilityAudit` on iOS, Google's Accessibility Test Framework on Android). The audit does not run
  under Robolectric, so the Android JVM UI tests check the semantics tree themselves: every control has a name, is at
  least 48 dp, and no text is cut off, at text size 1.0 and 2.0.

## MA-5 Lily dependency

Lily's Swift package and Kotlin libraries are unreleased, and live in subdirectories of the Lily monorepo, so neither a
Swift package URL nor a Maven coordinate exists yet. `scripts/fetch-lily.sh` clones the Lily monorepo at the commit it
pins into `vendor/lily-design-system/` (ignored by git); the Swift app depends on
`vendor/lily-design-system/lily-design-system-swift` as a local package, and the Kotlin app includes
`vendor/lily-design-system/lily-design-system-kotlin` as a Gradle composite build. Set `LILY_DESIGN_SYSTEM_DIR` to the root of
another Lily monorepo checkout to use it instead. When Lily publishes, the apps move to the published package and the script goes.

## MA-6 Tests

- A unit test for each call's request (method, path, headers, body) and for decoding each answer, including a
  `{"Custom": …}` status and an unknown field, against a stub transport; no test needs a running service.
- A unit test for each row of MA-3.
- Lists show the first page the service answers; following `Link: rel=next` is not built.
- The PKCE pieces (verifier, `S256` challenge, `state` check) are unit-tested with the RFC 7636 appendix B vector.
- UI tests drive the plans list, the plan screen, a move, and a create against the stub, and run the accessibility
  audit (MA-4).

## MA-7 Not built in the first slice

Editing or deleting a plan, goals, issues, comments, the board as columns, sprints, notifications, offline storage,
push notifications, locales other than English, tablet layouts beyond the system's default adaptation (MA-8.1),
managed app configuration and managed distribution (MA-8.3), store packaging and signing, and a CI job on a macOS or Android-emulator runner. Each is a later item, not a promise.

## MA-8 Devices: managed and personal, phones and tablets

The apps MUST run on **phones and tablets** (iPhone and iPad; Android phones and Android tablets), and on **managed
devices** and on **personal devices** (BYOD), from one build of each app. Only configuration and storage differ between
them; the service calls, the sign-in and the screens do not.

| Id | Requirement |
| --- | --- |
| MA-8.1 | **Phones and tablets.** Each screen of MA-1 adapts to phone and tablet widths, in portrait and landscape, and in a split-screen or multi-window layout (iPad Split View and Slide Over; Android multi-window). Text, touch targets and focus order meet MA-4 at each size, at the largest text size the platform offers. |
| MA-8.2 | **A personal device needs no enrolment.** The app works on a device that is not enrolled in any device management. It asks for no device-administrator right, no device identifier and no permission beyond network access. It reads nothing of the person's own data (contacts, photos, location, files) and writes nothing outside MA-2's two stores. |
| MA-8.3 | **A managed device can configure the app.** On a device enrolled in a management service (Apple Business Manager with an MDM; Android Enterprise), the server settings of MA-2 (the service's base URL, the issuer URL, the client id and the redirect path) MUST be accepted from the platform's managed app configuration (iOS `com.apple.configuration.managed`; Android managed configurations). A value so supplied is shown as managed and cannot be edited; a value not supplied stays editable. The app never requires management to run (MA-8.2). Managed distribution (a custom app in Apple Business Manager; a private app in managed Google Play) is packaging, not a behaviour of the app. |
| MA-8.4 | **Nothing to wipe but a token.** The tokens and the server settings are the only things the app keeps (MA-2). They are excluded from device backups and from cloud sync on both platforms (on iOS `kSecAttrAccessibleAfterFirstUnlockThisDeviceOnly` and no iCloud Keychain sync; on Android backup rules that exclude the token store and the preferences). Removing the app, or a management service removing its data, leaves no plan or task data on the device. |
| MA-8.5 | **Ending access.** A lost, stolen or surrendered device is dealt with where the credential is issued: the organization revokes the person's session or refresh token at its identity provider, and the next refresh fails and signs the app out (MA-2). The app holds no credential of its own to revoke. |
| MA-8.6 | **The service does not tell the two apart.** The app sends no claim that a device is managed, and the service treats every call by its token alone (service spec §9.6). A deployment that wants to allow only managed or compliant devices does it in its identity provider's policy, which acts on the sign-in in the system browser (MA-2). Whether a given provider's device policy works with this sign-in is a test for that deployment, not something this document asserts. |
| MA-8.7 | **Tests.** The UI tests of MA-6 run on one phone size and one tablet size for each app, in both orientations, and run the accessibility audit (MA-4) on each. A unit test covers MA-8.3's rule (a managed value wins and is not editable; an absent one stays editable). |

### MA-8 status

Nothing in MA-8 is verified on a real managed device or a real management service. Found in the apps' sources on
2026-11-23: the Android manifest asks only for the network permission and sets `allowBackup="false"` with backup and
data-extraction rule files (their contents were not read, so MA-8.4 is not confirmed for the preferences); the iOS token
store uses a device-only Keychain class (`kSecAttrAccessible`) and sets no synchronizable flag. No tablet-specific layout
code was found, so a tablet gets the system's default adaptation. **Not built:** the tablet layouts and the split-screen
checks of MA-8.1, managed app configuration (MA-8.3), and the tests of MA-8.7. Each is a later item, not a promise (MA-7).
