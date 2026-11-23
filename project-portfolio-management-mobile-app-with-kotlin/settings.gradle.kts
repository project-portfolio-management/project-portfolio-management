// The Android app for the portfolio service (../spec/mobile-apps.md). Lily Design System for Kotlin is not published (MA-5):
// it is included as a composite build from the monorepo checkout that scripts/fetch-lily.sh makes in ../vendor/, or from
// the checkout named by the Gradle property `lilyDesignSystemDir` or the environment variable LILY_DESIGN_SYSTEM_DIR (the
// Lily monorepo's root, as the script's own comment has it; the Kotlin build is its lily-design-system-kotlin directory).
pluginManagement {
    repositories {
        google()
        mavenCentral()
        gradlePluginPortal()
    }
}
dependencyResolutionManagement {
    repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
    repositories {
        google()
        mavenCentral()
    }
}

rootProject.name = "project-portfolio-management-mobile-app-with-kotlin"
include(":api", ":auth", ":app")

val lilyRoot: String = providers.gradleProperty("lilyDesignSystemDir")
    .orElse(providers.environmentVariable("LILY_DESIGN_SYSTEM_DIR"))
    .getOrElse(settingsDir.resolve("../vendor/lily-design-system").path)
val lilyKotlin = file(lilyRoot).resolve("lily-design-system-kotlin")
require(lilyKotlin.resolve("settings.gradle.kts").isFile) {
    "Lily Design System for Kotlin is not at $lilyKotlin: run scripts/fetch-lily.sh from the repository root, " +
        "or set LILY_DESIGN_SYSTEM_DIR to a Lily monorepo checkout"
}
includeBuild(lilyKotlin) {
    dependencySubstitution {
        substitute(module("io.github.lilydesignsystem:lily-core")).using(project(":lily-core"))
        substitute(module("io.github.lilydesignsystem:lily-compose")).using(project(":lily-compose"))
        substitute(module("io.github.lilydesignsystem:lily-compose-themes")).using(project(":lily-compose-themes"))
    }
}
