// MA-6: decoding each answer, including a `{"Custom": …}` status and a field the app does not know.

import Foundation
import PortfolioAPI
import PortfolioStub
import Testing

@Suite struct DecodingTests {
    func client(_ body: String, status: Int = 200) -> (PortfolioClient, ScriptedTransport) {
        let transport = ScriptedTransport(status: status, json: body)
        return (PortfolioClient(baseURL: base, transport: transport), transport)
    }

    @Test func planRefsIgnoreUnknownFields() async throws {
        let (client, _) = client(#"[{"pid":"a","name":"Harbour","kind":"Program","new":1},{"pid":"b","name":"Quay"}]"#)
        let refs = try await client.listPlans()
        #expect(refs == [PlanRef(pid: "a", name: "Harbour"), PlanRef(pid: "b", name: "Quay")])
    }

    @Test func searchAnswersRefs() async throws {
        let (client, transport) = client(#"[{"pid":"a","name":"Harbour"}]"#)
        #expect(try await client.searchPlans("harb") == [PlanRef(pid: "a", name: "Harbour")])
        #expect(transport.requests.first?.path == "/api/plans/search")
    }

    @Test func aPlanWithACustomStatusAndAnUnknownField() async throws {
        let (client, _) = client(
            #"""
            {"name":"Quay wall survey","kind":"Project","status":{"Custom":"Awaiting funding"},
             "code":"Q-1","owner_org_name":"Port","lead_ref":"person:1","parent_ref":"p-0",
             "start_date":"2026-03-02","target_date":null,"goals":[{"title":"g"}],
             "identifiers":[{"scheme":{"Custom":"x"},"value":"1"}],"from_a_newer_service":{"a":[1,2]}}
            """#)
        let plan = try await client.plan("p")
        #expect(plan.name == "Quay wall survey")
        #expect(plan.kind == .project)
        #expect(plan.status == .custom("Awaiting funding"))
        #expect(plan.code == "Q-1")
        #expect(plan.ownerOrgName == "Port")
        #expect(plan.parentRef == "p-0")
        #expect(plan.startDate == CalendarDate(year: 2026, month: 3, day: 2))
        #expect(plan.targetDate == nil)
    }

    @Test func theMinimalPlanIsAName() async throws {
        let (client, _) = client(#"{"name":"Bare"}"#)
        let plan = try await client.plan("p")
        #expect(plan.name == "Bare")
        #expect(plan.status == nil && plan.kind == nil && plan.keywords.isEmpty)
    }

    @Test(arguments: [
        ("\"Proposed\"", PlanStatus.proposed), ("\"Active\"", .active), ("\"OnHold\"", .onHold),
        ("\"Completed\"", .completed), ("\"Cancelled\"", .cancelled), ("{\"Custom\":\"Paused\"}", .custom("Paused")),
        ("\"Archived\"", .other("Archived")),
    ])
    func everyPlanStatus(wire: String, status: PlanStatus) throws {
        let decoded = try JSONDecoder().decode(PlanStatus.self, from: Data(wire.utf8))
        #expect(decoded == status)
        #expect(String(decoding: try JSONEncoder().encode(decoded), as: UTF8.self) == wire)
    }

    @Test func anUnknownKindIsKeptByItsText() throws {
        #expect(try JSONDecoder().decode(PlanKind.self, from: Data("\"Initiative\"".utf8)) == .other("Initiative"))
        #expect(try JSONDecoder().decode(PlanKind.self, from: Data("\"Pathway\"".utf8)) == .pathway)
    }

    @Test func tasksWithAnUnknownFieldAndAWorkflowStatus() async throws {
        let (client, _) = client(
            #"""
            {"tasks":[
              {"pid":"t1","title":"Survey","description":null,"status":"todo","assignee_ref":null,"points":3,
               "due_date":"2026-12-04","blocked_days":null,"sprint_pid":null,"created_at":"2026-01-12T09:00:00Z",
               "status_changed_at":"2026-01-12T09:00:00Z","done_at":null,"flow_type":"feature","whatever":true},
              {"pid":"t2","title":"Wait","description":"d","status":"blocked","assignee_ref":"person:1","points":null,
               "due_date":null,"blocked_days":4},
              {"pid":"t3","title":"QA","status":"qa_signoff"}
            ],"counts":{"todo":1},"total":3,"truncated":false}
            """#)
        let list = try await client.tasks(planPID: "p")
        #expect(list.total == 3)
        #expect(list.truncated == false)
        #expect(list.tasks.map(\.status) == [.todo, .blocked, .other("qa_signoff")])
        #expect(list.tasks[0].points == 3)
        #expect(list.tasks[0].dueDate == CalendarDate(iso: "2026-12-04"))
        #expect(list.tasks[1].blockedDays == 4)
        #expect(list.tasks[1].assigneeRef == "person:1")
    }

    @Test func theMoveAnswersTheTask() async throws {
        let (client, transport) = client(#"{"pid":"t1","title":"Survey","status":"done","done_at":"2026-11-23T10:00:00Z"}"#)
        let task = try await client.moveTask(planPID: "p", taskPID: "t1", to: .done)
        #expect(task.status == .done)
        #expect(transport.requests.first?.method == .patch)
    }

    @Test func theCreateAnswersARef() async throws {
        let (client, transport) = client(#"{"pid":"new","name":"Harbour"}"#, status: 201)
        #expect(try await client.createPlan(NewPlan(name: "Harbour")) == PlanRef(pid: "new", name: "Harbour"))
        #expect(transport.requests.first?.method == .post)
    }

    @Test func theReadinessCheckNeedsNoBody() async throws {
        let (client, _) = client("", status: 200)
        try await client.checkReady()
    }

    @Test func aBodyOfTheWrongShapeIsAnInvalidResponse() async {
        let (client, _) = client(#"{"plans":[]}"#)
        await #expect(throws: APIError.unexpected(status: 200, detail: nil)) { try await client.listPlans() }
    }

    @Test func aMalformedDateIsAnInvalidResponse() async {
        let (client, _) = client(#"{"name":"x","start_date":"2026-02-30"}"#)
        await #expect(throws: APIError.unexpected(status: 200, detail: nil)) { try await client.plan("p") }
    }
}

@Suite struct CalendarDateTests {
    @Test(arguments: ["2026-01-12", "0001-01-01", "9999-12-31", "2024-02-29"])
    func roundTrips(iso: String) throws {
        #expect(try #require(CalendarDate(iso: iso)).iso == iso)
    }

    @Test(arguments: ["2026-1-12", "2026-02-30", "2025-02-29", "20260112", "2026-01-12T00:00:00Z", "", "２０２６-01-12"])
    func refuses(iso: String) {
        #expect(CalendarDate(iso: iso) == nil)
    }

    @Test func isShownInTheLocaleWithoutMovingTheDay() {
        let date = CalendarDate(iso: "2026-01-01")!
        // The first of January stays the first whatever the device's zone: the date is never an instant.
        #expect(date.formatted(locale: Locale(identifier: "en_US")) == "Jan 1, 2026")
        #expect(date.formatted(locale: Locale(identifier: "en_GB")) == "1 Jan 2026")
        let saved = NSTimeZone.default
        defer { NSTimeZone.default = saved }
        for zone in ["Pacific/Honolulu", "Pacific/Kiritimati", "UTC"] {
            NSTimeZone.default = TimeZone(identifier: zone)!
            #expect(date.formatted(locale: Locale(identifier: "en_US")) == "Jan 1, 2026", "in \(zone)")
        }
    }

    @Test func aPickedDateIsReadInThePickersCalendar() {
        var calendar = Calendar(identifier: .gregorian)
        calendar.timeZone = TimeZone(identifier: "Pacific/Kiritimati")!
        let lateEvening = calendar.date(from: DateComponents(year: 2026, month: 3, day: 1, hour: 23, minute: 30))!
        #expect(CalendarDate(pickedDate: lateEvening, calendar: calendar).iso == "2026-03-01")
    }
}
