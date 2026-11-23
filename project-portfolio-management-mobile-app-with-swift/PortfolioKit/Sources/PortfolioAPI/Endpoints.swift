// Each service call of the first slice as a pure function from its inputs to an `HTTPRequest` (spec/mobile-apps.md MA-1).
// Every request asks for JSON and representation version 1.0 (service spec §9).

import Foundation

public enum Endpoints {
    /// The headers every request carries.
    public static let commonHeaders = ["Accept": "application/json", "Accepts-version": "1.0"]

    /// `GET /_health/ready` (MA-1.1).
    public static func ready(base: URL) -> HTTPRequest {
        request(.get, base: base, path: ["_health", "ready"], token: nil)
    }

    /// `GET /api/plans`, or `GET /api/plans?parent={pid}` for one plan's children (MA-1.3, MA-1.4).
    public static func listPlans(base: URL, parent: String? = nil, token: String?) -> HTTPRequest {
        request(.get, base: base, path: ["api", "plans"], query: parent.map { [("parent", $0)] } ?? [], token: token)
    }

    /// `GET /api/plans/search?q=` (MA-1.3).
    public static func searchPlans(base: URL, query: String, token: String?) -> HTTPRequest {
        request(.get, base: base, path: ["api", "plans", "search"], query: [("q", query)], token: token)
    }

    /// `GET /api/plans/{pid}` (MA-1.4).
    public static func plan(base: URL, pid: String, token: String?) -> HTTPRequest {
        request(.get, base: base, path: ["api", "plans", pid], token: token)
    }

    /// `GET /api/plans/{pid}/tasks` (MA-1.4).
    public static func tasks(base: URL, planPID: String, token: String?) -> HTTPRequest {
        request(.get, base: base, path: ["api", "plans", planPID, "tasks"], token: token)
    }

    /// `PATCH /api/plans/{pid}/tasks/{tid}` with `{"status": …}` (MA-1.5).
    public static func moveTask(base: URL, planPID: String, taskPID: String, to status: TaskStatus, token: String?)
        -> HTTPRequest
    {
        request(
            .patch, base: base, path: ["api", "plans", planPID, "tasks", taskPID], token: token,
            body: try? encoder.encode(MoveTaskBody(status: status)))
    }

    /// `POST /api/plans` (MA-1.6).
    public static func createPlan(base: URL, plan: NewPlan, token: String?) -> HTTPRequest {
        request(.post, base: base, path: ["api", "plans"], token: token, body: try? encoder.encode(plan))
    }

    static let encoder: JSONEncoder = {
        let encoder = JSONEncoder()
        encoder.outputFormatting = [.sortedKeys]
        return encoder
    }()

    /// Characters left as they are in a path segment or a query value: RFC 3986 unreserved only, so `+`, `&`, `/` and `=`
    /// are always escaped.
    static let unreserved = CharacterSet(charactersIn: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~")

    static func encode(_ value: String) -> String {
        value.addingPercentEncoding(withAllowedCharacters: unreserved) ?? value
    }

    /// Joins `path` onto the base URL's own path (a service may sit under a prefix such as `/ppm`).
    static func request(
        _ method: HTTPMethod, base: URL, path: [String], query: [(String, String)] = [], token: String?,
        body: Data? = nil
    ) -> HTTPRequest {
        var components = URLComponents(url: base, resolvingAgainstBaseURL: false) ?? URLComponents()
        var prefix = components.percentEncodedPath
        while prefix.hasSuffix("/") { prefix.removeLast() }
        components.percentEncodedPath = prefix + "/" + path.map(encode).joined(separator: "/")
        components.percentEncodedQuery =
            query.isEmpty ? nil : query.map { "\(encode($0.0))=\(encode($0.1))" }.joined(separator: "&")
        components.fragment = nil
        var headers = commonHeaders
        if let token { headers["Authorization"] = "Bearer \(token)" }
        if body != nil { headers["Content-Type"] = "application/json" }
        return HTTPRequest(method: method, url: components.url ?? base, headers: headers, body: body)
    }
}
