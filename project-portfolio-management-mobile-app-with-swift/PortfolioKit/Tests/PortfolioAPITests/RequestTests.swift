// MA-6: each call's request (method, path, headers, body), against a stub transport.

import Foundation
import PortfolioAPI
import PortfolioStub
import Testing

let base = URL(string: "https://portfolio.example.org")!

func json(_ data: Data?) throws -> [String: Any] {
    let data = try #require(data)
    return try #require(JSONSerialization.jsonObject(with: data) as? [String: Any])
}

@Suite struct RequestTests {
    func expectCommonHeaders(_ request: HTTPRequest, token: String?) {
        #expect(request.headers["Accept"] == "application/json")
        #expect(request.headers["Accepts-version"] == "1.0")
        #expect(request.headers["Authorization"] == token.map { "Bearer \($0)" })
    }

    @Test func readiness() {
        let request = Endpoints.ready(base: base)
        #expect(request.method == .get)
        #expect(request.url.absoluteString == "https://portfolio.example.org/_health/ready")
        #expect(request.body == nil)
        expectCommonHeaders(request, token: nil)
    }

    @Test func listPlans() {
        let request = Endpoints.listPlans(base: base, token: "t0k")
        #expect(request.method == .get)
        #expect(request.url.absoluteString == "https://portfolio.example.org/api/plans")
        #expect(request.body == nil)
        expectCommonHeaders(request, token: "t0k")
    }

    @Test func childPlans() {
        let request = Endpoints.listPlans(base: base, parent: "p-1", token: nil)
        #expect(request.path == "/api/plans")
        #expect(request.query == ["parent": "p-1"])
        expectCommonHeaders(request, token: nil)
    }

    @Test func searchEscapesTheQuery() {
        let request = Endpoints.searchPlans(base: base, query: "a+b & c/d=é", token: "t")
        #expect(request.method == .get)
        #expect(request.path == "/api/plans/search")
        #expect(request.url.query(percentEncoded: true) == "q=a%2Bb%20%26%20c%2Fd%3D%C3%A9")
        #expect(request.query == ["q": "a+b & c/d=é"])
        expectCommonHeaders(request, token: "t")
    }

    @Test func onePlan() {
        let request = Endpoints.plan(base: base, pid: "00000000-0000-4000-8000-000000000001", token: "t")
        #expect(request.method == .get)
        #expect(request.path == "/api/plans/00000000-0000-4000-8000-000000000001")
        expectCommonHeaders(request, token: "t")
    }

    @Test func aPIDIsOnePathSegment() {
        let request = Endpoints.plan(base: base, pid: "../admin?x", token: nil)
        #expect(request.path == "/api/plans/..%2Fadmin%3Fx")
        #expect(request.url.query == nil)
    }

    @Test func tasks() {
        let request = Endpoints.tasks(base: base, planPID: "p", token: "t")
        #expect(request.method == .get)
        #expect(request.path == "/api/plans/p/tasks")
        expectCommonHeaders(request, token: "t")
    }

    @Test func move() throws {
        let request = Endpoints.moveTask(base: base, planPID: "p", taskPID: "t1", to: .inReview, token: "t")
        #expect(request.method == .patch)
        #expect(request.path == "/api/plans/p/tasks/t1")
        #expect(request.headers["Content-Type"] == "application/json")
        expectCommonHeaders(request, token: "t")
        let body = try json(request.body)
        #expect(body.count == 1)
        #expect(body["status"] as? String == "in_review")
    }

    @Test func createSendsOnlyTheFieldsSet() throws {
        let request = Endpoints.createPlan(base: base, plan: NewPlan(name: "Harbour"), token: "t")
        #expect(request.method == .post)
        #expect(request.path == "/api/plans")
        #expect(request.headers["Content-Type"] == "application/json")
        expectCommonHeaders(request, token: "t")
        #expect(try json(request.body) as NSDictionary == ["name": "Harbour"] as NSDictionary)
    }

    @Test func createWithEveryField() throws {
        let plan = NewPlan(
            name: "Harbour", kind: .program, status: .onHold, code: "HRB-1",
            startDate: CalendarDate(iso: "2026-01-12"), targetDate: CalendarDate(iso: "2027-06-30"))
        let body = try json(Endpoints.createPlan(base: base, plan: plan, token: nil).body)
        #expect(
            body as NSDictionary
                == [
                    "name": "Harbour", "kind": "Program", "status": "OnHold", "code": "HRB-1",
                    "start_date": "2026-01-12", "target_date": "2027-06-30",
                ] as NSDictionary)
    }

    @Test func aCustomStatusIsSentAsAnObject() throws {
        let body = try json(Endpoints.createPlan(base: base, plan: NewPlan(name: "x", status: .custom("Paused")), token: nil).body)
        #expect(body["status"] as? [String: String] == ["Custom": "Paused"])
    }

    @Test func aServiceUnderAPathPrefix() {
        let prefixed = URL(string: "https://example.org/ppm/")!
        #expect(Endpoints.listPlans(base: prefixed, token: nil).url.absoluteString == "https://example.org/ppm/api/plans")
        #expect(Endpoints.ready(base: prefixed).url.absoluteString == "https://example.org/ppm/_health/ready")
    }

    @Test func theClientSendsTheTokenItIsGiven() async throws {
        let transport = ScriptedTransport(status: 200, json: "[]")
        let client = PortfolioClient(baseURL: base, transport: transport) { "abc" }
        _ = try await client.listPlans()
        #expect(transport.requests.first?.headers["Authorization"] == "Bearer abc")
    }

    @Test func aSignedOutTokenProviderIsA401() async {
        struct SignedOut: Error {}
        let transport = ScriptedTransport(status: 200, json: "[]")
        let client = PortfolioClient(baseURL: base, transport: transport) { throw SignedOut() }
        await #expect(throws: APIError.unauthorized) { try await client.listPlans() }
        #expect(transport.requests.isEmpty)
    }
}
