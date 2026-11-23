// MA-1.2: sign in through the system browser (`ASWebAuthenticationSession`, through SwiftUI's `webAuthenticationSession`),
// never a password field of the app's own (spec/mobile-apps.md MA-2). **Continue without signing in** is offered only for a
// development address.

import AuthenticationServices
import LilyUI
import SwiftUI

struct SignInScreen: View {
    let app: AppModel
    @Environment(\.webAuthenticationSession) private var webAuthenticationSession

    var body: some View {
        // Two whole forms rather than an `if` inside one: the accessibility audit (iOS 27.2) reports the Dynamic Type of a
        // Form row inside a taken `if` branch, or a ForEach of one, as unsupported, but not a row of a form chosen whole.
        Group {
            if app.offersWithoutSignIn {
                Form {
                    common
                    withoutSignIn
                    changeServer
                }
            } else {
                Form {
                    common
                    changeServer
                }
            }
        }
        .navigationTitle(Copy.signInTitle.text)
    }

    @ViewBuilder private var common: some View {
        if let notice = app.signInNotice {
            Section { LilyAlert(kind: .info, message: notice, politeness: .polite) }
        }
        Section {
            InsetText { Text(verbatim: Copy.signInIntro.text) }
            Text(verbatim: Copy.signInServer(app.settings?.serviceURL.absoluteString ?? ""))
                .font(.footnote)
        }
        if let error = app.signInError {
            Section { LilyAlert(kind: .error, heading: Copy.signInErrorHeading.text, message: error) }
        }
        Section {
            Button(Copy.signInButton.text) {
                Task {
                    await app.signIn { url, scheme in
                        try await webAuthenticationSession.authenticate(
                            using: url, callbackURLScheme: scheme, preferredBrowserSession: .ephemeral)
                    }
                }
            }
            .buttonStyle(.borderedProminent)
            .frame(minHeight: 44)
            .disabled(app.signingIn)
            .accessibilityIdentifier("sign-in")
            if app.signingIn { LoadingView(label: Copy.signInSigningIn.text) }
        }
    }

    private var withoutSignIn: some View {
        Section {
            // The hint is part of the button, not a row of its own: as a row after the button the audit reported its
            // Dynamic Type as unsupported (iOS 27.2).
            Button { app.continueWithoutSigningIn() } label: {
                VStack(alignment: .leading, spacing: 4) {
                    Text(verbatim: Copy.signInWithout.text)
                    Text(verbatim: Copy.signInWithoutHint.text).font(.footnote).foregroundStyle(.primary)
                }
                .frame(maxWidth: .infinity, minHeight: 44, alignment: .leading)
            }
            .accessibilityIdentifier("continue-without-sign-in")
        }
    }

    private var changeServer: some View {
        Section {
            Button(Copy.signInChangeServer.text) { app.changeServer() }
                .frame(minHeight: 44)
                .accessibilityIdentifier("change-server")
        }
    }
}
