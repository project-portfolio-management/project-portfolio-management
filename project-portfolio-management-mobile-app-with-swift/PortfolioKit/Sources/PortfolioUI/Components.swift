// Pieces the screens share: the failed-call block (Lily's alert with a native retry button), the loading row, and the few
// modifiers that differ between iOS and the macOS build the unit tests use.

import LilyUI
import SwiftUI

/// A failed call: Lily's error alert, and **Try again** when trying again can help (MA-3). Never a stack trace.
struct FailureView: View {
    let presentation: ErrorPresentation
    var retry: (() async -> Void)?

    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            LilyAlert(kind: .error, heading: presentation.heading, message: presentation.message)
            if presentation.canRetry, let retry {
                Button(Copy.retry.text) { Task { await retry() } }
                    .buttonStyle(.borderedProminent)
                    .frame(minHeight: 44)
                    .accessibilityIdentifier("retry")
            }
        }
        .frame(maxWidth: .infinity, alignment: .leading)
    }
}

/// A spinner with its words, so VoiceOver says what is happening.
struct LoadingView: View {
    var label: String = Copy.loading.text

    var body: some View {
        ProgressView { Text(verbatim: label) }
            .frame(maxWidth: .infinity, minHeight: 44)
    }
}

extension View {
    /// An address field: no capitalisation or correction, and the URL keyboard on iOS.
    func urlEntry() -> some View {
        #if os(iOS)
            self.keyboardType(.URL).textInputAutocapitalization(.never).autocorrectionDisabled()
        #else
            self.autocorrectionDisabled()
        #endif
    }

    /// Plain identifier entry (a client id, a code).
    func plainEntry() -> some View {
        #if os(iOS)
            self.textInputAutocapitalization(.never).autocorrectionDisabled()
        #else
            self.autocorrectionDisabled()
        #endif
    }

    /// The bordered look for a text field inside a Lily `Field`.
    func fieldBox() -> some View {
        self.textFieldStyle(.roundedBorder).frame(minHeight: 44)
    }

    func inlineTitle() -> some View {
        #if os(iOS)
            self.navigationBarTitleDisplayMode(.inline)
        #else
            self
        #endif
    }
}
