// MA-1.3: the active plans, with a search box (`GET /api/plans`, `GET /api/plans/search?q=`), and the way to a new plan and
// to sign out.

import AuthenticationServices
import LilyUI
import PortfolioAPI
import SwiftUI

enum Route: Hashable {
    case plan(PlanRef)
}

struct PlansScreen: View {
    let app: AppModel
    @State private var model: PlansViewModel
    @State private var path: [Route] = []
    @State private var creating = false
    /// A plan just created, opened once the sheet has gone (a push during the sheet's dismissal can be lost).
    @State private var created: PlanRef?
    @Environment(\.webAuthenticationSession) private var webAuthenticationSession

    init(app: AppModel) {
        self.app = app
        _model = State(initialValue: PlansViewModel(app: app))
    }

    var body: some View {
        NavigationStack(path: $path) {
            List {
                switch model.state {
                case .loading:
                    LoadingView()
                case .failed(let presentation):
                    FailureView(presentation: presentation) { await model.load() }
                case .loaded(let plans):
                    if plans.isEmpty {
                        Text(
                            verbatim: model.answeredQuery.isEmpty
                                ? Copy.plansEmpty.text : Copy.plansNoResults(model.answeredQuery))
                    }
                    ForEach(plans) { plan in
                        NavigationLink(value: Route.plan(plan)) {
                            Text(verbatim: plan.name).frame(minHeight: 44, alignment: .leading)
                        }
                        .accessibilityIdentifier("plan-row-\(plan.pid)")
                    }
                }
            }
            .accessibilityIdentifier("plans-list")
            .navigationTitle(Copy.plansTitle.text)
            .searchable(text: $model.query, prompt: Text(verbatim: Copy.plansSearch.text))
            .task(id: model.query) {
                // A short pause, so a search is not sent for every letter typed.
                if !model.query.isEmpty { try? await Task.sleep(for: .milliseconds(300)) }
                guard !Task.isCancelled else { return }
                await model.load()
            }
            .refreshable { await model.load() }
            .toolbar {
                ToolbarItem(placement: .primaryAction) {
                    Button { creating = true } label: {
                        Label { Text(verbatim: Copy.plansNew.text) } icon: { Image(systemName: "plus") }
                    }
                    .accessibilityIdentifier("new-plan")
                }
                ToolbarItem(placement: .cancellationAction) {
                    Menu {
                        Button(Copy.plansServer.text) { app.changeServer() }
                        Button(Copy.plansSignOut.text) {
                            Task {
                                await app.signOut { url in
                                    guard let scheme = app.settings?.oidc?.callbackScheme else { return }
                                    _ = try? await webAuthenticationSession.authenticate(
                                        using: url, callbackURLScheme: scheme, preferredBrowserSession: .ephemeral)
                                }
                            }
                        }
                        .accessibilityIdentifier("sign-out")
                    } label: {
                        Label { Text(verbatim: Copy.plansAccount.text) } icon: { Image(systemName: "person.crop.circle") }
                    }
                    .accessibilityIdentifier("account-menu")
                }
            }
            .navigationDestination(for: Route.self) { route in
                switch route {
                case .plan(let ref): PlanScreen(ref: ref, app: app)
                }
            }
        }
        .sheet(isPresented: $creating, onDismiss: openCreated) {
            NewPlanScreen(app: app) { ref in
                created = ref
                creating = false
            }
        }
    }

    private func openCreated() {
        guard let ref = created else { return }
        created = nil
        path.append(.plan(ref))
        Task { await model.load() }
    }
}
