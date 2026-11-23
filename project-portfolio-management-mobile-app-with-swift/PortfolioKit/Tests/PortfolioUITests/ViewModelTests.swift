// The screens' behaviour against the stub service: the list and search, the plan, a move (and a failed one), a create, and
// the server and sign-in rules.

import Foundation
import PortfolioAPI
import PortfolioAuth
import PortfolioStub
@testable import PortfolioUI
import Testing

@MainActor @Suite struct ViewModelTests {
    @Test func theListAndTheSearch() async {
        let model = PlansViewModel(app: readyApp(StubTransport()))
        await model.load()
        #expect(model.state == .loaded([
            PlanRef(pid: StubFixtures.harbourPID, name: "Harbour redevelopment"),
            PlanRef(pid: StubFixtures.libraryPID, name: "Library digitisation"),
        ]))
        model.query = "  quay "
        await model.load()
        #expect(model.state == .loaded([PlanRef(pid: StubFixtures.quayPID, name: "Quay wall survey")]))
        #expect(model.answeredQuery == "quay")
    }

    @Test func thePlanItsChildrenAndItsTasks() async throws {
        let model = PlanViewModel(ref: PlanRef(pid: StubFixtures.harbourPID, name: "Harbour"), app: readyApp(StubTransport()))
        await model.load()
        guard case .loaded(let content) = model.state else { Issue.record("not loaded"); return }
        #expect(content.plan.code == "HRB-2026")
        #expect(content.children.map(\.name) == ["Quay wall survey"])
        #expect(content.tasks.map(\.status) == [.todo, .inProgress, .inReview, .blocked, .done])
    }

    @Test func aMoveChangesTheStatusTheServiceAnswers() async throws {
        let model = PlanViewModel(ref: PlanRef(pid: StubFixtures.harbourPID, name: "Harbour"), app: readyApp(StubTransport()))
        await model.load()
        guard case .loaded(let content) = model.state else { Issue.record("not loaded"); return }
        #expect(await model.move(content.tasks[0], to: .inReview))
        guard case .loaded(let after) = model.state else { Issue.record("not loaded"); return }
        #expect(after.tasks[0].status == .inReview)
        #expect(model.moveError == nil)
    }

    @Test(arguments: [
        (422, #"{"detail":"WIP limit reached: `in_review` is capped at 2 for this item"}"#),
        (404, #"{"detail":"Resource was not found"}"#), (503, ""), (403, #"{"detail":"rule 1"}"#),
    ])
    func aFailedMoveLeavesTheTaskAsItWas(status: Int, body: String) async throws {
        let stub = StubTransport()
        let transport = ScriptedTransport { request in
            if request.method == .patch { return HTTPResponse(status: status, body: Data(body.utf8)) }
            return try await stub.send(request)
        }
        let model = PlanViewModel(ref: PlanRef(pid: StubFixtures.harbourPID, name: "Harbour"), app: readyApp(transport))
        await model.load()
        guard case .loaded(let before) = model.state else { Issue.record("not loaded"); return }
        #expect(await model.move(before.tasks[0], to: .done) == false)
        #expect(model.state == .loaded(before))
        #expect(model.moveError != nil)
        #expect(model.moving == nil)
    }

    @Test func aMoveWithNoNetworkLeavesTheTaskAsItWas() async {
        let stub = StubTransport()
        let transport = ScriptedTransport { request in
            if request.method == .patch { throw URLError(.notConnectedToInternet) }
            return try await stub.send(request)
        }
        let model = PlanViewModel(ref: PlanRef(pid: StubFixtures.harbourPID, name: "Harbour"), app: readyApp(transport))
        await model.load()
        guard case .loaded(let before) = model.state else { Issue.record("not loaded"); return }
        #expect(await model.move(before.tasks[1], to: .done) == false)
        #expect(model.state == .loaded(before))
        #expect(model.moveError?.canRetry == true)
    }

    @Test func aCreateSendsTheFormAndAnswersTheRef() async throws {
        let transport = ScriptedTransport(status: 201, json: #"{"pid":"p-9","name":"Ferry"}"#)
        let model = NewPlanViewModel(app: readyApp(transport))
        model.name = "  Ferry "
        model.kind = .project
        model.status = .proposed
        model.code = " "
        model.hasTarget = true
        var calendar = Calendar(identifier: .gregorian)
        calendar.timeZone = TimeZone(identifier: "Europe/London")!
        model.target = calendar.date(from: DateComponents(year: 2027, month: 6, day: 30, hour: 12))!
        #expect(model.body(calendar: calendar) == NewPlan(
            name: "Ferry", kind: .project, status: .proposed, targetDate: CalendarDate(iso: "2027-06-30")))
        #expect(await model.submit() == PlanRef(pid: "p-9", name: "Ferry"))
        let sent = try #require(transport.requests.first?.body)
        let body = try #require(JSONSerialization.jsonObject(with: sent) as? [String: Any])
        #expect(Set(body.keys) == ["name", "kind", "status", "target_date"])
    }

    @Test func theServerRules() {
        let app = AppModel(environment: AppEnvironment(transport: StubTransport(), store: InMemoryStore()))
        #expect(app.phase == .server)
        let model = ServerViewModel(app: app)
        model.serviceURL = "http://portfolio.example.org"
        #expect(model.validated() == nil)
        #expect(model.errors["serviceURL"] == Copy.serverErrorNeedsHTTPS.text)
        #expect(model.errors["issuer"] == nil)

        model.serviceURL = "https://portfolio.example.org"
        #expect(model.validated() == nil)
        #expect(model.errors["issuer"] == Copy.serverErrorIssuerRequired.text)

        model.issuer = "https://id.example.org/realms/ppm"
        #expect(model.validated() == nil)
        #expect(model.errors["clientID"] == Copy.serverErrorClientID.text)

        model.clientID = "ppm-mobile"
        #expect(model.save())
        #expect(app.phase == .signIn)
        #expect(app.offersWithoutSignIn == false)
        #expect(app.settings?.oidc?.redirectURI == OIDCSettings.defaultRedirectURI)
    }

    @Test func continueWithoutSigningInIsOnlyForThisDevice() {
        let app = AppModel(environment: AppEnvironment(transport: StubTransport(), store: InMemoryStore()))
        app.save(ServerSettings(serviceURL: URL(string: "https://portfolio.example.org")!, oidc: nil))
        #expect(!app.offersWithoutSignIn)
        app.continueWithoutSigningIn()
        #expect(app.phase == .signIn)

        app.save(ServerSettings(serviceURL: URL(string: "http://127.0.0.1:5150")!, oidc: nil))
        #expect(app.offersWithoutSignIn)
        app.continueWithoutSigningIn()
        #expect(app.phase == .ready)
        #expect(app.client != nil)
    }

    @Test func theReadinessCheck() async {
        let app = AppModel(environment: AppEnvironment(transport: StubTransport(), store: InMemoryStore()))
        let model = ServerViewModel(app: app)
        model.serviceURL = "http://127.0.0.1:5150"
        await model.checkAddress()
        #expect(model.check == .ready)

        let down = ServerViewModel(
            app: AppModel(environment: AppEnvironment(transport: ScriptedTransport(status: 503, json: ""), store: InMemoryStore())))
        down.serviceURL = "https://portfolio.example.org"
        await down.checkAddress()
        #expect(down.check == .failed(ServerViewModel.unreachable))
    }

    @Test func signingInThroughTheBrowser() async throws {
        let provider = ScriptedTransport { request in
            if request.url.path().hasSuffix("openid-configuration") {
                return HTTPResponse(status: 200, body: Data(#"""
                    {"issuer":"https://id.example.org","authorization_endpoint":"https://id.example.org/auth",
                     "token_endpoint":"https://id.example.org/token"}
                    """#.utf8))
            }
            if request.url.path() == "/token" {
                return HTTPResponse(status: 200, body: Data(#"{"access_token":"tok","expires_in":300}"#.utf8))
            }
            return HTTPResponse(status: 200, body: Data("[]".utf8))
        }
        let app = AppModel(environment: AppEnvironment(transport: provider, store: InMemoryStore()))
        app.save(ServerSettings(
            serviceURL: URL(string: "https://p.example.org")!,
            oidc: OIDCSettings(issuer: "https://id.example.org", clientID: "c")))
        await app.signIn { url, scheme in
            #expect(scheme == "io.github.projectportfoliomanagement.mobile")
            let state = URLComponents(url: url, resolvingAgainstBaseURL: false)?.queryItems?.first { $0.name == "state" }?.value
            return URL(string: "io.github.projectportfoliomanagement.mobile:/oauth2redirect?code=c&state=\(state ?? "")")!
        }
        #expect(app.signInError == nil)
        #expect(app.phase == .ready)
        _ = try await app.client?.listPlans()
        #expect(provider.requests.last?.headers["Authorization"] == "Bearer tok")

        await app.signOut()
        #expect(app.phase == .signIn)
        #expect(app.signInNotice == Copy.signInSignedOut.text)
    }

    @Test func aCancelledSignInSaysSo() async {
        let provider = ScriptedTransport { _ in
            HTTPResponse(status: 200, body: Data(#"""
                {"issuer":"https://id.example.org","authorization_endpoint":"https://id.example.org/auth",
                 "token_endpoint":"https://id.example.org/token"}
                """#.utf8))
        }
        struct Closed: Error {}
        let app = AppModel(environment: AppEnvironment(transport: provider, store: InMemoryStore()))
        app.save(ServerSettings(
            serviceURL: URL(string: "https://p.example.org")!, oidc: OIDCSettings(issuer: "https://id.example.org", clientID: "c")))
        await app.signIn { _, _ in throw Closed() }
        #expect(app.phase == .signIn)
        #expect(app.signInError == Copy.signInErrorCancelled.text)
    }
}
