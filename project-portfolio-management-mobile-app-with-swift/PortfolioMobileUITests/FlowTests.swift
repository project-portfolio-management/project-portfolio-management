// MA-6: the plans list, the plan screen, a move and a create, driven through the app against the stub service; and the
// server and sign-in screens as far as a development address goes (the browser sign-in needs a real provider).

import XCTest

final class FlowTests: XCTestCase {
    override func setUp() {
        continueAfterFailure = false
    }

    @MainActor func testListPlanMoveAndCreate() throws {
        let app = launchStub()
        XCTAssertTrue(app.navigationBars["Plans"].waitForExistence(timeout: 10))

        // MA-1.3: the active plans.
        let harbour = app.buttons["plan-row-\(Fixture.harbour)"]
        XCTAssertTrue(harbour.waitForExistence(timeout: 10))
        XCTAssertTrue(app.buttons["plan-row-00000000-0000-4000-8000-000000000003"].exists)
        XCTAssertFalse(app.buttons["plan-row-\(Fixture.quay)"].exists, "a child plan is not listed at the top")
        harbour.tap()

        // MA-1.4: the plan's fields, children and tasks.
        XCTAssertTrue(app.staticTexts["HRB-2026"].waitForExistence(timeout: 10))
        XCTAssertTrue(app.labelled(containing: "Active").firstMatch.exists)
        XCTAssertTrue(app.buttons["child-row-\(Fixture.quay)"].exists)
        let task = app.buttons["task-row-\(Fixture.surveyTask)"]
        reveal(task, in: app)
        XCTAssertTrue(task.label.contains("Status: To do"), task.label)

        // MA-1.5: move it to In review.
        task.tap()
        let inReview = app.buttons["move-in_review"]
        XCTAssertTrue(inReview.waitForExistence(timeout: 10))
        XCTAssertTrue(app.buttons["move-todo"].label.contains("Current status"))
        inReview.tap()
        waitUntilGone(inReview)
        XCTAssertTrue(task.waitForExistence(timeout: 10))
        XCTAssertTrue(task.label.contains("Status: In review"), task.label)

        // The child plan, with its custom status shown by its text.
        let child = app.buttons["child-row-\(Fixture.quay)"]
        reveal(child, in: app)
        app.swipeDown(velocity: .fast)
        app.swipeDown(velocity: .fast)
        child.tap()
        XCTAssertTrue(app.navigationBars["Quay wall survey"].waitForExistence(timeout: 10))
        XCTAssertTrue(app.labelled(containing: "Awaiting funding").firstMatch.waitForExistence(timeout: 10))
        XCTAssertTrue(app.staticTexts["This plan has no tasks."].exists)
        app.navigationBars.buttons.firstMatch.tap()
        XCTAssertTrue(app.navigationBars["Harbour redevelopment"].waitForExistence(timeout: 10))
        app.navigationBars.buttons.firstMatch.tap()

        // MA-1.6: a create with no name is refused with the service's problem, then a create that works.
        let new = app.buttons["new-plan"]
        XCTAssertTrue(new.waitForExistence(timeout: 10))
        new.tap()
        let save = app.buttons["new-plan-save"]
        XCTAssertTrue(save.waitForExistence(timeout: 10))
        save.tap()
        XCTAssertTrue(app.staticTexts["There is a problem"].waitForExistence(timeout: 10))
        XCTAssertTrue(app.buttons["name is required"].exists)
        let name = app.field("new-plan-name")
        name.tap()
        name.typeText("Ferry timetable")
        save.tap()
        let opened = app.navigationBars["Ferry timetable"].waitForExistence(timeout: 10)
        if !opened { print(app.debugDescription) }
        XCTAssertTrue(opened)
        app.navigationBars.buttons.firstMatch.tap()
        XCTAssertTrue(app.buttons.matching(NSPredicate(format: "label == 'Ferry timetable'")).firstMatch
            .waitForExistence(timeout: 10))

        // MA-3: a second plan of the same name is a 409.
        new.tap()
        XCTAssertTrue(name.waitForExistence(timeout: 10))
        name.tap()
        name.typeText("Ferry timetable")
        save.tap()
        XCTAssertTrue(app.labelled(containing: "A plan like this already exists.").firstMatch.waitForExistence(timeout: 10))
        app.buttons["new-plan-cancel"].tap()
    }

    @MainActor func testSearch() throws {
        let app = launchStub()
        XCTAssertTrue(app.buttons["plan-row-\(Fixture.harbour)"].waitForExistence(timeout: 10))
        let search = app.searchFields.firstMatch
        if !search.exists { app.swipeDown() }
        XCTAssertTrue(search.waitForExistence(timeout: 10))
        search.tap()
        search.typeText("quay")
        XCTAssertTrue(app.buttons["plan-row-\(Fixture.quay)"].waitForExistence(timeout: 10))
        waitUntilGone(app.buttons["plan-row-\(Fixture.harbour)"])
        search.typeText("zzz")
        XCTAssertTrue(app.staticTexts["No plans match “quayzzz”."].waitForExistence(timeout: 10))
    }

    @MainActor func testTheServerScreenRefusesPlainHTTPToAnotherHost() throws {
        let app = launchStub(start: "-stub-server-screen")
        XCTAssertTrue(app.navigationBars["Server"].waitForExistence(timeout: 10))
        let address = app.field("server-service-url")
        address.tap()
        address.typeText("http://portfolio.example.org")
        let save = app.buttons["server-save"]
        reveal(save, in: app)
        save.tap()
        let summary = app.staticTexts["There is a problem"]
        revealUpwards(summary, in: app)
        XCTAssertTrue(summary.exists)
        XCTAssertTrue(app.buttons.matching(NSPredicate(format: "label BEGINSWITH 'Use an https:// address'")).firstMatch.exists)
    }

    @MainActor func testServerScreenToPlansWithoutSigningIn() throws {
        let app = launchStub(start: "-stub-server-screen")
        XCTAssertTrue(app.navigationBars["Server"].waitForExistence(timeout: 10))
        let address = app.field("server-service-url")
        address.tap()
        address.typeText("http://127.0.0.1:5150")
        app.buttons["server-check"].tap()
        XCTAssertTrue(app.labelled(containing: "The service answered and is ready.").firstMatch.waitForExistence(timeout: 10))
        let save = app.buttons["server-save"]
        reveal(save, in: app)
        save.tap()

        XCTAssertTrue(app.navigationBars["Sign in"].waitForExistence(timeout: 10))
        XCTAssertTrue(app.buttons["sign-in"].exists)
        app.buttons["continue-without-sign-in"].tap()
        XCTAssertTrue(app.buttons["plan-row-\(Fixture.harbour)"].waitForExistence(timeout: 10))
    }
}
