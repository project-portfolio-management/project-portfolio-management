// The Android app (../spec/mobile-apps.md): Compose screens on lily-compose, Material 3 for the native controls Lily does
// not ship, Custom Tabs for sign-in.
import org.jetbrains.kotlin.gradle.dsl.JvmTarget

plugins {
    alias(libs.plugins.android.application)
    alias(libs.plugins.kotlin.compose)
}

android {
    namespace = "io.github.projectportfoliomanagement.mobile.app"
    compileSdk { version = release(37) { minorApiLevel = 2 } }
    defaultConfig {
        applicationId = "io.github.projectportfoliomanagement.mobile"
        minSdk = 24
        targetSdk = 37
        versionCode = 1
        versionName = "0.1.0"
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
        // java.time (ISO calendar dates, MA-1) on API 24 and 25.
        isCoreLibraryDesugaringEnabled = true
    }
    defaultConfig { testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner" }
    testOptions { unitTests { isIncludeAndroidResources = true } }
    // The stub service and the screen checks, shared by the Robolectric tests (test) and the ATF tests on a device
    // (androidTest).
    sourceSets {
        named("test") { kotlin.directories.add("src/sharedTest/kotlin") }
        named("androidTest") { kotlin.directories.add("src/sharedTest/kotlin") }
    }
}

kotlin { compilerOptions { jvmTarget.set(JvmTarget.JVM_17) } }

dependencies {
    implementation(project(":api"))
    implementation(project(":auth"))
    implementation(libs.lily.compose)
    implementation(libs.lily.compose.themes)
    implementation(platform(libs.compose.bom))
    implementation(libs.compose.foundation)
    implementation(libs.compose.ui)
    implementation(libs.compose.material3)
    implementation(libs.activity.compose)
    implementation(libs.lifecycle.viewmodel.compose)
    implementation(libs.browser)
    implementation(libs.core.ktx)
    coreLibraryDesugaring(libs.desugar.jdk.libs)

    testImplementation(platform(libs.compose.bom))
    testImplementation(libs.compose.ui.test.junit4)
    testImplementation(libs.compose.ui.test.junit4.accessibility)
    testImplementation(libs.junit)
    testImplementation(libs.robolectric)
    testImplementation(libs.androidx.test.core)
    testImplementation(libs.androidx.test.ext.junit)
    androidTestImplementation(platform(libs.compose.bom))
    androidTestImplementation(libs.compose.ui.test.junit4)
    androidTestImplementation(libs.compose.ui.test.junit4.accessibility)
    androidTestImplementation(libs.androidx.test.runner)
    androidTestImplementation(libs.androidx.test.ext.junit)
    debugImplementation(platform(libs.compose.bom))
    debugImplementation(libs.compose.ui.test.manifest)
}
