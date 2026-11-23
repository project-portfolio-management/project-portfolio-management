# Android app: platform specification

The contract is [`../../spec/mobile-apps.md`](../../spec/mobile-apps.md) (MA-1 to MA-7). This file records only what is
particular to Android. Where the two disagree, `../../spec/mobile-apps.md` wins.

Status: first slice, 0.1.0, unreleased (2026-11-23).

## Platform

| Item | Value |
| --- | --- |
| Application id | `io.github.projectportfoliomanagement.mobile` (namespace `io.github.projectportfoliomanagement.mobile.app`) |
| SDK | `minSdk` 24 (Lily's), `targetSdk` 37, `compileSdk` 37.2; core library desugaring for `java.time` on API 24 and 25 |
| Toolchain | AGP 9.5.1, Kotlin 2.4.21, Compose BOM 2026.11.01, Gradle 9.9.1, JVM bytecode 17: Lily's, so the composite build resolves |
| Third-party libraries | AndroidX (activity, lifecycle, browser, core), Compose and Material 3, kotlinx-serialization, OkHttp. No AppAuth. |

## MA-1 Screens

One activity (`MainActivity`) and Compose screens. Which screen shows first follows the state: no settings, the server
screen; no tokens (and not "without signing in"), the sign-in screen; otherwise the plans list, with a stack that system
back pops. The plan list and the task list read the first page the service answers (the service caps a list at 100); the
`Link: rel="next"` pages are not followed. The move dialog offers the five built-in task statuses; a plan whose workflow
(FR-26) refuses one answers 422, which is shown. A plan search waits 300 ms after the last keystroke.

## MA-2 Sign-in

- Hand-rolled over the `HttpTransport` the API uses (module `auth`), not AppAuth: discovery, PKCE `S256` (32 random
  octets, base64url), `state` (16 octets, compared in constant time), the code exchange, refresh, and the end-session URL.
- Scope `openid offline_access` (for a refresh token). Not set by `../../spec/mobile-apps.md`.
- The authorization and end-session pages open in a Custom Tab (`androidx.browser`). The redirect comes back to
  `MainActivity` (`launchMode="singleTask"`) through an intent filter on the scheme
  `io.github.projectportfoliomanagement.mobile`. The redirect URI setting may change only what follows the scheme.
- The sign-in under way (verifier, state) is kept in memory only. A process killed while the browser is open loses it; the
  person is told the sign-in was interrupted and tries again.
- Tokens: `KeystoreTokenStore` encrypts them with an AES-256-GCM key generated in the Android Keystore, and keeps the
  ciphertext in a private SharedPreferences file. A value that does not decrypt is forgotten. Backup and device transfer
  exclude everything (`allowBackup="false"`, `backup_rules.xml`, `data_extraction_rules.xml`).
- A token within 60 seconds of expiry is refreshed before a request; a failed refresh signs out.
- Changing the service address, issuer or client ID forgets the tokens.
- The server settings (address, issuer, client ID, redirect URI) are not secrets and are kept in plain SharedPreferences.
- HTTPS: the server screen refuses an `http://` address other than a development one, and
  `network_security_config.xml` permits cleartext only to `localhost`, `127.0.0.1` and `10.0.2.2`.

## MA-3 Errors

`api/ApiError.kt` maps each answer; `app/ui/Common.kt` gives the words. A 422's problems come from the problem's `errors`
member when it is a map of field to messages, else from `detail` split on `"; "` (the plan validator's form); a problem
names a field when it starts with a field name followed by `:`, `[`, `" is "` or `" must "`. A 400, a 409 outside create,
another 4xx, or a success the app cannot read is `Unexpected`, shown as an answer the app did not expect. On a failed
move the problems are shown in a `LilyAlert` (there is no form to summarise).

## MA-4 Design system and accessibility

- `PpmTheme` wraps `LilyTheme(LilyTokens.light | dark)` by `isSystemInDarkTheme()`, and gives Material 3 a colour scheme
  from the same tokens and Lily its text styles from Material's type scale (sp, so the font size setting applies).
- Lily: `Field` (every text field and choice), `ErrorSummary` (server and new-plan forms), `SummaryList` (plan fields),
  `Tag` (plan and task status), `RecordList` / `RecordListItem` (child plans, tasks), `InsetText`, `LilyAlert`. `LilyCard`
  is not used yet.
- Every control carries `TapTarget` (48 by 48 dp on its own node).
- Checks: `checkScreenSemantics()` (a name on every control, 48 dp targets, no text cut off) in the Robolectric tests,
  including at font scale 2.0; the Accessibility Test Framework in `AccessibilityChecksTest` on a device, at font scale
  1.0 and 2.0. ATF finds nothing under Robolectric, so it cannot run in the JVM tests.

## MA-6 Tests

| Where | What |
| --- | --- |
| `api/src/test` | each call's request and decoding (custom status, unknown fields), each MA-3 row, addresses, OkHttp against MockWebServer |
| `auth/src/test` | RFC 7636 appendix B verifier and challenge, `state`, discovery and issuer check, exchange, refresh, end session, `Session` |
| `app/src/test` | Robolectric: plans and search, plan screen, move (success and refusal), create (success, 422, local check, 409), 401, 5xx with retry, sign-in round trip, wrong `state`, development address, server screen; large text; the screen checks themselves |
| `app/src/androidTest` | ATF and the screen checks over every screen, on a device or emulator |
