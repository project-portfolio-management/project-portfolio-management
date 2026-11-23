// An in-memory stand-in for the portfolio service, answering the first slice's calls with the service's wire shapes. The
// UI tests launch the app with `-stub` to use it (spec/mobile-apps.md MA-6), and it lets the app be tried with no service.
// It is not the service: it checks only what the screens need (a blank name is `422`, a name already used is `409`, an
// unknown plan or task is `404`), and every answer carries a field the app does not know, to prove it is ignored.

import Foundation
import PortfolioAPI

public actor StubService {
    struct StoredPlan {
        var pid: String
        var json: [String: Any]
        var name: String { json["name"] as? String ?? "" }
        var parent: String? { json["parent_ref"] as? String }
    }

    private var plans: [StoredPlan]
    private var tasks: [String: [[String: Any]]]
    private var nextPlan = 100
    /// Every request the stub was sent, in order.
    public private(set) var received: [HTTPRequest] = []

    public init() {
        plans = StubFixtures.plans
        tasks = StubFixtures.tasks
    }

    public func handle(_ request: HTTPRequest) -> HTTPResponse {
        received.append(request)
        let parts = request.path.split(separator: "/").map { $0.removingPercentEncoding ?? String($0) }
        switch (request.method, parts.count) {
        case (.get, 2) where parts == ["_health", "ready"]:
            return json(200, ["status": "ok"])
        case (.get, 2) where parts == ["api", "plans"]:
            let parent = request.query["parent"]
            let rows = plans.filter { $0.parent == parent }.map(ref)
            return json(200, rows)
        case (.get, 3) where parts[0...2] == ["api", "plans", "search"]:
            let q = request.query["q", default: ""].trimmingCharacters(in: .whitespaces).lowercased()
            if q.isEmpty { return problem(400, "query parameter `q` is required") }
            return json(200, plans.filter { $0.name.lowercased().contains(q) }.map(ref))
        case (.get, 3) where parts[0...1] == ["api", "plans"]:
            guard let plan = plans.first(where: { $0.pid == parts[2] }) else { return notFound() }
            return json(200, plan.json)
        case (.get, 4) where parts[0...1] == ["api", "plans"] && parts[3] == "tasks":
            guard plans.contains(where: { $0.pid == parts[2] }) else { return notFound() }
            let rows = tasks[parts[2], default: []]
            return json(200, ["tasks": rows, "total": rows.count, "truncated": false, "counts": [:] as [String: Int]])
        case (.patch, 5) where parts[0...1] == ["api", "plans"] && parts[3] == "tasks":
            return move(plan: parts[2], task: parts[4], body: request.body)
        case (.post, 2) where parts == ["api", "plans"]:
            return create(body: request.body)
        default:
            return notFound()
        }
    }

    private func move(plan: String, task: String, body: Data?) -> HTTPResponse {
        guard var rows = tasks[plan], let index = rows.firstIndex(where: { $0["pid"] as? String == task }) else {
            return notFound()
        }
        guard let body, let object = try? JSONSerialization.jsonObject(with: body) as? [String: Any],
            let status = object["status"] as? String
        else { return problem(400, "the body must be {\"status\": …}") }
        guard TaskStatus.board.map(\.wireName).contains(status) else {
            return problem(422, "status must be one of [\"todo\", \"in_progress\", \"in_review\", \"done\", \"blocked\"]")
        }
        rows[index]["status"] = status
        rows[index]["blocked_days"] = status == "blocked" ? 0 : NSNull()
        tasks[plan] = rows
        return json(200, rows[index])
    }

    private func create(body: Data?) -> HTTPResponse {
        guard let body, var object = try? JSONSerialization.jsonObject(with: body) as? [String: Any] else {
            return problem(400, "the body must be a plan")
        }
        let name = (object["name"] as? String ?? "").trimmingCharacters(in: .whitespacesAndNewlines)
        if name.isEmpty { return problem(422, "name is required", code: "validation") }
        if plans.contains(where: { $0.name.lowercased() == name.lowercased() }) {
            return problem(409, "a plan with this name already exists", code: "duplicate")
        }
        nextPlan += 1
        let pid = "00000000-0000-4000-8000-\(String(format: "%012d", nextPlan))"
        object["name"] = name
        plans.append(StoredPlan(pid: pid, json: object))
        tasks[pid] = []
        return json(201, ["pid": pid, "name": name])
    }

    private func ref(_ plan: StoredPlan) -> [String: Any] {
        ["pid": plan.pid, "name": plan.name, "kind": plan.json["kind"] ?? NSNull()]
    }

    private func notFound() -> HTTPResponse { problem(404, "Resource was not found", code: "not_found") }

    private func json(_ status: Int, _ value: Any) -> HTTPResponse {
        let data = (try? JSONSerialization.data(withJSONObject: value, options: [.sortedKeys])) ?? Data()
        return HTTPResponse(status: status, headers: ["content-type": "application/json"], body: data)
    }

    private func problem(_ status: Int, _ detail: String, code: String? = nil) -> HTTPResponse {
        var body: [String: Any] = ["type": "about:blank", "title": "Error", "status": status, "detail": detail]
        if let code { body["code"] = code }
        let data = (try? JSONSerialization.data(withJSONObject: body, options: [.sortedKeys])) ?? Data()
        return HTTPResponse(status: status, headers: ["content-type": "application/problem+json"], body: data)
    }
}

/// A transport that answers from a `StubService`.
public struct StubTransport: HTTPTransport {
    public let service: StubService

    public init(service: StubService = StubService()) {
        self.service = service
    }

    public func send(_ request: HTTPRequest) async throws -> HTTPResponse {
        await service.handle(request)
    }
}

/// A transport that answers each request with a closure and records what it was sent; for unit tests.
public final class ScriptedTransport: HTTPTransport, @unchecked Sendable {
    public typealias Handler = @Sendable (HTTPRequest) async throws -> HTTPResponse
    private let lock = NSLock()
    private var log: [HTTPRequest] = []
    private let handler: Handler

    public init(_ handler: @escaping Handler) {
        self.handler = handler
    }

    /// Answers every request with `status` and `body`.
    public convenience init(status: Int, json body: String) {
        self.init { _ in HTTPResponse(status: status, body: Data(body.utf8)) }
    }

    /// Every request sent so far, in order.
    public var requests: [HTTPRequest] { lock.withLock { log } }

    public func send(_ request: HTTPRequest) async throws -> HTTPResponse {
        lock.withLock { log.append(request) }
        return try await handler(request)
    }
}
