# Changelog

## 0.1.0 (unreleased)

The first slice of the Android app, to `../spec/mobile-apps.md` (opened 2026-11-23).

Built:

- Gradle build (wrapper 9.9.1) with three modules: `api` and `auth` (Kotlin/JVM) and `app` (Android, Compose). Lily
  Design System for Kotlin is a composite build of `../vendor/lily-design-system/lily-design-system-kotlin`, or of the
  checkout `LILY_DESIGN_SYSTEM_DIR` names (MA-5).
- MA-1: the server, sign-in, plans (with search), plan (fields, child plans, tasks), move and new-plan screens, sending
  `Accept: application/json` and `Accepts-version: 1.0` on every request. `PlanStatus` reads a string or
  `{"Custom": …}`; unknown fields are ignored.
- MA-2: OIDC authorization code with PKCE `S256` in a Custom Tab, hand-rolled (no AppAuth); discovery with the issuer
  check; `state` check; refresh near expiry; sign-out with the provider's end-session page when it has one; tokens
  encrypted with an Android Keystore key; "Continue without signing in" only for a development address; HTTPS required
  otherwise.
- MA-3: every row, with `ErrorSummary` and field errors for a 422 on the new-plan form; a failed move leaves the task as
  it was.
- MA-4: Lily components and Material 3 native controls, Lily light or dark with the system, English `strings.xml`, 48 dp
  targets.
- MA-6: 25 `api` tests, 16 `auth` tests and 17 Robolectric tests, none needing a service; an Accessibility Test Framework
  test (2 tests) for a device.

Verified on 2026-11-23: `./gradlew assembleDebug test lint` passes (58 tests, lint finds no issues);
`:app:connectedDebugAndroidTest` passes on the Pixel 10 API 37 emulator at font scale 1.0 and 2.0.

Not built:

- Everything MA-7 lists: editing or deleting a plan, goals, issues, comments, the board as columns, sprints,
  notifications, offline storage, push, locales other than English, tablet layouts, store packaging and signing, CI.
- Following `Link: rel="next"`: a list shows the first page the service answers only.
- A move to a status a plan's own workflow declares beyond the five built-in ones.
- `LilyCard` is not used.

Not verified:

- Sign-in against a real identity provider, the Custom Tab, and the redirect intent filter on a device: the round trip is
  tested only with a stub transport and a redirect set directly.
- `KeystoreTokenStore`: no test (Robolectric has no Android Keystore), and not exercised on the emulator.
- `OkHttpTransport` against the real service: tested only against MockWebServer.
- A release build: `assembleRelease` builds an unsigned APK, with R8 off (no shrinking rules are written); it has not
  been installed or run.
- The date picker dialog: built, not driven by any test.
- TalkBack by hand, and the dark theme by eye: not done; the automated checks ran in the light theme only.
- Cleartext to `[::1]`: the app accepts it as a development address, but the network security config does not list it,
  so such a request would fail.
