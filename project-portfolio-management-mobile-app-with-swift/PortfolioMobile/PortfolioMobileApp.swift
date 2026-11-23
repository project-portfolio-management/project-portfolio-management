// The iOS and iPadOS app: a shell over PortfolioUI (PortfolioKit/). In a Debug build the launch argument `-stub` runs it
// against the in-memory stub service instead of the network, as the UI tests do; with it, `-stub-server-screen` starts at
// the server screen and `-stub-sign-in-screen` at the sign-in screen instead of the plans. A Release build always uses the
// network, the Keychain and user defaults.

import PortfolioUI
import SwiftUI

@main
struct PortfolioMobileApp: App {
    private let environment: AppEnvironment = {
        #if DEBUG
            let arguments = ProcessInfo.processInfo.arguments
            if arguments.contains("-stub") {
                if arguments.contains("-stub-server-screen") { return .stub(start: .server) }
                if arguments.contains("-stub-sign-in-screen") { return .stub(start: .signIn) }
                return .stub(start: .plans)
            }
        #endif
        return .live()
    }()

    var body: some Scene {
        WindowGroup { RootView(environment: environment) }
    }
}
