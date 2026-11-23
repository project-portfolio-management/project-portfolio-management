// MA-6: one test for each row of MA-3, as the client reports it. (What each screen then shows is in PortfolioUITests.)

import Foundation
import PortfolioAPI
import PortfolioStub
import Testing

@Suite struct ErrorMappingTests {
    func failure(status: Int, body: String = "") async -> APIError? {
        let client = PortfolioClient(baseURL: base, transport: ScriptedTransport(status: status, json: body))
        do {
            _ = try await client.listPlans()
            return nil
        } catch {
            return error
        }
    }

    @Test func row401() async {
        #expect(await failure(status: 401, body: #"{"title":"Unauthorized","status":401,"code":"unauthorized"}"#)
            == .unauthorized)
    }

    @Test func row403NamesTheRule() async {
        let body = #"{"title":"Forbidden","status":403,"code":"forbidden","detail":"rule 3: access=write needed"}"#
        #expect(await failure(status: 403, body: body) == .forbidden(rule: "rule 3: access=write needed"))
        #expect(await failure(status: 403) == .forbidden(rule: nil))
    }

    @Test func row404() async {
        #expect(await failure(status: 404, body: #"{"detail":"Resource was not found","code":"not_found"}"#) == .notFound)
    }

    @Test func row409() async {
        #expect(await failure(status: 409, body: #"{"code":"duplicate","detail":"That name is taken"}"#)
            == .conflict(detail: "That name is taken"))
    }

    @Test func row422ListsEveryProblemWithItsField() async {
        let body = #"""
            {"status":422,"code":"validation",
             "detail":"name is required; goals[0]: title must not be blank; parent_ref: a plan cannot contain itself; the WIP limit is reached"}
            """#
        #expect(
            await failure(status: 422, body: body)
                == .validation([
                    Problem(field: "name", message: "name is required"),
                    Problem(field: "goals", message: "goals[0]: title must not be blank"),
                    Problem(field: "parent_ref", message: "parent_ref: a plan cannot contain itself"),
                    Problem(field: nil, message: "the WIP limit is reached"),
                ]))
    }

    @Test func row422ReadsTheErrorsMember() async {
        let body = #"{"status":422,"errors":{"name":["is required"],"code":["is too long","has a space"]}}"#
        #expect(
            await failure(status: 422, body: body)
                == .validation([
                    Problem(field: "code", message: "code is too long"), Problem(field: "code", message: "code has a space"),
                    Problem(field: "name", message: "name is required"),
                ]))
    }

    @Test func aWordThatOnlyStartsLikeAFieldNamesNone() async {
        #expect(await failure(status: 422, body: #"{"detail":"names must differ"}"#)
            == .validation([Problem(field: nil, message: "names must differ")]))
    }

    @Test(arguments: [500, 502, 503, 504])
    func row5xx(status: Int) async {
        #expect(await failure(status: status, body: "<html>stack trace</html>") == .unreachable(.server(status: status)))
    }

    @Test func rowNoNetwork() async {
        let client = PortfolioClient(baseURL: base, transport: ScriptedTransport { _ in throw URLError(.notConnectedToInternet) })
        await #expect(throws: APIError.unreachable(.offline)) { try await client.listPlans() }
    }

    @Test func rowTimeout() async {
        let client = PortfolioClient(baseURL: base, transport: ScriptedTransport { _ in throw URLError(.timedOut) })
        await #expect(throws: APIError.unreachable(.timeout)) { try await client.listPlans() }
    }

    @Test func anotherStatusIsUnexpected() async {
        #expect(await failure(status: 400, body: #"{"detail":"query parameter `q` is required"}"#)
            == .unexpected(status: 400, detail: "query parameter `q` is required"))
    }
}

@Suite struct ServerAddressTests {
    @Test(arguments: [
        "https://portfolio.example.org", "http://localhost:5150", "http://127.0.0.1:5150", "http://[::1]:5150",
        "http://127.10.0.1", "https://10.0.0.5",
    ])
    func allowed(_ text: String) throws {
        #expect(try ServerAddress.parse(text).get().absoluteString == text)
    }

    @Test(arguments: ["http://portfolio.example.org", "http://10.0.2.2:5150", "http://192.168.1.4", "http://127.0.0.1.evil.org"])
    func needsHTTPS(_ text: String) {
        #expect(throws: ServerAddress.Problem.needsHTTPS) { try ServerAddress.parse(text).get() }
    }

    @Test(arguments: ["", "portfolio.example.org", "ftp://example.org", "https://"])
    func notAURL(_ text: String) {
        #expect(throws: ServerAddress.Problem.notAURL) { try ServerAddress.parse(text).get() }
    }

    @Test func developmentIsLoopbackOnly() {
        #expect(ServerAddress.isDevelopment(URL(string: "http://localhost")!))
        #expect(ServerAddress.isDevelopment(URL(string: "https://127.0.0.1")!))
        #expect(!ServerAddress.isDevelopment(URL(string: "http://10.0.2.2")!))
        #expect(!ServerAddress.isDevelopment(URL(string: "https://example.org")!))
    }
}
