// The service calls of the first slice (spec/mobile-apps.md MA-1), each one request and one decode. Every failure is an
// `APIError` (MA-3), never a raw transport or decoding error.

import Foundation

/// Supplies the bearer token for a call: nil when the service needs none (a development profile), or throws when the person
/// is signed out (the call then fails as `401` would).
public typealias TokenProvider = @Sendable () async throws -> String?

public struct PortfolioClient: Sendable {
    public let baseURL: URL
    private let transport: any HTTPTransport
    private let token: TokenProvider

    public init(baseURL: URL, transport: any HTTPTransport, token: @escaping TokenProvider = { nil }) {
        self.baseURL = baseURL
        self.transport = transport
        self.token = token
    }

    /// `GET /_health/ready`: succeeds when the service at this address is up and ready.
    public func checkReady() async throws(APIError) {
        _ = try await send(Endpoints.ready(base: baseURL))
    }

    public func listPlans(parent: String? = nil) async throws(APIError) -> [PlanRef] {
        let token = try await bearer()
        return try decode([PlanRef].self, try await send(Endpoints.listPlans(base: baseURL, parent: parent, token: token)))
    }

    public func searchPlans(_ query: String) async throws(APIError) -> [PlanRef] {
        let token = try await bearer()
        return try decode(
            [PlanRef].self, try await send(Endpoints.searchPlans(base: baseURL, query: query, token: token)))
    }

    public func plan(_ pid: String) async throws(APIError) -> Plan {
        let token = try await bearer()
        return try decode(Plan.self, try await send(Endpoints.plan(base: baseURL, pid: pid, token: token)))
    }

    public func tasks(planPID: String) async throws(APIError) -> TaskList {
        let token = try await bearer()
        return try decode(TaskList.self, try await send(Endpoints.tasks(base: baseURL, planPID: planPID, token: token)))
    }

    public func moveTask(planPID: String, taskPID: String, to status: TaskStatus) async throws(APIError) -> PlanTask {
        let token = try await bearer()
        let request = Endpoints.moveTask(base: baseURL, planPID: planPID, taskPID: taskPID, to: status, token: token)
        return try decode(PlanTask.self, try await send(request))
    }

    public func createPlan(_ plan: NewPlan) async throws(APIError) -> PlanRef {
        let token = try await bearer()
        return try decode(PlanRef.self, try await send(Endpoints.createPlan(base: baseURL, plan: plan, token: token)))
    }

    private func bearer() async throws(APIError) -> String? {
        do { return try await token() } catch { throw .unauthorized }
    }

    private func send(_ request: HTTPRequest) async throws(APIError) -> HTTPResponse {
        let response: HTTPResponse
        do { response = try await transport.send(request) } catch { throw APIError.from(transportError: error) }
        if let error = APIError.from(response) { throw error }
        return response
    }

    private func decode<T: Decodable>(_ type: T.Type, _ response: HTTPResponse) throws(APIError) -> T {
        do { return try JSONDecoder().decode(type, from: response.body) } catch { throw .unexpected(status: response.status, detail: nil) }
    }
}
