// MA-1.1: the service's address and the sign-in settings, with a readiness check (`GET /_health/ready`).

import LilyUI
import PortfolioAuth
import SwiftUI

struct ServerScreen: View {
    let app: AppModel
    @State private var model: ServerViewModel
    @State private var showSummary = false
    @FocusState private var focused: String?

    init(app: AppModel) {
        self.app = app
        _model = State(initialValue: ServerViewModel(app: app))
    }

    var body: some View {
        ScrollViewReader { scroller in
        Form {
            Section {
                InsetText { Text(verbatim: Copy.serverIntro.text) }
            }
            if showSummary && !model.orderedErrors.isEmpty {
                Section {
                    ErrorSummary(
                        title: Copy.errorSummaryTitle.text,
                        problems: model.orderedErrors.map { item in .init(item.message) { focused = item.field } })
                }
                .id("summary")
            }
            Section {
                Field(
                    label: Copy.serverServiceURL.text, description: Copy.serverServiceURLHint.text,
                    error: model.errors["serviceURL"], required: true, requiredLabel: Copy.fieldRequired.text
                ) {
                    TextField("", text: $model.serviceURL, axis: .vertical)
                        .urlEntry().fieldBox()
                        .focused($focused, equals: "serviceURL")
                        .accessibilityIdentifier("server-service-url")
                }
                Button(Copy.serverCheck.text) { Task { await model.checkAddress() } }
                    .frame(minHeight: 44)
                    .disabled(model.check == .checking)
                    .accessibilityIdentifier("server-check")
                switch model.check {
                case .idle: EmptyView()
                case .checking: LoadingView(label: Copy.serverChecking.text)
                case .ready: LilyAlert(kind: .success, message: Copy.serverReady.text, politeness: .polite)
                case .failed(let presentation): FailureView(presentation: presentation)
                }
            }
            Section {
                Field(
                    label: Copy.serverIssuer.text, description: Copy.serverIssuerHint.text, error: model.errors["issuer"]
                ) {
                    TextField("", text: $model.issuer, axis: .vertical)
                        .urlEntry().fieldBox()
                        .focused($focused, equals: "issuer")
                        .accessibilityIdentifier("server-issuer")
                }
                Field(label: Copy.serverClientID.text, error: model.errors["clientID"]) {
                    TextField("", text: $model.clientID, axis: .vertical)
                        .plainEntry().fieldBox()
                        .focused($focused, equals: "clientID")
                        .accessibilityIdentifier("server-client-id")
                }
                Field(
                    label: Copy.serverRedirectURI.text, description: Copy.serverRedirectURIHint.text,
                    error: model.errors["redirectURI"]
                ) {
                    TextField("", text: $model.redirectURI, axis: .vertical)
                        .urlEntry().fieldBox()
                        .focused($focused, equals: "redirectURI")
                        .accessibilityIdentifier("server-redirect-uri")
                }
            }
            Section {
                Button(Copy.serverSave.text) {
                    showSummary = !model.save()
                    // After the summary has been drawn, bring it into view.
                    if showSummary { Task { @MainActor in withAnimation { scroller.scrollTo("summary", anchor: .top) } } }
                }
                .buttonStyle(.borderedProminent)
                .frame(minHeight: 44)
                .accessibilityIdentifier("server-save")
            }
        }
        }
        .navigationTitle(Copy.serverTitle.text)
    }
}
