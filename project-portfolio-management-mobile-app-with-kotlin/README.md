# Project Portfolio Management: Android app

A native Android app for the portfolio service, for someone away from a desk who needs to look a plan up and move a task
on. It is not a second operator UI. Kotlin, Jetpack Compose, and Lily Design System for Kotlin (`lily-compose`,
`lily-compose-themes`). The contract is [`../spec/mobile-apps.md`](../spec/mobile-apps.md); what is particular to Android is
in [`spec/index.md`](spec/index.md).

Status: **first slice, 0.1.0, unreleased.** Not in a store, not signed, no CI job (MA-7). See [CHANGELOG.md](CHANGELOG.md)
for what is built, and what is not built or not verified.

## What it does

| Screen | Service calls |
| --- | --- |
| Server: the service address and the sign-in settings, with a check of the address | `GET /_health/ready` |
| Sign in and sign out (OIDC authorization code with PKCE, in a Custom Tab) | the identity provider |
| Plans, with a search box | `GET /api/plans`, `GET /api/plans/search?q=` |
| Plan: its fields, its child plans, its tasks | `GET /api/plans/{pid}`, `GET /api/plans?parent={pid}`, `GET /api/plans/{pid}/tasks` |
| Move a task to another status | `PATCH /api/plans/{pid}/tasks/{tid}` |
| New plan: name, kind, status, code, start and target dates | `POST /api/plans` |

## Layout

| Module | What it is |
| --- | --- |
| `api` | Kotlin/JVM, no Android: the wire shapes (kotlinx-serialization, unknown fields ignored, `PlanStatus` as a string or `{"Custom": …}`), the calls over a small `HttpTransport` interface, the MA-3 error mapping, and `OkHttpTransport`. |
| `auth` | Kotlin/JVM, no Android: OIDC discovery, PKCE `S256`, the `state` check, the code exchange, refresh, the end-session URL, and `Session` over a `TokenStore` interface. |
| `app` | The Android app: Compose screens on Lily, Material 3 for native controls Lily does not ship, Custom Tabs, `KeystoreTokenStore`, `strings.xml` (English only). |

## Requirements

- A JDK 17 or later (`JAVA_HOME` must point at one; the build compiles to JVM 17 bytecode, as Lily does).
- The Android SDK with platform 37.2 (`sdkmanager "platforms;android-37.2"`), and **`ANDROID_HOME` set to it**. Lily's
  included build reads the SDK location from `ANDROID_HOME`, because `vendor/` has no `local.properties` and is not ours
  to write. A `local.properties` with `sdk.dir=…` in this directory serves this build only.
- Lily Design System for Kotlin, from the repository root: `scripts/fetch-lily.sh`. To use another Lily checkout, set
  `LILY_DESIGN_SYSTEM_DIR` (or the Gradle property `lilyDesignSystemDir`) to that Lily monorepo's root.

## Build, test, run

```bash
cd project-portfolio-management-mobile-app-with-kotlin
export ANDROID_HOME="$HOME/Library/Android/sdk"   # or wherever the SDK is

./gradlew assembleDebug test lint                 # the gate: debug APK, every JVM and Robolectric test, Android lint
./gradlew :api:test :auth:test                    # the JVM tests only
./gradlew :app:testDebugUnitTest                  # the Robolectric UI tests only

# The Accessibility Test Framework on a device or emulator (API 34 or later), at both text sizes:
adb shell settings put system font_scale 1.0 && ./gradlew :app:connectedDebugAndroidTest
adb shell settings put system font_scale 2.0 && ./gradlew :app:connectedDebugAndroidTest
adb shell settings put system font_scale 1.0

./gradlew :app:installDebug                       # install on a running emulator or device
```

To try it against a local service on the emulator: run the service with enforcement off (a development profile), enter
`http://10.0.2.2:5150` as the service address with no issuer, and choose **Continue without signing in**. To sign in,
register a public client with the identity provider, with the redirect URI
`io.github.projectportfoliomanagement.mobile:/oauth2redirect`, and enter its issuer and client ID.

No test needs a running service or identity provider: every one runs against a stub transport.
