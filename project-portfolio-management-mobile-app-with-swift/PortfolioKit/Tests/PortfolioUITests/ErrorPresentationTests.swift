// MA-6: a test for each row of MA-3, as the screens show it.

import Foundation
import PortfolioAPI
import PortfolioAuth
import PortfolioStub
@testable import PortfolioUI
import Testing

@MainActor
func readyApp(_ transport: any HTTPTransport, url: URL = AppEnvironment.stubURL) -> AppModel {
    let store = InMemoryStore()
    try? StoredValue<ServerSettings>(store: store, key: AppModel.settingsKey)
        .save(ServerSettings(serviceURL: url, oidc: nil, withoutSignIn: true))
    return AppModel(environment: AppEnvironment(transport: transport, store: store))
}

@MainActor @Suite struct ErrorPresentationTests {
    @Test func row401SignsOutAndAsksToSignInAgain() async {
        let app = readyApp(ScriptedTransport(status: 401, json: "{}"))
        #expect(app.phase == .ready)
        let model = PlansViewModel(app: app)
        await model.load()
        #expect(app.phase == .signIn)
        #expect(app.signInNotice == Copy.signInRequired.text)
        #expect(ErrorPresentation.of(.unauthorized, subject: .plan) == nil)
    }

    @Test func row401AfterSigningInSaysTheSessionEnded() throws {
        let store = InMemoryStore()
        let oidc = OIDCSettings(issuer: "https://id.example.org", clientID: "c")
        try StoredValue<ServerSettings>(store: store, key: AppModel.settingsKey)
            .save(ServerSettings(serviceURL: URL(string: "https://p.example.org")!, oidc: oidc))
        try StoredValue<TokenSet>(store: store, key: AuthSession.tokensKey).save(TokenSet(accessToken: "a"))
        let app = AppModel(environment: AppEnvironment(transport: ScriptedTransport(status: 401, json: "{}"), store: store))
        #expect(app.phase == .ready)
        #expect(app.handle(.unauthorized))
        #expect(app.phase == .signIn)
        #expect(app.signInNotice == Copy.signInSessionEnded.text)
        #expect(try store.read(AuthSession.tokensKey) == nil)
    }

    @Test func row403ShowsTheRule() throws {
        let shown = try #require(ErrorPresentation.of(.forbidden(rule: "rule 3: access=write"), subject: .create))
        #expect(shown.message == "You are not allowed to do this.\nThe service's rule: rule 3: access=write")
        #expect(!shown.canRetry)
        #expect(ErrorPresentation.of(.forbidden(rule: nil), subject: .create)?.message == Copy.errorForbidden.text)
    }

    @Test func row404SaysThePlanOrTaskIsNotThere() {
        #expect(ErrorPresentation.of(.notFound, subject: .plan)?.message == Copy.errorNotFoundPlan.text)
        #expect(ErrorPresentation.of(.notFound, subject: .task)?.message == Copy.errorNotFoundTask.text)
    }

    @Test func row409OnCreate() async {
        let model = NewPlanViewModel(app: readyApp(StubTransport()))
        model.name = "Harbour redevelopment"
        #expect(await model.submit() == nil)
        #expect(model.failure?.message == "A plan like this already exists.")
        #expect(model.problems.isEmpty)
    }

    @Test func row422ShowsEveryProblemBesideItsField() async {
        let body = #"{"status":422,"detail":"name is required; start_date: must not be after target_date; the plan is locked"}"#
        let model = NewPlanViewModel(app: readyApp(ScriptedTransport(status: 422, json: body)))
        #expect(await model.submit() == nil)
        #expect(model.problems.count == 3)
        #expect(model.error(for: "name") == "name is required")
        #expect(model.error(for: "start_date") == "start_date: must not be after target_date")
        #expect(model.error(for: "code") == nil)
        #expect(model.failure == nil)
    }

    @Test func row422OnAMoveShowsTheServicesReason() async throws {
        let stub = StubTransport()
        let reason = "WIP limit reached: `done` is capped at 3 for this item"
        let transport = ScriptedTransport { request in
            if request.method == .patch {
                return HTTPResponse(status: 422, body: Data(#"{"status":422,"detail":"\#(reason)"}"#.utf8))
            }
            return try await stub.send(request)
        }
        let model = PlanViewModel(ref: PlanRef(pid: StubFixtures.harbourPID, name: "Harbour"), app: readyApp(transport))
        await model.load()
        guard case .loaded(let content) = model.state else { Issue.record("not loaded"); return }
        #expect(await model.move(content.tasks[0], to: .done) == false)
        #expect(model.moveError == ErrorPresentation(heading: Copy.moveFailed.text, message: reason, canRetry: false))
    }

    @Test func anyOtherAnswerNamesItsStatus() async {
        let model = PlansViewModel(app: readyApp(ScriptedTransport(status: 418, json: "{}")))
        await model.load()
        #expect(model.state == .failed(ErrorPresentation(
            heading: Copy.errorHeading.text, message: "The service gave an answer this app did not expect (HTTP 418).",
            canRetry: true)))
        let unreadable = PlansViewModel(app: readyApp(ScriptedTransport(status: 200, json: #"{"not":"a list"}"#)))
        await unreadable.load()
        guard case .failed(let shown) = unreadable.state else { Issue.record("not failed"); return }
        #expect(shown.message.contains("(HTTP 200)"))
    }

    @Test(arguments: [APIError.unreachable(.offline), .unreachable(.timeout), .unreachable(.server(status: 503))])
    func rowUnreachableOffersTryAgain(error: APIError) throws {
        let shown = try #require(ErrorPresentation.of(error, subject: .list))
        #expect(shown.heading == "The service could not be reached.")
        #expect(shown.canRetry)
    }

    @Test func aServerErrorNeverShowsItsBody() async {
        let model = PlansViewModel(app: readyApp(ScriptedTransport(status: 500, json: "thread 'main' panicked at src/lib.rs")))
        await model.load()
        guard case .failed(let shown) = model.state else { Issue.record("not failed"); return }
        #expect(!shown.message.contains("panicked"))
        #expect(shown.canRetry)
    }
}
