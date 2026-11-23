// OIDC sign-in for a public client: discovery, PKCE S256, state, code exchange, refresh, end session (MA-2). JVM only; the
// Android Keystore token store is in :app.
import org.jetbrains.kotlin.gradle.dsl.JvmTarget

plugins {
    alias(libs.plugins.kotlin.jvm)
    alias(libs.plugins.kotlin.serialization)
}

java {
    sourceCompatibility = JavaVersion.VERSION_17
    targetCompatibility = JavaVersion.VERSION_17
}

kotlin {
    explicitApi()
    compilerOptions { jvmTarget.set(JvmTarget.JVM_17) }
}

dependencies {
    api(project(":api"))
    testImplementation(libs.junit)
}
