// MA-4: the platform's accessibility audit (XCUIApplication.performAccessibilityAudit) over every screen of the first
// slice, at the default text size and the largest accessibility size. An ignored issue is listed in `isIgnored` with its
// reason; the list may only shrink.

import XCTest

final class AccessibilityAuditTests: XCTestCase {
    static let sizes: [String?] = [nil, "UICTContentSizeCategoryAccessibilityXXXL"]

    override func setUp() {
        continueAfterFailure = true
    }

    /// Issues the audit reports that are not the app's, each with its reason.
    @MainActor static func isIgnored(_ issue: XCUIAccessibilityAuditIssue, in app: XCUIApplication) -> Bool {
        guard issue.auditType == .contrast else { return false }
        // A contrast issue with no element: rows fading under the system bars' scroll-edge effect (as in Lily's own
        // examples). The app's colour pairs are Lily's, checked at 4.5:1 by LilyThemes' token tests.
        guard let element = issue.element else { return true }
        // The system search field's own placeholder, drawn by iOS in its own grey.
        if element.elementType == .searchField { return true }
        return false
    }

    /// Runs the audit and fails once per issue not ignored, naming the element, so a failure says where to look.
    @MainActor private func audit(_ app: XCUIApplication, _ screen: String, file: StaticString = #filePath, line: UInt = #line) {
        var found: [String] = []
        do {
            try app.performAccessibilityAudit { issue in
                if Self.isIgnored(issue, in: app) { return true }
                let element = issue.element.map {
                    "\($0.elementType.rawValue) '\($0.label)' id '\($0.identifier)' at \($0.frame)"
                } ?? "no element"
                found.append("\(issue.compactDescription): \(element). \(issue.detailedDescription)")
                return true
            }
        } catch {
            found.append("the audit did not run: \(error)")
        }
        for issue in found { XCTFail("\(screen): \(issue)", file: file, line: line) }
    }

    @MainActor func testThePlansList() {
        for size in Self.sizes {
            let app = launchStub(textSize: size)
            XCTAssertTrue(app.buttons["plan-row-\(Fixture.harbour)"].waitForExistence(timeout: 10))
            audit(app, "plans at \(size ?? "default")")
            app.terminate()
        }
    }

    @MainActor func testThePlanScreenAndTheMoveSheet() {
        for size in Self.sizes {
            let app = launchStub(textSize: size)
            let row = app.buttons["plan-row-\(Fixture.harbour)"]
            XCTAssertTrue(row.waitForExistence(timeout: 10))
            row.tap()
            XCTAssertTrue(app.staticTexts["HRB-2026"].waitForExistence(timeout: 10))
            audit(app, "plan at \(size ?? "default")")
            let task = app.buttons["task-row-\(Fixture.tenderTask)"]
            reveal(task, in: app)
            audit(app, "plan, scrolled, at \(size ?? "default")")
            task.tap()
            XCTAssertTrue(app.buttons["move-done"].waitForExistence(timeout: 10))
            audit(app, "move sheet at \(size ?? "default")")
            app.terminate()
        }
    }

    @MainActor func testTheNewPlanFormWithItsErrors() {
        for size in Self.sizes {
            let app = launchStub(textSize: size)
            let new = app.buttons["new-plan"]
            XCTAssertTrue(new.waitForExistence(timeout: 10))
            new.tap()
            XCTAssertTrue(app.navigationBars["New plan"].waitForExistence(timeout: 10))
            audit(app, "new plan at \(size ?? "default")")
            let save = app.buttons["new-plan-save"]
            reveal(save, in: app)
            audit(app, "new plan, scrolled, at \(size ?? "default")")
            save.tap()
            // The form scrolls the summary into view itself (a swipe down here would close the sheet).
            let shown = app.labelled(containing: "There is a problem").firstMatch.waitForExistence(timeout: 10)
            XCTAssertTrue(shown)
            audit(app, "new plan with errors at \(size ?? "default")")
            app.terminate()
        }
    }

    @MainActor func testTheServerAndSignInScreens() {
        for size in Self.sizes {
            let app = launchStub(start: "-stub-server-screen", textSize: size)
            XCTAssertTrue(app.navigationBars["Server"].waitForExistence(timeout: 10))
            audit(app, "server at \(size ?? "default")")
            let save = app.buttons["server-save"]
            reveal(save, in: app)
            audit(app, "server, scrolled, at \(size ?? "default")")
            app.terminate()

            let signIn = launchStub(start: "-stub-sign-in-screen", textSize: size)
            XCTAssertTrue(signIn.navigationBars["Sign in"].waitForExistence(timeout: 10))
            audit(signIn, "sign in at \(size ?? "default")")
            let change = signIn.buttons["change-server"]
            reveal(change, in: signIn)
            audit(signIn, "sign in, scrolled, at \(size ?? "default")")
            signIn.terminate()
        }
    }
}
