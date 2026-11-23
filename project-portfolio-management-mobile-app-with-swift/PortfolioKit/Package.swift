// swift-tools-version: 6.0
// PortfolioKit: the iOS and iPadOS app's code, as a package so `swift test` runs the API, sign-in and view-model tests on
// macOS without a simulator. The app project (`../PortfolioMobile.xcodeproj`) is a thin shell over `PortfolioUI`.
//
// Lily Design System for Swift is unreleased (spec/mobile-apps.md MA-5): it is a local package, by default the copy that
// `scripts/fetch-lily.sh` puts in the repository's `vendor/`, or the checkout named by `LILY_DESIGN_SYSTEM_DIR`.
import PackageDescription

let lilyPath = Context.environment["LILY_DESIGN_SYSTEM_DIR"].map { "\($0)/lily-design-system-swift" }
    ?? "../../vendor/lily-design-system/lily-design-system-swift"

let package = Package(
    name: "PortfolioKit",
    defaultLocalization: "en",
    platforms: [.iOS(.v18), .macOS(.v15)],
    products: [
        .library(name: "PortfolioAPI", targets: ["PortfolioAPI"]),
        .library(name: "PortfolioAuth", targets: ["PortfolioAuth"]),
        .library(name: "PortfolioStub", targets: ["PortfolioStub"]),
        .library(name: "PortfolioUI", targets: ["PortfolioUI"]),
    ],
    dependencies: [
        .package(name: "LilyDesignSystem", path: lilyPath),
    ],
    targets: [
        .target(name: "PortfolioAPI"),
        .target(name: "PortfolioAuth", dependencies: ["PortfolioAPI"]),
        .target(name: "PortfolioStub", dependencies: ["PortfolioAPI"]),
        .target(
            name: "PortfolioUI",
            dependencies: [
                "PortfolioAPI", "PortfolioAuth", "PortfolioStub",
                .product(name: "LilyUI", package: "LilyDesignSystem"),
                .product(name: "LilyThemes", package: "LilyDesignSystem"),
            ],
            resources: [.process("Resources")]),
        .testTarget(name: "PortfolioAPITests", dependencies: ["PortfolioAPI", "PortfolioStub"]),
        .testTarget(name: "PortfolioAuthTests", dependencies: ["PortfolioAuth", "PortfolioAPI"]),
        .testTarget(name: "PortfolioUITests", dependencies: ["PortfolioUI", "PortfolioAPI", "PortfolioAuth", "PortfolioStub"]),
    ],
    swiftLanguageModes: [.v6]
)
