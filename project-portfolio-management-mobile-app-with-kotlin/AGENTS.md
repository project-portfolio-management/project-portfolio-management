# AGENTS.md: Android app

Guide for AI coding agents working in `project-portfolio-management-mobile-app-with-kotlin`. Read
[`../spec/mobile-apps.md`](../spec/mobile-apps.md) (the contract, MA-1 to MA-7) and [`spec/index.md`](spec/index.md) (what
is particular to Android) first. Where this app and `../spec/mobile-apps.md` disagree, that document wins: fix the app, or
change the document on purpose in the same commit.

## Ground rules

1. **Spec first.** A behavioural change is one commit with a spec edit (here or in `../spec/mobile-apps.md`), a code edit
   and a test edit, as in the rest of the repository.
2. **Every word is the app's**, in `app/src/main/res/values/strings.xml`. English only; add no other locale and no
   machine-written translation (MA-4).
3. **Lily where Lily has the component** (`Field`, `ErrorSummary`, `SummaryList`, `Tag`, `LilyCard`, `InsetText`,
   `RecordList`, `LilyAlert`); Material 3 for buttons, dialogs, lists, the top bar and the date picker. Do not modify
   `../vendor/`; it is a pinned checkout.
4. **Tokens only in the protected store** (`KeystoreTokenStore`). Never log a token, never write one anywhere else.
5. **Say what is not built or not verified** in `CHANGELOG.md`. Do not weaken a test or a screen check to get green.

## Layout

```text
api/    Kotlin/JVM: Models.kt (wire shapes), ApiClient.kt (the calls), ApiError.kt (MA-3), Http.kt, OkHttpTransport.kt, Addresses.kt
auth/   Kotlin/JVM: Pkce.kt, Oidc.kt (discovery, authorize, exchange, refresh, end session), Session.kt (TokenStore, refresh)
app/    Android: AppGraph.kt, MainActivity.kt (redirect target), KeystoreTokenStore.kt, ServerSettings.kt, Browser.kt,
        ui/ (one file per screen with its view model; Common.kt, Forms.kt, Labels.kt, Theme.kt, PpmApp.kt navigation)
app/src/test/        Robolectric UI tests (AppUiTest, LargeTextTest, ScreenChecksTest)
app/src/androidTest/ AccessibilityChecksTest: ATF on a device or emulator
app/src/sharedTest/  RouteTransport (the stub service) and ScreenChecks, used by both
```

## Gates

```bash
export ANDROID_HOME="$HOME/Library/Android/sdk"
./gradlew assembleDebug test lint
# on an emulator or device, API 34+, at font_scale 1.0 and 2.0 (README.md):
./gradlew :app:connectedDebugAndroidTest
```

## Things that are easy to get wrong

- **Toolchain versions are Lily's.** AGP, Kotlin and the Compose BOM in `gradle/libs.versions.toml` must equal those in
  `../vendor/lily-design-system/lily-design-system-kotlin/gradle/libs.versions.toml`, and the Gradle wrapper its wrapper's
  version, or the composite build loads two Android Gradle plugins and fails. Bump them together with Lily's pin.
- **`ANDROID_HOME` must be set**: Lily's included build has no `local.properties`.
- **Lily is a composite build**, substituted for `io.github.lilydesignsystem:*:0.1.0` in `settings.gradle.kts`. That
  version is a placeholder; nothing is downloaded.
- **The Accessibility Test Framework does nothing under Robolectric.** `enableAccessibilityChecks()` passed a 10 dp
  unlabelled clickable there (checked 2026-11-23), so the Robolectric tests use `checkScreenSemantics()` instead (names,
  48 dp targets, text cut off), and ATF runs in `app/src/androidTest` on a device. `ScreenChecksTest` proves the checks
  fail on a bad screen; keep it.
- **Robolectric reports width overflow for text that fits**, so `checkScreenSemantics()` reads height overflow and
  ellipsis only.
- **Give every control `TapTarget`** (48 by 48 dp on the node itself). Material 3 draws buttons at 40 dp and pads the
  touch area invisibly; the semantics node does not include that padding.
- **The redirect scheme is fixed by the manifest** (`io.github.projectportfoliomanagement.mobile`). A redirect URI setting
  with another scheme never comes back; the server screen refuses one.
- **A `409` means a duplicate only on create** (`ApiClient.createPlan`); elsewhere it is `ApiError.Unexpected`.
- **Calls run on `AppGraph.io`**; a test passes `Dispatchers.Unconfined` so a call completes in step with the UI.
  Debounces belong in a `LaunchedEffect`, whose clock the Compose test drives, not in `viewModelScope`.
- **Dates are ISO calendar dates** (`YYYY-MM-DD`) shown with `localDate()`; never convert one through a time zone.
  Material's date picker works in UTC days, so `atZone(ZoneOffset.UTC)` is the conversion, not the device zone.
- **`HttpURLConnection` cannot send `PATCH`**, which is why the transport is OkHttp.
