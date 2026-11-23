// The stub the UI tests run against answers in the service's shapes, through the real client.

import Foundation
import PortfolioAPI
import PortfolioStub
import Testing

@Suite struct StubServiceTests {
    let client = PortfolioClient(baseURL: URL(string: "http://127.0.0.1:5150")!, transport: StubTransport())

    @Test func listsTopLevelPlansAndChildren() async throws {
        #expect(try await client.listPlans().map(\.name) == ["Harbour redevelopment", "Library digitisation"])
        #expect(try await client.listPlans(parent: StubFixtures.harbourPID).map(\.name) == ["Quay wall survey"])
    }

    @Test func searches() async throws {
        #expect(try await client.searchPlans("quay").map(\.pid) == [StubFixtures.quayPID])
    }

    @Test func movesATask() async throws {
        let moved = try await client.moveTask(planPID: StubFixtures.harbourPID, taskPID: StubFixtures.surveyTaskPID, to: .done)
        #expect(moved.status == .done)
        let list = try await client.tasks(planPID: StubFixtures.harbourPID)
        #expect(list.tasks.first { $0.pid == StubFixtures.surveyTaskPID }?.status == .done)
    }

    @Test func createsAndRefuses() async throws {
        let ref = try await client.createPlan(NewPlan(name: "Ferry timetable"))
        #expect(try await client.plan(ref.pid).name == "Ferry timetable")
        await #expect(throws: APIError.conflict(detail: "a plan with this name already exists")) {
            try await client.createPlan(NewPlan(name: "ferry timetable"))
        }
        await #expect(throws: APIError.validation([Problem(field: "name", message: "name is required")])) {
            try await client.createPlan(NewPlan(name: "  "))
        }
        await #expect(throws: APIError.notFound) { try await client.plan("nope") }
    }
}
