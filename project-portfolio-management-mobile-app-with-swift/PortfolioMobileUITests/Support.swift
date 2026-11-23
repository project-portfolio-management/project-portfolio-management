// Launching the app against the stub service (`-stub`, Debug builds only), and the fixture identifiers the tests use.

import XCTest

enum Fixture {
    static let harbour = "00000000-0000-4000-8000-000000000001"
    static let quay = "00000000-0000-4000-8000-000000000002"
    static let surveyTask = "10000000-0000-4000-8000-000000000001"
    static let tenderTask = "10000000-0000-4000-8000-000000000002"
}

extension XCTestCase {
    /// The app on the stub service: at the plans, or at the screen `start` names (`-stub-server-screen`,
    /// `-stub-sign-in-screen`), at `textSize` when given.
    @MainActor func launchStub(start: String? = nil, textSize: String? = nil) -> XCUIApplication {
        let app = XCUIApplication()
        app.launchArguments = ["-stub"]
        if let start { app.launchArguments.append(start) }
        if let textSize { app.launchArguments += ["-UIPreferredContentSizeCategoryName", textSize] }
        app.launch()
        return app
    }

    /// Scrolls `element` into reach, at most `tries` swipes.
    @MainActor func reveal(_ element: XCUIElement, in app: XCUIApplication, tries: Int = 12) {
        var left = tries
        while left > 0 && !(element.exists && element.isHittable) {
            app.swipeUp(velocity: .slow)
            left -= 1
        }
    }

    /// Scrolls back up until `element` is in reach.
    @MainActor func revealUpwards(_ element: XCUIElement, in app: XCUIApplication, tries: Int = 12) {
        var left = tries
        while left > 0 && !(element.exists && element.isHittable) {
            app.swipeDown(velocity: .slow)
            left -= 1
        }
    }

    @MainActor func waitUntilGone(_ element: XCUIElement, timeout: TimeInterval = 10) {
        expectation(for: NSPredicate(format: "exists == false"), evaluatedWith: element)
        waitForExpectations(timeout: timeout)
    }
}

extension XCUIApplication {
    /// Any element whose accessibility label contains `text` (Lily joins spoken parts with line breaks).
    /// The text control with this identifier: a one-line field is a text field, a wrapping one a text view.
    func field(_ identifier: String) -> XCUIElement {
        descendants(matching: .any).matching(identifier: identifier).firstMatch
    }

    func labelled(containing text: String) -> XCUIElementQuery {
        descendants(matching: .any).matching(NSPredicate(format: "label CONTAINS %@", text))
    }
}
