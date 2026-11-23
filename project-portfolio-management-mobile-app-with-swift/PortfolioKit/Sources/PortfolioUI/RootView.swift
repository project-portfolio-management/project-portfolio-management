// The app's root: the server screen, the sign-in screen or the plans, themed with Lily's light or dark theme as the system
// is (spec/mobile-apps.md MA-4).

import LilyThemes
import SwiftUI

public struct RootView: View {
    @State private var app: AppModel
    @Environment(\.colorScheme) private var colorScheme

    public init(environment: AppEnvironment) {
        _app = State(initialValue: AppModel(environment: environment))
    }

    public var body: some View {
        Group {
            switch app.phase {
            case .server: NavigationStack { ServerScreen(app: app) }
            case .signIn: NavigationStack { SignInScreen(app: app) }
            case .ready: PlansScreen(app: app)
            }
        }
        .lilyTheme(colorScheme == .dark ? .dark : .light)
    }
}
